//! Conversion implementations between protobuf types and native AST types.

use crate::protobuf;
use crate::ast::nodes::*;

// ============================================================================
// From protobuf to native AST types
// ============================================================================

impl ParseResult {
    /// Create a new ParseResult from a protobuf result.
    /// This stores the original protobuf for later deparsing.
    pub fn from_protobuf(pb: protobuf::ParseResult) -> Self {
        let stmts = pb.stmts.iter().map(|s| s.into()).collect();
        ParseResult {
            version: pb.version,
            stmts,
            original_protobuf: pb,
        }
    }

    /// Get a reference to the original protobuf for deparsing.
    pub fn as_protobuf(&self) -> &protobuf::ParseResult {
        &self.original_protobuf
    }
}

impl From<protobuf::ParseResult> for ParseResult {
    fn from(pb: protobuf::ParseResult) -> Self {
        ParseResult::from_protobuf(pb)
    }
}

impl From<&protobuf::ParseResult> for ParseResult {
    fn from(pb: &protobuf::ParseResult) -> Self {
        ParseResult::from_protobuf(pb.clone())
    }
}

impl From<protobuf::RawStmt> for RawStmt {
    fn from(pb: protobuf::RawStmt) -> Self {
        RawStmt {
            stmt: pb.stmt.map(|n| (*n).into()).unwrap_or(Node::Null),
            stmt_location: pb.stmt_location,
            stmt_len: pb.stmt_len,
        }
    }
}

impl From<&protobuf::RawStmt> for RawStmt {
    fn from(pb: &protobuf::RawStmt) -> Self {
        RawStmt {
            stmt: pb.stmt.as_ref().map(|n| n.as_ref().into()).unwrap_or(Node::Null),
            stmt_location: pb.stmt_location,
            stmt_len: pb.stmt_len,
        }
    }
}

impl From<protobuf::Node> for Node {
    fn from(pb: protobuf::Node) -> Self {
        match pb.node {
            Some(node) => node.into(),
            None => Node::Null,
        }
    }
}

impl From<&protobuf::Node> for Node {
    fn from(pb: &protobuf::Node) -> Self {
        match &pb.node {
            Some(node) => node.into(),
            None => Node::Null,
        }
    }
}

impl From<protobuf::node::Node> for Node {
    fn from(pb: protobuf::node::Node) -> Self {
        use protobuf::node::Node as PbNode;
        match pb {
            // Primitive types (not boxed)
            PbNode::Integer(v) => Node::Integer(v.into()),
            PbNode::Float(v) => Node::Float(v.into()),
            PbNode::Boolean(v) => Node::Boolean(v.into()),
            PbNode::String(v) => Node::String(v.into()),
            PbNode::BitString(v) => Node::BitString(v.into()),
            PbNode::List(v) => Node::List(v.items.into_iter().map(|n| n.into()).collect()),

            // Statement types (boxed in protobuf)
            PbNode::SelectStmt(v) => Node::SelectStmt(Box::new((*v).into())),
            PbNode::InsertStmt(v) => Node::InsertStmt(Box::new((*v).into())),
            PbNode::UpdateStmt(v) => Node::UpdateStmt(Box::new((*v).into())),
            PbNode::DeleteStmt(v) => Node::DeleteStmt(Box::new((*v).into())),
            PbNode::MergeStmt(v) => Node::MergeStmt(Box::new((*v).into())),

            // DDL statements (not boxed in protobuf)
            PbNode::CreateStmt(v) => Node::CreateStmt(Box::new(v.into())),
            PbNode::AlterTableStmt(v) => Node::AlterTableStmt(Box::new(v.into())),
            PbNode::DropStmt(v) => Node::DropStmt(Box::new(v.into())),
            PbNode::TruncateStmt(v) => Node::TruncateStmt(Box::new(v.into())),
            PbNode::IndexStmt(v) => Node::IndexStmt(Box::new((*v).into())),
            PbNode::CreateSchemaStmt(v) => Node::CreateSchemaStmt(Box::new(v.into())),
            PbNode::ViewStmt(v) => Node::ViewStmt(Box::new((*v).into())),
            PbNode::CreateFunctionStmt(v) => Node::CreateFunctionStmt(Box::new((*v).into())),
            PbNode::AlterFunctionStmt(v) => Node::AlterFunctionStmt(Box::new(v.into())),
            PbNode::CreateSeqStmt(v) => Node::CreateSeqStmt(Box::new(v.into())),
            PbNode::AlterSeqStmt(v) => Node::AlterSeqStmt(Box::new(v.into())),
            PbNode::CreateTrigStmt(v) => Node::CreateTrigStmt(Box::new((*v).into())),
            PbNode::RuleStmt(v) => Node::RuleStmt(Box::new((*v).into())),
            PbNode::CreateDomainStmt(v) => Node::CreateDomainStmt(Box::new((*v).into())),
            PbNode::CreateTableAsStmt(v) => Node::CreateTableAsStmt(Box::new((*v).into())),
            PbNode::RefreshMatViewStmt(v) => Node::RefreshMatViewStmt(Box::new(v.into())),

            // Transaction statements (not boxed in protobuf)
            PbNode::TransactionStmt(v) => Node::TransactionStmt(Box::new(v.into())),

            // Expression types (mixed boxing)
            PbNode::AExpr(v) => Node::AExpr(Box::new((*v).into())),
            PbNode::ColumnRef(v) => Node::ColumnRef(Box::new(v.into())),
            PbNode::ParamRef(v) => Node::ParamRef(Box::new(v.into())),
            PbNode::AConst(v) => Node::AConst(Box::new(v.into())),
            PbNode::TypeCast(v) => Node::TypeCast(Box::new((*v).into())),
            PbNode::CollateClause(v) => Node::CollateClause(Box::new((*v).into())),
            PbNode::FuncCall(v) => Node::FuncCall(Box::new((*v).into())),
            PbNode::AStar(_) => Node::AStar(AStar),
            PbNode::AIndices(v) => Node::AIndices(Box::new((*v).into())),
            PbNode::AIndirection(v) => Node::AIndirection(Box::new((*v).into())),
            PbNode::AArrayExpr(v) => Node::AArrayExpr(Box::new(v.into())),
            PbNode::SubLink(v) => Node::SubLink(Box::new((*v).into())),
            PbNode::BoolExpr(v) => Node::BoolExpr(Box::new((*v).into())),
            PbNode::NullTest(v) => Node::NullTest(Box::new((*v).into())),
            PbNode::BooleanTest(v) => Node::BooleanTest(Box::new((*v).into())),
            PbNode::CaseExpr(v) => Node::CaseExpr(Box::new((*v).into())),
            PbNode::CaseWhen(v) => Node::CaseWhen(Box::new((*v).into())),
            PbNode::CoalesceExpr(v) => Node::CoalesceExpr(Box::new((*v).into())),
            PbNode::MinMaxExpr(v) => Node::MinMaxExpr(Box::new((*v).into())),
            PbNode::RowExpr(v) => Node::RowExpr(Box::new((*v).into())),

            // Target/Result types (boxed in protobuf)
            PbNode::ResTarget(v) => Node::ResTarget(Box::new((*v).into())),

            // Table/Range types (mixed)
            PbNode::RangeVar(v) => Node::RangeVar(Box::new(v.into())),
            PbNode::RangeSubselect(v) => Node::RangeSubselect(Box::new((*v).into())),
            PbNode::RangeFunction(v) => Node::RangeFunction(Box::new(v.into())),
            PbNode::JoinExpr(v) => Node::JoinExpr(Box::new((*v).into())),

            // Clause types (mixed)
            PbNode::SortBy(v) => Node::SortBy(Box::new((*v).into())),
            PbNode::WindowDef(v) => Node::WindowDef(Box::new((*v).into())),
            PbNode::WithClause(v) => Node::WithClause(Box::new(v.into())),
            PbNode::CommonTableExpr(v) => Node::CommonTableExpr(Box::new((*v).into())),
            PbNode::IntoClause(v) => Node::IntoClause(Box::new((*v).into())),
            PbNode::OnConflictClause(v) => Node::OnConflictClause(Box::new((*v).into())),
            PbNode::LockingClause(v) => Node::LockingClause(Box::new(v.into())),
            PbNode::GroupingSet(v) => Node::GroupingSet(Box::new(v.into())),
            PbNode::MergeWhenClause(v) => Node::MergeWhenClause(Box::new((*v).into())),

            // Type-related (mixed)
            PbNode::TypeName(v) => Node::TypeName(Box::new(v.into())),
            PbNode::ColumnDef(v) => Node::ColumnDef(Box::new((*v).into())),
            PbNode::Constraint(v) => Node::Constraint(Box::new((*v).into())),
            PbNode::DefElem(v) => Node::DefElem(Box::new((*v).into())),
            PbNode::IndexElem(v) => Node::IndexElem(Box::new((*v).into())),

            // Alias and role types (not boxed)
            PbNode::Alias(v) => Node::Alias(Box::new(v.into())),
            PbNode::RoleSpec(v) => Node::RoleSpec(Box::new(v.into())),

            // Other commonly used types (mixed)
            PbNode::SortGroupClause(v) => Node::SortGroupClause(Box::new(v.into())),
            PbNode::FunctionParameter(v) => Node::FunctionParameter(Box::new((*v).into())),
            PbNode::AlterTableCmd(v) => Node::AlterTableCmd(Box::new((*v).into())),
            PbNode::AccessPriv(v) => Node::AccessPriv(Box::new(v.into())),
            PbNode::ObjectWithArgs(v) => Node::ObjectWithArgs(Box::new(v.into())),

            // Administrative statements (mixed)
            PbNode::VariableSetStmt(v) => Node::VariableSetStmt(Box::new(v.into())),
            PbNode::VariableShowStmt(v) => Node::VariableShowStmt(Box::new(v.into())),
            PbNode::ExplainStmt(v) => Node::ExplainStmt(Box::new((*v).into())),
            PbNode::CopyStmt(v) => Node::CopyStmt(Box::new((*v).into())),
            PbNode::GrantStmt(v) => Node::GrantStmt(Box::new(v.into())),
            PbNode::GrantRoleStmt(v) => Node::GrantRoleStmt(Box::new(v.into())),
            PbNode::LockStmt(v) => Node::LockStmt(Box::new(v.into())),
            PbNode::VacuumStmt(v) => Node::VacuumStmt(Box::new(v.into())),

            // Other statements (mixed)
            PbNode::DoStmt(v) => Node::DoStmt(Box::new(v.into())),
            PbNode::RenameStmt(v) => Node::RenameStmt(Box::new((*v).into())),
            PbNode::NotifyStmt(v) => Node::NotifyStmt(Box::new(v.into())),
            PbNode::ListenStmt(v) => Node::ListenStmt(Box::new(v.into())),
            PbNode::UnlistenStmt(v) => Node::UnlistenStmt(Box::new(v.into())),
            PbNode::CheckPointStmt(_) => Node::CheckPointStmt(Box::new(CheckPointStmt)),
            PbNode::DiscardStmt(v) => Node::DiscardStmt(Box::new(v.into())),
            PbNode::PrepareStmt(v) => Node::PrepareStmt(Box::new((*v).into())),
            PbNode::ExecuteStmt(v) => Node::ExecuteStmt(Box::new(v.into())),
            PbNode::DeallocateStmt(v) => Node::DeallocateStmt(Box::new(v.into())),
            PbNode::ClosePortalStmt(v) => Node::ClosePortalStmt(Box::new(v.into())),
            PbNode::FetchStmt(v) => Node::FetchStmt(Box::new(v.into())),

            // Fallback for any unhandled node types
            other => Node::Other(protobuf::Node { node: Some(other) }),
        }
    }
}

impl From<&protobuf::node::Node> for Node {
    fn from(pb: &protobuf::node::Node) -> Self {
        pb.clone().into()
    }
}

// Conversions from Box<T> for boxed protobuf fields
impl From<Box<protobuf::Node>> for Node {
    fn from(pb: Box<protobuf::Node>) -> Self {
        (*pb).into()
    }
}

impl From<Box<protobuf::IntoClause>> for IntoClause {
    fn from(pb: Box<protobuf::IntoClause>) -> Self {
        (*pb).into()
    }
}

impl From<Box<protobuf::OnConflictClause>> for OnConflictClause {
    fn from(pb: Box<protobuf::OnConflictClause>) -> Self {
        (*pb).into()
    }
}

impl From<Box<protobuf::CollateClause>> for CollateClause {
    fn from(pb: Box<protobuf::CollateClause>) -> Self {
        (*pb).into()
    }
}

impl From<Box<protobuf::SelectStmt>> for SelectStmt {
    fn from(pb: Box<protobuf::SelectStmt>) -> Self {
        (*pb).into()
    }
}

// Primitive type conversions
impl From<protobuf::Integer> for Integer {
    fn from(pb: protobuf::Integer) -> Self {
        Integer { ival: pb.ival }
    }
}

impl From<protobuf::Float> for Float {
    fn from(pb: protobuf::Float) -> Self {
        Float { fval: pb.fval }
    }
}

impl From<protobuf::Boolean> for Boolean {
    fn from(pb: protobuf::Boolean) -> Self {
        Boolean { boolval: pb.boolval }
    }
}

impl From<protobuf::String> for StringValue {
    fn from(pb: protobuf::String) -> Self {
        StringValue { sval: pb.sval }
    }
}

impl From<protobuf::BitString> for BitString {
    fn from(pb: protobuf::BitString) -> Self {
        BitString { bsval: pb.bsval }
    }
}

// Statement type conversions
impl From<protobuf::SelectStmt> for SelectStmt {
    fn from(pb: protobuf::SelectStmt) -> Self {
        SelectStmt {
            distinct_clause: pb.distinct_clause.into_iter().map(|n| n.into()).collect(),
            into_clause: pb.into_clause.map(|v| v.into()),
            target_list: pb.target_list.into_iter().map(|n| n.into()).collect(),
            from_clause: pb.from_clause.into_iter().map(|n| n.into()).collect(),
            where_clause: pb.where_clause.map(|n| n.into()),
            group_clause: pb.group_clause.into_iter().map(|n| n.into()).collect(),
            group_distinct: pb.group_distinct,
            having_clause: pb.having_clause.map(|n| n.into()),
            window_clause: pb.window_clause.into_iter().map(|n| n.into()).collect(),
            values_lists: pb.values_lists.into_iter().map(|n| n.into()).collect(),
            sort_clause: pb.sort_clause.into_iter().map(|n| n.into()).collect(),
            limit_offset: pb.limit_offset.map(|n| n.into()),
            limit_count: pb.limit_count.map(|n| n.into()),
            limit_option: pb.limit_option.into(),
            locking_clause: pb.locking_clause.into_iter().map(|n| n.into()).collect(),
            with_clause: pb.with_clause.map(|v| v.into()),
            op: pb.op.into(),
            all: pb.all,
            larg: pb.larg.map(|v| Box::new((*v).into())),
            rarg: pb.rarg.map(|v| Box::new((*v).into())),
        }
    }
}

impl From<protobuf::InsertStmt> for InsertStmt {
    fn from(pb: protobuf::InsertStmt) -> Self {
        InsertStmt {
            relation: pb.relation.map(|v| v.into()),
            cols: pb.cols.into_iter().map(|n| n.into()).collect(),
            select_stmt: pb.select_stmt.map(|n| n.into()),
            on_conflict_clause: pb.on_conflict_clause.map(|v| v.into()),
            returning_list: pb.returning_list.into_iter().map(|n| n.into()).collect(),
            with_clause: pb.with_clause.map(|v| v.into()),
            override_: pb.r#override.into(),
        }
    }
}

impl From<protobuf::UpdateStmt> for UpdateStmt {
    fn from(pb: protobuf::UpdateStmt) -> Self {
        UpdateStmt {
            relation: pb.relation.map(|v| v.into()),
            target_list: pb.target_list.into_iter().map(|n| n.into()).collect(),
            where_clause: pb.where_clause.map(|n| n.into()),
            from_clause: pb.from_clause.into_iter().map(|n| n.into()).collect(),
            returning_list: pb.returning_list.into_iter().map(|n| n.into()).collect(),
            with_clause: pb.with_clause.map(|v| v.into()),
        }
    }
}

impl From<protobuf::DeleteStmt> for DeleteStmt {
    fn from(pb: protobuf::DeleteStmt) -> Self {
        DeleteStmt {
            relation: pb.relation.map(|v| v.into()),
            using_clause: pb.using_clause.into_iter().map(|n| n.into()).collect(),
            where_clause: pb.where_clause.map(|n| n.into()),
            returning_list: pb.returning_list.into_iter().map(|n| n.into()).collect(),
            with_clause: pb.with_clause.map(|v| v.into()),
        }
    }
}

impl From<protobuf::MergeStmt> for MergeStmt {
    fn from(pb: protobuf::MergeStmt) -> Self {
        MergeStmt {
            relation: pb.relation.map(|v| v.into()),
            source_relation: pb.source_relation.map(|n| n.into()),
            join_condition: pb.join_condition.map(|n| n.into()),
            merge_when_clauses: pb.merge_when_clauses.into_iter().map(|n| n.into()).collect(),
            with_clause: pb.with_clause.map(|v| v.into()),
        }
    }
}

// DDL statement conversions
impl From<protobuf::CreateStmt> for CreateStmt {
    fn from(pb: protobuf::CreateStmt) -> Self {
        CreateStmt {
            relation: pb.relation.map(|v| v.into()),
            table_elts: pb.table_elts.into_iter().map(|n| n.into()).collect(),
            inh_relations: pb.inh_relations.into_iter().map(|n| n.into()).collect(),
            partbound: pb.partbound.map(|n| Node::Other(protobuf::Node { node: Some(protobuf::node::Node::PartitionBoundSpec(n)) })),
            partspec: pb.partspec.map(|n| Node::Other(protobuf::Node { node: Some(protobuf::node::Node::PartitionSpec(n)) })),
            of_typename: pb.of_typename.map(|v| v.into()),
            constraints: pb.constraints.into_iter().map(|n| n.into()).collect(),
            options: pb.options.into_iter().map(|n| n.into()).collect(),
            oncommit: pb.oncommit.into(),
            tablespacename: pb.tablespacename,
            access_method: pb.access_method,
            if_not_exists: pb.if_not_exists,
        }
    }
}

impl From<protobuf::AlterTableStmt> for AlterTableStmt {
    fn from(pb: protobuf::AlterTableStmt) -> Self {
        AlterTableStmt {
            relation: pb.relation.map(|v| v.into()),
            cmds: pb.cmds.into_iter().map(|n| n.into()).collect(),
            objtype: pb.objtype.into(),
            missing_ok: pb.missing_ok,
        }
    }
}

impl From<protobuf::DropStmt> for DropStmt {
    fn from(pb: protobuf::DropStmt) -> Self {
        DropStmt {
            objects: pb.objects.into_iter().map(|n| n.into()).collect(),
            remove_type: pb.remove_type.into(),
            behavior: pb.behavior.into(),
            missing_ok: pb.missing_ok,
            concurrent: pb.concurrent,
        }
    }
}

impl From<protobuf::TruncateStmt> for TruncateStmt {
    fn from(pb: protobuf::TruncateStmt) -> Self {
        TruncateStmt {
            relations: pb.relations.into_iter().map(|n| n.into()).collect(),
            restart_seqs: pb.restart_seqs,
            behavior: pb.behavior.into(),
        }
    }
}

impl From<protobuf::IndexStmt> for IndexStmt {
    fn from(pb: protobuf::IndexStmt) -> Self {
        IndexStmt {
            idxname: pb.idxname,
            relation: pb.relation.map(|v| v.into()),
            access_method: pb.access_method,
            table_space: pb.table_space,
            index_params: pb.index_params.into_iter().map(|n| n.into()).collect(),
            index_including_params: pb.index_including_params.into_iter().map(|n| n.into()).collect(),
            options: pb.options.into_iter().map(|n| n.into()).collect(),
            where_clause: pb.where_clause.map(|n| n.into()),
            exclude_op_names: pb.exclude_op_names.into_iter().map(|n| n.into()).collect(),
            idxcomment: pb.idxcomment,
            index_oid: pb.index_oid,
            old_number: pb.old_number,
            old_first_relfilelocator: pb.old_first_relfilelocator_subid,
            unique: pb.unique,
            nulls_not_distinct: pb.nulls_not_distinct,
            primary: pb.primary,
            is_constraint: pb.isconstraint,
            deferrable: pb.deferrable,
            initdeferred: pb.initdeferred,
            transformed: pb.transformed,
            concurrent: pb.concurrent,
            if_not_exists: pb.if_not_exists,
            reset_default_tblspc: pb.reset_default_tblspc,
        }
    }
}

impl From<protobuf::CreateSchemaStmt> for CreateSchemaStmt {
    fn from(pb: protobuf::CreateSchemaStmt) -> Self {
        CreateSchemaStmt {
            schemaname: pb.schemaname,
            authrole: pb.authrole.map(|v| v.into()),
            schema_elts: pb.schema_elts.into_iter().map(|n| n.into()).collect(),
            if_not_exists: pb.if_not_exists,
        }
    }
}

impl From<protobuf::ViewStmt> for ViewStmt {
    fn from(pb: protobuf::ViewStmt) -> Self {
        ViewStmt {
            view: pb.view.map(|v| v.into()),
            aliases: pb.aliases.into_iter().map(|n| n.into()).collect(),
            query: pb.query.map(|n| n.into()),
            replace: pb.replace,
            options: pb.options.into_iter().map(|n| n.into()).collect(),
            with_check_option: pb.with_check_option.into(),
        }
    }
}

impl From<protobuf::CreateFunctionStmt> for CreateFunctionStmt {
    fn from(pb: protobuf::CreateFunctionStmt) -> Self {
        CreateFunctionStmt {
            is_procedure: pb.is_procedure,
            replace: pb.replace,
            funcname: pb.funcname.into_iter().map(|n| n.into()).collect(),
            parameters: pb.parameters.into_iter().map(|n| n.into()).collect(),
            return_type: pb.return_type.map(|v| v.into()),
            options: pb.options.into_iter().map(|n| n.into()).collect(),
            sql_body: pb.sql_body.map(|n| n.into()),
        }
    }
}

impl From<protobuf::AlterFunctionStmt> for AlterFunctionStmt {
    fn from(pb: protobuf::AlterFunctionStmt) -> Self {
        AlterFunctionStmt {
            objtype: pb.objtype.into(),
            func: pb.func.map(|v| v.into()),
            actions: pb.actions.into_iter().map(|n| n.into()).collect(),
        }
    }
}

impl From<protobuf::CreateSeqStmt> for CreateSeqStmt {
    fn from(pb: protobuf::CreateSeqStmt) -> Self {
        CreateSeqStmt {
            sequence: pb.sequence.map(|v| v.into()),
            options: pb.options.into_iter().map(|n| n.into()).collect(),
            owner_id: pb.owner_id,
            for_identity: pb.for_identity,
            if_not_exists: pb.if_not_exists,
        }
    }
}

impl From<protobuf::AlterSeqStmt> for AlterSeqStmt {
    fn from(pb: protobuf::AlterSeqStmt) -> Self {
        AlterSeqStmt {
            sequence: pb.sequence.map(|v| v.into()),
            options: pb.options.into_iter().map(|n| n.into()).collect(),
            for_identity: pb.for_identity,
            missing_ok: pb.missing_ok,
        }
    }
}

impl From<protobuf::CreateTrigStmt> for CreateTrigStmt {
    fn from(pb: protobuf::CreateTrigStmt) -> Self {
        CreateTrigStmt {
            replace: pb.replace,
            isconstraint: pb.isconstraint,
            trigname: pb.trigname,
            relation: pb.relation.map(|v| v.into()),
            funcname: pb.funcname.into_iter().map(|n| n.into()).collect(),
            args: pb.args.into_iter().map(|n| n.into()).collect(),
            row: pb.row,
            timing: pb.timing,
            events: pb.events,
            columns: pb.columns.into_iter().map(|n| n.into()).collect(),
            when_clause: pb.when_clause.map(|n| n.into()),
            transition_rels: pb.transition_rels.into_iter().map(|n| n.into()).collect(),
            deferrable: pb.deferrable,
            initdeferred: pb.initdeferred,
            constrrel: pb.constrrel.map(|v| v.into()),
        }
    }
}

impl From<protobuf::RuleStmt> for RuleStmt {
    fn from(pb: protobuf::RuleStmt) -> Self {
        RuleStmt {
            relation: pb.relation.map(|v| v.into()),
            rulename: pb.rulename,
            where_clause: pb.where_clause.map(|n| n.into()),
            event: pb.event.into(),
            instead: pb.instead,
            actions: pb.actions.into_iter().map(|n| n.into()).collect(),
            replace: pb.replace,
        }
    }
}

impl From<protobuf::CreateDomainStmt> for CreateDomainStmt {
    fn from(pb: protobuf::CreateDomainStmt) -> Self {
        CreateDomainStmt {
            domainname: pb.domainname.into_iter().map(|n| n.into()).collect(),
            type_name: pb.type_name.map(|v| v.into()),
            coll_clause: pb.coll_clause.map(|v| v.into()),
            constraints: pb.constraints.into_iter().map(|n| n.into()).collect(),
        }
    }
}

impl From<protobuf::CreateTableAsStmt> for CreateTableAsStmt {
    fn from(pb: protobuf::CreateTableAsStmt) -> Self {
        CreateTableAsStmt {
            query: pb.query.map(|n| n.into()),
            into: pb.into.map(|v| v.into()),
            objtype: pb.objtype.into(),
            is_select_into: pb.is_select_into,
            if_not_exists: pb.if_not_exists,
        }
    }
}

impl From<protobuf::RefreshMatViewStmt> for RefreshMatViewStmt {
    fn from(pb: protobuf::RefreshMatViewStmt) -> Self {
        RefreshMatViewStmt {
            concurrent: pb.concurrent,
            skip_data: pb.skip_data,
            relation: pb.relation.map(|v| v.into()),
        }
    }
}

// Transaction statement
impl From<protobuf::TransactionStmt> for TransactionStmt {
    fn from(pb: protobuf::TransactionStmt) -> Self {
        TransactionStmt {
            kind: pb.kind.into(),
            options: pb.options.into_iter().map(|n| n.into()).collect(),
            savepoint_name: pb.savepoint_name,
            gid: pb.gid,
            chain: pb.chain,
        }
    }
}

// Expression type conversions
impl From<protobuf::AExpr> for AExpr {
    fn from(pb: protobuf::AExpr) -> Self {
        AExpr {
            kind: pb.kind.into(),
            name: pb.name.into_iter().map(|n| n.into()).collect(),
            lexpr: pb.lexpr.map(|n| n.into()),
            rexpr: pb.rexpr.map(|n| n.into()),
            location: pb.location,
        }
    }
}

impl From<protobuf::ColumnRef> for ColumnRef {
    fn from(pb: protobuf::ColumnRef) -> Self {
        ColumnRef {
            fields: pb.fields.into_iter().map(|n| n.into()).collect(),
            location: pb.location,
        }
    }
}

impl From<protobuf::ParamRef> for ParamRef {
    fn from(pb: protobuf::ParamRef) -> Self {
        ParamRef {
            number: pb.number,
            location: pb.location,
        }
    }
}

impl From<protobuf::AConst> for AConst {
    fn from(pb: protobuf::AConst) -> Self {
        AConst {
            val: pb.val.map(|v| v.into()),
            isnull: pb.isnull,
            location: pb.location,
        }
    }
}

impl From<protobuf::a_const::Val> for AConstValue {
    fn from(pb: protobuf::a_const::Val) -> Self {
        use protobuf::a_const::Val;
        match pb {
            Val::Ival(v) => AConstValue::Integer(v.into()),
            Val::Fval(v) => AConstValue::Float(v.into()),
            Val::Boolval(v) => AConstValue::Boolean(v.into()),
            Val::Sval(v) => AConstValue::String(v.into()),
            Val::Bsval(v) => AConstValue::BitString(v.into()),
        }
    }
}

impl From<protobuf::TypeCast> for TypeCast {
    fn from(pb: protobuf::TypeCast) -> Self {
        TypeCast {
            arg: pb.arg.map(|n| n.into()),
            type_name: pb.type_name.map(|v| v.into()),
            location: pb.location,
        }
    }
}

impl From<protobuf::CollateClause> for CollateClause {
    fn from(pb: protobuf::CollateClause) -> Self {
        CollateClause {
            arg: pb.arg.map(|n| n.into()),
            collname: pb.collname.into_iter().map(|n| n.into()).collect(),
            location: pb.location,
        }
    }
}

impl From<protobuf::FuncCall> for FuncCall {
    fn from(pb: protobuf::FuncCall) -> Self {
        FuncCall {
            funcname: pb.funcname.into_iter().map(|n| n.into()).collect(),
            args: pb.args.into_iter().map(|n| n.into()).collect(),
            agg_order: pb.agg_order.into_iter().map(|n| n.into()).collect(),
            agg_filter: pb.agg_filter.map(|n| n.into()),
            over: pb.over.map(|v| (*v).into()),
            agg_within_group: pb.agg_within_group,
            agg_star: pb.agg_star,
            agg_distinct: pb.agg_distinct,
            func_variadic: pb.func_variadic,
            funcformat: pb.funcformat.into(),
            location: pb.location,
        }
    }
}

impl From<protobuf::AIndices> for AIndices {
    fn from(pb: protobuf::AIndices) -> Self {
        AIndices {
            is_slice: pb.is_slice,
            lidx: pb.lidx.map(|n| n.into()),
            uidx: pb.uidx.map(|n| n.into()),
        }
    }
}

impl From<protobuf::AIndirection> for AIndirection {
    fn from(pb: protobuf::AIndirection) -> Self {
        AIndirection {
            arg: pb.arg.map(|n| n.into()),
            indirection: pb.indirection.into_iter().map(|n| n.into()).collect(),
        }
    }
}

impl From<protobuf::AArrayExpr> for AArrayExpr {
    fn from(pb: protobuf::AArrayExpr) -> Self {
        AArrayExpr {
            elements: pb.elements.into_iter().map(|n| n.into()).collect(),
            location: pb.location,
        }
    }
}

impl From<protobuf::SubLink> for SubLink {
    fn from(pb: protobuf::SubLink) -> Self {
        SubLink {
            sub_link_type: pb.sub_link_type.into(),
            sub_link_id: pb.sub_link_id,
            testexpr: pb.testexpr.map(|n| n.into()),
            oper_name: pb.oper_name.into_iter().map(|n| n.into()).collect(),
            subselect: pb.subselect.map(|n| n.into()),
            location: pb.location,
        }
    }
}

impl From<protobuf::BoolExpr> for BoolExpr {
    fn from(pb: protobuf::BoolExpr) -> Self {
        BoolExpr {
            boolop: pb.boolop.into(),
            args: pb.args.into_iter().map(|n| n.into()).collect(),
            location: pb.location,
        }
    }
}

impl From<protobuf::NullTest> for NullTest {
    fn from(pb: protobuf::NullTest) -> Self {
        NullTest {
            arg: pb.arg.map(|n| n.into()),
            nulltesttype: pb.nulltesttype.into(),
            argisrow: pb.argisrow,
            location: pb.location,
        }
    }
}

impl From<protobuf::BooleanTest> for BooleanTest {
    fn from(pb: protobuf::BooleanTest) -> Self {
        BooleanTest {
            arg: pb.arg.map(|n| n.into()),
            booltesttype: pb.booltesttype.into(),
            location: pb.location,
        }
    }
}

impl From<protobuf::CaseExpr> for CaseExpr {
    fn from(pb: protobuf::CaseExpr) -> Self {
        CaseExpr {
            arg: pb.arg.map(|n| n.into()),
            args: pb.args.into_iter().map(|n| n.into()).collect(),
            defresult: pb.defresult.map(|n| n.into()),
            location: pb.location,
        }
    }
}

impl From<protobuf::CaseWhen> for CaseWhen {
    fn from(pb: protobuf::CaseWhen) -> Self {
        CaseWhen {
            expr: pb.expr.map(|n| n.into()),
            result: pb.result.map(|n| n.into()),
            location: pb.location,
        }
    }
}

impl From<protobuf::CoalesceExpr> for CoalesceExpr {
    fn from(pb: protobuf::CoalesceExpr) -> Self {
        CoalesceExpr {
            args: pb.args.into_iter().map(|n| n.into()).collect(),
            location: pb.location,
        }
    }
}

impl From<protobuf::MinMaxExpr> for MinMaxExpr {
    fn from(pb: protobuf::MinMaxExpr) -> Self {
        MinMaxExpr {
            op: pb.op.into(),
            args: pb.args.into_iter().map(|n| n.into()).collect(),
            location: pb.location,
        }
    }
}

impl From<protobuf::RowExpr> for RowExpr {
    fn from(pb: protobuf::RowExpr) -> Self {
        RowExpr {
            args: pb.args.into_iter().map(|n| n.into()).collect(),
            row_format: pb.row_format.into(),
            colnames: pb.colnames.into_iter().map(|n| n.into()).collect(),
            location: pb.location,
        }
    }
}

// Target/Result type conversions
impl From<protobuf::ResTarget> for ResTarget {
    fn from(pb: protobuf::ResTarget) -> Self {
        ResTarget {
            name: pb.name,
            indirection: pb.indirection.into_iter().map(|n| n.into()).collect(),
            val: pb.val.map(|n| n.into()),
            location: pb.location,
        }
    }
}

// Table/Range type conversions
impl From<protobuf::RangeVar> for RangeVar {
    fn from(pb: protobuf::RangeVar) -> Self {
        RangeVar {
            catalogname: pb.catalogname,
            schemaname: pb.schemaname,
            relname: pb.relname,
            inh: pb.inh,
            relpersistence: pb.relpersistence,
            alias: pb.alias.map(|v| v.into()),
            location: pb.location,
        }
    }
}

impl From<protobuf::RangeSubselect> for RangeSubselect {
    fn from(pb: protobuf::RangeSubselect) -> Self {
        RangeSubselect {
            lateral: pb.lateral,
            subquery: pb.subquery.map(|n| n.into()),
            alias: pb.alias.map(|v| v.into()),
        }
    }
}

impl From<protobuf::RangeFunction> for RangeFunction {
    fn from(pb: protobuf::RangeFunction) -> Self {
        RangeFunction {
            lateral: pb.lateral,
            ordinality: pb.ordinality,
            is_rowsfrom: pb.is_rowsfrom,
            functions: pb.functions.into_iter().map(|n| n.into()).collect(),
            alias: pb.alias.map(|v| v.into()),
            coldeflist: pb.coldeflist.into_iter().map(|n| n.into()).collect(),
        }
    }
}

impl From<protobuf::JoinExpr> for JoinExpr {
    fn from(pb: protobuf::JoinExpr) -> Self {
        JoinExpr {
            jointype: pb.jointype.into(),
            is_natural: pb.is_natural,
            larg: pb.larg.map(|n| n.into()),
            rarg: pb.rarg.map(|n| n.into()),
            using_clause: pb.using_clause.into_iter().map(|n| n.into()).collect(),
            join_using_alias: pb.join_using_alias.map(|v| v.into()),
            quals: pb.quals.map(|n| n.into()),
            alias: pb.alias.map(|v| v.into()),
            rtindex: pb.rtindex,
        }
    }
}

// Clause type conversions
impl From<protobuf::SortBy> for SortBy {
    fn from(pb: protobuf::SortBy) -> Self {
        SortBy {
            node: pb.node.map(|n| n.into()),
            sortby_dir: pb.sortby_dir.into(),
            sortby_nulls: pb.sortby_nulls.into(),
            use_op: pb.use_op.into_iter().map(|n| n.into()).collect(),
            location: pb.location,
        }
    }
}

impl From<protobuf::WindowDef> for WindowDef {
    fn from(pb: protobuf::WindowDef) -> Self {
        WindowDef {
            name: pb.name,
            refname: pb.refname,
            partition_clause: pb.partition_clause.into_iter().map(|n| n.into()).collect(),
            order_clause: pb.order_clause.into_iter().map(|n| n.into()).collect(),
            frame_options: pb.frame_options,
            start_offset: pb.start_offset.map(|n| n.into()),
            end_offset: pb.end_offset.map(|n| n.into()),
            location: pb.location,
        }
    }
}

impl From<protobuf::WithClause> for WithClause {
    fn from(pb: protobuf::WithClause) -> Self {
        WithClause {
            ctes: pb.ctes.into_iter().map(|n| n.into()).collect(),
            recursive: pb.recursive,
            location: pb.location,
        }
    }
}

impl From<protobuf::CommonTableExpr> for CommonTableExpr {
    fn from(pb: protobuf::CommonTableExpr) -> Self {
        CommonTableExpr {
            ctename: pb.ctename,
            aliascolnames: pb.aliascolnames.into_iter().map(|n| n.into()).collect(),
            ctematerialized: pb.ctematerialized.into(),
            ctequery: pb.ctequery.map(|n| n.into()),
            search_clause: pb.search_clause.map(|n| Node::Other(protobuf::Node { node: Some(protobuf::node::Node::CtesearchClause(n)) })),
            cycle_clause: pb.cycle_clause.map(|n| Node::Other(protobuf::Node { node: Some(protobuf::node::Node::CtecycleClause(n)) })),
            location: pb.location,
            cterecursive: pb.cterecursive,
            cterefcount: pb.cterefcount,
            ctecolnames: pb.ctecolnames.into_iter().map(|n| n.into()).collect(),
            ctecoltypes: pb.ctecoltypes.into_iter().map(|n| n.into()).collect(),
            ctecoltypmods: pb.ctecoltypmods.into_iter().map(|n| n.into()).collect(),
            ctecolcollations: pb.ctecolcollations.into_iter().map(|n| n.into()).collect(),
        }
    }
}

impl From<protobuf::IntoClause> for IntoClause {
    fn from(pb: protobuf::IntoClause) -> Self {
        IntoClause {
            rel: pb.rel.map(|v| v.into()),
            col_names: pb.col_names.into_iter().map(|n| n.into()).collect(),
            access_method: pb.access_method,
            options: pb.options.into_iter().map(|n| n.into()).collect(),
            on_commit: pb.on_commit.into(),
            table_space_name: pb.table_space_name,
            view_query: pb.view_query.map(|n| n.into()),
            skip_data: pb.skip_data,
        }
    }
}

impl From<protobuf::OnConflictClause> for OnConflictClause {
    fn from(pb: protobuf::OnConflictClause) -> Self {
        OnConflictClause {
            action: pb.action.into(),
            infer: pb.infer.map(|n| Node::Other(protobuf::Node { node: Some(protobuf::node::Node::InferClause(n)) })),
            target_list: pb.target_list.into_iter().map(|n| n.into()).collect(),
            where_clause: pb.where_clause.map(|n| n.into()),
            location: pb.location,
        }
    }
}

impl From<protobuf::LockingClause> for LockingClause {
    fn from(pb: protobuf::LockingClause) -> Self {
        LockingClause {
            locked_rels: pb.locked_rels.into_iter().map(|n| n.into()).collect(),
            strength: pb.strength.into(),
            wait_policy: pb.wait_policy.into(),
        }
    }
}

impl From<protobuf::GroupingSet> for GroupingSet {
    fn from(pb: protobuf::GroupingSet) -> Self {
        GroupingSet {
            kind: pb.kind.into(),
            content: pb.content.into_iter().map(|n| n.into()).collect(),
            location: pb.location,
        }
    }
}

impl From<protobuf::MergeWhenClause> for MergeWhenClause {
    fn from(pb: protobuf::MergeWhenClause) -> Self {
        MergeWhenClause {
            matched: pb.matched,
            command_type: pb.command_type.into(),
            override_: pb.r#override.into(),
            condition: pb.condition.map(|n| n.into()),
            target_list: pb.target_list.into_iter().map(|n| n.into()).collect(),
            values: pb.values.into_iter().map(|n| n.into()).collect(),
        }
    }
}

// Type-related conversions
impl From<protobuf::TypeName> for TypeName {
    fn from(pb: protobuf::TypeName) -> Self {
        TypeName {
            names: pb.names.into_iter().map(|n| n.into()).collect(),
            type_oid: pb.type_oid,
            setof: pb.setof,
            pct_type: pb.pct_type,
            typmods: pb.typmods.into_iter().map(|n| n.into()).collect(),
            typemod: pb.typemod,
            array_bounds: pb.array_bounds.into_iter().map(|n| n.into()).collect(),
            location: pb.location,
        }
    }
}

impl From<protobuf::ColumnDef> for ColumnDef {
    fn from(pb: protobuf::ColumnDef) -> Self {
        ColumnDef {
            colname: pb.colname,
            type_name: pb.type_name.map(|v| v.into()),
            compression: pb.compression,
            inhcount: pb.inhcount,
            is_local: pb.is_local,
            is_not_null: pb.is_not_null,
            is_from_type: pb.is_from_type,
            storage: pb.storage,
            storage_name: pb.storage_name,
            raw_default: pb.raw_default.map(|n| n.into()),
            cooked_default: pb.cooked_default.map(|n| n.into()),
            identity: pb.identity,
            identity_sequence: pb.identity_sequence.map(|v| v.into()),
            generated: pb.generated,
            coll_clause: pb.coll_clause.map(|v| v.into()),
            coll_oid: pb.coll_oid,
            constraints: pb.constraints.into_iter().map(|n| n.into()).collect(),
            fdwoptions: pb.fdwoptions.into_iter().map(|n| n.into()).collect(),
            location: pb.location,
        }
    }
}

impl From<protobuf::Constraint> for Constraint {
    fn from(pb: protobuf::Constraint) -> Self {
        Constraint {
            contype: pb.contype.into(),
            conname: pb.conname,
            deferrable: pb.deferrable,
            initdeferred: pb.initdeferred,
            location: pb.location,
            is_no_inherit: pb.is_no_inherit,
            raw_expr: pb.raw_expr.map(|n| n.into()),
            cooked_expr: pb.cooked_expr,
            generated_when: pb.generated_when,
            nulls_not_distinct: pb.nulls_not_distinct,
            keys: pb.keys.into_iter().map(|n| n.into()).collect(),
            including: pb.including.into_iter().map(|n| n.into()).collect(),
            exclusions: pb.exclusions.into_iter().map(|n| n.into()).collect(),
            options: pb.options.into_iter().map(|n| n.into()).collect(),
            indexname: pb.indexname,
            indexspace: pb.indexspace,
            reset_default_tblspc: pb.reset_default_tblspc,
            access_method: pb.access_method,
            where_clause: pb.where_clause.map(|n| n.into()),
            pktable: pb.pktable.map(|v| v.into()),
            fk_attrs: pb.fk_attrs.into_iter().map(|n| n.into()).collect(),
            pk_attrs: pb.pk_attrs.into_iter().map(|n| n.into()).collect(),
            fk_matchtype: pb.fk_matchtype,
            fk_upd_action: pb.fk_upd_action,
            fk_del_action: pb.fk_del_action,
            fk_del_set_cols: pb.fk_del_set_cols.into_iter().map(|n| n.into()).collect(),
            old_conpfeqop: pb.old_conpfeqop.into_iter().map(|n| n.into()).collect(),
            old_pktable_oid: pb.old_pktable_oid,
            skip_validation: pb.skip_validation,
            initially_valid: pb.initially_valid,
        }
    }
}

impl From<protobuf::DefElem> for DefElem {
    fn from(pb: protobuf::DefElem) -> Self {
        DefElem {
            defnamespace: pb.defnamespace,
            defname: pb.defname,
            arg: pb.arg.map(|n| n.into()),
            defaction: pb.defaction.into(),
            location: pb.location,
        }
    }
}

impl From<protobuf::IndexElem> for IndexElem {
    fn from(pb: protobuf::IndexElem) -> Self {
        IndexElem {
            name: pb.name,
            expr: pb.expr.map(|n| n.into()),
            indexcolname: pb.indexcolname,
            collation: pb.collation.into_iter().map(|n| n.into()).collect(),
            opclass: pb.opclass.into_iter().map(|n| n.into()).collect(),
            opclassopts: pb.opclassopts.into_iter().map(|n| n.into()).collect(),
            ordering: pb.ordering.into(),
            nulls_ordering: pb.nulls_ordering.into(),
        }
    }
}

// Alias and role type conversions
impl From<protobuf::Alias> for Alias {
    fn from(pb: protobuf::Alias) -> Self {
        Alias {
            aliasname: pb.aliasname,
            colnames: pb.colnames.into_iter().map(|n| n.into()).collect(),
        }
    }
}

impl From<protobuf::RoleSpec> for RoleSpec {
    fn from(pb: protobuf::RoleSpec) -> Self {
        RoleSpec {
            roletype: pb.roletype.into(),
            rolename: pb.rolename,
            location: pb.location,
        }
    }
}

// Other type conversions
impl From<protobuf::SortGroupClause> for SortGroupClause {
    fn from(pb: protobuf::SortGroupClause) -> Self {
        SortGroupClause {
            tle_sort_group_ref: pb.tle_sort_group_ref,
            eqop: pb.eqop,
            sortop: pb.sortop,
            nulls_first: pb.nulls_first,
            hashable: pb.hashable,
        }
    }
}

impl From<protobuf::FunctionParameter> for FunctionParameter {
    fn from(pb: protobuf::FunctionParameter) -> Self {
        FunctionParameter {
            name: pb.name,
            arg_type: pb.arg_type.map(|v| v.into()),
            mode: pb.mode.into(),
            defexpr: pb.defexpr.map(|n| n.into()),
        }
    }
}

impl From<protobuf::AlterTableCmd> for AlterTableCmd {
    fn from(pb: protobuf::AlterTableCmd) -> Self {
        AlterTableCmd {
            subtype: pb.subtype.into(),
            name: pb.name,
            num: pb.num as i16,
            newowner: pb.newowner.map(|v| v.into()),
            def: pb.def.map(|n| n.into()),
            behavior: pb.behavior.into(),
            missing_ok: pb.missing_ok,
            recurse: pb.recurse,
        }
    }
}

impl From<protobuf::AccessPriv> for AccessPriv {
    fn from(pb: protobuf::AccessPriv) -> Self {
        AccessPriv {
            priv_name: pb.priv_name,
            cols: pb.cols.into_iter().map(|n| n.into()).collect(),
        }
    }
}

impl From<protobuf::ObjectWithArgs> for ObjectWithArgs {
    fn from(pb: protobuf::ObjectWithArgs) -> Self {
        ObjectWithArgs {
            objname: pb.objname.into_iter().map(|n| n.into()).collect(),
            objargs: pb.objargs.into_iter().map(|n| n.into()).collect(),
            objfuncargs: pb.objfuncargs.into_iter().map(|n| n.into()).collect(),
            args_unspecified: pb.args_unspecified,
        }
    }
}

// Administrative statement conversions
impl From<protobuf::VariableSetStmt> for VariableSetStmt {
    fn from(pb: protobuf::VariableSetStmt) -> Self {
        VariableSetStmt {
            kind: pb.kind.into(),
            name: pb.name,
            args: pb.args.into_iter().map(|n| n.into()).collect(),
            is_local: pb.is_local,
        }
    }
}

impl From<protobuf::VariableShowStmt> for VariableShowStmt {
    fn from(pb: protobuf::VariableShowStmt) -> Self {
        VariableShowStmt { name: pb.name }
    }
}

impl From<protobuf::ExplainStmt> for ExplainStmt {
    fn from(pb: protobuf::ExplainStmt) -> Self {
        ExplainStmt {
            query: pb.query.map(|n| n.into()),
            options: pb.options.into_iter().map(|n| n.into()).collect(),
        }
    }
}

impl From<protobuf::CopyStmt> for CopyStmt {
    fn from(pb: protobuf::CopyStmt) -> Self {
        CopyStmt {
            relation: pb.relation.map(|v| v.into()),
            query: pb.query.map(|n| n.into()),
            attlist: pb.attlist.into_iter().map(|n| n.into()).collect(),
            is_from: pb.is_from,
            is_program: pb.is_program,
            filename: pb.filename,
            options: pb.options.into_iter().map(|n| n.into()).collect(),
            where_clause: pb.where_clause.map(|n| n.into()),
        }
    }
}

impl From<protobuf::GrantStmt> for GrantStmt {
    fn from(pb: protobuf::GrantStmt) -> Self {
        GrantStmt {
            is_grant: pb.is_grant,
            targtype: pb.targtype.into(),
            objtype: pb.objtype.into(),
            objects: pb.objects.into_iter().map(|n| n.into()).collect(),
            privileges: pb.privileges.into_iter().map(|n| n.into()).collect(),
            grantees: pb.grantees.into_iter().map(|n| n.into()).collect(),
            grant_option: pb.grant_option,
            grantor: pb.grantor.map(|v| v.into()),
            behavior: pb.behavior.into(),
        }
    }
}

impl From<protobuf::GrantRoleStmt> for GrantRoleStmt {
    fn from(pb: protobuf::GrantRoleStmt) -> Self {
        GrantRoleStmt {
            granted_roles: pb.granted_roles.into_iter().map(|n| n.into()).collect(),
            grantee_roles: pb.grantee_roles.into_iter().map(|n| n.into()).collect(),
            is_grant: pb.is_grant,
            opt: pb.opt.into_iter().map(|n| n.into()).collect(),
            grantor: pb.grantor.map(|v| v.into()),
            behavior: pb.behavior.into(),
        }
    }
}

impl From<protobuf::LockStmt> for LockStmt {
    fn from(pb: protobuf::LockStmt) -> Self {
        LockStmt {
            relations: pb.relations.into_iter().map(|n| n.into()).collect(),
            mode: pb.mode,
            nowait: pb.nowait,
        }
    }
}

impl From<protobuf::VacuumStmt> for VacuumStmt {
    fn from(pb: protobuf::VacuumStmt) -> Self {
        VacuumStmt {
            options: pb.options.into_iter().map(|n| n.into()).collect(),
            rels: pb.rels.into_iter().map(|n| n.into()).collect(),
            is_vacuumcmd: pb.is_vacuumcmd,
        }
    }
}

// Other statement conversions
impl From<protobuf::DoStmt> for DoStmt {
    fn from(pb: protobuf::DoStmt) -> Self {
        DoStmt {
            args: pb.args.into_iter().map(|n| n.into()).collect(),
        }
    }
}

impl From<protobuf::RenameStmt> for RenameStmt {
    fn from(pb: protobuf::RenameStmt) -> Self {
        RenameStmt {
            rename_type: pb.rename_type.into(),
            relation_type: pb.relation_type.into(),
            relation: pb.relation.map(|v| v.into()),
            object: pb.object.map(|n| n.into()),
            subname: pb.subname,
            newname: pb.newname,
            behavior: pb.behavior.into(),
            missing_ok: pb.missing_ok,
        }
    }
}

impl From<protobuf::NotifyStmt> for NotifyStmt {
    fn from(pb: protobuf::NotifyStmt) -> Self {
        NotifyStmt {
            conditionname: pb.conditionname,
            payload: pb.payload,
        }
    }
}

impl From<protobuf::ListenStmt> for ListenStmt {
    fn from(pb: protobuf::ListenStmt) -> Self {
        ListenStmt {
            conditionname: pb.conditionname,
        }
    }
}

impl From<protobuf::UnlistenStmt> for UnlistenStmt {
    fn from(pb: protobuf::UnlistenStmt) -> Self {
        UnlistenStmt {
            conditionname: pb.conditionname,
        }
    }
}

impl From<protobuf::DiscardStmt> for DiscardStmt {
    fn from(pb: protobuf::DiscardStmt) -> Self {
        DiscardStmt {
            target: pb.target.into(),
        }
    }
}

impl From<protobuf::PrepareStmt> for PrepareStmt {
    fn from(pb: protobuf::PrepareStmt) -> Self {
        PrepareStmt {
            name: pb.name,
            argtypes: pb.argtypes.into_iter().map(|n| n.into()).collect(),
            query: pb.query.map(|n| n.into()),
        }
    }
}

impl From<protobuf::ExecuteStmt> for ExecuteStmt {
    fn from(pb: protobuf::ExecuteStmt) -> Self {
        ExecuteStmt {
            name: pb.name,
            params: pb.params.into_iter().map(|n| n.into()).collect(),
        }
    }
}

impl From<protobuf::DeallocateStmt> for DeallocateStmt {
    fn from(pb: protobuf::DeallocateStmt) -> Self {
        DeallocateStmt { name: pb.name }
    }
}

impl From<protobuf::ClosePortalStmt> for ClosePortalStmt {
    fn from(pb: protobuf::ClosePortalStmt) -> Self {
        ClosePortalStmt {
            portalname: pb.portalname,
        }
    }
}

impl From<protobuf::FetchStmt> for FetchStmt {
    fn from(pb: protobuf::FetchStmt) -> Self {
        FetchStmt {
            direction: pb.direction.into(),
            how_many: pb.how_many,
            portalname: pb.portalname,
            ismove: pb.ismove,
        }
    }
}

// ============================================================================
// Enum conversions
// ============================================================================

impl From<i32> for SetOperation {
    fn from(v: i32) -> Self {
        match v {
            1 => SetOperation::None, // SETOP_NONE
            2 => SetOperation::Union, // SETOP_UNION
            3 => SetOperation::Intersect, // SETOP_INTERSECT
            4 => SetOperation::Except, // SETOP_EXCEPT
            _ => SetOperation::None,
        }
    }
}

impl From<i32> for LimitOption {
    fn from(v: i32) -> Self {
        match v {
            1 => LimitOption::Default, // LIMIT_OPTION_DEFAULT
            2 => LimitOption::Count, // LIMIT_OPTION_COUNT
            3 => LimitOption::WithTies, // LIMIT_OPTION_WITH_TIES
            _ => LimitOption::Default,
        }
    }
}

impl From<i32> for AExprKind {
    fn from(v: i32) -> Self {
        match v {
            1 => AExprKind::Op, // AEXPR_OP
            2 => AExprKind::OpAny, // AEXPR_OP_ANY
            3 => AExprKind::OpAll, // AEXPR_OP_ALL
            4 => AExprKind::Distinct, // AEXPR_DISTINCT
            5 => AExprKind::NotDistinct, // AEXPR_NOT_DISTINCT
            6 => AExprKind::NullIf, // AEXPR_NULLIF
            7 => AExprKind::In, // AEXPR_IN
            8 => AExprKind::Like, // AEXPR_LIKE
            9 => AExprKind::ILike, // AEXPR_ILIKE
            10 => AExprKind::Similar, // AEXPR_SIMILAR
            11 => AExprKind::Between, // AEXPR_BETWEEN
            12 => AExprKind::NotBetween, // AEXPR_NOT_BETWEEN
            13 => AExprKind::BetweenSym, // AEXPR_BETWEEN_SYM
            14 => AExprKind::NotBetweenSym, // AEXPR_NOT_BETWEEN_SYM
            _ => AExprKind::Op,
        }
    }
}

impl From<i32> for BoolExprType {
    fn from(v: i32) -> Self {
        match v {
            1 => BoolExprType::And, // AND_EXPR
            2 => BoolExprType::Or, // OR_EXPR
            3 => BoolExprType::Not, // NOT_EXPR
            _ => BoolExprType::And,
        }
    }
}

impl From<i32> for SubLinkType {
    fn from(v: i32) -> Self {
        match v {
            1 => SubLinkType::Exists,
            2 => SubLinkType::All,
            3 => SubLinkType::Any,
            4 => SubLinkType::RowCompare,
            5 => SubLinkType::Expr,
            6 => SubLinkType::MultiExpr,
            7 => SubLinkType::Array,
            8 => SubLinkType::Cte,
            _ => SubLinkType::Exists,
        }
    }
}

impl From<i32> for NullTestType {
    fn from(v: i32) -> Self {
        match v {
            1 => NullTestType::IsNull,
            2 => NullTestType::IsNotNull,
            _ => NullTestType::IsNull,
        }
    }
}

impl From<i32> for BoolTestType {
    fn from(v: i32) -> Self {
        match v {
            1 => BoolTestType::IsTrue,
            2 => BoolTestType::IsNotTrue,
            3 => BoolTestType::IsFalse,
            4 => BoolTestType::IsNotFalse,
            5 => BoolTestType::IsUnknown,
            6 => BoolTestType::IsNotUnknown,
            _ => BoolTestType::IsTrue,
        }
    }
}

impl From<i32> for MinMaxOp {
    fn from(v: i32) -> Self {
        match v {
            1 => MinMaxOp::Greatest,
            2 => MinMaxOp::Least,
            _ => MinMaxOp::Greatest,
        }
    }
}

impl From<i32> for JoinType {
    fn from(v: i32) -> Self {
        match v {
            1 => JoinType::Inner,
            2 => JoinType::Left,
            3 => JoinType::Full,
            4 => JoinType::Right,
            5 => JoinType::Semi,
            6 => JoinType::Anti,
            7 => JoinType::RightAnti,
            8 => JoinType::UniqueOuter,
            9 => JoinType::UniqueInner,
            _ => JoinType::Inner,
        }
    }
}

impl From<i32> for SortByDir {
    fn from(v: i32) -> Self {
        match v {
            1 => SortByDir::Default,
            2 => SortByDir::Asc,
            3 => SortByDir::Desc,
            4 => SortByDir::Using,
            _ => SortByDir::Default,
        }
    }
}

impl From<i32> for SortByNulls {
    fn from(v: i32) -> Self {
        match v {
            1 => SortByNulls::Default,
            2 => SortByNulls::First,
            3 => SortByNulls::Last,
            _ => SortByNulls::Default,
        }
    }
}

impl From<i32> for CTEMaterialize {
    fn from(v: i32) -> Self {
        match v {
            1 => CTEMaterialize::Default,
            2 => CTEMaterialize::Always,
            3 => CTEMaterialize::Never,
            _ => CTEMaterialize::Default,
        }
    }
}

impl From<i32> for OnCommitAction {
    fn from(v: i32) -> Self {
        match v {
            1 => OnCommitAction::Noop,
            2 => OnCommitAction::PreserveRows,
            3 => OnCommitAction::DeleteRows,
            4 => OnCommitAction::Drop,
            _ => OnCommitAction::Noop,
        }
    }
}

impl From<i32> for ObjectType {
    fn from(v: i32) -> Self {
        // Use direct integer matching
        // Values from protobuf ObjectType enum
        match v {
            1 => ObjectType::AccessMethod,
            2 => ObjectType::Aggregate,
            11 => ObjectType::Cast,
            12 => ObjectType::Column,
            13 => ObjectType::Collation,
            14 => ObjectType::Conversion,
            15 => ObjectType::Database,
            16 => ObjectType::Default,
            17 => ObjectType::Constraint,
            18 => ObjectType::Domain,
            19 => ObjectType::EventTrigger,
            20 => ObjectType::Extension,
            21 => ObjectType::Fdw,
            22 => ObjectType::ForeignServer,
            23 => ObjectType::ForeignTable,
            24 => ObjectType::Function,
            25 => ObjectType::Index,
            26 => ObjectType::Language,
            27 => ObjectType::LargeObject,
            28 => ObjectType::MatView,
            29 => ObjectType::Operator,
            37 => ObjectType::Policy,
            38 => ObjectType::Procedure,
            39 => ObjectType::Publication,
            44 => ObjectType::Role,
            45 => ObjectType::Routine,
            46 => ObjectType::Rule,
            47 => ObjectType::Schema,
            48 => ObjectType::Sequence,
            49 => ObjectType::Subscription,
            50 => ObjectType::StatisticsObject,
            54 => ObjectType::Table,
            55 => ObjectType::Tablespace,
            57 => ObjectType::Transform,
            58 => ObjectType::Trigger,
            60 => ObjectType::Type,
            62 => ObjectType::View,
            _ => ObjectType::Table,
        }
    }
}

impl From<i32> for DropBehavior {
    fn from(v: i32) -> Self {
        match v {
            1 => DropBehavior::Restrict,
            2 => DropBehavior::Cascade,
            _ => DropBehavior::Restrict,
        }
    }
}

impl From<i32> for OnConflictAction {
    fn from(v: i32) -> Self {
        match v {
            1 => OnConflictAction::None,
            2 => OnConflictAction::Nothing,
            3 => OnConflictAction::Update,
            _ => OnConflictAction::None,
        }
    }
}

impl From<i32> for GroupingSetKind {
    fn from(v: i32) -> Self {
        match v {
            1 => GroupingSetKind::Empty,
            2 => GroupingSetKind::Simple,
            3 => GroupingSetKind::Rollup,
            4 => GroupingSetKind::Cube,
            5 => GroupingSetKind::Sets,
            _ => GroupingSetKind::Empty,
        }
    }
}

impl From<i32> for CmdType {
    fn from(v: i32) -> Self {
        match v {
            1 => CmdType::Unknown,
            2 => CmdType::Select,
            3 => CmdType::Update,
            4 => CmdType::Insert,
            5 => CmdType::Delete,
            6 => CmdType::Merge,
            7 => CmdType::Utility,
            8 => CmdType::Nothing,
            _ => CmdType::Unknown,
        }
    }
}

impl From<i32> for TransactionStmtKind {
    fn from(v: i32) -> Self {
        match v {
            1 => TransactionStmtKind::Begin,
            2 => TransactionStmtKind::Start,
            3 => TransactionStmtKind::Commit,
            4 => TransactionStmtKind::Rollback,
            5 => TransactionStmtKind::Savepoint,
            6 => TransactionStmtKind::Release,
            7 => TransactionStmtKind::RollbackTo,
            8 => TransactionStmtKind::Prepare,
            9 => TransactionStmtKind::CommitPrepared,
            10 => TransactionStmtKind::RollbackPrepared,
            _ => TransactionStmtKind::Begin,
        }
    }
}

impl From<i32> for ConstrType {
    fn from(v: i32) -> Self {
        match v {
            1 => ConstrType::Null,
            2 => ConstrType::NotNull,
            3 => ConstrType::Default,
            4 => ConstrType::Identity,
            5 => ConstrType::Generated,
            6 => ConstrType::Check,
            7 => ConstrType::Primary,
            8 => ConstrType::Unique,
            9 => ConstrType::Exclusion,
            10 => ConstrType::Foreign,
            11 => ConstrType::AttrDeferrable,
            12 => ConstrType::AttrNotDeferrable,
            13 => ConstrType::AttrDeferred,
            14 => ConstrType::AttrImmediate,
            _ => ConstrType::Null,
        }
    }
}

impl From<i32> for DefElemAction {
    fn from(v: i32) -> Self {
        match v {
            1 => DefElemAction::Unspec,
            2 => DefElemAction::Set,
            3 => DefElemAction::Add,
            4 => DefElemAction::Drop,
            _ => DefElemAction::Unspec,
        }
    }
}

impl From<i32> for RoleSpecType {
    fn from(v: i32) -> Self {
        match v {
            1 => RoleSpecType::CString,
            2 => RoleSpecType::CurrentRole,
            3 => RoleSpecType::CurrentUser,
            4 => RoleSpecType::SessionUser,
            5 => RoleSpecType::Public,
            _ => RoleSpecType::CString,
        }
    }
}

impl From<i32> for CoercionForm {
    fn from(v: i32) -> Self {
        match v {
            1 => CoercionForm::ExplicitCall,
            2 => CoercionForm::ExplicitCast,
            3 => CoercionForm::ImplicitCast,
            4 => CoercionForm::SqlSyntax,
            _ => CoercionForm::ExplicitCall,
        }
    }
}

impl From<i32> for VariableSetKind {
    fn from(v: i32) -> Self {
        match v {
            1 => VariableSetKind::Value,
            2 => VariableSetKind::Default,
            3 => VariableSetKind::Current,
            4 => VariableSetKind::Multi,
            5 => VariableSetKind::Reset,
            6 => VariableSetKind::ResetAll,
            _ => VariableSetKind::Value,
        }
    }
}

impl From<i32> for LockClauseStrength {
    fn from(v: i32) -> Self {
        match v {
            1 => LockClauseStrength::None,
            2 => LockClauseStrength::ForKeyShare,
            3 => LockClauseStrength::ForShare,
            4 => LockClauseStrength::ForNoKeyUpdate,
            5 => LockClauseStrength::ForUpdate,
            _ => LockClauseStrength::None,
        }
    }
}

impl From<i32> for LockWaitPolicy {
    fn from(v: i32) -> Self {
        match v {
            1 => LockWaitPolicy::Block,
            2 => LockWaitPolicy::Skip,
            3 => LockWaitPolicy::Error,
            _ => LockWaitPolicy::Block,
        }
    }
}

impl From<i32> for ViewCheckOption {
    fn from(v: i32) -> Self {
        match v {
            1 => ViewCheckOption::NoCheckOption,
            2 => ViewCheckOption::Local,
            3 => ViewCheckOption::Cascaded,
            _ => ViewCheckOption::NoCheckOption,
        }
    }
}

impl From<i32> for DiscardMode {
    fn from(v: i32) -> Self {
        match v {
            1 => DiscardMode::All,
            2 => DiscardMode::Plans,
            3 => DiscardMode::Sequences,
            4 => DiscardMode::Temp,
            _ => DiscardMode::All,
        }
    }
}

impl From<i32> for FetchDirection {
    fn from(v: i32) -> Self {
        match v {
            1 => FetchDirection::Forward,
            2 => FetchDirection::Backward,
            3 => FetchDirection::Absolute,
            4 => FetchDirection::Relative,
            _ => FetchDirection::Forward,
        }
    }
}

impl From<i32> for FunctionParameterMode {
    fn from(v: i32) -> Self {
        match v {
            105 => FunctionParameterMode::In, // 'i'
            111 => FunctionParameterMode::Out, // 'o'
            98 => FunctionParameterMode::InOut, // 'b'
            118 => FunctionParameterMode::Variadic, // 'v'
            116 => FunctionParameterMode::Table, // 't'
            _ => FunctionParameterMode::In,
        }
    }
}

impl From<i32> for AlterTableType {
    fn from(v: i32) -> Self {
        // AlterTableType has many variants, use default for simplicity
        // The values start at 1 and go up
        match v {
            1 => AlterTableType::AddColumn,
            2 => AlterTableType::AddColumnToView,
            3 => AlterTableType::ColumnDefault,
            4 => AlterTableType::CookedColumnDefault,
            5 => AlterTableType::DropNotNull,
            6 => AlterTableType::SetNotNull,
            7 => AlterTableType::DropExpression,
            8 => AlterTableType::CheckNotNull,
            9 => AlterTableType::SetStatistics,
            10 => AlterTableType::SetOptions,
            11 => AlterTableType::ResetOptions,
            12 => AlterTableType::SetStorage,
            13 => AlterTableType::SetCompression,
            14 => AlterTableType::DropColumn,
            15 => AlterTableType::AddIndex,
            16 => AlterTableType::ReAddIndex,
            17 => AlterTableType::AddConstraint,
            18 => AlterTableType::ReAddConstraint,
            19 => AlterTableType::AddIndexConstraint,
            20 => AlterTableType::AlterConstraint,
            21 => AlterTableType::ValidateConstraint,
            22 => AlterTableType::DropConstraint,
            23 => AlterTableType::ClusterOn,
            24 => AlterTableType::DropCluster,
            25 => AlterTableType::SetLogged,
            26 => AlterTableType::SetUnLogged,
            27 => AlterTableType::SetAccessMethod,
            28 => AlterTableType::DropOids,
            29 => AlterTableType::SetTableSpace,
            30 => AlterTableType::SetRelOptions,
            31 => AlterTableType::ResetRelOptions,
            32 => AlterTableType::ReplaceRelOptions,
            33 => AlterTableType::EnableTrig,
            34 => AlterTableType::EnableAlwaysTrig,
            35 => AlterTableType::EnableReplicaTrig,
            36 => AlterTableType::DisableTrig,
            37 => AlterTableType::EnableTrigAll,
            38 => AlterTableType::DisableTrigAll,
            39 => AlterTableType::EnableTrigUser,
            40 => AlterTableType::DisableTrigUser,
            41 => AlterTableType::EnableRule,
            42 => AlterTableType::EnableAlwaysRule,
            43 => AlterTableType::EnableReplicaRule,
            44 => AlterTableType::DisableRule,
            45 => AlterTableType::AddInherit,
            46 => AlterTableType::DropInherit,
            47 => AlterTableType::AddOf,
            48 => AlterTableType::DropOf,
            49 => AlterTableType::ReplicaIdentity,
            50 => AlterTableType::EnableRowSecurity,
            51 => AlterTableType::DisableRowSecurity,
            52 => AlterTableType::ForceRowSecurity,
            53 => AlterTableType::NoForceRowSecurity,
            54 => AlterTableType::GenericOptions,
            55 => AlterTableType::AttachPartition,
            56 => AlterTableType::DetachPartition,
            57 => AlterTableType::DetachPartitionFinalize,
            58 => AlterTableType::AddIdentity,
            59 => AlterTableType::SetIdentity,
            60 => AlterTableType::DropIdentity,
            61 => AlterTableType::ReAddStatistics,
            _ => AlterTableType::AddColumn,
        }
    }
}

impl From<i32> for GrantTargetType {
    fn from(v: i32) -> Self {
        match v {
            1 => GrantTargetType::Object,
            2 => GrantTargetType::AllInSchema,
            3 => GrantTargetType::Defaults,
            _ => GrantTargetType::Object,
        }
    }
}

impl From<i32> for OverridingKind {
    fn from(v: i32) -> Self {
        match v {
            1 => OverridingKind::NotSet,
            2 => OverridingKind::UserValue,
            3 => OverridingKind::SystemValue,
            _ => OverridingKind::NotSet,
        }
    }
}

