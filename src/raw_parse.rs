//! Direct parsing that bypasses protobuf serialization/deserialization.
//!
//! This module provides a faster alternative to the protobuf-based parsing by
//! directly reading PostgreSQL's internal parse tree structures and converting
//! them to Rust protobuf types.

use crate::bindings;
use crate::bindings_raw;
use crate::parse_result::ParseResult;
use crate::protobuf;
use crate::{Error, Result};
use std::ffi::{CStr, CString};

/// Parses a SQL statement directly into protobuf types without going through protobuf serialization.
///
/// This function is faster than `parse` because it skips the protobuf encode/decode step.
/// The parse tree is read directly from PostgreSQL's internal C structures.
///
/// # Example
///
/// ```rust
/// let result = pg_query::parse_raw("SELECT * FROM users").unwrap();
/// assert_eq!(result.tables(), vec!["users"]);
/// ```
pub fn parse_raw(statement: &str) -> Result<ParseResult> {
    let input = CString::new(statement)?;
    let result = unsafe { bindings_raw::pg_query_parse_raw(input.as_ptr()) };

    let parse_result = if !result.error.is_null() {
        let message = unsafe { CStr::from_ptr((*result.error).message) }
            .to_string_lossy()
            .to_string();
        Err(Error::Parse(message))
    } else {
        // Convert the C parse tree to protobuf types
        let tree = result.tree;
        let stmts = unsafe { convert_list_to_raw_stmts(tree) };
        let protobuf = protobuf::ParseResult {
            version: bindings::PG_VERSION_NUM as i32,
            stmts,
        };
        Ok(ParseResult::new(protobuf, String::new()))
    };

    unsafe { bindings_raw::pg_query_free_raw_parse_result(result) };
    parse_result
}

/// Converts a PostgreSQL List of RawStmt nodes to protobuf RawStmt vector.
unsafe fn convert_list_to_raw_stmts(list: *mut bindings_raw::List) -> Vec<protobuf::RawStmt> {
    if list.is_null() {
        return Vec::new();
    }

    let list_ref = &*list;
    let length = list_ref.length as usize;
    let mut stmts = Vec::with_capacity(length);

    for i in 0..length {
        let cell = list_ref.elements.add(i);
        let node_ptr = (*cell).ptr_value as *mut bindings_raw::Node;

        if !node_ptr.is_null() {
            let node_tag = (*node_ptr).type_;
            if node_tag == bindings_raw::NodeTag_T_RawStmt {
                let raw_stmt = node_ptr as *mut bindings_raw::RawStmt;
                stmts.push(convert_raw_stmt(&*raw_stmt));
            }
        }
    }

    stmts
}

/// Converts a C RawStmt to a protobuf RawStmt.
unsafe fn convert_raw_stmt(raw_stmt: &bindings_raw::RawStmt) -> protobuf::RawStmt {
    protobuf::RawStmt {
        stmt: convert_node_boxed(raw_stmt.stmt),
        stmt_location: raw_stmt.stmt_location,
        stmt_len: raw_stmt.stmt_len,
    }
}

/// Converts a C Node pointer to a boxed protobuf Node (for fields that expect Option<Box<Node>>).
unsafe fn convert_node_boxed(node_ptr: *mut bindings_raw::Node) -> Option<Box<protobuf::Node>> {
    convert_node(node_ptr).map(Box::new)
}

/// Converts a C Node pointer to a protobuf Node.
unsafe fn convert_node(node_ptr: *mut bindings_raw::Node) -> Option<protobuf::Node> {
    if node_ptr.is_null() {
        return None;
    }

    let node_tag = (*node_ptr).type_;
    let node = match node_tag {
        // Types that need Box
        bindings_raw::NodeTag_T_SelectStmt => {
            let stmt = node_ptr as *mut bindings_raw::SelectStmt;
            Some(protobuf::node::Node::SelectStmt(Box::new(convert_select_stmt(&*stmt))))
        }
        bindings_raw::NodeTag_T_InsertStmt => {
            let stmt = node_ptr as *mut bindings_raw::InsertStmt;
            Some(protobuf::node::Node::InsertStmt(Box::new(convert_insert_stmt(&*stmt))))
        }
        bindings_raw::NodeTag_T_UpdateStmt => {
            let stmt = node_ptr as *mut bindings_raw::UpdateStmt;
            Some(protobuf::node::Node::UpdateStmt(Box::new(convert_update_stmt(&*stmt))))
        }
        bindings_raw::NodeTag_T_DeleteStmt => {
            let stmt = node_ptr as *mut bindings_raw::DeleteStmt;
            Some(protobuf::node::Node::DeleteStmt(Box::new(convert_delete_stmt(&*stmt))))
        }
        bindings_raw::NodeTag_T_ResTarget => {
            let rt = node_ptr as *mut bindings_raw::ResTarget;
            Some(protobuf::node::Node::ResTarget(Box::new(convert_res_target(&*rt))))
        }
        bindings_raw::NodeTag_T_A_Expr => {
            let expr = node_ptr as *mut bindings_raw::A_Expr;
            Some(protobuf::node::Node::AExpr(Box::new(convert_a_expr(&*expr))))
        }
        bindings_raw::NodeTag_T_A_Const => {
            let aconst = node_ptr as *mut bindings_raw::A_Const;
            Some(protobuf::node::Node::AConst(convert_a_const(&*aconst)))
        }
        bindings_raw::NodeTag_T_FuncCall => {
            let fc = node_ptr as *mut bindings_raw::FuncCall;
            Some(protobuf::node::Node::FuncCall(Box::new(convert_func_call(&*fc))))
        }
        bindings_raw::NodeTag_T_TypeCast => {
            let tc = node_ptr as *mut bindings_raw::TypeCast;
            Some(protobuf::node::Node::TypeCast(Box::new(convert_type_cast(&*tc))))
        }
        bindings_raw::NodeTag_T_JoinExpr => {
            let je = node_ptr as *mut bindings_raw::JoinExpr;
            Some(protobuf::node::Node::JoinExpr(Box::new(convert_join_expr(&*je))))
        }
        bindings_raw::NodeTag_T_SortBy => {
            let sb = node_ptr as *mut bindings_raw::SortBy;
            Some(protobuf::node::Node::SortBy(Box::new(convert_sort_by(&*sb))))
        }
        bindings_raw::NodeTag_T_BoolExpr => {
            let be = node_ptr as *mut bindings_raw::BoolExpr;
            Some(protobuf::node::Node::BoolExpr(Box::new(convert_bool_expr(&*be))))
        }
        bindings_raw::NodeTag_T_SubLink => {
            let sl = node_ptr as *mut bindings_raw::SubLink;
            Some(protobuf::node::Node::SubLink(Box::new(convert_sub_link(&*sl))))
        }
        bindings_raw::NodeTag_T_NullTest => {
            let nt = node_ptr as *mut bindings_raw::NullTest;
            Some(protobuf::node::Node::NullTest(Box::new(convert_null_test(&*nt))))
        }
        bindings_raw::NodeTag_T_CaseExpr => {
            let ce = node_ptr as *mut bindings_raw::CaseExpr;
            Some(protobuf::node::Node::CaseExpr(Box::new(convert_case_expr(&*ce))))
        }
        bindings_raw::NodeTag_T_CaseWhen => {
            let cw = node_ptr as *mut bindings_raw::CaseWhen;
            Some(protobuf::node::Node::CaseWhen(Box::new(convert_case_when(&*cw))))
        }
        bindings_raw::NodeTag_T_CoalesceExpr => {
            let ce = node_ptr as *mut bindings_raw::CoalesceExpr;
            Some(protobuf::node::Node::CoalesceExpr(Box::new(convert_coalesce_expr(&*ce))))
        }
        bindings_raw::NodeTag_T_CommonTableExpr => {
            let cte = node_ptr as *mut bindings_raw::CommonTableExpr;
            Some(protobuf::node::Node::CommonTableExpr(Box::new(convert_common_table_expr(&*cte))))
        }
        bindings_raw::NodeTag_T_ColumnDef => {
            let cd = node_ptr as *mut bindings_raw::ColumnDef;
            Some(protobuf::node::Node::ColumnDef(Box::new(convert_column_def(&*cd))))
        }
        bindings_raw::NodeTag_T_Constraint => {
            let c = node_ptr as *mut bindings_raw::Constraint;
            Some(protobuf::node::Node::Constraint(Box::new(convert_constraint(&*c))))
        }
        bindings_raw::NodeTag_T_DropStmt => {
            let ds = node_ptr as *mut bindings_raw::DropStmt;
            Some(protobuf::node::Node::DropStmt(convert_drop_stmt(&*ds)))
        }
        bindings_raw::NodeTag_T_IndexStmt => {
            let is = node_ptr as *mut bindings_raw::IndexStmt;
            Some(protobuf::node::Node::IndexStmt(Box::new(convert_index_stmt(&*is))))
        }
        bindings_raw::NodeTag_T_IndexElem => {
            let ie = node_ptr as *mut bindings_raw::IndexElem;
            Some(protobuf::node::Node::IndexElem(Box::new(convert_index_elem(&*ie))))
        }
        bindings_raw::NodeTag_T_DefElem => {
            let de = node_ptr as *mut bindings_raw::DefElem;
            Some(protobuf::node::Node::DefElem(Box::new(convert_def_elem(&*de))))
        }
        bindings_raw::NodeTag_T_WindowDef => {
            let wd = node_ptr as *mut bindings_raw::WindowDef;
            Some(protobuf::node::Node::WindowDef(Box::new(convert_window_def(&*wd))))
        }
        // Types that don't need Box
        bindings_raw::NodeTag_T_RangeVar => {
            let rv = node_ptr as *mut bindings_raw::RangeVar;
            Some(protobuf::node::Node::RangeVar(convert_range_var(&*rv)))
        }
        bindings_raw::NodeTag_T_ColumnRef => {
            let cr = node_ptr as *mut bindings_raw::ColumnRef;
            Some(protobuf::node::Node::ColumnRef(convert_column_ref(&*cr)))
        }
        bindings_raw::NodeTag_T_A_Star => {
            Some(protobuf::node::Node::AStar(protobuf::AStar {}))
        }
        bindings_raw::NodeTag_T_TypeName => {
            let tn = node_ptr as *mut bindings_raw::TypeName;
            Some(protobuf::node::Node::TypeName(convert_type_name(&*tn)))
        }
        bindings_raw::NodeTag_T_Alias => {
            let alias = node_ptr as *mut bindings_raw::Alias;
            Some(protobuf::node::Node::Alias(convert_alias(&*alias)))
        }
        bindings_raw::NodeTag_T_String => {
            let s = node_ptr as *mut bindings_raw::String;
            Some(protobuf::node::Node::String(convert_string(&*s)))
        }
        bindings_raw::NodeTag_T_Integer => {
            let i = node_ptr as *mut bindings_raw::Integer;
            Some(protobuf::node::Node::Integer(protobuf::Integer { ival: (*i).ival }))
        }
        bindings_raw::NodeTag_T_Float => {
            let f = node_ptr as *mut bindings_raw::Float;
            let fval = if (*f).fval.is_null() {
                String::new()
            } else {
                CStr::from_ptr((*f).fval).to_string_lossy().to_string()
            };
            Some(protobuf::node::Node::Float(protobuf::Float { fval }))
        }
        bindings_raw::NodeTag_T_Boolean => {
            let b = node_ptr as *mut bindings_raw::Boolean;
            Some(protobuf::node::Node::Boolean(protobuf::Boolean { boolval: (*b).boolval }))
        }
        bindings_raw::NodeTag_T_ParamRef => {
            let pr = node_ptr as *mut bindings_raw::ParamRef;
            Some(protobuf::node::Node::ParamRef(protobuf::ParamRef {
                number: (*pr).number,
                location: (*pr).location,
            }))
        }
        bindings_raw::NodeTag_T_WithClause => {
            let wc = node_ptr as *mut bindings_raw::WithClause;
            Some(protobuf::node::Node::WithClause(convert_with_clause(&*wc)))
        }
        bindings_raw::NodeTag_T_CreateStmt => {
            let cs = node_ptr as *mut bindings_raw::CreateStmt;
            Some(protobuf::node::Node::CreateStmt(convert_create_stmt(&*cs)))
        }
        bindings_raw::NodeTag_T_List => {
            let list = node_ptr as *mut bindings_raw::List;
            Some(protobuf::node::Node::List(convert_list(&*list)))
        }
        bindings_raw::NodeTag_T_LockingClause => {
            let lc = node_ptr as *mut bindings_raw::LockingClause;
            Some(protobuf::node::Node::LockingClause(convert_locking_clause(&*lc)))
        }
        bindings_raw::NodeTag_T_MinMaxExpr => {
            let mme = node_ptr as *mut bindings_raw::MinMaxExpr;
            Some(protobuf::node::Node::MinMaxExpr(Box::new(convert_min_max_expr(&*mme))))
        }
        bindings_raw::NodeTag_T_GroupingSet => {
            let gs = node_ptr as *mut bindings_raw::GroupingSet;
            Some(protobuf::node::Node::GroupingSet(convert_grouping_set(&*gs)))
        }
        bindings_raw::NodeTag_T_RangeSubselect => {
            let rs = node_ptr as *mut bindings_raw::RangeSubselect;
            Some(protobuf::node::Node::RangeSubselect(Box::new(convert_range_subselect(&*rs))))
        }
        bindings_raw::NodeTag_T_A_ArrayExpr => {
            let ae = node_ptr as *mut bindings_raw::A_ArrayExpr;
            Some(protobuf::node::Node::AArrayExpr(convert_a_array_expr(&*ae)))
        }
        bindings_raw::NodeTag_T_A_Indirection => {
            let ai = node_ptr as *mut bindings_raw::A_Indirection;
            Some(protobuf::node::Node::AIndirection(Box::new(convert_a_indirection(&*ai))))
        }
        bindings_raw::NodeTag_T_A_Indices => {
            let ai = node_ptr as *mut bindings_raw::A_Indices;
            Some(protobuf::node::Node::AIndices(Box::new(convert_a_indices(&*ai))))
        }
        bindings_raw::NodeTag_T_AlterTableStmt => {
            let ats = node_ptr as *mut bindings_raw::AlterTableStmt;
            Some(protobuf::node::Node::AlterTableStmt(convert_alter_table_stmt(&*ats)))
        }
        bindings_raw::NodeTag_T_AlterTableCmd => {
            let atc = node_ptr as *mut bindings_raw::AlterTableCmd;
            Some(protobuf::node::Node::AlterTableCmd(Box::new(convert_alter_table_cmd(&*atc))))
        }
        bindings_raw::NodeTag_T_CopyStmt => {
            let cs = node_ptr as *mut bindings_raw::CopyStmt;
            Some(protobuf::node::Node::CopyStmt(Box::new(convert_copy_stmt(&*cs))))
        }
        bindings_raw::NodeTag_T_TruncateStmt => {
            let ts = node_ptr as *mut bindings_raw::TruncateStmt;
            Some(protobuf::node::Node::TruncateStmt(convert_truncate_stmt(&*ts)))
        }
        bindings_raw::NodeTag_T_ViewStmt => {
            let vs = node_ptr as *mut bindings_raw::ViewStmt;
            Some(protobuf::node::Node::ViewStmt(Box::new(convert_view_stmt(&*vs))))
        }
        bindings_raw::NodeTag_T_ExplainStmt => {
            let es = node_ptr as *mut bindings_raw::ExplainStmt;
            Some(protobuf::node::Node::ExplainStmt(Box::new(convert_explain_stmt(&*es))))
        }
        bindings_raw::NodeTag_T_CreateTableAsStmt => {
            let ctas = node_ptr as *mut bindings_raw::CreateTableAsStmt;
            Some(protobuf::node::Node::CreateTableAsStmt(Box::new(convert_create_table_as_stmt(&*ctas))))
        }
        bindings_raw::NodeTag_T_PrepareStmt => {
            let ps = node_ptr as *mut bindings_raw::PrepareStmt;
            Some(protobuf::node::Node::PrepareStmt(Box::new(convert_prepare_stmt(&*ps))))
        }
        bindings_raw::NodeTag_T_ExecuteStmt => {
            let es = node_ptr as *mut bindings_raw::ExecuteStmt;
            Some(protobuf::node::Node::ExecuteStmt(convert_execute_stmt(&*es)))
        }
        bindings_raw::NodeTag_T_DeallocateStmt => {
            let ds = node_ptr as *mut bindings_raw::DeallocateStmt;
            Some(protobuf::node::Node::DeallocateStmt(convert_deallocate_stmt(&*ds)))
        }
        bindings_raw::NodeTag_T_SetToDefault => {
            let std = node_ptr as *mut bindings_raw::SetToDefault;
            Some(protobuf::node::Node::SetToDefault(Box::new(convert_set_to_default(&*std))))
        }
        bindings_raw::NodeTag_T_MultiAssignRef => {
            let mar = node_ptr as *mut bindings_raw::MultiAssignRef;
            Some(protobuf::node::Node::MultiAssignRef(Box::new(convert_multi_assign_ref(&*mar))))
        }
        bindings_raw::NodeTag_T_RowExpr => {
            let re = node_ptr as *mut bindings_raw::RowExpr;
            Some(protobuf::node::Node::RowExpr(Box::new(convert_row_expr(&*re))))
        }
        _ => {
            // For unhandled node types, return None
            // In the future, we could add more node types here
            None
        }
    };

    node.map(|n| protobuf::Node { node: Some(n) })
}

/// Converts a PostgreSQL List to a protobuf List of Nodes.
unsafe fn convert_list(list: &bindings_raw::List) -> protobuf::List {
    let items = convert_list_to_nodes(list as *const bindings_raw::List as *mut bindings_raw::List);
    protobuf::List { items }
}

/// Converts a PostgreSQL List pointer to a Vec of protobuf Nodes.
unsafe fn convert_list_to_nodes(list: *mut bindings_raw::List) -> Vec<protobuf::Node> {
    if list.is_null() {
        return Vec::new();
    }

    let list_ref = &*list;
    let length = list_ref.length as usize;
    let mut nodes = Vec::with_capacity(length);

    for i in 0..length {
        let cell = list_ref.elements.add(i);
        let node_ptr = (*cell).ptr_value as *mut bindings_raw::Node;

        if let Some(node) = convert_node(node_ptr) {
            nodes.push(node);
        }
    }

    nodes
}

// ============================================================================
// Statement Conversions
// ============================================================================

unsafe fn convert_select_stmt(stmt: &bindings_raw::SelectStmt) -> protobuf::SelectStmt {
    protobuf::SelectStmt {
        distinct_clause: convert_list_to_nodes(stmt.distinctClause),
        into_clause: convert_into_clause(stmt.intoClause),
        target_list: convert_list_to_nodes(stmt.targetList),
        from_clause: convert_list_to_nodes(stmt.fromClause),
        where_clause: convert_node_boxed(stmt.whereClause),
        group_clause: convert_list_to_nodes(stmt.groupClause),
        group_distinct: stmt.groupDistinct,
        having_clause: convert_node_boxed(stmt.havingClause),
        window_clause: convert_list_to_nodes(stmt.windowClause),
        values_lists: convert_list_to_nodes(stmt.valuesLists),
        sort_clause: convert_list_to_nodes(stmt.sortClause),
        limit_offset: convert_node_boxed(stmt.limitOffset),
        limit_count: convert_node_boxed(stmt.limitCount),
        limit_option: stmt.limitOption as i32 + 1, // Protobuf enums have UNDEFINED=0, so C values need +1
        locking_clause: convert_list_to_nodes(stmt.lockingClause),
        with_clause: convert_with_clause_opt(stmt.withClause),
        op: stmt.op as i32 + 1, // Protobuf SetOperation has UNDEFINED=0, so C values need +1
        all: stmt.all,
        larg: if stmt.larg.is_null() { None } else { Some(Box::new(convert_select_stmt(&*stmt.larg))) },
        rarg: if stmt.rarg.is_null() { None } else { Some(Box::new(convert_select_stmt(&*stmt.rarg))) },
    }
}

unsafe fn convert_insert_stmt(stmt: &bindings_raw::InsertStmt) -> protobuf::InsertStmt {
    protobuf::InsertStmt {
        relation: if stmt.relation.is_null() { None } else { Some(convert_range_var(&*stmt.relation)) },
        cols: convert_list_to_nodes(stmt.cols),
        select_stmt: convert_node_boxed(stmt.selectStmt),
        on_conflict_clause: convert_on_conflict_clause(stmt.onConflictClause),
        returning_list: convert_list_to_nodes(stmt.returningList),
        with_clause: convert_with_clause_opt(stmt.withClause),
        r#override: stmt.override_ as i32 + 1,
    }
}

unsafe fn convert_update_stmt(stmt: &bindings_raw::UpdateStmt) -> protobuf::UpdateStmt {
    protobuf::UpdateStmt {
        relation: if stmt.relation.is_null() { None } else { Some(convert_range_var(&*stmt.relation)) },
        target_list: convert_list_to_nodes(stmt.targetList),
        where_clause: convert_node_boxed(stmt.whereClause),
        from_clause: convert_list_to_nodes(stmt.fromClause),
        returning_list: convert_list_to_nodes(stmt.returningList),
        with_clause: convert_with_clause_opt(stmt.withClause),
    }
}

unsafe fn convert_delete_stmt(stmt: &bindings_raw::DeleteStmt) -> protobuf::DeleteStmt {
    protobuf::DeleteStmt {
        relation: if stmt.relation.is_null() { None } else { Some(convert_range_var(&*stmt.relation)) },
        using_clause: convert_list_to_nodes(stmt.usingClause),
        where_clause: convert_node_boxed(stmt.whereClause),
        returning_list: convert_list_to_nodes(stmt.returningList),
        with_clause: convert_with_clause_opt(stmt.withClause),
    }
}

unsafe fn convert_create_stmt(stmt: &bindings_raw::CreateStmt) -> protobuf::CreateStmt {
    protobuf::CreateStmt {
        relation: if stmt.relation.is_null() { None } else { Some(convert_range_var(&*stmt.relation)) },
        table_elts: convert_list_to_nodes(stmt.tableElts),
        inh_relations: convert_list_to_nodes(stmt.inhRelations),
        partbound: convert_partition_bound_spec_opt(stmt.partbound),
        partspec: convert_partition_spec_opt(stmt.partspec),
        of_typename: if stmt.ofTypename.is_null() { None } else { Some(convert_type_name(&*stmt.ofTypename)) },
        constraints: convert_list_to_nodes(stmt.constraints),
        options: convert_list_to_nodes(stmt.options),
        oncommit: stmt.oncommit as i32 + 1,
        tablespacename: convert_c_string(stmt.tablespacename),
        access_method: convert_c_string(stmt.accessMethod),
        if_not_exists: stmt.if_not_exists,
    }
}

unsafe fn convert_drop_stmt(stmt: &bindings_raw::DropStmt) -> protobuf::DropStmt {
    protobuf::DropStmt {
        objects: convert_list_to_nodes(stmt.objects),
        remove_type: stmt.removeType as i32 + 1,
        behavior: stmt.behavior as i32 + 1,
        missing_ok: stmt.missing_ok,
        concurrent: stmt.concurrent,
    }
}

unsafe fn convert_index_stmt(stmt: &bindings_raw::IndexStmt) -> protobuf::IndexStmt {
    protobuf::IndexStmt {
        idxname: convert_c_string(stmt.idxname),
        relation: if stmt.relation.is_null() { None } else { Some(convert_range_var(&*stmt.relation)) },
        access_method: convert_c_string(stmt.accessMethod),
        table_space: convert_c_string(stmt.tableSpace),
        index_params: convert_list_to_nodes(stmt.indexParams),
        index_including_params: convert_list_to_nodes(stmt.indexIncludingParams),
        options: convert_list_to_nodes(stmt.options),
        where_clause: convert_node_boxed(stmt.whereClause),
        exclude_op_names: convert_list_to_nodes(stmt.excludeOpNames),
        idxcomment: convert_c_string(stmt.idxcomment),
        index_oid: stmt.indexOid,
        old_number: stmt.oldNumber,
        old_create_subid: stmt.oldCreateSubid,
        old_first_relfilelocator_subid: stmt.oldFirstRelfilelocatorSubid,
        unique: stmt.unique,
        nulls_not_distinct: stmt.nulls_not_distinct,
        primary: stmt.primary,
        isconstraint: stmt.isconstraint,
        deferrable: stmt.deferrable,
        initdeferred: stmt.initdeferred,
        transformed: stmt.transformed,
        concurrent: stmt.concurrent,
        if_not_exists: stmt.if_not_exists,
        reset_default_tblspc: stmt.reset_default_tblspc,
    }
}

// ============================================================================
// Expression/Clause Conversions
// ============================================================================

unsafe fn convert_range_var(rv: &bindings_raw::RangeVar) -> protobuf::RangeVar {
    protobuf::RangeVar {
        catalogname: convert_c_string(rv.catalogname),
        schemaname: convert_c_string(rv.schemaname),
        relname: convert_c_string(rv.relname),
        inh: rv.inh,
        relpersistence: String::from_utf8_lossy(&[rv.relpersistence as u8]).to_string(),
        alias: if rv.alias.is_null() { None } else { Some(convert_alias(&*rv.alias)) },
        location: rv.location,
    }
}

unsafe fn convert_column_ref(cr: &bindings_raw::ColumnRef) -> protobuf::ColumnRef {
    protobuf::ColumnRef {
        fields: convert_list_to_nodes(cr.fields),
        location: cr.location,
    }
}

unsafe fn convert_res_target(rt: &bindings_raw::ResTarget) -> protobuf::ResTarget {
    protobuf::ResTarget {
        name: convert_c_string(rt.name),
        indirection: convert_list_to_nodes(rt.indirection),
        val: convert_node_boxed(rt.val),
        location: rt.location,
    }
}

unsafe fn convert_a_expr(expr: &bindings_raw::A_Expr) -> protobuf::AExpr {
    protobuf::AExpr {
        kind: expr.kind as i32 + 1,
        name: convert_list_to_nodes(expr.name),
        lexpr: convert_node_boxed(expr.lexpr),
        rexpr: convert_node_boxed(expr.rexpr),
        location: expr.location,
    }
}

unsafe fn convert_a_const(aconst: &bindings_raw::A_Const) -> protobuf::AConst {
    let val = if aconst.isnull {
        None
    } else {
        // Check the node tag in the val union to determine the type
        let node_tag = aconst.val.node.type_;
        match node_tag {
            bindings_raw::NodeTag_T_Integer => {
                Some(protobuf::a_const::Val::Ival(protobuf::Integer {
                    ival: aconst.val.ival.ival,
                }))
            }
            bindings_raw::NodeTag_T_Float => {
                let fval = if aconst.val.fval.fval.is_null() {
                    std::string::String::new()
                } else {
                    CStr::from_ptr(aconst.val.fval.fval).to_string_lossy().to_string()
                };
                Some(protobuf::a_const::Val::Fval(protobuf::Float { fval }))
            }
            bindings_raw::NodeTag_T_Boolean => {
                Some(protobuf::a_const::Val::Boolval(protobuf::Boolean {
                    boolval: aconst.val.boolval.boolval,
                }))
            }
            bindings_raw::NodeTag_T_String => {
                let sval = if aconst.val.sval.sval.is_null() {
                    std::string::String::new()
                } else {
                    CStr::from_ptr(aconst.val.sval.sval).to_string_lossy().to_string()
                };
                Some(protobuf::a_const::Val::Sval(protobuf::String { sval }))
            }
            bindings_raw::NodeTag_T_BitString => {
                let bsval = if aconst.val.bsval.bsval.is_null() {
                    std::string::String::new()
                } else {
                    CStr::from_ptr(aconst.val.bsval.bsval).to_string_lossy().to_string()
                };
                Some(protobuf::a_const::Val::Bsval(protobuf::BitString { bsval }))
            }
            _ => None,
        }
    };

    protobuf::AConst {
        isnull: aconst.isnull,
        val,
        location: aconst.location,
    }
}

unsafe fn convert_func_call(fc: &bindings_raw::FuncCall) -> protobuf::FuncCall {
    protobuf::FuncCall {
        funcname: convert_list_to_nodes(fc.funcname),
        args: convert_list_to_nodes(fc.args),
        agg_order: convert_list_to_nodes(fc.agg_order),
        agg_filter: convert_node_boxed(fc.agg_filter),
        over: if fc.over.is_null() { None } else { Some(Box::new(convert_window_def(&*fc.over))) },
        agg_within_group: fc.agg_within_group,
        agg_star: fc.agg_star,
        agg_distinct: fc.agg_distinct,
        func_variadic: fc.func_variadic,
        funcformat: fc.funcformat as i32 + 1,
        location: fc.location,
    }
}

unsafe fn convert_type_cast(tc: &bindings_raw::TypeCast) -> protobuf::TypeCast {
    protobuf::TypeCast {
        arg: convert_node_boxed(tc.arg),
        type_name: if tc.typeName.is_null() { None } else { Some(convert_type_name(&*tc.typeName)) },
        location: tc.location,
    }
}

unsafe fn convert_type_name(tn: &bindings_raw::TypeName) -> protobuf::TypeName {
    protobuf::TypeName {
        names: convert_list_to_nodes(tn.names),
        type_oid: tn.typeOid,
        setof: tn.setof,
        pct_type: tn.pct_type,
        typmods: convert_list_to_nodes(tn.typmods),
        typemod: tn.typemod,
        array_bounds: convert_list_to_nodes(tn.arrayBounds),
        location: tn.location,
    }
}

unsafe fn convert_alias(alias: &bindings_raw::Alias) -> protobuf::Alias {
    protobuf::Alias {
        aliasname: convert_c_string(alias.aliasname),
        colnames: convert_list_to_nodes(alias.colnames),
    }
}

unsafe fn convert_join_expr(je: &bindings_raw::JoinExpr) -> protobuf::JoinExpr {
    protobuf::JoinExpr {
        jointype: je.jointype as i32 + 1,
        is_natural: je.isNatural,
        larg: convert_node_boxed(je.larg),
        rarg: convert_node_boxed(je.rarg),
        using_clause: convert_list_to_nodes(je.usingClause),
        join_using_alias: if je.join_using_alias.is_null() { None } else { Some(convert_alias(&*je.join_using_alias)) },
        quals: convert_node_boxed(je.quals),
        alias: if je.alias.is_null() { None } else { Some(convert_alias(&*je.alias)) },
        rtindex: je.rtindex,
    }
}

unsafe fn convert_sort_by(sb: &bindings_raw::SortBy) -> protobuf::SortBy {
    protobuf::SortBy {
        node: convert_node_boxed(sb.node),
        sortby_dir: sb.sortby_dir as i32 + 1,
        sortby_nulls: sb.sortby_nulls as i32 + 1,
        use_op: convert_list_to_nodes(sb.useOp),
        location: sb.location,
    }
}

unsafe fn convert_bool_expr(be: &bindings_raw::BoolExpr) -> protobuf::BoolExpr {
    protobuf::BoolExpr {
        xpr: None, // Xpr is internal
        boolop: be.boolop as i32 + 1,
        args: convert_list_to_nodes(be.args),
        location: be.location,
    }
}

unsafe fn convert_sub_link(sl: &bindings_raw::SubLink) -> protobuf::SubLink {
    protobuf::SubLink {
        xpr: None,
        sub_link_type: sl.subLinkType as i32 + 1,
        sub_link_id: sl.subLinkId,
        testexpr: convert_node_boxed(sl.testexpr),
        oper_name: convert_list_to_nodes(sl.operName),
        subselect: convert_node_boxed(sl.subselect),
        location: sl.location,
    }
}

unsafe fn convert_null_test(nt: &bindings_raw::NullTest) -> protobuf::NullTest {
    protobuf::NullTest {
        xpr: None,
        arg: convert_node_boxed(nt.arg as *mut bindings_raw::Node),
        nulltesttype: nt.nulltesttype as i32 + 1,
        argisrow: nt.argisrow,
        location: nt.location,
    }
}

unsafe fn convert_case_expr(ce: &bindings_raw::CaseExpr) -> protobuf::CaseExpr {
    protobuf::CaseExpr {
        xpr: None,
        casetype: ce.casetype,
        casecollid: ce.casecollid,
        arg: convert_node_boxed(ce.arg as *mut bindings_raw::Node),
        args: convert_list_to_nodes(ce.args),
        defresult: convert_node_boxed(ce.defresult as *mut bindings_raw::Node),
        location: ce.location,
    }
}

unsafe fn convert_case_when(cw: &bindings_raw::CaseWhen) -> protobuf::CaseWhen {
    protobuf::CaseWhen {
        xpr: None,
        expr: convert_node_boxed(cw.expr as *mut bindings_raw::Node),
        result: convert_node_boxed(cw.result as *mut bindings_raw::Node),
        location: cw.location,
    }
}

unsafe fn convert_coalesce_expr(ce: &bindings_raw::CoalesceExpr) -> protobuf::CoalesceExpr {
    protobuf::CoalesceExpr {
        xpr: None,
        coalescetype: ce.coalescetype,
        coalescecollid: ce.coalescecollid,
        args: convert_list_to_nodes(ce.args),
        location: ce.location,
    }
}

unsafe fn convert_with_clause(wc: &bindings_raw::WithClause) -> protobuf::WithClause {
    protobuf::WithClause {
        ctes: convert_list_to_nodes(wc.ctes),
        recursive: wc.recursive,
        location: wc.location,
    }
}

unsafe fn convert_with_clause_opt(wc: *mut bindings_raw::WithClause) -> Option<protobuf::WithClause> {
    if wc.is_null() {
        None
    } else {
        Some(convert_with_clause(&*wc))
    }
}

unsafe fn convert_common_table_expr(cte: &bindings_raw::CommonTableExpr) -> protobuf::CommonTableExpr {
    protobuf::CommonTableExpr {
        ctename: convert_c_string(cte.ctename),
        aliascolnames: convert_list_to_nodes(cte.aliascolnames),
        ctematerialized: cte.ctematerialized as i32 + 1,
        ctequery: convert_node_boxed(cte.ctequery),
        search_clause: convert_cte_search_clause_opt(cte.search_clause),
        cycle_clause: convert_cte_cycle_clause_opt(cte.cycle_clause),
        location: cte.location,
        cterecursive: cte.cterecursive,
        cterefcount: cte.cterefcount,
        ctecolnames: convert_list_to_nodes(cte.ctecolnames),
        ctecoltypes: convert_list_to_nodes(cte.ctecoltypes),
        ctecoltypmods: convert_list_to_nodes(cte.ctecoltypmods),
        ctecolcollations: convert_list_to_nodes(cte.ctecolcollations),
    }
}

unsafe fn convert_window_def(wd: &bindings_raw::WindowDef) -> protobuf::WindowDef {
    protobuf::WindowDef {
        name: convert_c_string(wd.name),
        refname: convert_c_string(wd.refname),
        partition_clause: convert_list_to_nodes(wd.partitionClause),
        order_clause: convert_list_to_nodes(wd.orderClause),
        frame_options: wd.frameOptions,
        start_offset: convert_node_boxed(wd.startOffset),
        end_offset: convert_node_boxed(wd.endOffset),
        location: wd.location,
    }
}

unsafe fn convert_into_clause(ic: *mut bindings_raw::IntoClause) -> Option<Box<protobuf::IntoClause>> {
    if ic.is_null() {
        return None;
    }
    let ic_ref = &*ic;
    Some(Box::new(protobuf::IntoClause {
        rel: if ic_ref.rel.is_null() { None } else { Some(convert_range_var(&*ic_ref.rel)) },
        col_names: convert_list_to_nodes(ic_ref.colNames),
        access_method: convert_c_string(ic_ref.accessMethod),
        options: convert_list_to_nodes(ic_ref.options),
        on_commit: ic_ref.onCommit as i32 + 1,
        table_space_name: convert_c_string(ic_ref.tableSpaceName),
        view_query: convert_node_boxed(ic_ref.viewQuery),
        skip_data: ic_ref.skipData,
    }))
}

unsafe fn convert_infer_clause(ic: *mut bindings_raw::InferClause) -> Option<Box<protobuf::InferClause>> {
    if ic.is_null() {
        return None;
    }
    let ic_ref = &*ic;
    Some(Box::new(protobuf::InferClause {
        index_elems: convert_list_to_nodes(ic_ref.indexElems),
        where_clause: convert_node_boxed(ic_ref.whereClause),
        conname: convert_c_string(ic_ref.conname),
        location: ic_ref.location,
    }))
}

unsafe fn convert_on_conflict_clause(oc: *mut bindings_raw::OnConflictClause) -> Option<Box<protobuf::OnConflictClause>> {
    if oc.is_null() {
        return None;
    }
    let oc_ref = &*oc;
    Some(Box::new(protobuf::OnConflictClause {
        action: oc_ref.action as i32 + 1,
        infer: convert_infer_clause(oc_ref.infer),
        target_list: convert_list_to_nodes(oc_ref.targetList),
        where_clause: convert_node_boxed(oc_ref.whereClause),
        location: oc_ref.location,
    }))
}

unsafe fn convert_column_def(cd: &bindings_raw::ColumnDef) -> protobuf::ColumnDef {
    protobuf::ColumnDef {
        colname: convert_c_string(cd.colname),
        type_name: if cd.typeName.is_null() { None } else { Some(convert_type_name(&*cd.typeName)) },
        compression: convert_c_string(cd.compression),
        inhcount: cd.inhcount,
        is_local: cd.is_local,
        is_not_null: cd.is_not_null,
        is_from_type: cd.is_from_type,
        storage: if cd.storage == 0 { String::new() } else { String::from_utf8_lossy(&[cd.storage as u8]).to_string() },
        storage_name: convert_c_string(cd.storage_name),
        raw_default: convert_node_boxed(cd.raw_default),
        cooked_default: convert_node_boxed(cd.cooked_default),
        identity: if cd.identity == 0 { String::new() } else { String::from_utf8_lossy(&[cd.identity as u8]).to_string() },
        identity_sequence: if cd.identitySequence.is_null() { None } else { Some(convert_range_var(&*cd.identitySequence)) },
        generated: if cd.generated == 0 { String::new() } else { String::from_utf8_lossy(&[cd.generated as u8]).to_string() },
        coll_clause: convert_collate_clause_opt(cd.collClause),
        coll_oid: cd.collOid,
        constraints: convert_list_to_nodes(cd.constraints),
        fdwoptions: convert_list_to_nodes(cd.fdwoptions),
        location: cd.location,
    }
}

unsafe fn convert_constraint(c: &bindings_raw::Constraint) -> protobuf::Constraint {
    protobuf::Constraint {
        contype: c.contype as i32 + 1,
        conname: convert_c_string(c.conname),
        deferrable: c.deferrable,
        initdeferred: c.initdeferred,
        location: c.location,
        is_no_inherit: c.is_no_inherit,
        raw_expr: convert_node_boxed(c.raw_expr),
        cooked_expr: convert_c_string(c.cooked_expr),
        generated_when: if c.generated_when == 0 { String::new() } else { String::from_utf8_lossy(&[c.generated_when as u8]).to_string() },
        nulls_not_distinct: c.nulls_not_distinct,
        keys: convert_list_to_nodes(c.keys),
        including: convert_list_to_nodes(c.including),
        exclusions: convert_list_to_nodes(c.exclusions),
        options: convert_list_to_nodes(c.options),
        indexname: convert_c_string(c.indexname),
        indexspace: convert_c_string(c.indexspace),
        reset_default_tblspc: c.reset_default_tblspc,
        access_method: convert_c_string(c.access_method),
        where_clause: convert_node_boxed(c.where_clause),
        pktable: if c.pktable.is_null() { None } else { Some(convert_range_var(&*c.pktable)) },
        fk_attrs: convert_list_to_nodes(c.fk_attrs),
        pk_attrs: convert_list_to_nodes(c.pk_attrs),
        fk_matchtype: if c.fk_matchtype == 0 { String::new() } else { String::from_utf8_lossy(&[c.fk_matchtype as u8]).to_string() },
        fk_upd_action: if c.fk_upd_action == 0 { String::new() } else { String::from_utf8_lossy(&[c.fk_upd_action as u8]).to_string() },
        fk_del_action: if c.fk_del_action == 0 { String::new() } else { String::from_utf8_lossy(&[c.fk_del_action as u8]).to_string() },
        fk_del_set_cols: convert_list_to_nodes(c.fk_del_set_cols),
        old_conpfeqop: convert_list_to_nodes(c.old_conpfeqop),
        old_pktable_oid: c.old_pktable_oid,
        skip_validation: c.skip_validation,
        initially_valid: c.initially_valid,
    }
}

unsafe fn convert_index_elem(ie: &bindings_raw::IndexElem) -> protobuf::IndexElem {
    protobuf::IndexElem {
        name: convert_c_string(ie.name),
        expr: convert_node_boxed(ie.expr),
        indexcolname: convert_c_string(ie.indexcolname),
        collation: convert_list_to_nodes(ie.collation),
        opclass: convert_list_to_nodes(ie.opclass),
        opclassopts: convert_list_to_nodes(ie.opclassopts),
        ordering: ie.ordering as i32 + 1,
        nulls_ordering: ie.nulls_ordering as i32 + 1,
    }
}

unsafe fn convert_def_elem(de: &bindings_raw::DefElem) -> protobuf::DefElem {
    protobuf::DefElem {
        defnamespace: convert_c_string(de.defnamespace),
        defname: convert_c_string(de.defname),
        arg: convert_node_boxed(de.arg),
        defaction: de.defaction as i32 + 1,
        location: de.location,
    }
}

unsafe fn convert_string(s: &bindings_raw::String) -> protobuf::String {
    protobuf::String {
        sval: convert_c_string(s.sval),
    }
}

unsafe fn convert_locking_clause(lc: &bindings_raw::LockingClause) -> protobuf::LockingClause {
    protobuf::LockingClause {
        locked_rels: convert_list_to_nodes(lc.lockedRels),
        strength: lc.strength as i32 + 1,
        wait_policy: lc.waitPolicy as i32 + 1,
    }
}

unsafe fn convert_min_max_expr(mme: &bindings_raw::MinMaxExpr) -> protobuf::MinMaxExpr {
    protobuf::MinMaxExpr {
        xpr: None, // Expression type info, not needed for parse tree
        minmaxtype: mme.minmaxtype,
        minmaxcollid: mme.minmaxcollid,
        inputcollid: mme.inputcollid,
        op: mme.op as i32 + 1,
        args: convert_list_to_nodes(mme.args),
        location: mme.location,
    }
}

unsafe fn convert_grouping_set(gs: &bindings_raw::GroupingSet) -> protobuf::GroupingSet {
    protobuf::GroupingSet {
        kind: gs.kind as i32 + 1,
        content: convert_list_to_nodes(gs.content),
        location: gs.location,
    }
}

unsafe fn convert_range_subselect(rs: &bindings_raw::RangeSubselect) -> protobuf::RangeSubselect {
    protobuf::RangeSubselect {
        lateral: rs.lateral,
        subquery: convert_node_boxed(rs.subquery),
        alias: if rs.alias.is_null() { None } else { Some(convert_alias(&*rs.alias)) },
    }
}

unsafe fn convert_a_array_expr(ae: &bindings_raw::A_ArrayExpr) -> protobuf::AArrayExpr {
    protobuf::AArrayExpr {
        elements: convert_list_to_nodes(ae.elements),
        location: ae.location,
    }
}

unsafe fn convert_a_indirection(ai: &bindings_raw::A_Indirection) -> protobuf::AIndirection {
    protobuf::AIndirection {
        arg: convert_node_boxed(ai.arg),
        indirection: convert_list_to_nodes(ai.indirection),
    }
}

unsafe fn convert_a_indices(ai: &bindings_raw::A_Indices) -> protobuf::AIndices {
    protobuf::AIndices {
        is_slice: ai.is_slice,
        lidx: convert_node_boxed(ai.lidx),
        uidx: convert_node_boxed(ai.uidx),
    }
}

unsafe fn convert_alter_table_stmt(ats: &bindings_raw::AlterTableStmt) -> protobuf::AlterTableStmt {
    protobuf::AlterTableStmt {
        relation: if ats.relation.is_null() { None } else { Some(convert_range_var(&*ats.relation)) },
        cmds: convert_list_to_nodes(ats.cmds),
        objtype: ats.objtype as i32 + 1,
        missing_ok: ats.missing_ok,
    }
}

unsafe fn convert_alter_table_cmd(atc: &bindings_raw::AlterTableCmd) -> protobuf::AlterTableCmd {
    protobuf::AlterTableCmd {
        subtype: atc.subtype as i32 + 1,
        name: convert_c_string(atc.name),
        num: atc.num as i32,
        newowner: if atc.newowner.is_null() { None } else { Some(convert_role_spec(&*atc.newowner)) },
        def: convert_node_boxed(atc.def),
        behavior: atc.behavior as i32 + 1,
        missing_ok: atc.missing_ok,
        recurse: atc.recurse,
    }
}

unsafe fn convert_role_spec(rs: &bindings_raw::RoleSpec) -> protobuf::RoleSpec {
    protobuf::RoleSpec {
        roletype: rs.roletype as i32 + 1,
        rolename: convert_c_string(rs.rolename),
        location: rs.location,
    }
}

unsafe fn convert_copy_stmt(cs: &bindings_raw::CopyStmt) -> protobuf::CopyStmt {
    protobuf::CopyStmt {
        relation: if cs.relation.is_null() { None } else { Some(convert_range_var(&*cs.relation)) },
        query: convert_node_boxed(cs.query),
        attlist: convert_list_to_nodes(cs.attlist),
        is_from: cs.is_from,
        is_program: cs.is_program,
        filename: convert_c_string(cs.filename),
        options: convert_list_to_nodes(cs.options),
        where_clause: convert_node_boxed(cs.whereClause),
    }
}

unsafe fn convert_truncate_stmt(ts: &bindings_raw::TruncateStmt) -> protobuf::TruncateStmt {
    protobuf::TruncateStmt {
        relations: convert_list_to_nodes(ts.relations),
        restart_seqs: ts.restart_seqs,
        behavior: ts.behavior as i32 + 1,
    }
}

unsafe fn convert_view_stmt(vs: &bindings_raw::ViewStmt) -> protobuf::ViewStmt {
    protobuf::ViewStmt {
        view: if vs.view.is_null() { None } else { Some(convert_range_var(&*vs.view)) },
        aliases: convert_list_to_nodes(vs.aliases),
        query: convert_node_boxed(vs.query),
        replace: vs.replace,
        options: convert_list_to_nodes(vs.options),
        with_check_option: vs.withCheckOption as i32 + 1,
    }
}

unsafe fn convert_explain_stmt(es: &bindings_raw::ExplainStmt) -> protobuf::ExplainStmt {
    protobuf::ExplainStmt {
        query: convert_node_boxed(es.query),
        options: convert_list_to_nodes(es.options),
    }
}

unsafe fn convert_create_table_as_stmt(ctas: &bindings_raw::CreateTableAsStmt) -> protobuf::CreateTableAsStmt {
    protobuf::CreateTableAsStmt {
        query: convert_node_boxed(ctas.query),
        into: convert_into_clause(ctas.into),
        objtype: ctas.objtype as i32 + 1,
        is_select_into: ctas.is_select_into,
        if_not_exists: ctas.if_not_exists,
    }
}

unsafe fn convert_prepare_stmt(ps: &bindings_raw::PrepareStmt) -> protobuf::PrepareStmt {
    protobuf::PrepareStmt {
        name: convert_c_string(ps.name),
        argtypes: convert_list_to_nodes(ps.argtypes),
        query: convert_node_boxed(ps.query),
    }
}

unsafe fn convert_execute_stmt(es: &bindings_raw::ExecuteStmt) -> protobuf::ExecuteStmt {
    protobuf::ExecuteStmt {
        name: convert_c_string(es.name),
        params: convert_list_to_nodes(es.params),
    }
}

unsafe fn convert_deallocate_stmt(ds: &bindings_raw::DeallocateStmt) -> protobuf::DeallocateStmt {
    protobuf::DeallocateStmt {
        name: convert_c_string(ds.name),
    }
}

unsafe fn convert_set_to_default(std: &bindings_raw::SetToDefault) -> protobuf::SetToDefault {
    protobuf::SetToDefault {
        xpr: None, // Expression type info, not needed for parse tree
        type_id: std.typeId,
        type_mod: std.typeMod,
        collation: std.collation,
        location: std.location,
    }
}

unsafe fn convert_multi_assign_ref(mar: &bindings_raw::MultiAssignRef) -> protobuf::MultiAssignRef {
    protobuf::MultiAssignRef {
        source: convert_node_boxed(mar.source),
        colno: mar.colno,
        ncolumns: mar.ncolumns,
    }
}

unsafe fn convert_row_expr(re: &bindings_raw::RowExpr) -> protobuf::RowExpr {
    protobuf::RowExpr {
        xpr: None, // Expression type info, not needed for parse tree
        args: convert_list_to_nodes(re.args),
        row_typeid: re.row_typeid,
        row_format: re.row_format as i32 + 1,
        colnames: convert_list_to_nodes(re.colnames),
        location: re.location,
    }
}

unsafe fn convert_collate_clause(cc: &bindings_raw::CollateClause) -> protobuf::CollateClause {
    protobuf::CollateClause {
        arg: convert_node_boxed(cc.arg),
        collname: convert_list_to_nodes(cc.collname),
        location: cc.location,
    }
}

unsafe fn convert_collate_clause_opt(cc: *mut bindings_raw::CollateClause) -> Option<Box<protobuf::CollateClause>> {
    if cc.is_null() {
        None
    } else {
        Some(Box::new(convert_collate_clause(&*cc)))
    }
}

unsafe fn convert_partition_spec(ps: &bindings_raw::PartitionSpec) -> protobuf::PartitionSpec {
    protobuf::PartitionSpec {
        strategy: ps.strategy as i32 + 1,
        part_params: convert_list_to_nodes(ps.partParams),
        location: ps.location,
    }
}

unsafe fn convert_partition_spec_opt(ps: *mut bindings_raw::PartitionSpec) -> Option<Box<protobuf::PartitionSpec>> {
    if ps.is_null() {
        None
    } else {
        Some(Box::new(convert_partition_spec(&*ps)))
    }
}

unsafe fn convert_partition_bound_spec(pbs: &bindings_raw::PartitionBoundSpec) -> protobuf::PartitionBoundSpec {
    protobuf::PartitionBoundSpec {
        strategy: if pbs.strategy == 0 { String::new() } else { String::from_utf8_lossy(&[pbs.strategy as u8]).to_string() },
        is_default: pbs.is_default,
        modulus: pbs.modulus,
        remainder: pbs.remainder,
        listdatums: convert_list_to_nodes(pbs.listdatums),
        lowerdatums: convert_list_to_nodes(pbs.lowerdatums),
        upperdatums: convert_list_to_nodes(pbs.upperdatums),
        location: pbs.location,
    }
}

unsafe fn convert_partition_bound_spec_opt(pbs: *mut bindings_raw::PartitionBoundSpec) -> Option<Box<protobuf::PartitionBoundSpec>> {
    if pbs.is_null() {
        None
    } else {
        Some(Box::new(convert_partition_bound_spec(&*pbs)))
    }
}

unsafe fn convert_cte_search_clause(csc: &bindings_raw::CTESearchClause) -> protobuf::CtesearchClause {
    protobuf::CtesearchClause {
        search_col_list: convert_list_to_nodes(csc.search_col_list),
        search_breadth_first: csc.search_breadth_first,
        search_seq_column: convert_c_string(csc.search_seq_column),
        location: csc.location,
    }
}

unsafe fn convert_cte_search_clause_opt(csc: *mut bindings_raw::CTESearchClause) -> Option<Box<protobuf::CtesearchClause>> {
    if csc.is_null() {
        None
    } else {
        Some(Box::new(convert_cte_search_clause(&*csc)))
    }
}

unsafe fn convert_cte_cycle_clause(ccc: &bindings_raw::CTECycleClause) -> protobuf::CtecycleClause {
    protobuf::CtecycleClause {
        cycle_col_list: convert_list_to_nodes(ccc.cycle_col_list),
        cycle_mark_column: convert_c_string(ccc.cycle_mark_column),
        cycle_mark_value: convert_node_boxed(ccc.cycle_mark_value),
        cycle_mark_default: convert_node_boxed(ccc.cycle_mark_default),
        cycle_path_column: convert_c_string(ccc.cycle_path_column),
        location: ccc.location,
        cycle_mark_type: ccc.cycle_mark_type,
        cycle_mark_typmod: ccc.cycle_mark_typmod,
        cycle_mark_collation: ccc.cycle_mark_collation,
        cycle_mark_neop: ccc.cycle_mark_neop,
    }
}

unsafe fn convert_cte_cycle_clause_opt(ccc: *mut bindings_raw::CTECycleClause) -> Option<Box<protobuf::CtecycleClause>> {
    if ccc.is_null() {
        None
    } else {
        Some(Box::new(convert_cte_cycle_clause(&*ccc)))
    }
}

// ============================================================================
// Utility Functions
// ============================================================================

/// Converts a C string pointer to a Rust String.
unsafe fn convert_c_string(ptr: *const i8) -> std::string::String {
    if ptr.is_null() {
        std::string::String::new()
    } else {
        CStr::from_ptr(ptr).to_string_lossy().to_string()
    }
}
