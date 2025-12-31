//! Native Rust AST node types for PostgreSQL parse trees.
//!
//! These types mirror the PostgreSQL parse tree structure but use idiomatic Rust
//! patterns instead of protobuf-style Option<Box<T>> wrappers.

use crate::protobuf;

/// Top-level parse result containing all parsed statements.
#[derive(Debug, Clone)]
pub struct ParseResult {
    /// PostgreSQL version number (e.g., 160001 for 16.0.1)
    pub version: i32,
    /// List of parsed statements
    pub stmts: Vec<RawStmt>,
    /// Original protobuf for deparsing (hidden implementation detail)
    pub(crate) original_protobuf: protobuf::ParseResult,
}

// as_protobuf method is defined in convert.rs

/// A raw statement wrapper with location information.
#[derive(Debug, Clone)]
pub struct RawStmt {
    /// The statement node
    pub stmt: Node,
    /// Character offset in source where statement starts
    pub stmt_location: i32,
    /// Length of statement in characters (0 means "rest of string")
    pub stmt_len: i32,
}

/// The main AST node enum containing all possible node types.
///
/// This enum eliminates the need for `Option<Box<Node>>` wrappers throughout
/// the AST by using a flat enum with all node types as variants.
#[derive(Debug, Clone)]
pub enum Node {
    // Primitive value types
    Integer(Integer),
    Float(Float),
    Boolean(Boolean),
    String(StringValue),
    BitString(BitString),
    Null,

    // List type
    List(Vec<Node>),

    // Statement types
    SelectStmt(Box<SelectStmt>),
    InsertStmt(Box<InsertStmt>),
    UpdateStmt(Box<UpdateStmt>),
    DeleteStmt(Box<DeleteStmt>),
    MergeStmt(Box<MergeStmt>),

    // DDL statements
    CreateStmt(Box<CreateStmt>),
    AlterTableStmt(Box<AlterTableStmt>),
    DropStmt(Box<DropStmt>),
    TruncateStmt(Box<TruncateStmt>),
    IndexStmt(Box<IndexStmt>),
    CreateSchemaStmt(Box<CreateSchemaStmt>),
    ViewStmt(Box<ViewStmt>),
    CreateFunctionStmt(Box<CreateFunctionStmt>),
    AlterFunctionStmt(Box<AlterFunctionStmt>),
    CreateSeqStmt(Box<CreateSeqStmt>),
    AlterSeqStmt(Box<AlterSeqStmt>),
    CreateTrigStmt(Box<CreateTrigStmt>),
    RuleStmt(Box<RuleStmt>),
    CreateDomainStmt(Box<CreateDomainStmt>),
    CreateTableAsStmt(Box<CreateTableAsStmt>),
    RefreshMatViewStmt(Box<RefreshMatViewStmt>),

    // Transaction statement
    TransactionStmt(Box<TransactionStmt>),

    // Expression types
    AExpr(Box<AExpr>),
    ColumnRef(Box<ColumnRef>),
    ParamRef(Box<ParamRef>),
    AConst(Box<AConst>),
    TypeCast(Box<TypeCast>),
    CollateClause(Box<CollateClause>),
    FuncCall(Box<FuncCall>),
    AStar(AStar),
    AIndices(Box<AIndices>),
    AIndirection(Box<AIndirection>),
    AArrayExpr(Box<AArrayExpr>),
    SubLink(Box<SubLink>),
    BoolExpr(Box<BoolExpr>),
    NullTest(Box<NullTest>),
    BooleanTest(Box<BooleanTest>),
    CaseExpr(Box<CaseExpr>),
    CaseWhen(Box<CaseWhen>),
    CoalesceExpr(Box<CoalesceExpr>),
    MinMaxExpr(Box<MinMaxExpr>),
    RowExpr(Box<RowExpr>),

    // Target/Result types
    ResTarget(Box<ResTarget>),

    // Table/Range types
    RangeVar(Box<RangeVar>),
    RangeSubselect(Box<RangeSubselect>),
    RangeFunction(Box<RangeFunction>),
    JoinExpr(Box<JoinExpr>),

    // Clause types
    SortBy(Box<SortBy>),
    WindowDef(Box<WindowDef>),
    WithClause(Box<WithClause>),
    CommonTableExpr(Box<CommonTableExpr>),
    IntoClause(Box<IntoClause>),
    OnConflictClause(Box<OnConflictClause>),
    LockingClause(Box<LockingClause>),
    GroupingSet(Box<GroupingSet>),
    MergeWhenClause(Box<MergeWhenClause>),

    // Type-related
    TypeName(Box<TypeName>),
    ColumnDef(Box<ColumnDef>),
    Constraint(Box<Constraint>),
    DefElem(Box<DefElem>),
    IndexElem(Box<IndexElem>),

    // Alias and role types
    Alias(Box<Alias>),
    RoleSpec(Box<RoleSpec>),

    // Other commonly used types
    SortGroupClause(Box<SortGroupClause>),
    FunctionParameter(Box<FunctionParameter>),
    AlterTableCmd(Box<AlterTableCmd>),
    AccessPriv(Box<AccessPriv>),
    ObjectWithArgs(Box<ObjectWithArgs>),

    // Administrative statements
    VariableSetStmt(Box<VariableSetStmt>),
    VariableShowStmt(Box<VariableShowStmt>),
    ExplainStmt(Box<ExplainStmt>),
    CopyStmt(Box<CopyStmt>),
    GrantStmt(Box<GrantStmt>),
    GrantRoleStmt(Box<GrantRoleStmt>),
    LockStmt(Box<LockStmt>),
    VacuumStmt(Box<VacuumStmt>),

    // Other statements
    DoStmt(Box<DoStmt>),
    RenameStmt(Box<RenameStmt>),
    NotifyStmt(Box<NotifyStmt>),
    ListenStmt(Box<ListenStmt>),
    UnlistenStmt(Box<UnlistenStmt>),
    CheckPointStmt(Box<CheckPointStmt>),
    DiscardStmt(Box<DiscardStmt>),
    PrepareStmt(Box<PrepareStmt>),
    ExecuteStmt(Box<ExecuteStmt>),
    DeallocateStmt(Box<DeallocateStmt>),
    ClosePortalStmt(Box<ClosePortalStmt>),
    FetchStmt(Box<FetchStmt>),

    // Fallback for unhandled node types - stores the original protobuf
    Other(protobuf::Node),
}

// ============================================================================
// Primitive value types
// ============================================================================

/// Integer value
#[derive(Debug, Clone, Default)]
pub struct Integer {
    pub ival: i32,
}

/// Float value (stored as string)
#[derive(Debug, Clone, Default)]
pub struct Float {
    pub fval: String,
}

/// Boolean value
#[derive(Debug, Clone, Default)]
pub struct Boolean {
    pub boolval: bool,
}

/// String value
#[derive(Debug, Clone, Default)]
pub struct StringValue {
    pub sval: String,
}

/// Bit string value
#[derive(Debug, Clone, Default)]
pub struct BitString {
    pub bsval: String,
}

/// A star (*) in column reference
#[derive(Debug, Clone, Default)]
pub struct AStar;

// ============================================================================
// Core statement types
// ============================================================================

/// SELECT statement
#[derive(Debug, Clone, Default)]
pub struct SelectStmt {
    pub distinct_clause: Vec<Node>,
    pub into_clause: Option<IntoClause>,
    pub target_list: Vec<Node>,
    pub from_clause: Vec<Node>,
    pub where_clause: Option<Node>,
    pub group_clause: Vec<Node>,
    pub group_distinct: bool,
    pub having_clause: Option<Node>,
    pub window_clause: Vec<Node>,
    pub values_lists: Vec<Node>,
    pub sort_clause: Vec<Node>,
    pub limit_offset: Option<Node>,
    pub limit_count: Option<Node>,
    pub limit_option: LimitOption,
    pub locking_clause: Vec<Node>,
    pub with_clause: Option<WithClause>,
    pub op: SetOperation,
    pub all: bool,
    pub larg: Option<Box<SelectStmt>>,
    pub rarg: Option<Box<SelectStmt>>,
}

/// INSERT statement
#[derive(Debug, Clone, Default)]
pub struct InsertStmt {
    pub relation: Option<RangeVar>,
    pub cols: Vec<Node>,
    pub select_stmt: Option<Node>,
    pub on_conflict_clause: Option<OnConflictClause>,
    pub returning_list: Vec<Node>,
    pub with_clause: Option<WithClause>,
    pub override_: OverridingKind,
}

/// UPDATE statement
#[derive(Debug, Clone, Default)]
pub struct UpdateStmt {
    pub relation: Option<RangeVar>,
    pub target_list: Vec<Node>,
    pub where_clause: Option<Node>,
    pub from_clause: Vec<Node>,
    pub returning_list: Vec<Node>,
    pub with_clause: Option<WithClause>,
}

/// DELETE statement
#[derive(Debug, Clone, Default)]
pub struct DeleteStmt {
    pub relation: Option<RangeVar>,
    pub using_clause: Vec<Node>,
    pub where_clause: Option<Node>,
    pub returning_list: Vec<Node>,
    pub with_clause: Option<WithClause>,
}

/// MERGE statement
#[derive(Debug, Clone, Default)]
pub struct MergeStmt {
    pub relation: Option<RangeVar>,
    pub source_relation: Option<Node>,
    pub join_condition: Option<Node>,
    pub merge_when_clauses: Vec<Node>,
    pub with_clause: Option<WithClause>,
}

// ============================================================================
// DDL statement types
// ============================================================================

/// CREATE TABLE statement
#[derive(Debug, Clone, Default)]
pub struct CreateStmt {
    pub relation: Option<RangeVar>,
    pub table_elts: Vec<Node>,
    pub inh_relations: Vec<Node>,
    pub partbound: Option<Node>,
    pub partspec: Option<Node>,
    pub of_typename: Option<TypeName>,
    pub constraints: Vec<Node>,
    pub options: Vec<Node>,
    pub oncommit: OnCommitAction,
    pub tablespacename: String,
    pub access_method: String,
    pub if_not_exists: bool,
}

/// ALTER TABLE statement
#[derive(Debug, Clone, Default)]
pub struct AlterTableStmt {
    pub relation: Option<RangeVar>,
    pub cmds: Vec<Node>,
    pub objtype: ObjectType,
    pub missing_ok: bool,
}

/// DROP statement
#[derive(Debug, Clone, Default)]
pub struct DropStmt {
    pub objects: Vec<Node>,
    pub remove_type: ObjectType,
    pub behavior: DropBehavior,
    pub missing_ok: bool,
    pub concurrent: bool,
}

/// TRUNCATE statement
#[derive(Debug, Clone, Default)]
pub struct TruncateStmt {
    pub relations: Vec<Node>,
    pub restart_seqs: bool,
    pub behavior: DropBehavior,
}

/// CREATE INDEX statement
#[derive(Debug, Clone, Default)]
pub struct IndexStmt {
    pub idxname: String,
    pub relation: Option<RangeVar>,
    pub access_method: String,
    pub table_space: String,
    pub index_params: Vec<Node>,
    pub index_including_params: Vec<Node>,
    pub options: Vec<Node>,
    pub where_clause: Option<Node>,
    pub exclude_op_names: Vec<Node>,
    pub idxcomment: String,
    pub index_oid: u32,
    pub old_number: u32,
    pub old_first_relfilelocator: u32,
    pub unique: bool,
    pub nulls_not_distinct: bool,
    pub primary: bool,
    pub is_constraint: bool,
    pub deferrable: bool,
    pub initdeferred: bool,
    pub transformed: bool,
    pub concurrent: bool,
    pub if_not_exists: bool,
    pub reset_default_tblspc: bool,
}

/// CREATE SCHEMA statement
#[derive(Debug, Clone, Default)]
pub struct CreateSchemaStmt {
    pub schemaname: String,
    pub authrole: Option<RoleSpec>,
    pub schema_elts: Vec<Node>,
    pub if_not_exists: bool,
}

/// CREATE VIEW statement
#[derive(Debug, Clone, Default)]
pub struct ViewStmt {
    pub view: Option<RangeVar>,
    pub aliases: Vec<Node>,
    pub query: Option<Node>,
    pub replace: bool,
    pub options: Vec<Node>,
    pub with_check_option: ViewCheckOption,
}

/// CREATE FUNCTION statement
#[derive(Debug, Clone, Default)]
pub struct CreateFunctionStmt {
    pub is_procedure: bool,
    pub replace: bool,
    pub funcname: Vec<Node>,
    pub parameters: Vec<Node>,
    pub return_type: Option<TypeName>,
    pub options: Vec<Node>,
    pub sql_body: Option<Node>,
}

/// ALTER FUNCTION statement
#[derive(Debug, Clone, Default)]
pub struct AlterFunctionStmt {
    pub objtype: ObjectType,
    pub func: Option<ObjectWithArgs>,
    pub actions: Vec<Node>,
}

/// CREATE SEQUENCE statement
#[derive(Debug, Clone, Default)]
pub struct CreateSeqStmt {
    pub sequence: Option<RangeVar>,
    pub options: Vec<Node>,
    pub owner_id: u32,
    pub for_identity: bool,
    pub if_not_exists: bool,
}

/// ALTER SEQUENCE statement
#[derive(Debug, Clone, Default)]
pub struct AlterSeqStmt {
    pub sequence: Option<RangeVar>,
    pub options: Vec<Node>,
    pub for_identity: bool,
    pub missing_ok: bool,
}

/// CREATE TRIGGER statement
#[derive(Debug, Clone, Default)]
pub struct CreateTrigStmt {
    pub replace: bool,
    pub isconstraint: bool,
    pub trigname: String,
    pub relation: Option<RangeVar>,
    pub funcname: Vec<Node>,
    pub args: Vec<Node>,
    pub row: bool,
    pub timing: i32,
    pub events: i32,
    pub columns: Vec<Node>,
    pub when_clause: Option<Node>,
    pub transition_rels: Vec<Node>,
    pub deferrable: bool,
    pub initdeferred: bool,
    pub constrrel: Option<RangeVar>,
}

/// CREATE RULE statement
#[derive(Debug, Clone, Default)]
pub struct RuleStmt {
    pub relation: Option<RangeVar>,
    pub rulename: String,
    pub where_clause: Option<Node>,
    pub event: CmdType,
    pub instead: bool,
    pub actions: Vec<Node>,
    pub replace: bool,
}

/// CREATE DOMAIN statement
#[derive(Debug, Clone, Default)]
pub struct CreateDomainStmt {
    pub domainname: Vec<Node>,
    pub type_name: Option<TypeName>,
    pub coll_clause: Option<CollateClause>,
    pub constraints: Vec<Node>,
}

/// CREATE TABLE AS statement
#[derive(Debug, Clone, Default)]
pub struct CreateTableAsStmt {
    pub query: Option<Node>,
    pub into: Option<IntoClause>,
    pub objtype: ObjectType,
    pub is_select_into: bool,
    pub if_not_exists: bool,
}

/// REFRESH MATERIALIZED VIEW statement
#[derive(Debug, Clone, Default)]
pub struct RefreshMatViewStmt {
    pub concurrent: bool,
    pub skip_data: bool,
    pub relation: Option<RangeVar>,
}

// ============================================================================
// Transaction statement
// ============================================================================

/// Transaction statement (BEGIN, COMMIT, ROLLBACK, etc.)
#[derive(Debug, Clone, Default)]
pub struct TransactionStmt {
    pub kind: TransactionStmtKind,
    pub options: Vec<Node>,
    pub savepoint_name: String,
    pub gid: String,
    pub chain: bool,
}

// ============================================================================
// Expression types
// ============================================================================

/// An expression with an operator (e.g., "a + b", "x = 1")
#[derive(Debug, Clone, Default)]
pub struct AExpr {
    pub kind: AExprKind,
    pub name: Vec<Node>,
    pub lexpr: Option<Node>,
    pub rexpr: Option<Node>,
    pub location: i32,
}

/// Column reference (e.g., "table.column")
#[derive(Debug, Clone, Default)]
pub struct ColumnRef {
    pub fields: Vec<Node>,
    pub location: i32,
}

/// Parameter reference ($1, $2, etc.)
#[derive(Debug, Clone, Default)]
pub struct ParamRef {
    pub number: i32,
    pub location: i32,
}

/// A constant value
#[derive(Debug, Clone, Default)]
pub struct AConst {
    pub val: Option<AConstValue>,
    pub isnull: bool,
    pub location: i32,
}

/// Value types for AConst
#[derive(Debug, Clone)]
pub enum AConstValue {
    Integer(Integer),
    Float(Float),
    Boolean(Boolean),
    String(StringValue),
    BitString(BitString),
}

/// Type cast expression
#[derive(Debug, Clone, Default)]
pub struct TypeCast {
    pub arg: Option<Node>,
    pub type_name: Option<TypeName>,
    pub location: i32,
}

/// COLLATE clause
#[derive(Debug, Clone, Default)]
pub struct CollateClause {
    pub arg: Option<Node>,
    pub collname: Vec<Node>,
    pub location: i32,
}

/// Function call
#[derive(Debug, Clone, Default)]
pub struct FuncCall {
    pub funcname: Vec<Node>,
    pub args: Vec<Node>,
    pub agg_order: Vec<Node>,
    pub agg_filter: Option<Node>,
    pub over: Option<WindowDef>,
    pub agg_within_group: bool,
    pub agg_star: bool,
    pub agg_distinct: bool,
    pub func_variadic: bool,
    pub funcformat: CoercionForm,
    pub location: i32,
}

/// Array subscript indices
#[derive(Debug, Clone, Default)]
pub struct AIndices {
    pub is_slice: bool,
    pub lidx: Option<Node>,
    pub uidx: Option<Node>,
}

/// Array subscript or field selection
#[derive(Debug, Clone, Default)]
pub struct AIndirection {
    pub arg: Option<Node>,
    pub indirection: Vec<Node>,
}

/// ARRAY[] constructor
#[derive(Debug, Clone, Default)]
pub struct AArrayExpr {
    pub elements: Vec<Node>,
    pub location: i32,
}

/// Subquery link (subquery in expression context)
#[derive(Debug, Clone, Default)]
pub struct SubLink {
    pub sub_link_type: SubLinkType,
    pub sub_link_id: i32,
    pub testexpr: Option<Node>,
    pub oper_name: Vec<Node>,
    pub subselect: Option<Node>,
    pub location: i32,
}

/// Boolean expression (AND, OR, NOT)
#[derive(Debug, Clone, Default)]
pub struct BoolExpr {
    pub boolop: BoolExprType,
    pub args: Vec<Node>,
    pub location: i32,
}

/// NULL test expression
#[derive(Debug, Clone, Default)]
pub struct NullTest {
    pub arg: Option<Node>,
    pub nulltesttype: NullTestType,
    pub argisrow: bool,
    pub location: i32,
}

/// Boolean test (IS TRUE, IS FALSE, etc.)
#[derive(Debug, Clone, Default)]
pub struct BooleanTest {
    pub arg: Option<Node>,
    pub booltesttype: BoolTestType,
    pub location: i32,
}

/// CASE expression
#[derive(Debug, Clone, Default)]
pub struct CaseExpr {
    pub arg: Option<Node>,
    pub args: Vec<Node>,
    pub defresult: Option<Node>,
    pub location: i32,
}

/// WHEN clause of CASE
#[derive(Debug, Clone, Default)]
pub struct CaseWhen {
    pub expr: Option<Node>,
    pub result: Option<Node>,
    pub location: i32,
}

/// COALESCE expression
#[derive(Debug, Clone, Default)]
pub struct CoalesceExpr {
    pub args: Vec<Node>,
    pub location: i32,
}

/// GREATEST or LEAST expression
#[derive(Debug, Clone, Default)]
pub struct MinMaxExpr {
    pub op: MinMaxOp,
    pub args: Vec<Node>,
    pub location: i32,
}

/// ROW() expression
#[derive(Debug, Clone, Default)]
pub struct RowExpr {
    pub args: Vec<Node>,
    pub row_format: CoercionForm,
    pub colnames: Vec<Node>,
    pub location: i32,
}

// ============================================================================
// Target/Result types
// ============================================================================

/// Result target (column in SELECT list or assignment target)
#[derive(Debug, Clone, Default)]
pub struct ResTarget {
    pub name: String,
    pub indirection: Vec<Node>,
    pub val: Option<Node>,
    pub location: i32,
}

// ============================================================================
// Table/Range types
// ============================================================================

/// Table/relation reference
#[derive(Debug, Clone, Default)]
pub struct RangeVar {
    pub catalogname: String,
    pub schemaname: String,
    pub relname: String,
    pub inh: bool,
    pub relpersistence: String,
    pub alias: Option<Alias>,
    pub location: i32,
}

/// Subquery in FROM clause
#[derive(Debug, Clone, Default)]
pub struct RangeSubselect {
    pub lateral: bool,
    pub subquery: Option<Node>,
    pub alias: Option<Alias>,
}

/// Function call in FROM clause
#[derive(Debug, Clone, Default)]
pub struct RangeFunction {
    pub lateral: bool,
    pub ordinality: bool,
    pub is_rowsfrom: bool,
    pub functions: Vec<Node>,
    pub alias: Option<Alias>,
    pub coldeflist: Vec<Node>,
}

/// JOIN expression
#[derive(Debug, Clone, Default)]
pub struct JoinExpr {
    pub jointype: JoinType,
    pub is_natural: bool,
    pub larg: Option<Node>,
    pub rarg: Option<Node>,
    pub using_clause: Vec<Node>,
    pub join_using_alias: Option<Alias>,
    pub quals: Option<Node>,
    pub alias: Option<Alias>,
    pub rtindex: i32,
}

// ============================================================================
// Clause types
// ============================================================================

/// ORDER BY clause element
#[derive(Debug, Clone, Default)]
pub struct SortBy {
    pub node: Option<Node>,
    pub sortby_dir: SortByDir,
    pub sortby_nulls: SortByNulls,
    pub use_op: Vec<Node>,
    pub location: i32,
}

/// WINDOW definition
#[derive(Debug, Clone, Default)]
pub struct WindowDef {
    pub name: String,
    pub refname: String,
    pub partition_clause: Vec<Node>,
    pub order_clause: Vec<Node>,
    pub frame_options: i32,
    pub start_offset: Option<Node>,
    pub end_offset: Option<Node>,
    pub location: i32,
}

/// WITH clause
#[derive(Debug, Clone, Default)]
pub struct WithClause {
    pub ctes: Vec<Node>,
    pub recursive: bool,
    pub location: i32,
}

/// Common Table Expression (CTE)
#[derive(Debug, Clone, Default)]
pub struct CommonTableExpr {
    pub ctename: String,
    pub aliascolnames: Vec<Node>,
    pub ctematerialized: CTEMaterialize,
    pub ctequery: Option<Node>,
    pub search_clause: Option<Node>,
    pub cycle_clause: Option<Node>,
    pub location: i32,
    pub cterecursive: bool,
    pub cterefcount: i32,
    pub ctecolnames: Vec<Node>,
    pub ctecoltypes: Vec<Node>,
    pub ctecoltypmods: Vec<Node>,
    pub ctecolcollations: Vec<Node>,
}

/// INTO clause for SELECT INTO
#[derive(Debug, Clone, Default)]
pub struct IntoClause {
    pub rel: Option<RangeVar>,
    pub col_names: Vec<Node>,
    pub access_method: String,
    pub options: Vec<Node>,
    pub on_commit: OnCommitAction,
    pub table_space_name: String,
    pub view_query: Option<Node>,
    pub skip_data: bool,
}

/// ON CONFLICT clause for INSERT
#[derive(Debug, Clone, Default)]
pub struct OnConflictClause {
    pub action: OnConflictAction,
    pub infer: Option<Node>,
    pub target_list: Vec<Node>,
    pub where_clause: Option<Node>,
    pub location: i32,
}

/// FOR UPDATE/SHARE clause
#[derive(Debug, Clone, Default)]
pub struct LockingClause {
    pub locked_rels: Vec<Node>,
    pub strength: LockClauseStrength,
    pub wait_policy: LockWaitPolicy,
}

/// GROUPING SETS clause element
#[derive(Debug, Clone, Default)]
pub struct GroupingSet {
    pub kind: GroupingSetKind,
    pub content: Vec<Node>,
    pub location: i32,
}

/// MERGE WHEN clause
#[derive(Debug, Clone, Default)]
pub struct MergeWhenClause {
    pub matched: bool,
    pub command_type: CmdType,
    pub override_: OverridingKind,
    pub condition: Option<Node>,
    pub target_list: Vec<Node>,
    pub values: Vec<Node>,
}

// ============================================================================
// Type-related
// ============================================================================

/// Type name
#[derive(Debug, Clone, Default)]
pub struct TypeName {
    pub names: Vec<Node>,
    pub type_oid: u32,
    pub setof: bool,
    pub pct_type: bool,
    pub typmods: Vec<Node>,
    pub typemod: i32,
    pub array_bounds: Vec<Node>,
    pub location: i32,
}

/// Column definition
#[derive(Debug, Clone, Default)]
pub struct ColumnDef {
    pub colname: String,
    pub type_name: Option<TypeName>,
    pub compression: String,
    pub inhcount: i32,
    pub is_local: bool,
    pub is_not_null: bool,
    pub is_from_type: bool,
    pub storage: String,
    pub storage_name: String,
    pub raw_default: Option<Node>,
    pub cooked_default: Option<Node>,
    pub identity: String,
    pub identity_sequence: Option<RangeVar>,
    pub generated: String,
    pub coll_clause: Option<CollateClause>,
    pub coll_oid: u32,
    pub constraints: Vec<Node>,
    pub fdwoptions: Vec<Node>,
    pub location: i32,
}

/// Constraint definition
#[derive(Debug, Clone, Default)]
pub struct Constraint {
    pub contype: ConstrType,
    pub conname: String,
    pub deferrable: bool,
    pub initdeferred: bool,
    pub location: i32,
    pub is_no_inherit: bool,
    pub raw_expr: Option<Node>,
    pub cooked_expr: String,
    pub generated_when: String,
    pub nulls_not_distinct: bool,
    pub keys: Vec<Node>,
    pub including: Vec<Node>,
    pub exclusions: Vec<Node>,
    pub options: Vec<Node>,
    pub indexname: String,
    pub indexspace: String,
    pub reset_default_tblspc: bool,
    pub access_method: String,
    pub where_clause: Option<Node>,
    pub pktable: Option<RangeVar>,
    pub fk_attrs: Vec<Node>,
    pub pk_attrs: Vec<Node>,
    pub fk_matchtype: String,
    pub fk_upd_action: String,
    pub fk_del_action: String,
    pub fk_del_set_cols: Vec<Node>,
    pub old_conpfeqop: Vec<Node>,
    pub old_pktable_oid: u32,
    pub skip_validation: bool,
    pub initially_valid: bool,
}

/// Definition element (generic)
#[derive(Debug, Clone, Default)]
pub struct DefElem {
    pub defnamespace: String,
    pub defname: String,
    pub arg: Option<Node>,
    pub defaction: DefElemAction,
    pub location: i32,
}

/// Index element
#[derive(Debug, Clone, Default)]
pub struct IndexElem {
    pub name: String,
    pub expr: Option<Node>,
    pub indexcolname: String,
    pub collation: Vec<Node>,
    pub opclass: Vec<Node>,
    pub opclassopts: Vec<Node>,
    pub ordering: SortByDir,
    pub nulls_ordering: SortByNulls,
}

// ============================================================================
// Alias and role types
// ============================================================================

/// Alias
#[derive(Debug, Clone, Default)]
pub struct Alias {
    pub aliasname: String,
    pub colnames: Vec<Node>,
}

/// Role specification
#[derive(Debug, Clone, Default)]
pub struct RoleSpec {
    pub roletype: RoleSpecType,
    pub rolename: String,
    pub location: i32,
}

// ============================================================================
// Other commonly used types
// ============================================================================

/// Sort/Group clause
#[derive(Debug, Clone, Default)]
pub struct SortGroupClause {
    pub tle_sort_group_ref: u32,
    pub eqop: u32,
    pub sortop: u32,
    pub nulls_first: bool,
    pub hashable: bool,
}

/// Function parameter
#[derive(Debug, Clone, Default)]
pub struct FunctionParameter {
    pub name: String,
    pub arg_type: Option<TypeName>,
    pub mode: FunctionParameterMode,
    pub defexpr: Option<Node>,
}

/// ALTER TABLE command
#[derive(Debug, Clone, Default)]
pub struct AlterTableCmd {
    pub subtype: AlterTableType,
    pub name: String,
    pub num: i16,
    pub newowner: Option<RoleSpec>,
    pub def: Option<Node>,
    pub behavior: DropBehavior,
    pub missing_ok: bool,
    pub recurse: bool,
}

/// Access privilege
#[derive(Debug, Clone, Default)]
pub struct AccessPriv {
    pub priv_name: String,
    pub cols: Vec<Node>,
}

/// Object with arguments
#[derive(Debug, Clone, Default)]
pub struct ObjectWithArgs {
    pub objname: Vec<Node>,
    pub objargs: Vec<Node>,
    pub objfuncargs: Vec<Node>,
    pub args_unspecified: bool,
}

// ============================================================================
// Administrative statements
// ============================================================================

/// SET variable statement
#[derive(Debug, Clone, Default)]
pub struct VariableSetStmt {
    pub kind: VariableSetKind,
    pub name: String,
    pub args: Vec<Node>,
    pub is_local: bool,
}

/// SHOW variable statement
#[derive(Debug, Clone, Default)]
pub struct VariableShowStmt {
    pub name: String,
}

/// EXPLAIN statement
#[derive(Debug, Clone, Default)]
pub struct ExplainStmt {
    pub query: Option<Node>,
    pub options: Vec<Node>,
}

/// COPY statement
#[derive(Debug, Clone, Default)]
pub struct CopyStmt {
    pub relation: Option<RangeVar>,
    pub query: Option<Node>,
    pub attlist: Vec<Node>,
    pub is_from: bool,
    pub is_program: bool,
    pub filename: String,
    pub options: Vec<Node>,
    pub where_clause: Option<Node>,
}

/// GRANT/REVOKE statement
#[derive(Debug, Clone, Default)]
pub struct GrantStmt {
    pub is_grant: bool,
    pub targtype: GrantTargetType,
    pub objtype: ObjectType,
    pub objects: Vec<Node>,
    pub privileges: Vec<Node>,
    pub grantees: Vec<Node>,
    pub grant_option: bool,
    pub grantor: Option<RoleSpec>,
    pub behavior: DropBehavior,
}

/// GRANT/REVOKE role statement
#[derive(Debug, Clone, Default)]
pub struct GrantRoleStmt {
    pub granted_roles: Vec<Node>,
    pub grantee_roles: Vec<Node>,
    pub is_grant: bool,
    pub opt: Vec<Node>,
    pub grantor: Option<RoleSpec>,
    pub behavior: DropBehavior,
}

/// LOCK statement
#[derive(Debug, Clone, Default)]
pub struct LockStmt {
    pub relations: Vec<Node>,
    pub mode: i32,
    pub nowait: bool,
}

/// VACUUM/ANALYZE statement
#[derive(Debug, Clone, Default)]
pub struct VacuumStmt {
    pub options: Vec<Node>,
    pub rels: Vec<Node>,
    pub is_vacuumcmd: bool,
}

// ============================================================================
// Other statements
// ============================================================================

/// DO statement
#[derive(Debug, Clone, Default)]
pub struct DoStmt {
    pub args: Vec<Node>,
}

/// RENAME statement
#[derive(Debug, Clone, Default)]
pub struct RenameStmt {
    pub rename_type: ObjectType,
    pub relation_type: ObjectType,
    pub relation: Option<RangeVar>,
    pub object: Option<Node>,
    pub subname: String,
    pub newname: String,
    pub behavior: DropBehavior,
    pub missing_ok: bool,
}

/// NOTIFY statement
#[derive(Debug, Clone, Default)]
pub struct NotifyStmt {
    pub conditionname: String,
    pub payload: String,
}

/// LISTEN statement
#[derive(Debug, Clone, Default)]
pub struct ListenStmt {
    pub conditionname: String,
}

/// UNLISTEN statement
#[derive(Debug, Clone, Default)]
pub struct UnlistenStmt {
    pub conditionname: String,
}

/// CHECKPOINT statement
#[derive(Debug, Clone, Default)]
pub struct CheckPointStmt;

/// DISCARD statement
#[derive(Debug, Clone, Default)]
pub struct DiscardStmt {
    pub target: DiscardMode,
}

/// PREPARE statement
#[derive(Debug, Clone, Default)]
pub struct PrepareStmt {
    pub name: String,
    pub argtypes: Vec<Node>,
    pub query: Option<Node>,
}

/// EXECUTE statement
#[derive(Debug, Clone, Default)]
pub struct ExecuteStmt {
    pub name: String,
    pub params: Vec<Node>,
}

/// DEALLOCATE statement
#[derive(Debug, Clone, Default)]
pub struct DeallocateStmt {
    pub name: String,
}

/// CLOSE cursor statement
#[derive(Debug, Clone, Default)]
pub struct ClosePortalStmt {
    pub portalname: String,
}

/// FETCH/MOVE statement
#[derive(Debug, Clone, Default)]
pub struct FetchStmt {
    pub direction: FetchDirection,
    pub how_many: i64,
    pub portalname: String,
    pub ismove: bool,
}

// ============================================================================
// Enums
// ============================================================================

/// SET operation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SetOperation {
    #[default]
    None,
    Union,
    Intersect,
    Except,
}

/// LIMIT option
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LimitOption {
    #[default]
    Default,
    Count,
    WithTies,
}

/// A_Expr kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AExprKind {
    #[default]
    Op,
    OpAny,
    OpAll,
    Distinct,
    NotDistinct,
    NullIf,
    In,
    Like,
    ILike,
    Similar,
    Between,
    NotBetween,
    BetweenSym,
    NotBetweenSym,
}

/// Boolean expression type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BoolExprType {
    #[default]
    And,
    Or,
    Not,
}

/// Sublink type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SubLinkType {
    #[default]
    Exists,
    All,
    Any,
    RowCompare,
    Expr,
    MultiExpr,
    Array,
    Cte,
}

/// NULL test type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NullTestType {
    #[default]
    IsNull,
    IsNotNull,
}

/// Boolean test type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BoolTestType {
    #[default]
    IsTrue,
    IsNotTrue,
    IsFalse,
    IsNotFalse,
    IsUnknown,
    IsNotUnknown,
}

/// Min/Max operation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MinMaxOp {
    #[default]
    Greatest,
    Least,
}

/// JOIN type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum JoinType {
    #[default]
    Inner,
    Left,
    Full,
    Right,
    Semi,
    Anti,
    RightAnti,
    UniqueOuter,
    UniqueInner,
}

/// SORT BY direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortByDir {
    #[default]
    Default,
    Asc,
    Desc,
    Using,
}

/// SORT BY nulls
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortByNulls {
    #[default]
    Default,
    First,
    Last,
}

/// CTE materialization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CTEMaterialize {
    #[default]
    Default,
    Always,
    Never,
}

/// ON COMMIT action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OnCommitAction {
    #[default]
    Noop,
    PreserveRows,
    DeleteRows,
    Drop,
}

/// Object type for DDL
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ObjectType {
    #[default]
    Table,
    Index,
    Sequence,
    View,
    MatView,
    Type,
    Schema,
    Function,
    Procedure,
    Routine,
    Aggregate,
    Operator,
    Language,
    Cast,
    Trigger,
    EventTrigger,
    Rule,
    Database,
    Tablespace,
    Role,
    Extension,
    Fdw,
    ForeignServer,
    ForeignTable,
    Policy,
    Publication,
    Subscription,
    Collation,
    Conversion,
    Default,
    Domain,
    Constraint,
    Column,
    AccessMethod,
    LargeObject,
    Transform,
    StatisticsObject,
}

/// DROP behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DropBehavior {
    #[default]
    Restrict,
    Cascade,
}

/// ON CONFLICT action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OnConflictAction {
    #[default]
    None,
    Nothing,
    Update,
}

/// GROUPING SET kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GroupingSetKind {
    #[default]
    Empty,
    Simple,
    Rollup,
    Cube,
    Sets,
}

/// Command type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CmdType {
    #[default]
    Unknown,
    Select,
    Update,
    Insert,
    Delete,
    Merge,
    Utility,
    Nothing,
}

/// Transaction statement kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TransactionStmtKind {
    #[default]
    Begin,
    Start,
    Commit,
    Rollback,
    Savepoint,
    Release,
    RollbackTo,
    Prepare,
    CommitPrepared,
    RollbackPrepared,
}

/// Constraint type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConstrType {
    #[default]
    Null,
    NotNull,
    Default,
    Identity,
    Generated,
    Check,
    Primary,
    Unique,
    Exclusion,
    Foreign,
    AttrDeferrable,
    AttrNotDeferrable,
    AttrDeferred,
    AttrImmediate,
}

/// DefElem action
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DefElemAction {
    #[default]
    Unspec,
    Set,
    Add,
    Drop,
}

/// Role spec type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RoleSpecType {
    #[default]
    CString,
    CurrentRole,
    CurrentUser,
    SessionUser,
    Public,
}

/// Coercion form
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CoercionForm {
    #[default]
    ExplicitCall,
    ExplicitCast,
    ImplicitCast,
    SqlSyntax,
}

/// Variable SET kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VariableSetKind {
    #[default]
    Value,
    Default,
    Current,
    Multi,
    Reset,
    ResetAll,
}

/// Lock clause strength
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LockClauseStrength {
    #[default]
    None,
    ForKeyShare,
    ForShare,
    ForNoKeyUpdate,
    ForUpdate,
}

/// Lock wait policy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum LockWaitPolicy {
    #[default]
    Block,
    Skip,
    Error,
}

/// View check option
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ViewCheckOption {
    #[default]
    NoCheckOption,
    Local,
    Cascaded,
}

/// Discard mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiscardMode {
    #[default]
    All,
    Plans,
    Sequences,
    Temp,
}

/// Fetch direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FetchDirection {
    #[default]
    Forward,
    Backward,
    Absolute,
    Relative,
}

/// Function parameter mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FunctionParameterMode {
    #[default]
    In,
    Out,
    InOut,
    Variadic,
    Table,
}

/// ALTER TABLE command type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlterTableType {
    #[default]
    AddColumn,
    AddColumnToView,
    ColumnDefault,
    CookedColumnDefault,
    DropNotNull,
    SetNotNull,
    DropExpression,
    CheckNotNull,
    SetStatistics,
    SetOptions,
    ResetOptions,
    SetStorage,
    SetCompression,
    DropColumn,
    AddIndex,
    ReAddIndex,
    AddConstraint,
    ReAddConstraint,
    AddIndexConstraint,
    AlterConstraint,
    ValidateConstraint,
    DropConstraint,
    ClusterOn,
    DropCluster,
    SetLogged,
    SetUnLogged,
    SetAccessMethod,
    DropOids,
    SetTableSpace,
    SetRelOptions,
    ResetRelOptions,
    ReplaceRelOptions,
    EnableTrig,
    EnableAlwaysTrig,
    EnableReplicaTrig,
    DisableTrig,
    EnableTrigAll,
    DisableTrigAll,
    EnableTrigUser,
    DisableTrigUser,
    EnableRule,
    EnableAlwaysRule,
    EnableReplicaRule,
    DisableRule,
    AddInherit,
    DropInherit,
    AddOf,
    DropOf,
    ReplicaIdentity,
    EnableRowSecurity,
    DisableRowSecurity,
    ForceRowSecurity,
    NoForceRowSecurity,
    GenericOptions,
    AttachPartition,
    DetachPartition,
    DetachPartitionFinalize,
    AddIdentity,
    SetIdentity,
    DropIdentity,
    ReAddStatistics,
}

/// GRANT target type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GrantTargetType {
    #[default]
    Object,
    AllInSchema,
    Defaults,
}

/// Overriding kind
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OverridingKind {
    #[default]
    NotSet,
    UserValue,
    SystemValue,
}
