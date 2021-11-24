use std::cell::UnsafeCell;
use std::cmp::Ordering;

use crate::*;

#[derive(Debug)]
enum TruncationAttr {
    TargetList,
    WhereClause,
    CTEQuery,
    Cols,
}

#[derive(Debug)]
struct PossibleTruncation<'a> {
    attr: TruncationAttr,
    node: NodeRef<'a>,
    depth: i32,
    length: i32,
}

pub fn truncate(protobuf: &protobuf::ParseResult, max_length: usize) -> Result<String> {
    let output = protobuf.deparse()?;
    if output.len() <= max_length {
        return Ok(output)
    }

    unsafe {
        // SAFETY: within this scope nobody expects to have exclusive access to `protobuf`'s contents,
        // so we can have multiple shared accesses. https://doc.rust-lang.org/stable/std/cell/struct.UnsafeCell.html
        let mut protobuf = UnsafeCell::new(protobuf.clone());
        let protobuf_ref = &*protobuf.get();
        let protobuf_mut = &mut *protobuf.get_mut();

        let mut truncations: Vec<PossibleTruncation> = Vec::new();
        for (node, depth, _context) in protobuf_ref.nodes().into_iter() {
            match node {
                NodeRef::SelectStmt(s) => {
                    truncations.push(PossibleTruncation {
                        attr: TruncationAttr::TargetList, node: node, depth: depth,
                        length: target_list_len(s.target_list.clone())
                    });
                    if let Some(clause) = s.where_clause.as_ref() {
                        truncations.push(PossibleTruncation {
                            attr: TruncationAttr::WhereClause, node: node, depth: depth,
                            length: where_clause_len((*clause).clone())
                        });
                    }
                }
                NodeRef::UpdateStmt(s) => {
                    truncations.push(PossibleTruncation {
                        attr: TruncationAttr::TargetList, node: node, depth: depth,
                        length: target_list_len(s.target_list.clone())
                    });
                    if let Some(clause) = s.where_clause.as_ref() {
                        truncations.push(PossibleTruncation {
                            attr: TruncationAttr::WhereClause, node: node, depth: depth,
                            length: where_clause_len((*clause).clone())
                        });
                    }
                }
                NodeRef::DeleteStmt(s) => {
                    if let Some(clause) = s.where_clause.as_ref() {
                        truncations.push(PossibleTruncation {
                            attr: TruncationAttr::WhereClause, node: node, depth: depth,
                            length: where_clause_len((*clause).clone())
                        });
                    }
                }
                NodeRef::CopyStmt(s) => {
                    if let Some(clause) = s.where_clause.as_ref() {
                        truncations.push(PossibleTruncation {
                            attr: TruncationAttr::WhereClause, node: node, depth: depth,
                            length: where_clause_len((*clause).clone())
                        });
                    }
                }
                NodeRef::InsertStmt(s) => {
                    if s.cols.len() > 0 {
                        truncations.push(PossibleTruncation {
                            attr: TruncationAttr::Cols, node: node, depth: depth,
                            length: cols_len(s.cols.clone())
                        });
                    }
                }
                NodeRef::IndexStmt(s) => {
                    if let Some(clause) = s.where_clause.as_ref() {
                        truncations.push(PossibleTruncation {
                            attr: TruncationAttr::WhereClause, node: node, depth: depth,
                            length: where_clause_len((*clause).clone())
                        });
                    }
                }
                NodeRef::RuleStmt(s) => {
                    if let Some(clause) = s.where_clause.as_ref() {
                        truncations.push(PossibleTruncation {
                            attr: TruncationAttr::WhereClause, node: node, depth: depth,
                            length: where_clause_len((*clause).clone())
                        });
                    }
                }
                NodeRef::CommonTableExpr(c) => {
                    if let Some(cte) = c.ctequery.as_ref() {
                        truncations.push(PossibleTruncation {
                            attr: TruncationAttr::CTEQuery, node: node, depth: depth + 1,
                            length: cte.deparse().unwrap().len() as i32
                        });
                    }
                }
                NodeRef::InferClause(s) => {
                    if let Some(clause) = s.where_clause.as_ref() {
                        truncations.push(PossibleTruncation {
                            attr: TruncationAttr::WhereClause, node: node, depth: depth,
                            length: where_clause_len((*clause).clone())
                        });
                    }
                }
                NodeRef::OnConflictClause(s) => {
                    truncations.push(PossibleTruncation {
                        attr: TruncationAttr::TargetList, node: node, depth: depth,
                        length: target_list_len(s.target_list.clone())
                    });
                    if let Some(clause) = s.where_clause.as_ref() {
                        truncations.push(PossibleTruncation {
                            attr: TruncationAttr::WhereClause, node: node, depth: depth,
                            length: where_clause_len((*clause).clone())
                        });
                    }
                }
                _ => ()
            }
        }

        truncations.sort_by(|a, b| {
            match a.depth.cmp(&b.depth).reverse() {
                Ordering::Equal => a.length.cmp(&b.length).reverse(),
                other => other,
            }
        });

        for truncation in truncations.into_iter() {
            let node = protobuf_mut.find_mut(truncation.node);
            match (node, truncation.attr) {
                (NodeMut::SelectStmt(ref mut n), TruncationAttr::TargetList) => {
                    n.target_list = vec![dummy_target()];
                }
                (NodeMut::SelectStmt(ref mut n), TruncationAttr::WhereClause) => {
                    n.where_clause = Some(dummy_column());
                }
                (NodeMut::UpdateStmt(ref mut n), TruncationAttr::TargetList) => {
                    n.target_list = vec![dummy_target()];
                }
                (NodeMut::UpdateStmt(ref mut n), TruncationAttr::WhereClause) => {
                    n.where_clause = Some(dummy_column());
                }
                (NodeMut::DeleteStmt(ref mut n), TruncationAttr::WhereClause) => {
                    n.where_clause = Some(dummy_column());
                }
                (NodeMut::CopyStmt(ref mut n), TruncationAttr::WhereClause) => {
                    n.where_clause = Some(dummy_column());
                }
                (NodeMut::InsertStmt(ref mut n), TruncationAttr::Cols) => {
                    n.cols = vec![dummy_target()];
                }
                (NodeMut::IndexStmt(ref mut n), TruncationAttr::WhereClause) => {
                    n.where_clause = Some(dummy_column());
                }
                (NodeMut::RuleStmt(ref mut n), TruncationAttr::WhereClause) => {
                    n.where_clause = Some(dummy_column());
                }
                (NodeMut::CommonTableExpr(ref mut n), TruncationAttr::CTEQuery) => {
                    n.ctequery = Some(dummy_select(vec![], Some(dummy_column())))
                }
                (NodeMut::InferClause(ref mut n), TruncationAttr::WhereClause) => {
                    n.where_clause = Some(dummy_column());
                }
                (NodeMut::OnConflictClause(ref mut n), TruncationAttr::TargetList) => {
                    n.target_list = vec![dummy_target()];
                }
                (NodeMut::OnConflictClause(ref mut n), TruncationAttr::WhereClause) => {
                    n.where_clause = Some(dummy_column());
                }
                _ => panic!("unimplemented truncation")
            }
            let output = protobuf_ref.deparse()?;
            let output = output.replace("SELECT WHERE \"…\"", "...").replace("\"…\"", "...");
            // the unwanted AS doesn't happen in the Ruby version, and I'm not sure why it is here
            let output = output.replace("SELECT ... AS ...", "SELECT ...");
            if output.len() <= max_length {
                return Ok(output)
            }
        }
    }

    // We couldn't do a proper smart truncation, so we need a hard cut-off
    return Ok(format!("{}...", &output[0..=max_length - 4]))
}


fn dummy_column() -> Box<Node> {
    Box::new(Node { node: Some(NodeEnum::ColumnRef(protobuf::ColumnRef {
        location: 0, fields: vec![
            Node { node: Some(NodeEnum::String(protobuf::String { str: "…".to_string() }))}
        ]
    }))})
}

fn dummy_target() -> Node {
    Node { node: Some(NodeEnum::ResTarget(Box::new(protobuf::ResTarget {
        name: "…".to_string(), location: 0, indirection: vec![], val: Some(dummy_column())
    })))}
}

fn dummy_select(target_list: Vec<Node>, where_clause: Option<Box<Node>>) -> Box<Node> {
    Box::new(Node { node: Some(NodeEnum::SelectStmt(Box::new(protobuf::SelectStmt {
        distinct_clause: vec![], into_clause: None, target_list: target_list, from_clause: vec![],
        where_clause: where_clause, group_clause: vec![], having_clause: None,
        window_clause: vec![], values_lists: vec![], sort_clause: vec![],
        limit_offset: None, limit_count: None, limit_option: 1,
        locking_clause: vec![], with_clause: None, op: 1,
        all: false, larg: None, rarg: None
    })))})
}

fn dummy_insert(cols: Vec<Node>) -> Box<Node> {
    Box::new(Node { node: Some(NodeEnum::InsertStmt(Box::new(protobuf::InsertStmt {
        relation: Some(protobuf::RangeVar {
            catalogname: "".to_string(),
            schemaname: "".to_string(),
            relname: "x".to_string(),
            inh: true,
            relpersistence: "p".to_string(),
            alias: None,
            location: 0
        }),
        cols: cols,
        select_stmt: None,
        on_conflict_clause: None,
        returning_list: vec![],
        with_clause: None,
        r#override: 1,
    })))})
}

fn target_list_len(nodes: Vec<Node>) -> i32 {
    let fragment = dummy_select(nodes, None).deparse().unwrap();
    fragment.len() as i32 - 7 // "SELECT "
}

fn where_clause_len(node: Box<Node>) -> i32 {
    let fragment = dummy_select(vec![], Some(node)).deparse().unwrap();
    fragment.len() as i32 - 13 // "SELECT WHERE "
}

fn cols_len(nodes: Vec<Node>) -> i32 {
    let fragment = dummy_insert(nodes).deparse().unwrap();
    fragment.len() as i32 - 31 // "INSERT INTO x () DEFAULT VALUES"
}
