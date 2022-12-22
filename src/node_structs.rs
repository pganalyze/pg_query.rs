use crate::*;

impl Node {
    pub fn deparse(&self) -> Result<String> {
        crate::deparse(&protobuf::ParseResult {
            version: crate::bindings::PG_VERSION_NUM as i32,
            stmts: vec![protobuf::RawStmt { stmt: Some(Box::new(self.clone())), stmt_location: 0, stmt_len: 0 }],
        })
    }
}

impl protobuf::Alias {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::Alias(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::Alias(self)
    }
}

impl protobuf::RangeVar {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RangeVar(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RangeVar(self)
    }
}

impl protobuf::TableFunc {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::TableFunc(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::TableFunc(self)
    }
}

impl protobuf::Var {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::Var(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::Var(self)
    }
}

impl protobuf::Param {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::Param(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::Param(self)
    }
}

impl protobuf::Aggref {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::Aggref(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::Aggref(self)
    }
}

impl protobuf::GroupingFunc {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::GroupingFunc(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::GroupingFunc(self)
    }
}

impl protobuf::WindowFunc {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::WindowFunc(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::WindowFunc(self)
    }
}

impl protobuf::SubscriptingRef {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::SubscriptingRef(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::SubscriptingRef(self)
    }
}

impl protobuf::FuncExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::FuncExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::FuncExpr(self)
    }
}

impl protobuf::NamedArgExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::NamedArgExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::NamedArgExpr(self)
    }
}

impl protobuf::OpExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::OpExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::OpExpr(self)
    }
}

impl protobuf::DistinctExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::DistinctExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::DistinctExpr(self)
    }
}

impl protobuf::NullIfExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::NullIfExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::NullIfExpr(self)
    }
}

impl protobuf::ScalarArrayOpExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ScalarArrayOpExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ScalarArrayOpExpr(self)
    }
}

impl protobuf::BoolExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::BoolExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::BoolExpr(self)
    }
}

impl protobuf::SubLink {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::SubLink(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::SubLink(self)
    }
}

impl protobuf::SubPlan {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::SubPlan(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::SubPlan(self)
    }
}

impl protobuf::AlternativeSubPlan {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlternativeSubPlan(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlternativeSubPlan(self)
    }
}

impl protobuf::FieldSelect {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::FieldSelect(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::FieldSelect(self)
    }
}

impl protobuf::FieldStore {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::FieldStore(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::FieldStore(self)
    }
}

impl protobuf::RelabelType {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RelabelType(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RelabelType(self)
    }
}

impl protobuf::CoerceViaIo {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CoerceViaIo(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CoerceViaIo(self)
    }
}

impl protobuf::ArrayCoerceExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ArrayCoerceExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ArrayCoerceExpr(self)
    }
}

impl protobuf::ConvertRowtypeExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ConvertRowtypeExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ConvertRowtypeExpr(self)
    }
}

impl protobuf::CollateExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CollateExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CollateExpr(self)
    }
}

impl protobuf::CaseExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CaseExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CaseExpr(self)
    }
}

impl protobuf::CaseWhen {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CaseWhen(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CaseWhen(self)
    }
}

impl protobuf::CaseTestExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CaseTestExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CaseTestExpr(self)
    }
}

impl protobuf::ArrayExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ArrayExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ArrayExpr(self)
    }
}

impl protobuf::RowExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RowExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RowExpr(self)
    }
}

impl protobuf::RowCompareExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RowCompareExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RowCompareExpr(self)
    }
}

impl protobuf::CoalesceExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CoalesceExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CoalesceExpr(self)
    }
}

impl protobuf::MinMaxExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::MinMaxExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::MinMaxExpr(self)
    }
}

impl protobuf::SqlValueFunction {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::SqlvalueFunction(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::SqlvalueFunction(self)
    }
}

impl protobuf::XmlExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::XmlExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::XmlExpr(self)
    }
}

impl protobuf::NullTest {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::NullTest(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::NullTest(self)
    }
}

impl protobuf::BooleanTest {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::BooleanTest(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::BooleanTest(self)
    }
}

impl protobuf::CoerceToDomain {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CoerceToDomain(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CoerceToDomain(self)
    }
}

impl protobuf::CoerceToDomainValue {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CoerceToDomainValue(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CoerceToDomainValue(self)
    }
}

impl protobuf::SetToDefault {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::SetToDefault(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::SetToDefault(self)
    }
}

impl protobuf::CurrentOfExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CurrentOfExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CurrentOfExpr(self)
    }
}

impl protobuf::NextValueExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::NextValueExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::NextValueExpr(self)
    }
}

impl protobuf::InferenceElem {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::InferenceElem(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::InferenceElem(self)
    }
}

impl protobuf::TargetEntry {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::TargetEntry(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::TargetEntry(self)
    }
}

impl protobuf::RangeTblRef {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RangeTblRef(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RangeTblRef(self)
    }
}

impl protobuf::JoinExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::JoinExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::JoinExpr(self)
    }
}

impl protobuf::FromExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::FromExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::FromExpr(self)
    }
}

impl protobuf::OnConflictExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::OnConflictExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::OnConflictExpr(self)
    }
}

impl protobuf::IntoClause {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::IntoClause(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::IntoClause(self)
    }
}

impl protobuf::RawStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RawStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RawStmt(self)
    }
}

impl protobuf::Query {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::Query(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::Query(self)
    }
}

impl protobuf::InsertStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::InsertStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::InsertStmt(self)
    }
}

impl protobuf::DeleteStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::DeleteStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::DeleteStmt(self)
    }
}

impl protobuf::UpdateStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::UpdateStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::UpdateStmt(self)
    }
}

impl protobuf::SelectStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::SelectStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::SelectStmt(self)
    }
}

impl protobuf::AlterTableStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterTableStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterTableStmt(self)
    }
}

impl protobuf::AlterTableCmd {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterTableCmd(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterTableCmd(self)
    }
}

impl protobuf::AlterDomainStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterDomainStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterDomainStmt(self)
    }
}

impl protobuf::SetOperationStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::SetOperationStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::SetOperationStmt(self)
    }
}

impl protobuf::GrantStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::GrantStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::GrantStmt(self)
    }
}

impl protobuf::GrantRoleStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::GrantRoleStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::GrantRoleStmt(self)
    }
}

impl protobuf::AlterDefaultPrivilegesStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterDefaultPrivilegesStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterDefaultPrivilegesStmt(self)
    }
}

impl protobuf::ClosePortalStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ClosePortalStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ClosePortalStmt(self)
    }
}

impl protobuf::ClusterStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ClusterStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ClusterStmt(self)
    }
}

impl protobuf::CopyStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CopyStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CopyStmt(self)
    }
}

impl protobuf::CreateStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateStmt(self)
    }
}

impl protobuf::DefineStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::DefineStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::DefineStmt(self)
    }
}

impl protobuf::DropStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::DropStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::DropStmt(self)
    }
}

impl protobuf::TruncateStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::TruncateStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::TruncateStmt(self)
    }
}

impl protobuf::CommentStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CommentStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CommentStmt(self)
    }
}

impl protobuf::FetchStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::FetchStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::FetchStmt(self)
    }
}

impl protobuf::IndexStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::IndexStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::IndexStmt(self)
    }
}

impl protobuf::CreateFunctionStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateFunctionStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateFunctionStmt(self)
    }
}

impl protobuf::AlterFunctionStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterFunctionStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterFunctionStmt(self)
    }
}

impl protobuf::DoStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::DoStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::DoStmt(self)
    }
}

impl protobuf::RenameStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RenameStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RenameStmt(self)
    }
}

impl protobuf::RuleStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RuleStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RuleStmt(self)
    }
}

impl protobuf::NotifyStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::NotifyStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::NotifyStmt(self)
    }
}

impl protobuf::ListenStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ListenStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ListenStmt(self)
    }
}

impl protobuf::UnlistenStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::UnlistenStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::UnlistenStmt(self)
    }
}

impl protobuf::TransactionStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::TransactionStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::TransactionStmt(self)
    }
}

impl protobuf::ViewStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ViewStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ViewStmt(self)
    }
}

impl protobuf::LoadStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::LoadStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::LoadStmt(self)
    }
}

impl protobuf::CreateDomainStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateDomainStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateDomainStmt(self)
    }
}

impl protobuf::CreatedbStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreatedbStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreatedbStmt(self)
    }
}

impl protobuf::DropdbStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::DropdbStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::DropdbStmt(self)
    }
}

impl protobuf::VacuumStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::VacuumStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::VacuumStmt(self)
    }
}

impl protobuf::ExplainStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ExplainStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ExplainStmt(self)
    }
}

impl protobuf::CreateTableAsStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateTableAsStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateTableAsStmt(self)
    }
}

impl protobuf::CreateSeqStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateSeqStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateSeqStmt(self)
    }
}

impl protobuf::AlterSeqStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterSeqStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterSeqStmt(self)
    }
}

impl protobuf::VariableSetStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::VariableSetStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::VariableSetStmt(self)
    }
}

impl protobuf::VariableShowStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::VariableShowStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::VariableShowStmt(self)
    }
}

impl protobuf::DiscardStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::DiscardStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::DiscardStmt(self)
    }
}

impl protobuf::CreateTrigStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateTrigStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateTrigStmt(self)
    }
}

impl protobuf::CreatePLangStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreatePlangStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreatePlangStmt(self)
    }
}

impl protobuf::CreateRoleStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateRoleStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateRoleStmt(self)
    }
}

impl protobuf::AlterRoleStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterRoleStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterRoleStmt(self)
    }
}

impl protobuf::DropRoleStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::DropRoleStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::DropRoleStmt(self)
    }
}

impl protobuf::LockStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::LockStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::LockStmt(self)
    }
}

impl protobuf::ConstraintsSetStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ConstraintsSetStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ConstraintsSetStmt(self)
    }
}

impl protobuf::ReindexStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ReindexStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ReindexStmt(self)
    }
}

impl protobuf::CheckPointStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CheckPointStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CheckPointStmt(self)
    }
}

impl protobuf::CreateSchemaStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateSchemaStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateSchemaStmt(self)
    }
}

impl protobuf::AlterDatabaseStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterDatabaseStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterDatabaseStmt(self)
    }
}

impl protobuf::AlterDatabaseSetStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterDatabaseSetStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterDatabaseSetStmt(self)
    }
}

impl protobuf::AlterRoleSetStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterRoleSetStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterRoleSetStmt(self)
    }
}

impl protobuf::CreateConversionStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateConversionStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateConversionStmt(self)
    }
}

impl protobuf::CreateCastStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateCastStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateCastStmt(self)
    }
}

impl protobuf::CreateOpClassStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateOpClassStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateOpClassStmt(self)
    }
}

impl protobuf::CreateOpFamilyStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateOpFamilyStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateOpFamilyStmt(self)
    }
}

impl protobuf::AlterOpFamilyStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterOpFamilyStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterOpFamilyStmt(self)
    }
}

impl protobuf::PrepareStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::PrepareStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::PrepareStmt(self)
    }
}

impl protobuf::ExecuteStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ExecuteStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ExecuteStmt(self)
    }
}

impl protobuf::DeallocateStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::DeallocateStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::DeallocateStmt(self)
    }
}

impl protobuf::DeclareCursorStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::DeclareCursorStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::DeclareCursorStmt(self)
    }
}

impl protobuf::CreateTableSpaceStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateTableSpaceStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateTableSpaceStmt(self)
    }
}

impl protobuf::DropTableSpaceStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::DropTableSpaceStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::DropTableSpaceStmt(self)
    }
}

impl protobuf::AlterObjectDependsStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterObjectDependsStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterObjectDependsStmt(self)
    }
}

impl protobuf::AlterObjectSchemaStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterObjectSchemaStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterObjectSchemaStmt(self)
    }
}

impl protobuf::AlterOwnerStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterOwnerStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterOwnerStmt(self)
    }
}

impl protobuf::AlterOperatorStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterOperatorStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterOperatorStmt(self)
    }
}

impl protobuf::AlterTypeStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterTypeStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterTypeStmt(self)
    }
}

impl protobuf::DropOwnedStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::DropOwnedStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::DropOwnedStmt(self)
    }
}

impl protobuf::ReassignOwnedStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ReassignOwnedStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ReassignOwnedStmt(self)
    }
}

impl protobuf::CompositeTypeStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CompositeTypeStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CompositeTypeStmt(self)
    }
}

impl protobuf::CreateEnumStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateEnumStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateEnumStmt(self)
    }
}

impl protobuf::CreateRangeStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateRangeStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateRangeStmt(self)
    }
}

impl protobuf::AlterEnumStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterEnumStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterEnumStmt(self)
    }
}

impl protobuf::AlterTsDictionaryStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterTsdictionaryStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterTsdictionaryStmt(self)
    }
}

impl protobuf::AlterTsConfigurationStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterTsconfigurationStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterTsconfigurationStmt(self)
    }
}

impl protobuf::CreateFdwStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateFdwStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateFdwStmt(self)
    }
}

impl protobuf::AlterFdwStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterFdwStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterFdwStmt(self)
    }
}

impl protobuf::CreateForeignServerStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateForeignServerStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateForeignServerStmt(self)
    }
}

impl protobuf::AlterForeignServerStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterForeignServerStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterForeignServerStmt(self)
    }
}

impl protobuf::CreateUserMappingStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateUserMappingStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateUserMappingStmt(self)
    }
}

impl protobuf::AlterUserMappingStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterUserMappingStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterUserMappingStmt(self)
    }
}

impl protobuf::DropUserMappingStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::DropUserMappingStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::DropUserMappingStmt(self)
    }
}

impl protobuf::AlterTableSpaceOptionsStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterTableSpaceOptionsStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterTableSpaceOptionsStmt(self)
    }
}

impl protobuf::AlterTableMoveAllStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterTableMoveAllStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterTableMoveAllStmt(self)
    }
}

impl protobuf::SecLabelStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::SecLabelStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::SecLabelStmt(self)
    }
}

impl protobuf::CreateForeignTableStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateForeignTableStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateForeignTableStmt(self)
    }
}

impl protobuf::ImportForeignSchemaStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ImportForeignSchemaStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ImportForeignSchemaStmt(self)
    }
}

impl protobuf::CreateExtensionStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateExtensionStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateExtensionStmt(self)
    }
}

impl protobuf::AlterExtensionStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterExtensionStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterExtensionStmt(self)
    }
}

impl protobuf::AlterExtensionContentsStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterExtensionContentsStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterExtensionContentsStmt(self)
    }
}

impl protobuf::CreateEventTrigStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateEventTrigStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateEventTrigStmt(self)
    }
}

impl protobuf::AlterEventTrigStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterEventTrigStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterEventTrigStmt(self)
    }
}

impl protobuf::RefreshMatViewStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RefreshMatViewStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RefreshMatViewStmt(self)
    }
}

impl protobuf::ReplicaIdentityStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ReplicaIdentityStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ReplicaIdentityStmt(self)
    }
}

impl protobuf::AlterSystemStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterSystemStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterSystemStmt(self)
    }
}

impl protobuf::CreatePolicyStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreatePolicyStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreatePolicyStmt(self)
    }
}

impl protobuf::AlterPolicyStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterPolicyStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterPolicyStmt(self)
    }
}

impl protobuf::CreateTransformStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateTransformStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateTransformStmt(self)
    }
}

impl protobuf::CreateAmStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateAmStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateAmStmt(self)
    }
}

impl protobuf::CreatePublicationStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreatePublicationStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreatePublicationStmt(self)
    }
}

impl protobuf::AlterPublicationStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterPublicationStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterPublicationStmt(self)
    }
}

impl protobuf::CreateSubscriptionStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateSubscriptionStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateSubscriptionStmt(self)
    }
}

impl protobuf::AlterSubscriptionStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterSubscriptionStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterSubscriptionStmt(self)
    }
}

impl protobuf::DropSubscriptionStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::DropSubscriptionStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::DropSubscriptionStmt(self)
    }
}

impl protobuf::CreateStatsStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateStatsStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateStatsStmt(self)
    }
}

impl protobuf::AlterCollationStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterCollationStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterCollationStmt(self)
    }
}

impl protobuf::CallStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CallStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CallStmt(self)
    }
}

impl protobuf::AlterStatsStmt {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AlterStatsStmt(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AlterStatsStmt(self)
    }
}

impl protobuf::AExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AExpr(self)
    }
}

impl protobuf::ColumnRef {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ColumnRef(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ColumnRef(self)
    }
}

impl protobuf::ParamRef {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ParamRef(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ParamRef(self)
    }
}

impl protobuf::AConst {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AConst(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AConst(self)
    }
}

impl protobuf::FuncCall {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::FuncCall(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::FuncCall(self)
    }
}

impl protobuf::AStar {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AStar(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AStar(self)
    }
}

impl protobuf::AIndices {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AIndices(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AIndices(self)
    }
}

impl protobuf::AIndirection {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AIndirection(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AIndirection(self)
    }
}

impl protobuf::AArrayExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AArrayExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AArrayExpr(self)
    }
}

impl protobuf::ResTarget {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ResTarget(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ResTarget(self)
    }
}

impl protobuf::MultiAssignRef {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::MultiAssignRef(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::MultiAssignRef(self)
    }
}

impl protobuf::TypeCast {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::TypeCast(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::TypeCast(self)
    }
}

impl protobuf::CollateClause {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CollateClause(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CollateClause(self)
    }
}

impl protobuf::SortBy {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::SortBy(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::SortBy(self)
    }
}

impl protobuf::WindowDef {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::WindowDef(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::WindowDef(self)
    }
}

impl protobuf::RangeSubselect {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RangeSubselect(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RangeSubselect(self)
    }
}

impl protobuf::RangeFunction {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RangeFunction(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RangeFunction(self)
    }
}

impl protobuf::RangeTableSample {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RangeTableSample(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RangeTableSample(self)
    }
}

impl protobuf::RangeTableFunc {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RangeTableFunc(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RangeTableFunc(self)
    }
}

impl protobuf::RangeTableFuncCol {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RangeTableFuncCol(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RangeTableFuncCol(self)
    }
}

impl protobuf::TypeName {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::TypeName(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::TypeName(self)
    }
}

impl protobuf::ColumnDef {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ColumnDef(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ColumnDef(self)
    }
}

impl protobuf::IndexElem {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::IndexElem(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::IndexElem(self)
    }
}

impl protobuf::Constraint {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::Constraint(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::Constraint(self)
    }
}

impl protobuf::DefElem {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::DefElem(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::DefElem(self)
    }
}

impl protobuf::RangeTblEntry {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RangeTblEntry(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RangeTblEntry(self)
    }
}

impl protobuf::RangeTblFunction {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RangeTblFunction(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RangeTblFunction(self)
    }
}

impl protobuf::TableSampleClause {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::TableSampleClause(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::TableSampleClause(self)
    }
}

impl protobuf::WithCheckOption {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::WithCheckOption(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::WithCheckOption(self)
    }
}

impl protobuf::SortGroupClause {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::SortGroupClause(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::SortGroupClause(self)
    }
}

impl protobuf::GroupingSet {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::GroupingSet(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::GroupingSet(self)
    }
}

impl protobuf::WindowClause {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::WindowClause(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::WindowClause(self)
    }
}

impl protobuf::ObjectWithArgs {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::ObjectWithArgs(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::ObjectWithArgs(self)
    }
}

impl protobuf::AccessPriv {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::AccessPriv(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::AccessPriv(self)
    }
}

impl protobuf::CreateOpClassItem {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CreateOpClassItem(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CreateOpClassItem(self)
    }
}

impl protobuf::TableLikeClause {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::TableLikeClause(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::TableLikeClause(self)
    }
}

impl protobuf::FunctionParameter {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::FunctionParameter(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::FunctionParameter(self)
    }
}

impl protobuf::LockingClause {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::LockingClause(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::LockingClause(self)
    }
}

impl protobuf::RowMarkClause {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RowMarkClause(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RowMarkClause(self)
    }
}

impl protobuf::XmlSerialize {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::XmlSerialize(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::XmlSerialize(self)
    }
}

impl protobuf::WithClause {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::WithClause(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::WithClause(self)
    }
}

impl protobuf::InferClause {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::InferClause(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::InferClause(self)
    }
}

impl protobuf::OnConflictClause {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::OnConflictClause(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::OnConflictClause(self)
    }
}

impl protobuf::CommonTableExpr {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CommonTableExpr(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CommonTableExpr(self)
    }
}

impl protobuf::RoleSpec {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::RoleSpec(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::RoleSpec(self)
    }
}

impl protobuf::TriggerTransition {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::TriggerTransition(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::TriggerTransition(self)
    }
}

impl protobuf::PartitionElem {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::PartitionElem(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::PartitionElem(self)
    }
}

impl protobuf::PartitionSpec {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::PartitionSpec(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::PartitionSpec(self)
    }
}

impl protobuf::PartitionBoundSpec {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::PartitionBoundSpec(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::PartitionBoundSpec(self)
    }
}

impl protobuf::PartitionRangeDatum {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::PartitionRangeDatum(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::PartitionRangeDatum(self)
    }
}

impl protobuf::PartitionCmd {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::PartitionCmd(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::PartitionCmd(self)
    }
}

impl protobuf::VacuumRelation {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::VacuumRelation(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::VacuumRelation(self)
    }
}

impl protobuf::InlineCodeBlock {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::InlineCodeBlock(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::InlineCodeBlock(self)
    }
}

impl protobuf::CallContext {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::CallContext(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::CallContext(self)
    }
}

impl protobuf::Integer {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::Integer(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::Integer(self)
    }
}

impl protobuf::Float {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::Float(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::Float(self)
    }
}

impl protobuf::String {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::String(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::String(self)
    }
}

impl protobuf::BitString {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::BitString(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::BitString(self)
    }
}

impl protobuf::List {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::List(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::List(self)
    }
}

impl protobuf::IntList {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::IntList(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::IntList(self)
    }
}

impl protobuf::OidList {
    pub fn to_ref(&self) -> NodeRef {
        NodeRef::OidList(self)
    }
    pub fn to_mut(&mut self) -> NodeMut {
        NodeMut::OidList(self)
    }
}
