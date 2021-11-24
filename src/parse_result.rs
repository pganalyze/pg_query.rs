use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::string::String;

use itertools::join;

use crate::*;

macro_rules! cast {
    ($target: expr, $pat: path) => {
        {
            if let $pat(a) = $target { // #1
                a
            } else {
                panic!(
                    "mismatch variant when cast to {}", 
                    stringify!($pat)); // #2
            }
        }
    };
}

impl protobuf::ParseResult {
    pub fn deparse(&self) -> Result<String> {
        crate::deparse(self)
    }

    // Note: this doesn't iterate over every possible node type, since we only care about a subset of nodes.
    pub fn nodes(&self) -> Vec<(NodeRef, i32, Context)> {
        self.stmts.iter().filter_map(|s| {
            if let Some(node) = s.stmt.as_ref() {        // RawStmt  -> Node
                if let Some(node) = node.node.as_ref() { // Node     -> NodeEnum
                    return Some(node.nodes())            // NodeEnum -> NodeRef
                }
            }
            None
        }).flatten().collect()
    }

    // Converts an immutable reference to a mutable reference.
    pub fn find_mut(&mut self, to_find: NodeRef) -> NodeMut {
        let s = format!("{:?}", self);
        for s in self.stmts.iter_mut() {
            if let Some(node) = s.stmt.as_mut() {                    // RawStmt  -> Node
                if let Some(node) = node.node.as_mut() {             // Node     -> NodeEnum
                    if let Some(mut_ref) = node.find_mut(&to_find) { // NodeEnum -> NodeMut
                        return mut_ref
                    }
                }
            }
        }
        panic!("unable to find node {:?} in {:?}", to_find, s);
    }
}

pub struct ParseResult {
    pub protobuf: protobuf::ParseResult,
    pub warnings: Vec<String>,
    tables: Vec<(String, Context)>,
    pub aliases: HashMap<String, String>,
    pub cte_names: Vec<String>,
    functions: Vec<(String, Context)>,
}

impl ParseResult {
    pub fn new(protobuf: protobuf::ParseResult, stderr: String) -> Self {
        let warnings = stderr.lines().filter_map(|l| 
            if l.starts_with("WARNING") { Some(l.trim().into()) } else { None }
        ).collect();
        let mut tables: HashSet<(String, Context)> = HashSet::new();
        let mut aliases: HashMap<String, String> = HashMap::new();
        let mut cte_names: HashSet<String> = HashSet::new();
        let mut functions: HashSet<(String, Context)> = HashSet::new();

        for (node, _depth, context) in protobuf.nodes().into_iter() {
            match node {
                NodeRef::CommonTableExpr(s) => {
                    cte_names.insert(s.ctename.to_owned());
                }
                NodeRef::RangeVar(v) => {
                    // TODO: this incorrectly returns no tables: parse('with f as (select * from f limit 1) select * from f')
                    let table = if v.schemaname.len() > 0 {
                        format!("{}.{}", v.schemaname, v.relname)
                    } else {
                        v.relname.to_owned()
                    };
                    if cte_names.contains(&table) { continue }
                    tables.insert((table.to_owned(), context));
                    v.alias.as_ref().and_then(|alias| aliases.insert(alias.aliasname.to_owned(), table));
                }
                NodeRef::FuncCall(c) => {
                    let funcname = join(c.funcname.iter().filter_map(|n| {
                        n.node.as_ref().and_then(|n| Some(&cast!(n, NodeEnum::String).str))
                    }), ".");
                    functions.insert((funcname, Context::Call));
                }
                NodeRef::DropStmt(s) => {
                    match protobuf::ObjectType::from_i32(s.remove_type) {
                        Some(protobuf::ObjectType::ObjectTable) => {
                            for o in &s.objects {
                                if let Some(NodeEnum::List(list)) = &o.node {
                                    let table = join(list.items.iter().filter_map(|i| {
                                        i.node.as_ref().and_then(|n| Some(&cast!(n, NodeEnum::String).str))
                                    }), ".");
                                    tables.insert((table, Context::DDL));
                                };
                            }
                        }
                        Some(protobuf::ObjectType::ObjectRule) | Some(protobuf::ObjectType::ObjectTrigger) => {
                            for o in &s.objects {
                                if let Some(NodeEnum::List(list)) = &o.node {
                                    // Unlike ObjectTable, this ignores the last string (the rule/trigger name)
                                    let table = join(list.items[0..list.items.len() - 1].iter().filter_map(|i| {
                                        i.node.as_ref().and_then(|n| Some(&cast!(n, NodeEnum::String).str))
                                    }), ".");
                                    tables.insert((table, Context::DDL));
                                };
                            }
                        }
                        Some(protobuf::ObjectType::ObjectFunction) => {
                            // Only one function can be dropped in a statement
                            if let Some(NodeEnum::ObjectWithArgs(object)) = &s.objects[0].node {
                                if let Some(NodeEnum::String(string)) = &object.objname[0].node {
                                    functions.insert((string.str.to_string(), Context::DDL));
                                }
                            }
                        }
                        _ => ()
                    }
                }
                NodeRef::CreateFunctionStmt(s) => {
                    if let Some(NodeEnum::String(string)) = &s.funcname[0].node {
                        functions.insert((string.str.to_string(), Context::DDL));
                    }
                }
                NodeRef::RenameStmt(s) => {
                    match protobuf::ObjectType::from_i32(s.rename_type) {
                        Some(protobuf::ObjectType::ObjectFunction) => {
                            if let Some(object) = &s.object {
                                if let Some(NodeEnum::ObjectWithArgs(object)) = &object.node {
                                    if let Some(NodeEnum::String(string)) = &object.objname[0].node {
                                        functions.insert((string.str.to_string(), Context::DDL));
                                        functions.insert((s.newname.to_string(), Context::DDL));
                                    }
                                }
                            }
                        }
                        _ => ()
                    }
                }
                _ => ()
            }
        }

        Self {
            protobuf: protobuf,
            warnings: warnings,
            tables: Vec::from_iter(tables),
            aliases: aliases,
            cte_names: Vec::from_iter(cte_names),
            functions: Vec::from_iter(functions),
        }
    }

    pub fn tables(&self) -> Vec<String> {
        let mut tables = HashSet::new();
        self.tables.iter().for_each(|(t, _c)| { tables.insert(t.to_string()); });
        Vec::from_iter(tables)
    }

    pub fn select_tables(&self) -> Vec<String> {
        self.tables.iter().filter_map(|(table, context)| {
            match context {
                Context::Select => Some(table.to_string()),
                _ => None
            }
        }).collect()
    }

    pub fn dml_tables(&self) -> Vec<String> {
        self.tables.iter().filter_map(|(table, context)| {
            match context {
                Context::DML => Some(table.to_string()),
                _ => None
            }
        }).collect()
    }

    pub fn ddl_tables(&self) -> Vec<String> {
        self.tables.iter().filter_map(|(table, context)| {
            match context {
                Context::DDL => Some(table.to_string()),
                _ => None
            }
        }).collect()
    }

    pub fn functions(&self) -> Vec<String> {
        let mut functions = HashSet::new();
        self.functions.iter().for_each(|(f, _c)| { functions.insert(f.to_string()); });
        Vec::from_iter(functions)
    }

    pub fn ddl_functions(&self) -> Vec<String> {
        self.functions.iter().filter_map(|(function, context)| {
            match context {
                Context::DDL => Some(function.to_string()),
                _ => None
            }
        }).collect()
    }

    pub fn call_functions(&self) -> Vec<String> {
        self.functions.iter().filter_map(|(function, context)| {
            match context {
                Context::Call => Some(function.to_string()),
                _ => None
            }
        }).collect()
    }

    pub fn deparse(&self) -> Result<String> {
        crate::deparse(&self.protobuf)
    }

    pub fn truncate(&self, max_length: usize) -> Result<String> {
        crate::truncate(&self.protobuf, max_length)
    }
}
