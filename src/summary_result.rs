use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::string::String;

use crate::protobuf::summary_result::Context;
use crate::*;

/// Result from calling [summary].
/// Where possible, this is API-compatible with [ParseResult].
#[derive(Debug, PartialEq)]
pub struct SummaryResult {
    pub protobuf: protobuf::SummaryResult,
    pub warnings: Vec<String>,
    pub tables: Vec<Table>,
    pub aliases: HashMap<String, String>,
    pub cte_names: Vec<String>,
    pub functions: Vec<Function>,
    pub filter_columns: Vec<FilterColumn>,
    pub truncated_query: Option<String>,
    pub statement_types: Vec<String>,
}

impl PartialOrd for FilterColumn {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FilterColumn {
    fn cmp(&self, other: &Self) -> Ordering {
        let schema_cmp = self.schema.cmp(&other.schema);
        let table_cmp = self.table.cmp(&other.table);
        let column_cmp = self.column.cmp(&other.column);

        if schema_cmp != Ordering::Equal {
            schema_cmp
        } else if table_cmp != Ordering::Equal {
            table_cmp
        } else {
            column_cmp
        }
    }
}

impl SummaryResult {
    pub fn new(protobuf: protobuf::SummaryResult, stderr: String) -> Self {
        let warnings = stderr.lines().filter_map(|l| if l.starts_with("WARNING") { Some(l.trim().into()) } else { None }).collect();
        let mut tables: HashSet<Table> = HashSet::new();
        let aliases: HashMap<String, String>;
        let cte_names: HashSet<String>;
        let mut functions: HashSet<Function> = HashSet::new();
        let mut filter_columns: HashSet<FilterColumn> = HashSet::new();
        let truncated_query = (!protobuf.truncated_query.is_empty()).then(|| protobuf.truncated_query.to_owned());
        let statement_types: Vec<String>;

        for table in &protobuf.tables {
            tables.insert(Table::from(table));
        }

        aliases = protobuf.aliases.clone();
        cte_names = HashSet::from_iter(protobuf.cte_names.to_owned());

        for function in &protobuf.functions {
            functions.insert(Function::from(function));
        }

        for filter_column in &protobuf.filter_columns {
            filter_columns.insert(FilterColumn::from(filter_column));
        }

        statement_types = protobuf.statement_types.clone();

        Self {
            protobuf,
            warnings,
            tables: Vec::from_iter(tables),
            aliases: aliases,
            cte_names: Vec::from_iter(cte_names),
            functions: Vec::from_iter(functions),
            filter_columns: Vec::from_iter(filter_columns),
            truncated_query,
            statement_types,
        }
    }

    /// Returns all referenced tables in the query
    pub fn tables(&self) -> Vec<String> {
        let mut tables = HashSet::new();
        self.tables.iter().for_each(|table| {
            tables.insert(table.name.clone());
        });
        Vec::from_iter(tables)
    }

    /// Returns only tables that were selected from
    pub fn select_tables(&self) -> Vec<String> {
        self.tables
            .iter()
            .filter_map(|table| match &table.context {
                Context::Select => Some(table.name.to_string()),
                _ => None,
            })
            .collect()
    }

    /// Returns only tables that were modified by the query
    pub fn dml_tables(&self) -> Vec<String> {
        self.tables
            .iter()
            .filter_map(|table| match &table.context {
                Context::Dml => Some(table.name.to_string()),
                _ => None,
            })
            .collect()
    }

    /// Returns only tables that were modified by DDL statements
    pub fn ddl_tables(&self) -> Vec<String> {
        self.tables
            .iter()
            .filter_map(|table| match &table.context {
                Context::Ddl => Some(table.name.to_string()),
                _ => None,
            })
            .collect()
    }

    /// Returns all function references
    pub fn functions(&self) -> Vec<String> {
        let mut functions = HashSet::new();
        self.functions.iter().for_each(|f| {
            functions.insert(f.name.to_string());
        });
        Vec::from_iter(functions)
    }

    /// Returns DDL functions
    pub fn ddl_functions(&self) -> Vec<String> {
        self.functions
            .iter()
            .filter_map(|function| match &function.context {
                Context::Ddl => Some(function.name.to_string()),
                _ => None,
            })
            .collect()
    }

    /// Returns functions that were called
    pub fn call_functions(&self) -> Vec<String> {
        self.functions
            .iter()
            .filter_map(|function| match &function.context {
                Context::Call => Some(function.name.to_string()),
                _ => None,
            })
            .collect()
    }

    /// Returns all statement types in the query
    pub fn statement_types(&self) -> Vec<&str> {
        // Converts statement_types from Vec<String> to Vec<&str> for
        // strict API compatibility with ParseResult.
        self.statement_types.iter().map(AsRef::as_ref).collect()
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Table {
    pub name: String,
    pub schema_name: String,
    pub rel_name: String,
    pub context: Context,
}

impl From<&protobuf::summary_result::Table> for Table {
    fn from(v: &protobuf::summary_result::Table) -> Self {
        Self {
            name: v.name.to_owned(),
            schema_name: v.schema_name.to_owned(),
            rel_name: v.rel_name.to_owned(),
            context: Context::try_from(v.context).unwrap(), // FIXME: avoid unwrap()
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Function {
    pub name: String,
    pub function_name: String,
    pub schema_name: Option<String>,
    pub context: Context,
}

impl From<&protobuf::summary_result::Function> for Function {
    fn from(v: &protobuf::summary_result::Function) -> Self {
        let schema_name = (!v.schema_name.is_empty()).then(|| v.schema_name.to_owned());

        Function {
            name: v.name.to_owned(),
            function_name: v.function_name.to_owned(),
            schema_name: schema_name,
            context: Context::try_from(v.context).unwrap(), // FIXME: avoid unwrap()
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct FilterColumn {
    pub schema: Option<String>,
    pub table: Option<String>,
    pub column: String,
}

impl From<&protobuf::summary_result::FilterColumn> for FilterColumn {
    fn from(v: &protobuf::summary_result::FilterColumn) -> Self {
        let schema = (!v.schema.is_empty()).then(|| v.schema.to_owned());
        let table = (!v.table.is_empty()).then(|| v.table.to_owned());
        let column = v.column.to_owned();

        Self { schema, table, column }
    }
}
