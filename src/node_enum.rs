use crate::*;

pub use protobuf::node::Node as NodeEnum;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Context { None, Select, DML, DDL, Call }

impl NodeEnum {
    pub fn deparse(&self) -> Result<String> {
        crate::deparse(&protobuf::ParseResult {
            version: crate::bindings::PG_VERSION_NUM as i32,
            stmts: vec![
                protobuf::RawStmt {
                    stmt: Some(Box::new(Node { node: Some(self.clone()) })),
                    stmt_location: 0,
                    stmt_len: 0,
                }
            ]
        })
    }

    pub fn nodes(&self) -> Vec<(NodeRef, i32, Context)> {
        let mut iter = vec![(self.to_ref(), 0, Context::None)];
        let mut nodes = Vec::new();
        while !iter.is_empty() {
            let (node, depth, context) = iter.remove(0);
            let depth = depth + 1;
            match node {
                //
                // The following statement types do not modify tables
                //
                NodeRef::SelectStmt(s) => {
                    s.target_list.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, Context::Select));
                        }
                    });
                    if let Some(n) = &s.where_clause {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, Context::Select));
                        }
                    }
                    s.sort_clause.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, Context::Select));
                        }
                    });
                    s.group_clause.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, Context::Select));
                        }
                    });
                    if let Some(n) = &s.having_clause {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, Context::Select));
                        }
                    }
                    if let Some(clause) = &s.with_clause {
                        clause.ctes.iter().for_each(|n| {
                            if let Some(n) = n.node.as_ref() {
                                iter.push((n.to_ref(), depth, Context::Select));
                            }
                        });
                    }
                    match protobuf::SetOperation::from_i32(s.op) {
                        Some(protobuf::SetOperation::SetopNone) => {
                            s.from_clause.iter().for_each(|n| {
                                if let Some(n) = n.node.as_ref() {
                                    iter.push((n.to_ref(), depth, Context::Select));
                                }
                            });
                        }
                        Some(protobuf::SetOperation::SetopUnion) => {
                            if let Some(left) = s.larg.as_ref() {
                                iter.push((left.to_ref(), depth, Context::Select));
                            }
                            if let Some(right) = s.rarg.as_ref() {
                                iter.push((right.to_ref(), depth, Context::Select));
                            }
                        }
                        Some(protobuf::SetOperation::SetopExcept) => {
                            if let Some(left) = s.larg.as_ref() {
                                iter.push((left.to_ref(), depth, Context::Select));
                            }
                            if let Some(right) = s.rarg.as_ref() {
                                iter.push((right.to_ref(), depth, Context::Select));
                            }
                        }
                        Some(protobuf::SetOperation::SetopIntersect) => {
                            if let Some(left) = s.larg.as_ref() {
                                iter.push((left.to_ref(), depth, Context::Select));
                            }
                            if let Some(right) = s.rarg.as_ref() {
                                iter.push((right.to_ref(), depth, Context::Select));
                            }
                        }
                        Some(protobuf::SetOperation::Undefined) | None => (),
                    }
                }
                NodeRef::InsertStmt(s) => {
                    if let Some(n) = &s.select_stmt {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, Context::DML));
                        }
                    }
                    if let Some(rel) = s.relation.as_ref() {
                        iter.push((rel.to_ref(), depth, Context::DML));
                    }
                    if let Some(clause) = &s.with_clause {
                        clause.ctes.iter().for_each(|n| {
                            if let Some(n) = n.node.as_ref() {
                                iter.push((n.to_ref(), depth, Context::DML));
                            }
                        });
                    }
                    if let Some(n) = &s.on_conflict_clause {
                        iter.push((n.to_ref(), depth, Context::DML));
                    }
                }
                NodeRef::UpdateStmt(s) => {
                    s.target_list.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, Context::DML));
                        }
                    });
                    s.where_clause.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, Context::DML));
                        }
                    });
                    s.from_clause.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, Context::Select));
                        }
                    });
                    if let Some(rel) = s.relation.as_ref() {
                        iter.push((rel.to_ref(), depth, Context::DML));
                    }
                    if let Some(clause) = &s.with_clause {
                        clause.ctes.iter().for_each(|n| {
                            if let Some(n) = n.node.as_ref() {
                                iter.push((n.to_ref(), depth, Context::DML));
                            }
                        });
                    }
                }
                NodeRef::DeleteStmt(s) => {
                    if let Some(n) = &s.where_clause {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, Context::DML));
                        }
                    }
                    if let Some(rel) = s.relation.as_ref() {
                        iter.push((rel.to_ref(), depth, Context::DML));
                    }
                    if let Some(clause) = &s.with_clause {
                        clause.ctes.iter().for_each(|n| {
                            if let Some(n) = n.node.as_ref() {
                                iter.push((n.to_ref(), depth, Context::DML));
                            }
                        });
                    }
                    s.using_clause.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, Context::Select));
                        }
                    });
                }
                NodeRef::CommonTableExpr(s) => {
                    if let Some(n) = &s.ctequery {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    }
                }
                NodeRef::CopyStmt(s) => {
                    if let Some(n) = &s.query {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, Context::DML));
                        }
                    }
                    if let Some(rel) = s.relation.as_ref() {
                        iter.push((rel.to_ref(), depth, Context::DML));
                    }
                }
                //
                // The following statement types are DDL (changing table structure)
                //
                NodeRef::AlterTableStmt(s) => {
                    if let Some(rel) = s.relation.as_ref() {
                        iter.push((rel.to_ref(), depth, Context::DDL));
                    }
                }
                NodeRef::CreateStmt(s) => {
                    if let Some(rel) = s.relation.as_ref() {
                        iter.push((rel.to_ref(), depth, Context::DDL));
                    }
                }
                NodeRef::CreateTableAsStmt(s) => {
                    if let Some(n) = &s.query {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, Context::DDL));
                        }
                    }
                    if let Some(n) = &s.into {
                        if let Some(rel) = n.rel.as_ref() {
                            iter.push((rel.to_ref(), depth, Context::DDL));
                        }
                    }
                }
                NodeRef::TruncateStmt(s) => {
                    s.relations.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, Context::DDL));
                        }
                    });
                }
                NodeRef::ViewStmt(s) => {
                    if let Some(n) = &s.query {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, Context::DDL));
                        }
                    }
                    if let Some(rel) = s.view.as_ref() {
                        iter.push((rel.to_ref(), depth, Context::DDL));
                    }
                }
                NodeRef::IndexStmt(s) => {
                    if let Some(rel) = s.relation.as_ref() {
                        iter.push((rel.to_ref(), depth, Context::DDL));
                    }
                }
                NodeRef::CreateTrigStmt(s) => {
                    if let Some(rel) = s.relation.as_ref() {
                        iter.push((rel.to_ref(), depth, Context::DDL));
                    }
                }
                NodeRef::RuleStmt(s) => {
                    if let Some(rel) = s.relation.as_ref() {
                        iter.push((rel.to_ref(), depth, Context::DDL));
                    }
                }
                NodeRef::VacuumStmt(s) => {
                    for node in &s.rels {
                        if let Some(NodeEnum::VacuumRelation(r)) = &node.node {
                            if let Some(rel) = r.relation.as_ref() {
                                iter.push((rel.to_ref(), depth, Context::DDL));
                            }
                        }
                    }
                }
                NodeRef::RefreshMatViewStmt(s) => {
                    if let Some(rel) = s.relation.as_ref() {
                        iter.push((rel.to_ref(), depth, Context::DDL));
                    }
                }
                NodeRef::GrantStmt(s) => {
                    match protobuf::ObjectType::from_i32(s.objtype) {
                        Some(protobuf::ObjectType::ObjectTable) => {
                            s.objects.iter().for_each(|n| {
                                if let Some(n) = n.node.as_ref() {
                                    iter.push((n.to_ref(), depth, Context::DDL));
                                }
                            });
                        }
                        _ => ()
                    }
                }
                NodeRef::LockStmt(s) => {
                    s.relations.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, Context::DDL));
                        }
                    });
                }
                NodeRef::ExplainStmt(s) => {
                    if let Some(n) = &s.query {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    }
                }
                //
                // Subselect items
                //
                NodeRef::AExpr(e) => {
                    if let Some(n) = &e.lexpr {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    }
                    if let Some(n) = &e.rexpr {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    }
                }
                NodeRef::BoolExpr(e) => {
                    e.args.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    });
                }
                NodeRef::CoalesceExpr(e) => {
                    e.args.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    });
                }
                NodeRef::MinMaxExpr(e) => {
                    e.args.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    });
                }
                NodeRef::ResTarget(t) => {
                    if let Some(n) = &t.val {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    }
                }
                NodeRef::SubLink(l) => {
                    if let Some(n) = &l.subselect {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    }
                }
                NodeRef::FuncCall(c) => {
                    c.args.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    });
                }
                NodeRef::CaseExpr(c) => {
                    c.args.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    });
                    if let Some(n) = &c.defresult {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    }
                }
                NodeRef::CaseWhen(w) => {
                    if let Some(n) = &w.expr {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    }
                    if let Some(n) = &w.result {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    }
                }
                NodeRef::SortBy(n) => {
                    if let Some(n) = &n.node {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    }
                }
                //
                // from-clause items
                //
                NodeRef::List(l) => {
                    l.items.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    });
                }
                NodeRef::JoinExpr(e) => {
                    [&e.larg, &e.rarg, &e.quals].iter().for_each(|n| {
                        if let Some(n) = n {
                            if let Some(n) = n.node.as_ref() {
                                iter.push((n.to_ref(), depth, context));
                            }
                        }
                    });
                }
                NodeRef::RowExpr(e) => {
                    e.args.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    });
                }
                NodeRef::RangeSubselect(s) => {
                    if let Some(n) = &s.subquery {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    }
                }
                NodeRef::RangeFunction(f) => {
                    f.functions.iter().for_each(|n| {
                        if let Some(n) = n.node.as_ref() {
                            iter.push((n.to_ref(), depth, context));
                        }
                    });
                }
                _ => ()
            }
            nodes.push((node, depth, context));
        }
        nodes
    }

    // this shouldn't need to track depth or context, but I'm keeping them for now for debugging
    pub fn find_mut(&mut self, to_find: &NodeRef) -> Option<NodeMut> {
        let mut iter: Vec<(NodeMut, i32, Context)> = vec![(self.to_mut(), 0, Context::None)];
        while !iter.is_empty() {
            let (node, depth, context) = iter.remove(0);
            if node.eq(to_find) { return Some(node) }
            let depth = depth + 1;
            match node {
                NodeMut::SelectStmt(s) => {
                    s.target_list.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, Context::Select));
                        }
                    });
                    if let Some(n) = s.where_clause.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, Context::Select));
                        }
                    }
                    s.sort_clause.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, Context::Select));
                        }
                    });
                    s.group_clause.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, Context::Select));
                        }
                    });
                    if let Some(n) = s.having_clause.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, Context::Select));
                        }
                    }
                    if let Some(clause) = s.with_clause.as_mut() {
                        clause.ctes.iter_mut().for_each(|n| {
                            if let Some(n) = n.node.as_mut() {
                                iter.push((n.to_mut(), depth, Context::Select));
                            }
                        });
                    }
                    match protobuf::SetOperation::from_i32(s.op) {
                        Some(protobuf::SetOperation::SetopNone) => {
                            s.from_clause.iter_mut().for_each(|n| {
                                if let Some(n) = n.node.as_mut() {
                                    iter.push((n.to_mut(), depth, Context::Select));
                                }
                            });
                        }
                        Some(protobuf::SetOperation::SetopUnion) => {
                            if let Some(left) = s.larg.as_mut() {
                                iter.push((left.to_mut(), depth, Context::Select));
                            }
                            if let Some(right) = s.rarg.as_mut() {
                                iter.push((right.to_mut(), depth, Context::Select));
                            }
                        }
                        Some(protobuf::SetOperation::SetopExcept) => {
                            if let Some(left) = s.larg.as_mut() {
                                iter.push((left.to_mut(), depth, Context::Select));
                            }
                            if let Some(right) = s.rarg.as_mut() {
                                iter.push((right.to_mut(), depth, Context::Select));
                            }
                        }
                        Some(protobuf::SetOperation::SetopIntersect) => {
                            if let Some(left) = s.larg.as_mut() {
                                iter.push((left.to_mut(), depth, Context::Select));
                            }
                            if let Some(right) = s.rarg.as_mut() {
                                iter.push((right.to_mut(), depth, Context::Select));
                            }
                        }
                        Some(protobuf::SetOperation::Undefined) | None => (),
                    }
                }
                NodeMut::InsertStmt(s) => {
                    if let Some(n) = s.select_stmt.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, Context::DML));
                        }
                    }
                    if let Some(rel) = s.relation.as_mut() {
                        iter.push((rel.to_mut(), depth, Context::DML));
                    }
                    if let Some(clause) = s.with_clause.as_mut() {
                        clause.ctes.iter_mut().for_each(|n| {
                            if let Some(n) = n.node.as_mut() {
                                iter.push((n.to_mut(), depth, Context::DML));
                            }
                        });
                    }
                    if let Some(n) = s.on_conflict_clause.as_mut() {
                        iter.push((n.to_mut(), depth, Context::DML));
                    }
                }
                NodeMut::UpdateStmt(s) => {
                    s.target_list.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, Context::DML));
                        }
                    });
                    s.where_clause.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, Context::DML));
                        }
                    });
                    s.from_clause.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, Context::Select));
                        }
                    });
                    if let Some(rel) = s.relation.as_mut() {
                        iter.push((rel.to_mut(), depth, Context::DML));
                    }
                    if let Some(clause) = s.with_clause.as_mut() {
                        clause.ctes.iter_mut().for_each(|n| {
                            if let Some(n) = n.node.as_mut() {
                                iter.push((n.to_mut(), depth, Context::DML));
                            }
                        });
                    }
                }
                NodeMut::DeleteStmt(s) => {
                    if let Some(n) = s.where_clause.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, Context::DML));
                        }
                    }
                    if let Some(rel) = s.relation.as_mut() {
                        iter.push((rel.to_mut(), depth, Context::DML));
                    }
                    if let Some(clause) = s.with_clause.as_mut() {
                        clause.ctes.iter_mut().for_each(|n| {
                            if let Some(n) = n.node.as_mut() {
                                iter.push((n.to_mut(), depth, Context::DML));
                            }
                        });
                    }
                    s.using_clause.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, Context::Select));
                        }
                    });
                }
                NodeMut::CommonTableExpr(s) => {
                    if let Some(n) = s.ctequery.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    }
                }
                NodeMut::CopyStmt(s) => {
                    if let Some(n) = s.query.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, Context::DML));
                        }
                    }
                    if let Some(rel) = s.relation.as_mut() {
                        iter.push((rel.to_mut(), depth, Context::DML));
                    }
                }
                //
                // The following statement types are DDL (changing table structure)
                //
                NodeMut::AlterTableStmt(s) => {
                    if let Some(rel) = s.relation.as_mut() {
                        iter.push((rel.to_mut(), depth, Context::DDL));
                    }
                }
                NodeMut::CreateStmt(s) => {
                    if let Some(rel) = s.relation.as_mut() {
                        iter.push((rel.to_mut(), depth, Context::DDL));
                    }
                }
                NodeMut::CreateTableAsStmt(s) => {
                    if let Some(n) = s.query.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, Context::DDL));
                        }
                    }
                    if let Some(n) = s.into.as_mut() {
                        if let Some(rel) = n.rel.as_mut() {
                            iter.push((rel.to_mut(), depth, Context::DDL));
                        }
                    }
                }
                NodeMut::TruncateStmt(s) => {
                    s.relations.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, Context::DDL));
                        }
                    });
                }
                NodeMut::ViewStmt(s) => {
                    if let Some(n) = s.query.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, Context::DDL));
                        }
                    }
                    if let Some(rel) = s.view.as_mut() {
                        iter.push((rel.to_mut(), depth, Context::DDL));
                    }
                }
                NodeMut::IndexStmt(s) => {
                    if let Some(rel) = s.relation.as_mut() {
                        iter.push((rel.to_mut(), depth, Context::DDL));
                    }
                }
                NodeMut::CreateTrigStmt(s) => {
                    if let Some(rel) = s.relation.as_mut() {
                        iter.push((rel.to_mut(), depth, Context::DDL));
                    }
                }
                NodeMut::RuleStmt(s) => {
                    if let Some(rel) = s.relation.as_mut() {
                        iter.push((rel.to_mut(), depth, Context::DDL));
                    }
                }
                NodeMut::VacuumStmt(s) => {
                    for node in s.rels.iter_mut() {
                        if let Some(NodeEnum::VacuumRelation(r)) = node.node.as_mut() {
                            if let Some(rel) = r.relation.as_mut() {
                                iter.push((rel.to_mut(), depth, Context::DDL));
                            }
                        }
                    }
                }
                NodeMut::RefreshMatViewStmt(s) => {
                    if let Some(rel) = s.relation.as_mut() {
                        iter.push((rel.to_mut(), depth, Context::DDL));
                    }
                }
                NodeMut::GrantStmt(s) => {
                    match protobuf::ObjectType::from_i32(s.objtype) {
                        Some(protobuf::ObjectType::ObjectTable) => {
                            s.objects.iter_mut().for_each(|n| {
                                if let Some(n) = n.node.as_mut() {
                                    iter.push((n.to_mut(), depth, Context::DDL));
                                }
                            });
                        }
                        _ => ()
                    }
                }
                NodeMut::LockStmt(s) => {
                    s.relations.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, Context::DDL));
                        }
                    });
                }
                NodeMut::ExplainStmt(s) => {
                    if let Some(n) = s.query.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    }
                }
                //
                // Subselect items
                //
                NodeMut::AExpr(e) => {
                    if let Some(n) = e.lexpr.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    }
                    if let Some(n) = e.rexpr.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    }
                }
                NodeMut::BoolExpr(e) => {
                    e.args.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    });
                }
                NodeMut::CoalesceExpr(e) => {
                    e.args.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    });
                }
                NodeMut::MinMaxExpr(e) => {
                    e.args.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    });
                }
                NodeMut::ResTarget(t) => {
                    if let Some(n) = t.val.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    }
                }
                NodeMut::SubLink(l) => {
                    if let Some(n) = l.subselect.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    }
                }
                NodeMut::FuncCall(c) => {
                    c.args.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    });
                }
                NodeMut::CaseExpr(c) => {
                    c.args.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    });
                    if let Some(n) = c.defresult.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    }
                }
                NodeMut::CaseWhen(w) => {
                    if let Some(n) = w.expr.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    }
                    if let Some(n) = w.result.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    }
                }
                NodeMut::SortBy(n) => {
                    if let Some(n) = n.node.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    }
                }
                //
                // from-clause items
                //
                NodeMut::List(l) => {
                    l.items.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    });
                }
                NodeMut::JoinExpr(e) => {
                    if let Some(n) = e.larg.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    }
                    if let Some(n) = e.rarg.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    }
                    if let Some(n) = e.quals.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    }
                }
                NodeMut::RowExpr(e) => {
                    e.args.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    });
                }
                NodeMut::RangeSubselect(s) => {
                    if let Some(n) = s.subquery.as_mut() {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    }
                }
                NodeMut::RangeFunction(f) => {
                    f.functions.iter_mut().for_each(|n| {
                        if let Some(n) = n.node.as_mut() {
                            iter.push((n.to_mut(), depth, context));
                        }
                    });
                }
                _ => ()
            }
        }
        None
    }

    pub fn to_ref(&self) -> NodeRef {
        match self {
            NodeEnum::Alias(n) => NodeRef::Alias(n),
            NodeEnum::RangeVar(n) => NodeRef::RangeVar(n),
            NodeEnum::TableFunc(n) => NodeRef::TableFunc(n),
            NodeEnum::Expr(n) => NodeRef::Expr(n),
            NodeEnum::Var(n) => NodeRef::Var(n),
            NodeEnum::Param(n) => NodeRef::Param(n),
            NodeEnum::Aggref(n) => NodeRef::Aggref(n),
            NodeEnum::GroupingFunc(n) => NodeRef::GroupingFunc(n),
            NodeEnum::WindowFunc(n) => NodeRef::WindowFunc(n),
            NodeEnum::SubscriptingRef(n) => NodeRef::SubscriptingRef(n),
            NodeEnum::FuncExpr(n) => NodeRef::FuncExpr(n),
            NodeEnum::NamedArgExpr(n) => NodeRef::NamedArgExpr(n),
            NodeEnum::OpExpr(n) => NodeRef::OpExpr(n),
            NodeEnum::DistinctExpr(n) => NodeRef::DistinctExpr(n),
            NodeEnum::NullIfExpr(n) => NodeRef::NullIfExpr(n),
            NodeEnum::ScalarArrayOpExpr(n) => NodeRef::ScalarArrayOpExpr(n),
            NodeEnum::BoolExpr(n) => NodeRef::BoolExpr(n),
            NodeEnum::SubLink(n) => NodeRef::SubLink(n),
            NodeEnum::SubPlan(n) => NodeRef::SubPlan(n),
            NodeEnum::AlternativeSubPlan(n) => NodeRef::AlternativeSubPlan(n),
            NodeEnum::FieldSelect(n) => NodeRef::FieldSelect(n),
            NodeEnum::FieldStore(n) => NodeRef::FieldStore(n),
            NodeEnum::RelabelType(n) => NodeRef::RelabelType(n),
            NodeEnum::CoerceViaIo(n) => NodeRef::CoerceViaIo(n),
            NodeEnum::ArrayCoerceExpr(n) => NodeRef::ArrayCoerceExpr(n),
            NodeEnum::ConvertRowtypeExpr(n) => NodeRef::ConvertRowtypeExpr(n),
            NodeEnum::CollateExpr(n) => NodeRef::CollateExpr(n),
            NodeEnum::CaseExpr(n) => NodeRef::CaseExpr(n),
            NodeEnum::CaseWhen(n) => NodeRef::CaseWhen(n),
            NodeEnum::CaseTestExpr(n) => NodeRef::CaseTestExpr(n),
            NodeEnum::ArrayExpr(n) => NodeRef::ArrayExpr(n),
            NodeEnum::RowExpr(n) => NodeRef::RowExpr(n),
            NodeEnum::RowCompareExpr(n) => NodeRef::RowCompareExpr(n),
            NodeEnum::CoalesceExpr(n) => NodeRef::CoalesceExpr(n),
            NodeEnum::MinMaxExpr(n) => NodeRef::MinMaxExpr(n),
            NodeEnum::SqlvalueFunction(n) => NodeRef::SqlvalueFunction(n),
            NodeEnum::XmlExpr(n) => NodeRef::XmlExpr(n),
            NodeEnum::NullTest(n) => NodeRef::NullTest(n),
            NodeEnum::BooleanTest(n) => NodeRef::BooleanTest(n),
            NodeEnum::CoerceToDomain(n) => NodeRef::CoerceToDomain(n),
            NodeEnum::CoerceToDomainValue(n) => NodeRef::CoerceToDomainValue(n),
            NodeEnum::SetToDefault(n) => NodeRef::SetToDefault(n),
            NodeEnum::CurrentOfExpr(n) => NodeRef::CurrentOfExpr(n),
            NodeEnum::NextValueExpr(n) => NodeRef::NextValueExpr(n),
            NodeEnum::InferenceElem(n) => NodeRef::InferenceElem(n),
            NodeEnum::TargetEntry(n) => NodeRef::TargetEntry(n),
            NodeEnum::RangeTblRef(n) => NodeRef::RangeTblRef(n),
            NodeEnum::JoinExpr(n) => NodeRef::JoinExpr(n),
            NodeEnum::FromExpr(n) => NodeRef::FromExpr(n),
            NodeEnum::OnConflictExpr(n) => NodeRef::OnConflictExpr(n),
            NodeEnum::IntoClause(n) => NodeRef::IntoClause(n),
            NodeEnum::RawStmt(n) => NodeRef::RawStmt(n),
            NodeEnum::Query(n) => NodeRef::Query(n),
            NodeEnum::InsertStmt(n) => NodeRef::InsertStmt(n),
            NodeEnum::DeleteStmt(n) => NodeRef::DeleteStmt(n),
            NodeEnum::UpdateStmt(n) => NodeRef::UpdateStmt(n),
            NodeEnum::SelectStmt(n) => NodeRef::SelectStmt(n),
            NodeEnum::AlterTableStmt(n) => NodeRef::AlterTableStmt(n),
            NodeEnum::AlterTableCmd(n) => NodeRef::AlterTableCmd(n),
            NodeEnum::AlterDomainStmt(n) => NodeRef::AlterDomainStmt(n),
            NodeEnum::SetOperationStmt(n) => NodeRef::SetOperationStmt(n),
            NodeEnum::GrantStmt(n) => NodeRef::GrantStmt(n),
            NodeEnum::GrantRoleStmt(n) => NodeRef::GrantRoleStmt(n),
            NodeEnum::AlterDefaultPrivilegesStmt(n) => NodeRef::AlterDefaultPrivilegesStmt(n),
            NodeEnum::ClosePortalStmt(n) => NodeRef::ClosePortalStmt(n),
            NodeEnum::ClusterStmt(n) => NodeRef::ClusterStmt(n),
            NodeEnum::CopyStmt(n) => NodeRef::CopyStmt(n),
            NodeEnum::CreateStmt(n) => NodeRef::CreateStmt(n),
            NodeEnum::DefineStmt(n) => NodeRef::DefineStmt(n),
            NodeEnum::DropStmt(n) => NodeRef::DropStmt(n),
            NodeEnum::TruncateStmt(n) => NodeRef::TruncateStmt(n),
            NodeEnum::CommentStmt(n) => NodeRef::CommentStmt(n),
            NodeEnum::FetchStmt(n) => NodeRef::FetchStmt(n),
            NodeEnum::IndexStmt(n) => NodeRef::IndexStmt(n),
            NodeEnum::CreateFunctionStmt(n) => NodeRef::CreateFunctionStmt(n),
            NodeEnum::AlterFunctionStmt(n) => NodeRef::AlterFunctionStmt(n),
            NodeEnum::DoStmt(n) => NodeRef::DoStmt(n),
            NodeEnum::RenameStmt(n) => NodeRef::RenameStmt(n),
            NodeEnum::RuleStmt(n) => NodeRef::RuleStmt(n),
            NodeEnum::NotifyStmt(n) => NodeRef::NotifyStmt(n),
            NodeEnum::ListenStmt(n) => NodeRef::ListenStmt(n),
            NodeEnum::UnlistenStmt(n) => NodeRef::UnlistenStmt(n),
            NodeEnum::TransactionStmt(n) => NodeRef::TransactionStmt(n),
            NodeEnum::ViewStmt(n) => NodeRef::ViewStmt(n),
            NodeEnum::LoadStmt(n) => NodeRef::LoadStmt(n),
            NodeEnum::CreateDomainStmt(n) => NodeRef::CreateDomainStmt(n),
            NodeEnum::CreatedbStmt(n) => NodeRef::CreatedbStmt(n),
            NodeEnum::DropdbStmt(n) => NodeRef::DropdbStmt(n),
            NodeEnum::VacuumStmt(n) => NodeRef::VacuumStmt(n),
            NodeEnum::ExplainStmt(n) => NodeRef::ExplainStmt(n),
            NodeEnum::CreateTableAsStmt(n) => NodeRef::CreateTableAsStmt(n),
            NodeEnum::CreateSeqStmt(n) => NodeRef::CreateSeqStmt(n),
            NodeEnum::AlterSeqStmt(n) => NodeRef::AlterSeqStmt(n),
            NodeEnum::VariableSetStmt(n) => NodeRef::VariableSetStmt(n),
            NodeEnum::VariableShowStmt(n) => NodeRef::VariableShowStmt(n),
            NodeEnum::DiscardStmt(n) => NodeRef::DiscardStmt(n),
            NodeEnum::CreateTrigStmt(n) => NodeRef::CreateTrigStmt(n),
            NodeEnum::CreatePlangStmt(n) => NodeRef::CreatePlangStmt(n),
            NodeEnum::CreateRoleStmt(n) => NodeRef::CreateRoleStmt(n),
            NodeEnum::AlterRoleStmt(n) => NodeRef::AlterRoleStmt(n),
            NodeEnum::DropRoleStmt(n) => NodeRef::DropRoleStmt(n),
            NodeEnum::LockStmt(n) => NodeRef::LockStmt(n),
            NodeEnum::ConstraintsSetStmt(n) => NodeRef::ConstraintsSetStmt(n),
            NodeEnum::ReindexStmt(n) => NodeRef::ReindexStmt(n),
            NodeEnum::CheckPointStmt(n) => NodeRef::CheckPointStmt(n),
            NodeEnum::CreateSchemaStmt(n) => NodeRef::CreateSchemaStmt(n),
            NodeEnum::AlterDatabaseStmt(n) => NodeRef::AlterDatabaseStmt(n),
            NodeEnum::AlterDatabaseSetStmt(n) => NodeRef::AlterDatabaseSetStmt(n),
            NodeEnum::AlterRoleSetStmt(n) => NodeRef::AlterRoleSetStmt(n),
            NodeEnum::CreateConversionStmt(n) => NodeRef::CreateConversionStmt(n),
            NodeEnum::CreateCastStmt(n) => NodeRef::CreateCastStmt(n),
            NodeEnum::CreateOpClassStmt(n) => NodeRef::CreateOpClassStmt(n),
            NodeEnum::CreateOpFamilyStmt(n) => NodeRef::CreateOpFamilyStmt(n),
            NodeEnum::AlterOpFamilyStmt(n) => NodeRef::AlterOpFamilyStmt(n),
            NodeEnum::PrepareStmt(n) => NodeRef::PrepareStmt(n),
            NodeEnum::ExecuteStmt(n) => NodeRef::ExecuteStmt(n),
            NodeEnum::DeallocateStmt(n) => NodeRef::DeallocateStmt(n),
            NodeEnum::DeclareCursorStmt(n) => NodeRef::DeclareCursorStmt(n),
            NodeEnum::CreateTableSpaceStmt(n) => NodeRef::CreateTableSpaceStmt(n),
            NodeEnum::DropTableSpaceStmt(n) => NodeRef::DropTableSpaceStmt(n),
            NodeEnum::AlterObjectDependsStmt(n) => NodeRef::AlterObjectDependsStmt(n),
            NodeEnum::AlterObjectSchemaStmt(n) => NodeRef::AlterObjectSchemaStmt(n),
            NodeEnum::AlterOwnerStmt(n) => NodeRef::AlterOwnerStmt(n),
            NodeEnum::AlterOperatorStmt(n) => NodeRef::AlterOperatorStmt(n),
            NodeEnum::AlterTypeStmt(n) => NodeRef::AlterTypeStmt(n),
            NodeEnum::DropOwnedStmt(n) => NodeRef::DropOwnedStmt(n),
            NodeEnum::ReassignOwnedStmt(n) => NodeRef::ReassignOwnedStmt(n),
            NodeEnum::CompositeTypeStmt(n) => NodeRef::CompositeTypeStmt(n),
            NodeEnum::CreateEnumStmt(n) => NodeRef::CreateEnumStmt(n),
            NodeEnum::CreateRangeStmt(n) => NodeRef::CreateRangeStmt(n),
            NodeEnum::AlterEnumStmt(n) => NodeRef::AlterEnumStmt(n),
            NodeEnum::AlterTsdictionaryStmt(n) => NodeRef::AlterTsdictionaryStmt(n),
            NodeEnum::AlterTsconfigurationStmt(n) => NodeRef::AlterTsconfigurationStmt(n),
            NodeEnum::CreateFdwStmt(n) => NodeRef::CreateFdwStmt(n),
            NodeEnum::AlterFdwStmt(n) => NodeRef::AlterFdwStmt(n),
            NodeEnum::CreateForeignServerStmt(n) => NodeRef::CreateForeignServerStmt(n),
            NodeEnum::AlterForeignServerStmt(n) => NodeRef::AlterForeignServerStmt(n),
            NodeEnum::CreateUserMappingStmt(n) => NodeRef::CreateUserMappingStmt(n),
            NodeEnum::AlterUserMappingStmt(n) => NodeRef::AlterUserMappingStmt(n),
            NodeEnum::DropUserMappingStmt(n) => NodeRef::DropUserMappingStmt(n),
            NodeEnum::AlterTableSpaceOptionsStmt(n) => NodeRef::AlterTableSpaceOptionsStmt(n),
            NodeEnum::AlterTableMoveAllStmt(n) => NodeRef::AlterTableMoveAllStmt(n),
            NodeEnum::SecLabelStmt(n) => NodeRef::SecLabelStmt(n),
            NodeEnum::CreateForeignTableStmt(n) => NodeRef::CreateForeignTableStmt(n),
            NodeEnum::ImportForeignSchemaStmt(n) => NodeRef::ImportForeignSchemaStmt(n),
            NodeEnum::CreateExtensionStmt(n) => NodeRef::CreateExtensionStmt(n),
            NodeEnum::AlterExtensionStmt(n) => NodeRef::AlterExtensionStmt(n),
            NodeEnum::AlterExtensionContentsStmt(n) => NodeRef::AlterExtensionContentsStmt(n),
            NodeEnum::CreateEventTrigStmt(n) => NodeRef::CreateEventTrigStmt(n),
            NodeEnum::AlterEventTrigStmt(n) => NodeRef::AlterEventTrigStmt(n),
            NodeEnum::RefreshMatViewStmt(n) => NodeRef::RefreshMatViewStmt(n),
            NodeEnum::ReplicaIdentityStmt(n) => NodeRef::ReplicaIdentityStmt(n),
            NodeEnum::AlterSystemStmt(n) => NodeRef::AlterSystemStmt(n),
            NodeEnum::CreatePolicyStmt(n) => NodeRef::CreatePolicyStmt(n),
            NodeEnum::AlterPolicyStmt(n) => NodeRef::AlterPolicyStmt(n),
            NodeEnum::CreateTransformStmt(n) => NodeRef::CreateTransformStmt(n),
            NodeEnum::CreateAmStmt(n) => NodeRef::CreateAmStmt(n),
            NodeEnum::CreatePublicationStmt(n) => NodeRef::CreatePublicationStmt(n),
            NodeEnum::AlterPublicationStmt(n) => NodeRef::AlterPublicationStmt(n),
            NodeEnum::CreateSubscriptionStmt(n) => NodeRef::CreateSubscriptionStmt(n),
            NodeEnum::AlterSubscriptionStmt(n) => NodeRef::AlterSubscriptionStmt(n),
            NodeEnum::DropSubscriptionStmt(n) => NodeRef::DropSubscriptionStmt(n),
            NodeEnum::CreateStatsStmt(n) => NodeRef::CreateStatsStmt(n),
            NodeEnum::AlterCollationStmt(n) => NodeRef::AlterCollationStmt(n),
            NodeEnum::CallStmt(n) => NodeRef::CallStmt(n),
            NodeEnum::AlterStatsStmt(n) => NodeRef::AlterStatsStmt(n),
            NodeEnum::AExpr(n) => NodeRef::AExpr(n),
            NodeEnum::ColumnRef(n) => NodeRef::ColumnRef(n),
            NodeEnum::ParamRef(n) => NodeRef::ParamRef(n),
            NodeEnum::AConst(n) => NodeRef::AConst(n),
            NodeEnum::FuncCall(n) => NodeRef::FuncCall(n),
            NodeEnum::AStar(n) => NodeRef::AStar(n),
            NodeEnum::AIndices(n) => NodeRef::AIndices(n),
            NodeEnum::AIndirection(n) => NodeRef::AIndirection(n),
            NodeEnum::AArrayExpr(n) => NodeRef::AArrayExpr(n),
            NodeEnum::ResTarget(n) => NodeRef::ResTarget(n),
            NodeEnum::MultiAssignRef(n) => NodeRef::MultiAssignRef(n),
            NodeEnum::TypeCast(n) => NodeRef::TypeCast(n),
            NodeEnum::CollateClause(n) => NodeRef::CollateClause(n),
            NodeEnum::SortBy(n) => NodeRef::SortBy(n),
            NodeEnum::WindowDef(n) => NodeRef::WindowDef(n),
            NodeEnum::RangeSubselect(n) => NodeRef::RangeSubselect(n),
            NodeEnum::RangeFunction(n) => NodeRef::RangeFunction(n),
            NodeEnum::RangeTableSample(n) => NodeRef::RangeTableSample(n),
            NodeEnum::RangeTableFunc(n) => NodeRef::RangeTableFunc(n),
            NodeEnum::RangeTableFuncCol(n) => NodeRef::RangeTableFuncCol(n),
            NodeEnum::TypeName(n) => NodeRef::TypeName(n),
            NodeEnum::ColumnDef(n) => NodeRef::ColumnDef(n),
            NodeEnum::IndexElem(n) => NodeRef::IndexElem(n),
            NodeEnum::Constraint(n) => NodeRef::Constraint(n),
            NodeEnum::DefElem(n) => NodeRef::DefElem(n),
            NodeEnum::RangeTblEntry(n) => NodeRef::RangeTblEntry(n),
            NodeEnum::RangeTblFunction(n) => NodeRef::RangeTblFunction(n),
            NodeEnum::TableSampleClause(n) => NodeRef::TableSampleClause(n),
            NodeEnum::WithCheckOption(n) => NodeRef::WithCheckOption(n),
            NodeEnum::SortGroupClause(n) => NodeRef::SortGroupClause(n),
            NodeEnum::GroupingSet(n) => NodeRef::GroupingSet(n),
            NodeEnum::WindowClause(n) => NodeRef::WindowClause(n),
            NodeEnum::ObjectWithArgs(n) => NodeRef::ObjectWithArgs(n),
            NodeEnum::AccessPriv(n) => NodeRef::AccessPriv(n),
            NodeEnum::CreateOpClassItem(n) => NodeRef::CreateOpClassItem(n),
            NodeEnum::TableLikeClause(n) => NodeRef::TableLikeClause(n),
            NodeEnum::FunctionParameter(n) => NodeRef::FunctionParameter(n),
            NodeEnum::LockingClause(n) => NodeRef::LockingClause(n),
            NodeEnum::RowMarkClause(n) => NodeRef::RowMarkClause(n),
            NodeEnum::XmlSerialize(n) => NodeRef::XmlSerialize(n),
            NodeEnum::WithClause(n) => NodeRef::WithClause(n),
            NodeEnum::InferClause(n) => NodeRef::InferClause(n),
            NodeEnum::OnConflictClause(n) => NodeRef::OnConflictClause(n),
            NodeEnum::CommonTableExpr(n) => NodeRef::CommonTableExpr(n),
            NodeEnum::RoleSpec(n) => NodeRef::RoleSpec(n),
            NodeEnum::TriggerTransition(n) => NodeRef::TriggerTransition(n),
            NodeEnum::PartitionElem(n) => NodeRef::PartitionElem(n),
            NodeEnum::PartitionSpec(n) => NodeRef::PartitionSpec(n),
            NodeEnum::PartitionBoundSpec(n) => NodeRef::PartitionBoundSpec(n),
            NodeEnum::PartitionRangeDatum(n) => NodeRef::PartitionRangeDatum(n),
            NodeEnum::PartitionCmd(n) => NodeRef::PartitionCmd(n),
            NodeEnum::VacuumRelation(n) => NodeRef::VacuumRelation(n),
            NodeEnum::InlineCodeBlock(n) => NodeRef::InlineCodeBlock(n),
            NodeEnum::CallContext(n) => NodeRef::CallContext(n),
            NodeEnum::Integer(n) => NodeRef::Integer(n),
            NodeEnum::Float(n) => NodeRef::Float(n),
            NodeEnum::String(n) => NodeRef::String(n),
            NodeEnum::BitString(n) => NodeRef::BitString(n),
            NodeEnum::Null(n) => NodeRef::Null(n),
            NodeEnum::List(n) => NodeRef::List(n),
            NodeEnum::IntList(n) => NodeRef::IntList(n),
            NodeEnum::OidList(n) => NodeRef::OidList(n),
        }
    }

    pub fn to_mut(&mut self) -> NodeMut {
        match self {
            NodeEnum::Alias(n) => NodeMut::Alias(n),
            NodeEnum::RangeVar(n) => NodeMut::RangeVar(n),
            NodeEnum::TableFunc(n) => NodeMut::TableFunc(n),
            NodeEnum::Expr(n) => NodeMut::Expr(n),
            NodeEnum::Var(n) => NodeMut::Var(n),
            NodeEnum::Param(n) => NodeMut::Param(n),
            NodeEnum::Aggref(n) => NodeMut::Aggref(n),
            NodeEnum::GroupingFunc(n) => NodeMut::GroupingFunc(n),
            NodeEnum::WindowFunc(n) => NodeMut::WindowFunc(n),
            NodeEnum::SubscriptingRef(n) => NodeMut::SubscriptingRef(n),
            NodeEnum::FuncExpr(n) => NodeMut::FuncExpr(n),
            NodeEnum::NamedArgExpr(n) => NodeMut::NamedArgExpr(n),
            NodeEnum::OpExpr(n) => NodeMut::OpExpr(n),
            NodeEnum::DistinctExpr(n) => NodeMut::DistinctExpr(n),
            NodeEnum::NullIfExpr(n) => NodeMut::NullIfExpr(n),
            NodeEnum::ScalarArrayOpExpr(n) => NodeMut::ScalarArrayOpExpr(n),
            NodeEnum::BoolExpr(n) => NodeMut::BoolExpr(n),
            NodeEnum::SubLink(n) => NodeMut::SubLink(n),
            NodeEnum::SubPlan(n) => NodeMut::SubPlan(n),
            NodeEnum::AlternativeSubPlan(n) => NodeMut::AlternativeSubPlan(n),
            NodeEnum::FieldSelect(n) => NodeMut::FieldSelect(n),
            NodeEnum::FieldStore(n) => NodeMut::FieldStore(n),
            NodeEnum::RelabelType(n) => NodeMut::RelabelType(n),
            NodeEnum::CoerceViaIo(n) => NodeMut::CoerceViaIo(n),
            NodeEnum::ArrayCoerceExpr(n) => NodeMut::ArrayCoerceExpr(n),
            NodeEnum::ConvertRowtypeExpr(n) => NodeMut::ConvertRowtypeExpr(n),
            NodeEnum::CollateExpr(n) => NodeMut::CollateExpr(n),
            NodeEnum::CaseExpr(n) => NodeMut::CaseExpr(n),
            NodeEnum::CaseWhen(n) => NodeMut::CaseWhen(n),
            NodeEnum::CaseTestExpr(n) => NodeMut::CaseTestExpr(n),
            NodeEnum::ArrayExpr(n) => NodeMut::ArrayExpr(n),
            NodeEnum::RowExpr(n) => NodeMut::RowExpr(n),
            NodeEnum::RowCompareExpr(n) => NodeMut::RowCompareExpr(n),
            NodeEnum::CoalesceExpr(n) => NodeMut::CoalesceExpr(n),
            NodeEnum::MinMaxExpr(n) => NodeMut::MinMaxExpr(n),
            NodeEnum::SqlvalueFunction(n) => NodeMut::SqlvalueFunction(n),
            NodeEnum::XmlExpr(n) => NodeMut::XmlExpr(n),
            NodeEnum::NullTest(n) => NodeMut::NullTest(n),
            NodeEnum::BooleanTest(n) => NodeMut::BooleanTest(n),
            NodeEnum::CoerceToDomain(n) => NodeMut::CoerceToDomain(n),
            NodeEnum::CoerceToDomainValue(n) => NodeMut::CoerceToDomainValue(n),
            NodeEnum::SetToDefault(n) => NodeMut::SetToDefault(n),
            NodeEnum::CurrentOfExpr(n) => NodeMut::CurrentOfExpr(n),
            NodeEnum::NextValueExpr(n) => NodeMut::NextValueExpr(n),
            NodeEnum::InferenceElem(n) => NodeMut::InferenceElem(n),
            NodeEnum::TargetEntry(n) => NodeMut::TargetEntry(n),
            NodeEnum::RangeTblRef(n) => NodeMut::RangeTblRef(n),
            NodeEnum::JoinExpr(n) => NodeMut::JoinExpr(n),
            NodeEnum::FromExpr(n) => NodeMut::FromExpr(n),
            NodeEnum::OnConflictExpr(n) => NodeMut::OnConflictExpr(n),
            NodeEnum::IntoClause(n) => NodeMut::IntoClause(n),
            NodeEnum::RawStmt(n) => NodeMut::RawStmt(n),
            NodeEnum::Query(n) => NodeMut::Query(n),
            NodeEnum::InsertStmt(n) => NodeMut::InsertStmt(n),
            NodeEnum::DeleteStmt(n) => NodeMut::DeleteStmt(n),
            NodeEnum::UpdateStmt(n) => NodeMut::UpdateStmt(n),
            NodeEnum::SelectStmt(n) => NodeMut::SelectStmt(n),
            NodeEnum::AlterTableStmt(n) => NodeMut::AlterTableStmt(n),
            NodeEnum::AlterTableCmd(n) => NodeMut::AlterTableCmd(n),
            NodeEnum::AlterDomainStmt(n) => NodeMut::AlterDomainStmt(n),
            NodeEnum::SetOperationStmt(n) => NodeMut::SetOperationStmt(n),
            NodeEnum::GrantStmt(n) => NodeMut::GrantStmt(n),
            NodeEnum::GrantRoleStmt(n) => NodeMut::GrantRoleStmt(n),
            NodeEnum::AlterDefaultPrivilegesStmt(n) => NodeMut::AlterDefaultPrivilegesStmt(n),
            NodeEnum::ClosePortalStmt(n) => NodeMut::ClosePortalStmt(n),
            NodeEnum::ClusterStmt(n) => NodeMut::ClusterStmt(n),
            NodeEnum::CopyStmt(n) => NodeMut::CopyStmt(n),
            NodeEnum::CreateStmt(n) => NodeMut::CreateStmt(n),
            NodeEnum::DefineStmt(n) => NodeMut::DefineStmt(n),
            NodeEnum::DropStmt(n) => NodeMut::DropStmt(n),
            NodeEnum::TruncateStmt(n) => NodeMut::TruncateStmt(n),
            NodeEnum::CommentStmt(n) => NodeMut::CommentStmt(n),
            NodeEnum::FetchStmt(n) => NodeMut::FetchStmt(n),
            NodeEnum::IndexStmt(n) => NodeMut::IndexStmt(n),
            NodeEnum::CreateFunctionStmt(n) => NodeMut::CreateFunctionStmt(n),
            NodeEnum::AlterFunctionStmt(n) => NodeMut::AlterFunctionStmt(n),
            NodeEnum::DoStmt(n) => NodeMut::DoStmt(n),
            NodeEnum::RenameStmt(n) => NodeMut::RenameStmt(n),
            NodeEnum::RuleStmt(n) => NodeMut::RuleStmt(n),
            NodeEnum::NotifyStmt(n) => NodeMut::NotifyStmt(n),
            NodeEnum::ListenStmt(n) => NodeMut::ListenStmt(n),
            NodeEnum::UnlistenStmt(n) => NodeMut::UnlistenStmt(n),
            NodeEnum::TransactionStmt(n) => NodeMut::TransactionStmt(n),
            NodeEnum::ViewStmt(n) => NodeMut::ViewStmt(n),
            NodeEnum::LoadStmt(n) => NodeMut::LoadStmt(n),
            NodeEnum::CreateDomainStmt(n) => NodeMut::CreateDomainStmt(n),
            NodeEnum::CreatedbStmt(n) => NodeMut::CreatedbStmt(n),
            NodeEnum::DropdbStmt(n) => NodeMut::DropdbStmt(n),
            NodeEnum::VacuumStmt(n) => NodeMut::VacuumStmt(n),
            NodeEnum::ExplainStmt(n) => NodeMut::ExplainStmt(n),
            NodeEnum::CreateTableAsStmt(n) => NodeMut::CreateTableAsStmt(n),
            NodeEnum::CreateSeqStmt(n) => NodeMut::CreateSeqStmt(n),
            NodeEnum::AlterSeqStmt(n) => NodeMut::AlterSeqStmt(n),
            NodeEnum::VariableSetStmt(n) => NodeMut::VariableSetStmt(n),
            NodeEnum::VariableShowStmt(n) => NodeMut::VariableShowStmt(n),
            NodeEnum::DiscardStmt(n) => NodeMut::DiscardStmt(n),
            NodeEnum::CreateTrigStmt(n) => NodeMut::CreateTrigStmt(n),
            NodeEnum::CreatePlangStmt(n) => NodeMut::CreatePlangStmt(n),
            NodeEnum::CreateRoleStmt(n) => NodeMut::CreateRoleStmt(n),
            NodeEnum::AlterRoleStmt(n) => NodeMut::AlterRoleStmt(n),
            NodeEnum::DropRoleStmt(n) => NodeMut::DropRoleStmt(n),
            NodeEnum::LockStmt(n) => NodeMut::LockStmt(n),
            NodeEnum::ConstraintsSetStmt(n) => NodeMut::ConstraintsSetStmt(n),
            NodeEnum::ReindexStmt(n) => NodeMut::ReindexStmt(n),
            NodeEnum::CheckPointStmt(n) => NodeMut::CheckPointStmt(n),
            NodeEnum::CreateSchemaStmt(n) => NodeMut::CreateSchemaStmt(n),
            NodeEnum::AlterDatabaseStmt(n) => NodeMut::AlterDatabaseStmt(n),
            NodeEnum::AlterDatabaseSetStmt(n) => NodeMut::AlterDatabaseSetStmt(n),
            NodeEnum::AlterRoleSetStmt(n) => NodeMut::AlterRoleSetStmt(n),
            NodeEnum::CreateConversionStmt(n) => NodeMut::CreateConversionStmt(n),
            NodeEnum::CreateCastStmt(n) => NodeMut::CreateCastStmt(n),
            NodeEnum::CreateOpClassStmt(n) => NodeMut::CreateOpClassStmt(n),
            NodeEnum::CreateOpFamilyStmt(n) => NodeMut::CreateOpFamilyStmt(n),
            NodeEnum::AlterOpFamilyStmt(n) => NodeMut::AlterOpFamilyStmt(n),
            NodeEnum::PrepareStmt(n) => NodeMut::PrepareStmt(n),
            NodeEnum::ExecuteStmt(n) => NodeMut::ExecuteStmt(n),
            NodeEnum::DeallocateStmt(n) => NodeMut::DeallocateStmt(n),
            NodeEnum::DeclareCursorStmt(n) => NodeMut::DeclareCursorStmt(n),
            NodeEnum::CreateTableSpaceStmt(n) => NodeMut::CreateTableSpaceStmt(n),
            NodeEnum::DropTableSpaceStmt(n) => NodeMut::DropTableSpaceStmt(n),
            NodeEnum::AlterObjectDependsStmt(n) => NodeMut::AlterObjectDependsStmt(n),
            NodeEnum::AlterObjectSchemaStmt(n) => NodeMut::AlterObjectSchemaStmt(n),
            NodeEnum::AlterOwnerStmt(n) => NodeMut::AlterOwnerStmt(n),
            NodeEnum::AlterOperatorStmt(n) => NodeMut::AlterOperatorStmt(n),
            NodeEnum::AlterTypeStmt(n) => NodeMut::AlterTypeStmt(n),
            NodeEnum::DropOwnedStmt(n) => NodeMut::DropOwnedStmt(n),
            NodeEnum::ReassignOwnedStmt(n) => NodeMut::ReassignOwnedStmt(n),
            NodeEnum::CompositeTypeStmt(n) => NodeMut::CompositeTypeStmt(n),
            NodeEnum::CreateEnumStmt(n) => NodeMut::CreateEnumStmt(n),
            NodeEnum::CreateRangeStmt(n) => NodeMut::CreateRangeStmt(n),
            NodeEnum::AlterEnumStmt(n) => NodeMut::AlterEnumStmt(n),
            NodeEnum::AlterTsdictionaryStmt(n) => NodeMut::AlterTsdictionaryStmt(n),
            NodeEnum::AlterTsconfigurationStmt(n) => NodeMut::AlterTsconfigurationStmt(n),
            NodeEnum::CreateFdwStmt(n) => NodeMut::CreateFdwStmt(n),
            NodeEnum::AlterFdwStmt(n) => NodeMut::AlterFdwStmt(n),
            NodeEnum::CreateForeignServerStmt(n) => NodeMut::CreateForeignServerStmt(n),
            NodeEnum::AlterForeignServerStmt(n) => NodeMut::AlterForeignServerStmt(n),
            NodeEnum::CreateUserMappingStmt(n) => NodeMut::CreateUserMappingStmt(n),
            NodeEnum::AlterUserMappingStmt(n) => NodeMut::AlterUserMappingStmt(n),
            NodeEnum::DropUserMappingStmt(n) => NodeMut::DropUserMappingStmt(n),
            NodeEnum::AlterTableSpaceOptionsStmt(n) => NodeMut::AlterTableSpaceOptionsStmt(n),
            NodeEnum::AlterTableMoveAllStmt(n) => NodeMut::AlterTableMoveAllStmt(n),
            NodeEnum::SecLabelStmt(n) => NodeMut::SecLabelStmt(n),
            NodeEnum::CreateForeignTableStmt(n) => NodeMut::CreateForeignTableStmt(n),
            NodeEnum::ImportForeignSchemaStmt(n) => NodeMut::ImportForeignSchemaStmt(n),
            NodeEnum::CreateExtensionStmt(n) => NodeMut::CreateExtensionStmt(n),
            NodeEnum::AlterExtensionStmt(n) => NodeMut::AlterExtensionStmt(n),
            NodeEnum::AlterExtensionContentsStmt(n) => NodeMut::AlterExtensionContentsStmt(n),
            NodeEnum::CreateEventTrigStmt(n) => NodeMut::CreateEventTrigStmt(n),
            NodeEnum::AlterEventTrigStmt(n) => NodeMut::AlterEventTrigStmt(n),
            NodeEnum::RefreshMatViewStmt(n) => NodeMut::RefreshMatViewStmt(n),
            NodeEnum::ReplicaIdentityStmt(n) => NodeMut::ReplicaIdentityStmt(n),
            NodeEnum::AlterSystemStmt(n) => NodeMut::AlterSystemStmt(n),
            NodeEnum::CreatePolicyStmt(n) => NodeMut::CreatePolicyStmt(n),
            NodeEnum::AlterPolicyStmt(n) => NodeMut::AlterPolicyStmt(n),
            NodeEnum::CreateTransformStmt(n) => NodeMut::CreateTransformStmt(n),
            NodeEnum::CreateAmStmt(n) => NodeMut::CreateAmStmt(n),
            NodeEnum::CreatePublicationStmt(n) => NodeMut::CreatePublicationStmt(n),
            NodeEnum::AlterPublicationStmt(n) => NodeMut::AlterPublicationStmt(n),
            NodeEnum::CreateSubscriptionStmt(n) => NodeMut::CreateSubscriptionStmt(n),
            NodeEnum::AlterSubscriptionStmt(n) => NodeMut::AlterSubscriptionStmt(n),
            NodeEnum::DropSubscriptionStmt(n) => NodeMut::DropSubscriptionStmt(n),
            NodeEnum::CreateStatsStmt(n) => NodeMut::CreateStatsStmt(n),
            NodeEnum::AlterCollationStmt(n) => NodeMut::AlterCollationStmt(n),
            NodeEnum::CallStmt(n) => NodeMut::CallStmt(n),
            NodeEnum::AlterStatsStmt(n) => NodeMut::AlterStatsStmt(n),
            NodeEnum::AExpr(n) => NodeMut::AExpr(n),
            NodeEnum::ColumnRef(n) => NodeMut::ColumnRef(n),
            NodeEnum::ParamRef(n) => NodeMut::ParamRef(n),
            NodeEnum::AConst(n) => NodeMut::AConst(n),
            NodeEnum::FuncCall(n) => NodeMut::FuncCall(n),
            NodeEnum::AStar(n) => NodeMut::AStar(n),
            NodeEnum::AIndices(n) => NodeMut::AIndices(n),
            NodeEnum::AIndirection(n) => NodeMut::AIndirection(n),
            NodeEnum::AArrayExpr(n) => NodeMut::AArrayExpr(n),
            NodeEnum::ResTarget(n) => NodeMut::ResTarget(n),
            NodeEnum::MultiAssignRef(n) => NodeMut::MultiAssignRef(n),
            NodeEnum::TypeCast(n) => NodeMut::TypeCast(n),
            NodeEnum::CollateClause(n) => NodeMut::CollateClause(n),
            NodeEnum::SortBy(n) => NodeMut::SortBy(n),
            NodeEnum::WindowDef(n) => NodeMut::WindowDef(n),
            NodeEnum::RangeSubselect(n) => NodeMut::RangeSubselect(n),
            NodeEnum::RangeFunction(n) => NodeMut::RangeFunction(n),
            NodeEnum::RangeTableSample(n) => NodeMut::RangeTableSample(n),
            NodeEnum::RangeTableFunc(n) => NodeMut::RangeTableFunc(n),
            NodeEnum::RangeTableFuncCol(n) => NodeMut::RangeTableFuncCol(n),
            NodeEnum::TypeName(n) => NodeMut::TypeName(n),
            NodeEnum::ColumnDef(n) => NodeMut::ColumnDef(n),
            NodeEnum::IndexElem(n) => NodeMut::IndexElem(n),
            NodeEnum::Constraint(n) => NodeMut::Constraint(n),
            NodeEnum::DefElem(n) => NodeMut::DefElem(n),
            NodeEnum::RangeTblEntry(n) => NodeMut::RangeTblEntry(n),
            NodeEnum::RangeTblFunction(n) => NodeMut::RangeTblFunction(n),
            NodeEnum::TableSampleClause(n) => NodeMut::TableSampleClause(n),
            NodeEnum::WithCheckOption(n) => NodeMut::WithCheckOption(n),
            NodeEnum::SortGroupClause(n) => NodeMut::SortGroupClause(n),
            NodeEnum::GroupingSet(n) => NodeMut::GroupingSet(n),
            NodeEnum::WindowClause(n) => NodeMut::WindowClause(n),
            NodeEnum::ObjectWithArgs(n) => NodeMut::ObjectWithArgs(n),
            NodeEnum::AccessPriv(n) => NodeMut::AccessPriv(n),
            NodeEnum::CreateOpClassItem(n) => NodeMut::CreateOpClassItem(n),
            NodeEnum::TableLikeClause(n) => NodeMut::TableLikeClause(n),
            NodeEnum::FunctionParameter(n) => NodeMut::FunctionParameter(n),
            NodeEnum::LockingClause(n) => NodeMut::LockingClause(n),
            NodeEnum::RowMarkClause(n) => NodeMut::RowMarkClause(n),
            NodeEnum::XmlSerialize(n) => NodeMut::XmlSerialize(n),
            NodeEnum::WithClause(n) => NodeMut::WithClause(n),
            NodeEnum::InferClause(n) => NodeMut::InferClause(n),
            NodeEnum::OnConflictClause(n) => NodeMut::OnConflictClause(n),
            NodeEnum::CommonTableExpr(n) => NodeMut::CommonTableExpr(n),
            NodeEnum::RoleSpec(n) => NodeMut::RoleSpec(n),
            NodeEnum::TriggerTransition(n) => NodeMut::TriggerTransition(n),
            NodeEnum::PartitionElem(n) => NodeMut::PartitionElem(n),
            NodeEnum::PartitionSpec(n) => NodeMut::PartitionSpec(n),
            NodeEnum::PartitionBoundSpec(n) => NodeMut::PartitionBoundSpec(n),
            NodeEnum::PartitionRangeDatum(n) => NodeMut::PartitionRangeDatum(n),
            NodeEnum::PartitionCmd(n) => NodeMut::PartitionCmd(n),
            NodeEnum::VacuumRelation(n) => NodeMut::VacuumRelation(n),
            NodeEnum::InlineCodeBlock(n) => NodeMut::InlineCodeBlock(n),
            NodeEnum::CallContext(n) => NodeMut::CallContext(n),
            NodeEnum::Integer(n) => NodeMut::Integer(n),
            NodeEnum::Float(n) => NodeMut::Float(n),
            NodeEnum::String(n) => NodeMut::String(n),
            NodeEnum::BitString(n) => NodeMut::BitString(n),
            NodeEnum::Null(n) => NodeMut::Null(n),
            NodeEnum::List(n) => NodeMut::List(n),
            NodeEnum::IntList(n) => NodeMut::IntList(n),
            NodeEnum::OidList(n) => NodeMut::OidList(n),
        }
    }
}
