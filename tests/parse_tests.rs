#![allow(non_snake_case)]
#![cfg(test)]

#[cfg(test)]
use itertools::sorted;

#[cfg(test)]
use std::thread::Builder;

use pg_query::{
    parse,
    protobuf::{self, a_const::Val},
    Error, NodeEnum, NodeRef, TriggerType,
};

#[macro_use]
mod support;
use support::*;

#[test]
fn it_parses_simple_query() {
    let result = parse("SELECT 1").unwrap();
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_handles_errors() {
    let error = parse("CREATE RANDOM ix_test ON contacts.person;").err().unwrap();
    assert_eq!(error, Error::Parse("syntax error at or near \"RANDOM\"".into()));

    let error = parse("SELECT 'ERR").err().unwrap();
    assert_eq!(error, Error::Parse("unterminated quoted string at or near \"'ERR\"".into()));
}

#[test]
fn it_handles_recursion_without_error_1() {
    let query = "SELECT a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(a(b))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))))";
    let result = Builder::new().stack_size(20 * 1024 * 1024).spawn(move || parse(query)).unwrap().join().unwrap().unwrap();
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_handles_recursion_without_error_2() {
    // The Ruby version of pg_query fails here because of Ruby protobuf limitations
    let query = r#"SELECT * FROM "t0"
        JOIN "t1" ON (1) JOIN "t2" ON (1) JOIN "t3" ON (1) JOIN "t4" ON (1) JOIN "t5" ON (1)
        JOIN "t6" ON (1) JOIN "t7" ON (1) JOIN "t8" ON (1) JOIN "t9" ON (1) JOIN "t10" ON (1)
        JOIN "t11" ON (1) JOIN "t12" ON (1) JOIN "t13" ON (1) JOIN "t14" ON (1) JOIN "t15" ON (1)
        JOIN "t16" ON (1) JOIN "t17" ON (1) JOIN "t18" ON (1) JOIN "t19" ON (1) JOIN "t20" ON (1)
        JOIN "t21" ON (1) JOIN "t22" ON (1) JOIN "t23" ON (1) JOIN "t24" ON (1) JOIN "t25" ON (1)
        JOIN "t26" ON (1) JOIN "t27" ON (1) JOIN "t28" ON (1) JOIN "t29" ON (1)"#;
    let result = parse(query).unwrap();
    assert_eq!(result.tables().len(), 30);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_parses_real_queries() {
    let query = "
        SELECT memory_total_bytes, memory_free_bytes, memory_pagecache_bytes, memory_buffers_bytes, memory_applications_bytes,
            (memory_swap_total_bytes - memory_swap_free_bytes) AS swap, date_part($0, s.collected_at) AS collected_at
        FROM snapshots s JOIN system_snapshots ON (snapshot_id = s.id)
        WHERE s.database_id = $0 AND s.collected_at BETWEEN $0 AND $0
        ORDER BY collected_at";
    let result = parse(query).unwrap();
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["snapshots", "system_snapshots"]);
    assert_eq!(select_tables, ["snapshots", "system_snapshots"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_parses_empty_queries() {
    let result = parse("-- nothing").unwrap();
    assert_eq!(result.protobuf.nodes().len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.statement_types().len(), 0);
}

#[test]
fn it_parses_floats_with_leading_dot() {
    let result = parse("SELECT .1").unwrap();
    let select = cast!(result.protobuf.nodes()[0].0, NodeRef::SelectStmt);
    let target = cast!(select.target_list[0].node.as_ref().unwrap(), NodeEnum::ResTarget);
    let a_const = cast!(target.val.as_ref().unwrap().node.as_ref().unwrap(), NodeEnum::AConst);
    let float = cast!(a_const.val.as_ref().unwrap(), Val::Fval);
    assert_eq!(float.fval, ".1");
    assert_eq!(a_const.location, 7);
}

#[test]
fn it_parses_bit_strings_hex_notation() {
    let result = parse("SELECT X'EFFF'").unwrap();
    let select = cast!(result.protobuf.nodes()[0].0, NodeRef::SelectStmt);
    let target = cast!(select.target_list[0].node.as_ref().unwrap(), NodeEnum::ResTarget);
    let a_const = cast!(target.val.as_ref().unwrap().node.as_ref().unwrap(), NodeEnum::AConst);
    let bit_string = cast!(a_const.val.as_ref().unwrap(), Val::Bsval);
    assert_eq!(bit_string.bsval, "xEFFF");
    assert_eq!(a_const.location, 7);
}

#[test]
fn it_parses_ALTER_TABLE() {
    let result = parse("ALTER TABLE test ADD PRIMARY KEY (gid)").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["test"]);
    assert_eq!(result.ddl_tables(), ["test"]);
    assert_eq!(result.statement_types(), ["AlterTableStmt"]);
    let alter = cast!(result.protobuf.nodes()[0].0, NodeRef::AlterTableStmt);
    let cmd = cast!(alter.cmds[0].node.as_ref().unwrap(), NodeEnum::AlterTableCmd);
    assert_debug_eq!(
        cmd,
        r#"AlterTableCmd {
    subtype: AtAddConstraint,
    name: "",
    num: 0,
    newowner: None,
    def: Some(
        Node {
            node: Some(
                Constraint(
                    Constraint {
                        contype: ConstrPrimary,
                        conname: "",
                        deferrable: false,
                        initdeferred: false,
                        skip_validation: false,
                        initially_valid: false,
                        is_no_inherit: false,
                        raw_expr: None,
                        cooked_expr: "",
                        generated_when: "",
                        inhcount: 0,
                        nulls_not_distinct: false,
                        keys: [
                            Node {
                                node: Some(
                                    String(
                                        String {
                                            sval: "gid",
                                        },
                                    ),
                                ),
                            },
                        ],
                        including: [],
                        exclusions: [],
                        options: [],
                        indexname: "",
                        indexspace: "",
                        reset_default_tblspc: false,
                        access_method: "",
                        where_clause: None,
                        pktable: None,
                        fk_attrs: [],
                        pk_attrs: [],
                        fk_matchtype: "",
                        fk_upd_action: "",
                        fk_del_action: "",
                        fk_del_set_cols: [],
                        old_conpfeqop: [],
                        old_pktable_oid: 0,
                        location: 21,
                    },
                ),
            ),
        },
    ),
    behavior: DropRestrict,
    missing_ok: false,
    recurse: false,
}"#
    );
}

#[test]
fn it_parses_SET() {
    let result = parse("SET statement_timeout=1").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.ddl_tables().len(), 0);
    assert_eq!(result.statement_types(), ["VariableSetStmt"]);
    let set = cast!(result.protobuf.nodes()[0].0, NodeRef::VariableSetStmt);
    let a_const = cast!(set.args[0].node.as_ref().unwrap(), NodeEnum::AConst);
    let val = cast!(a_const.val.as_ref().unwrap(), Val::Ival);
    assert_eq!(val.ival, 1);
    assert_eq!(a_const.location, 22);
}

#[test]
fn it_parses_SHOW() {
    let result = parse("SHOW work_mem").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.statement_types(), ["VariableShowStmt"]);
    let show = cast!(result.protobuf.nodes()[0].0, NodeRef::VariableShowStmt);
    assert_eq!(show.name, "work_mem");
}

#[test]
fn it_parses_COPY() {
    let result = parse("COPY test (id) TO stdout").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["test"]);
    assert_eq!(result.statement_types(), ["CopyStmt"]);
    let copy = cast!(result.protobuf.nodes()[0].0, NodeRef::CopyStmt);
    assert_debug_eq!(
        copy,
        r#"CopyStmt {
    relation: Some(
        RangeVar {
            catalogname: "",
            schemaname: "",
            relname: "test",
            inh: true,
            relpersistence: "p",
            alias: None,
            location: 5,
        },
    ),
    query: None,
    attlist: [
        Node {
            node: Some(
                String(
                    String {
                        sval: "id",
                    },
                ),
            ),
        },
    ],
    is_from: false,
    is_program: false,
    filename: "",
    options: [],
    where_clause: None,
}"#
    );
}

#[test]
fn it_parses_DROP_TABLE() {
    let result = parse("drop table abc.test123 cascade").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["abc.test123"]);
    assert_eq!(result.ddl_tables(), ["abc.test123"]);
    assert_eq!(result.statement_types(), ["DropStmt"]);
    let drop = cast!(result.protobuf.nodes()[0].0, NodeRef::DropStmt);
    assert_eq!(protobuf::DropBehavior::from_i32(drop.behavior), Some(protobuf::DropBehavior::DropCascade));

    let result = parse("drop table abc.test123, test").unwrap();
    let tables: Vec<String> = sorted(result.tables()).collect();
    let ddl_tables: Vec<String> = sorted(result.ddl_tables()).collect();
    assert_eq!(tables, ["abc.test123", "test"]);
    assert_eq!(ddl_tables, ["abc.test123", "test"]);
}

#[test]
fn it_parses_COMMIT() {
    let result = parse("COMMIT").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.statement_types(), ["TransactionStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::TransactionStmt);
    assert_eq!(protobuf::TransactionStmtKind::from_i32(stmt.kind), Some(protobuf::TransactionStmtKind::TransStmtCommit));
}

#[test]
fn it_parses_CHECKPOINT() {
    let result = parse("CHECKPOINT").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.statement_types(), ["CheckPointStmt"]);
    cast!(result.protobuf.nodes()[0].0, NodeRef::CheckPointStmt);
}

#[test]
fn it_parses_VACUUM() {
    let result = parse("VACUUM my_table").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["my_table"]);
    assert_eq!(result.ddl_tables(), ["my_table"]);
    assert_eq!(result.statement_types(), ["VacuumStmt"]);
    cast!(result.protobuf.nodes()[0].0, NodeRef::VacuumStmt);
}

#[test]
fn it_parses_EXPLAIN() {
    let result = parse("EXPLAIN DELETE FROM test").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["test"]);
    assert_eq!(result.statement_types(), ["ExplainStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::ExplainStmt);
    cast!(stmt.query.as_ref().unwrap().node.as_ref().unwrap(), NodeEnum::DeleteStmt);
}

#[test]
fn it_parses_SELECT_INTO() {
    let result = parse("CREATE TEMP TABLE test AS SELECT 1").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["test"]);
    assert_eq!(result.ddl_tables(), ["test"]);
    assert_eq!(result.statement_types(), ["CreateTableAsStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::CreateTableAsStmt);
    let select = cast!(stmt.query.as_ref().unwrap().node.as_ref().unwrap(), NodeEnum::SelectStmt);
    let target = cast!(select.target_list[0].node.as_ref().unwrap(), NodeEnum::ResTarget);
    let a_const = cast!(target.val.as_ref().unwrap().node.as_ref().unwrap(), NodeEnum::AConst);
    let val = cast!(a_const.val.as_ref().unwrap(), Val::Ival);
    assert_eq!(val.ival, 1);
    let into = stmt.into.as_ref().unwrap();
    assert_debug_eq!(
        into,
        r#"IntoClause {
    rel: Some(
        RangeVar {
            catalogname: "",
            schemaname: "",
            relname: "test",
            inh: true,
            relpersistence: "t",
            alias: None,
            location: 18,
        },
    ),
    col_names: [],
    access_method: "",
    options: [],
    on_commit: OncommitNoop,
    table_space_name: "",
    view_query: None,
    skip_data: false,
}"#
    );
}

#[test]
fn it_parses_LOCK() {
    let result = parse("LOCK TABLE public.schema_migrations IN ACCESS SHARE MODE").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["public.schema_migrations"]);
    assert_eq!(result.statement_types(), ["LockStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::LockStmt);
    assert_eq!(stmt.mode, 1);
}

#[test]
fn it_parses_CREATE_TABLE() {
    let result = parse("CREATE TABLE test (a int4)").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["test"]);
    assert_eq!(result.ddl_tables(), ["test"]);
    assert_eq!(result.statement_types(), ["CreateStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::CreateStmt);
    let column = cast!(stmt.table_elts[0].node.as_ref().unwrap(), NodeEnum::ColumnDef);
    assert_debug_eq!(
        column,
        r#"ColumnDef {
    colname: "a",
    type_name: Some(
        TypeName {
            names: [
                Node {
                    node: Some(
                        String(
                            String {
                                sval: "int4",
                            },
                        ),
                    ),
                },
            ],
            type_oid: 0,
            setof: false,
            pct_type: false,
            typmods: [],
            typemod: -1,
            array_bounds: [],
            location: 21,
        },
    ),
    compression: "",
    inhcount: 0,
    is_local: true,
    is_not_null: false,
    is_from_type: false,
    storage: "",
    storage_name: "",
    raw_default: None,
    cooked_default: None,
    identity: "",
    identity_sequence: None,
    generated: "",
    coll_clause: None,
    coll_oid: 0,
    constraints: [],
    fdwoptions: [],
    location: 19,
}"#
    );
}

#[test]
fn it_parses_CREATE_TABLE_AS() {
    let result = parse("CREATE TABLE foo AS SELECT * FROM bar;").unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    assert_eq!(tables, ["bar", "foo"]);
    assert_eq!(result.ddl_tables(), ["foo"]);
    assert_eq!(result.select_tables(), ["bar"]);
    assert_eq!(result.statement_types(), ["CreateTableAsStmt"]);

    let sql = "CREATE TABLE foo AS SELECT id FROM bar UNION SELECT id from baz;";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["bar", "baz", "foo"]);
    assert_eq!(result.ddl_tables(), ["foo"]);
    assert_eq!(select_tables, ["bar", "baz"]);
    assert_eq!(result.statement_types(), ["CreateTableAsStmt"]);
}

#[test]
fn it_fails_to_parse_CREATE_TABLE_WITH_OIDS() {
    let error = parse("CREATE TABLE test (a int4) WITH OIDS").err().unwrap();
    assert_eq!(error, Error::Parse("syntax error at or near \"OIDS\"".to_string()));
}

#[test]
fn it_parses_CREATE_INDEX() {
    let result = parse("CREATE INDEX testidx ON test USING btree (a, (lower(b) || upper(c))) WHERE pow(a, 2) > 25").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["test"]);
    assert_eq!(result.ddl_tables(), ["test"]);
    assert_eq!(result.statement_types(), ["IndexStmt"]);
    let call_functions: Vec<String> = sorted(result.call_functions()).collect();
    assert_eq!(call_functions, ["lower", "pow", "upper"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::IndexStmt);
    assert_eq!(stmt.idxname, "testidx".to_string());
}

#[test]
fn it_parses_CREATE_SCHEMA() {
    let result = parse("CREATE SCHEMA IF NOT EXISTS test AUTHORIZATION joe").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.statement_types(), ["CreateSchemaStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::CreateSchemaStmt);
    assert_debug_eq!(
        stmt,
        r#"CreateSchemaStmt {
    schemaname: "test",
    authrole: Some(
        RoleSpec {
            roletype: RolespecCstring,
            rolename: "joe",
            location: 47,
        },
    ),
    schema_elts: [],
    if_not_exists: true,
}"#
    );
}

#[test]
fn it_parses_CREATE_VIEW() {
    let result = parse("CREATE VIEW myview AS SELECT * FROM mytab").unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    assert_eq!(tables, ["mytab", "myview"]);
    assert_eq!(result.ddl_tables(), ["myview"]);
    assert_eq!(result.select_tables(), ["mytab"]);
    assert_eq!(result.statement_types(), ["ViewStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::ViewStmt);
    assert_debug_eq!(
        stmt,
        r#"ViewStmt {
    view: Some(
        RangeVar {
            catalogname: "",
            schemaname: "",
            relname: "myview",
            inh: true,
            relpersistence: "p",
            alias: None,
            location: 12,
        },
    ),
    aliases: [],
    query: Some(
        Node {
            node: Some(
                SelectStmt(
                    SelectStmt {
                        distinct_clause: [],
                        into_clause: None,
                        target_list: [
                            Node {
                                node: Some(
                                    ResTarget(
                                        ResTarget {
                                            name: "",
                                            indirection: [],
                                            val: Some(
                                                Node {
                                                    node: Some(
                                                        ColumnRef(
                                                            ColumnRef {
                                                                fields: [
                                                                    Node {
                                                                        node: Some(
                                                                            AStar(
                                                                                AStar,
                                                                            ),
                                                                        ),
                                                                    },
                                                                ],
                                                                location: 29,
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ),
                                            location: 29,
                                        },
                                    ),
                                ),
                            },
                        ],
                        from_clause: [
                            Node {
                                node: Some(
                                    RangeVar(
                                        RangeVar {
                                            catalogname: "",
                                            schemaname: "",
                                            relname: "mytab",
                                            inh: true,
                                            relpersistence: "p",
                                            alias: None,
                                            location: 36,
                                        },
                                    ),
                                ),
                            },
                        ],
                        where_clause: None,
                        group_clause: [],
                        group_distinct: false,
                        having_clause: None,
                        window_clause: [],
                        values_lists: [],
                        sort_clause: [],
                        limit_offset: None,
                        limit_count: None,
                        limit_option: Default,
                        locking_clause: [],
                        with_clause: None,
                        op: SetopNone,
                        all: false,
                        larg: None,
                        rarg: None,
                    },
                ),
            ),
        },
    ),
    replace: false,
    options: [],
    with_check_option: NoCheckOption,
}"#
    );
}

#[test]
fn it_parses_REFRESH_MATERIALIZED_VIEW() {
    let result = parse("REFRESH MATERIALIZED VIEW myview").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["myview"]);
    assert_eq!(result.ddl_tables(), ["myview"]);
    assert_eq!(result.statement_types(), ["RefreshMatViewStmt"]);
    cast!(result.protobuf.nodes()[0].0, NodeRef::RefreshMatViewStmt);
}

#[test]
fn it_parses_CREATE_RULE() {
    let sql = "CREATE RULE shoe_ins_protect AS ON INSERT TO shoe DO INSTEAD NOTHING";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["shoe"]);
    assert_eq!(result.ddl_tables(), ["shoe"]);
    assert_eq!(result.statement_types(), ["RuleStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::RuleStmt);
    assert_eq!(stmt.rulename, "shoe_ins_protect");
    assert_eq!(protobuf::CmdType::from_i32(stmt.event), Some(protobuf::CmdType::CmdInsert));
}

#[test]
fn it_parses_CREATE_TRIGGER() {
    let sql = "CREATE TRIGGER check_update BEFORE UPDATE ON accounts FOR EACH ROW EXECUTE PROCEDURE check_account_update()";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["accounts"]);
    assert_eq!(result.ddl_tables(), ["accounts"]);
    assert_eq!(result.statement_types(), ["CreateTrigStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::CreateTrigStmt);
    let func = cast!(stmt.funcname[0].node.as_ref().unwrap(), NodeEnum::String);
    assert_eq!(func.sval, "check_account_update");
    assert_eq!(TriggerType::from_i32(stmt.timing), Some(TriggerType::Before));
    assert_eq!(TriggerType::from_i32(stmt.events), Some(TriggerType::Update));
}

#[test]
fn it_parses_DROP_SCHEMA() {
    let result = parse("DROP SCHEMA myschema").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.statement_types(), ["DropStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::DropStmt);
    assert_debug_eq!(
        stmt,
        r#"DropStmt {
    objects: [
        Node {
            node: Some(
                String(
                    String {
                        sval: "myschema",
                    },
                ),
            ),
        },
    ],
    remove_type: ObjectSchema,
    behavior: DropRestrict,
    missing_ok: false,
    concurrent: false,
}"#
    );
}

#[test]
fn it_parses_DROP_VIEW() {
    let result = parse("DROP VIEW myview, myview2").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.statement_types(), ["DropStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::DropStmt);
    assert_debug_eq!(
        stmt,
        r#"DropStmt {
    objects: [
        Node {
            node: Some(
                List(
                    List {
                        items: [
                            Node {
                                node: Some(
                                    String(
                                        String {
                                            sval: "myview",
                                        },
                                    ),
                                ),
                            },
                        ],
                    },
                ),
            ),
        },
        Node {
            node: Some(
                List(
                    List {
                        items: [
                            Node {
                                node: Some(
                                    String(
                                        String {
                                            sval: "myview2",
                                        },
                                    ),
                                ),
                            },
                        ],
                    },
                ),
            ),
        },
    ],
    remove_type: ObjectView,
    behavior: DropRestrict,
    missing_ok: false,
    concurrent: false,
}"#
    );
}

#[test]
fn it_parses_DROP_INDEX() {
    let result = parse("DROP INDEX CONCURRENTLY myindex").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.statement_types(), ["DropStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::DropStmt);
    assert_debug_eq!(
        stmt,
        r#"DropStmt {
    objects: [
        Node {
            node: Some(
                List(
                    List {
                        items: [
                            Node {
                                node: Some(
                                    String(
                                        String {
                                            sval: "myindex",
                                        },
                                    ),
                                ),
                            },
                        ],
                    },
                ),
            ),
        },
    ],
    remove_type: ObjectIndex,
    behavior: DropRestrict,
    missing_ok: false,
    concurrent: true,
}"#
    );
}

#[test]
fn it_parses_DROP_RULE() {
    let result = parse("DROP RULE myrule ON mytable CASCADE").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["mytable"]);
    assert_eq!(result.ddl_tables(), ["mytable"]);
    assert_eq!(result.statement_types(), ["DropStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::DropStmt);
    assert_debug_eq!(
        stmt,
        r#"DropStmt {
    objects: [
        Node {
            node: Some(
                List(
                    List {
                        items: [
                            Node {
                                node: Some(
                                    String(
                                        String {
                                            sval: "mytable",
                                        },
                                    ),
                                ),
                            },
                            Node {
                                node: Some(
                                    String(
                                        String {
                                            sval: "myrule",
                                        },
                                    ),
                                ),
                            },
                        ],
                    },
                ),
            ),
        },
    ],
    remove_type: ObjectRule,
    behavior: DropCascade,
    missing_ok: false,
    concurrent: false,
}"#
    );
}

#[test]
fn it_parses_DROP_TRIGGER() {
    let result = parse("DROP TRIGGER IF EXISTS mytrigger ON mytable RESTRICT").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["mytable"]);
    assert_eq!(result.ddl_tables(), ["mytable"]);
    assert_eq!(result.statement_types(), ["DropStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::DropStmt);
    assert_debug_eq!(
        stmt,
        r#"DropStmt {
    objects: [
        Node {
            node: Some(
                List(
                    List {
                        items: [
                            Node {
                                node: Some(
                                    String(
                                        String {
                                            sval: "mytable",
                                        },
                                    ),
                                ),
                            },
                            Node {
                                node: Some(
                                    String(
                                        String {
                                            sval: "mytrigger",
                                        },
                                    ),
                                ),
                            },
                        ],
                    },
                ),
            ),
        },
    ],
    remove_type: ObjectTrigger,
    behavior: DropRestrict,
    missing_ok: true,
    concurrent: false,
}"#
    );
}

#[test]
fn it_parses_GRANT() {
    let result = parse("GRANT INSERT, UPDATE ON mytable TO myuser").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["mytable"]);
    assert_eq!(result.ddl_tables(), ["mytable"]);
    assert_eq!(result.statement_types(), ["GrantStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::GrantStmt);
    assert_debug_eq!(
        stmt,
        r#"GrantStmt {
    is_grant: true,
    targtype: AclTargetObject,
    objtype: ObjectTable,
    objects: [
        Node {
            node: Some(
                RangeVar(
                    RangeVar {
                        catalogname: "",
                        schemaname: "",
                        relname: "mytable",
                        inh: true,
                        relpersistence: "p",
                        alias: None,
                        location: 24,
                    },
                ),
            ),
        },
    ],
    privileges: [
        Node {
            node: Some(
                AccessPriv(
                    AccessPriv {
                        priv_name: "insert",
                        cols: [],
                    },
                ),
            ),
        },
        Node {
            node: Some(
                AccessPriv(
                    AccessPriv {
                        priv_name: "update",
                        cols: [],
                    },
                ),
            ),
        },
    ],
    grantees: [
        Node {
            node: Some(
                RoleSpec(
                    RoleSpec {
                        roletype: RolespecCstring,
                        rolename: "myuser",
                        location: 35,
                    },
                ),
            ),
        },
    ],
    grant_option: false,
    grantor: None,
    behavior: DropRestrict,
}"#
    );
}

#[test]
fn it_parses_REVOKE() {
    let result = parse("REVOKE admins FROM joe").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.statement_types(), ["GrantRoleStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::GrantRoleStmt);
    assert_debug_eq!(
        stmt,
        r#"GrantRoleStmt {
    granted_roles: [
        Node {
            node: Some(
                AccessPriv(
                    AccessPriv {
                        priv_name: "admins",
                        cols: [],
                    },
                ),
            ),
        },
    ],
    grantee_roles: [
        Node {
            node: Some(
                RoleSpec(
                    RoleSpec {
                        roletype: RolespecCstring,
                        rolename: "joe",
                        location: 19,
                    },
                ),
            ),
        },
    ],
    is_grant: false,
    opt: [],
    grantor: None,
    behavior: DropRestrict,
}"#
    );
}

#[test]
fn it_parses_TRUNCATE() {
    let result = parse(r#"TRUNCATE bigtable, "fattable" RESTART IDENTITY"#).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let ddl_tables: Vec<String> = sorted(result.ddl_tables()).collect();
    assert_eq!(tables, ["bigtable", "fattable"]);
    assert_eq!(ddl_tables, ["bigtable", "fattable"]);
    assert_eq!(result.statement_types(), ["TruncateStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::TruncateStmt);
    assert_debug_eq!(
        stmt,
        r#"TruncateStmt {
    relations: [
        Node {
            node: Some(
                RangeVar(
                    RangeVar {
                        catalogname: "",
                        schemaname: "",
                        relname: "bigtable",
                        inh: true,
                        relpersistence: "p",
                        alias: None,
                        location: 9,
                    },
                ),
            ),
        },
        Node {
            node: Some(
                RangeVar(
                    RangeVar {
                        catalogname: "",
                        schemaname: "",
                        relname: "fattable",
                        inh: true,
                        relpersistence: "p",
                        alias: None,
                        location: 19,
                    },
                ),
            ),
        },
    ],
    restart_seqs: true,
    behavior: DropRestrict,
}"#
    );
}

#[test]
fn it_parses_WITH() {
    let result = parse("WITH a AS (SELECT * FROM x WHERE x.y = $1 AND x.z = 1) SELECT * FROM a").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["x"]);
    assert_eq!(result.cte_names, ["a"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_parses_multi_line_functions() {
    let sql = "CREATE OR REPLACE FUNCTION thing(parameter_thing text)
  RETURNS bigint AS
$BODY$
DECLARE
        local_thing_id BIGINT := 0;
BEGIN
        SELECT thing_id INTO local_thing_id FROM thing_map
        WHERE
                thing_map_field = parameter_thing
        ORDER BY 1 LIMIT 1;

        IF NOT FOUND THEN
                local_thing_id = 0;
        END IF;
        RETURN local_thing_id;
END;
$BODY$
  LANGUAGE plpgsql STABLE";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.functions(), ["thing"]);
    assert_eq!(result.ddl_functions(), ["thing"]);
    assert_eq!(result.call_functions().len(), 0);
    assert_eq!(result.statement_types(), ["CreateFunctionStmt"]);
    let stmt = cast!(result.protobuf.nodes()[0].0, NodeRef::CreateFunctionStmt);
    assert_debug_eq!(
        stmt,
        r#"CreateFunctionStmt {
    is_procedure: false,
    replace: true,
    funcname: [
        Node {
            node: Some(
                String(
                    String {
                        sval: "thing",
                    },
                ),
            ),
        },
    ],
    parameters: [
        Node {
            node: Some(
                FunctionParameter(
                    FunctionParameter {
                        name: "parameter_thing",
                        arg_type: Some(
                            TypeName {
                                names: [
                                    Node {
                                        node: Some(
                                            String(
                                                String {
                                                    sval: "text",
                                                },
                                            ),
                                        ),
                                    },
                                ],
                                type_oid: 0,
                                setof: false,
                                pct_type: false,
                                typmods: [],
                                typemod: -1,
                                array_bounds: [],
                                location: 49,
                            },
                        ),
                        mode: FuncParamDefault,
                        defexpr: None,
                    },
                ),
            ),
        },
    ],
    return_type: Some(
        TypeName {
            names: [
                Node {
                    node: Some(
                        String(
                            String {
                                sval: "pg_catalog",
                            },
                        ),
                    ),
                },
                Node {
                    node: Some(
                        String(
                            String {
                                sval: "int8",
                            },
                        ),
                    ),
                },
            ],
            type_oid: 0,
            setof: false,
            pct_type: false,
            typmods: [],
            typemod: -1,
            array_bounds: [],
            location: 65,
        },
    ),
    options: [
        Node {
            node: Some(
                DefElem(
                    DefElem {
                        defnamespace: "",
                        defname: "as",
                        arg: Some(
                            Node {
                                node: Some(
                                    List(
                                        List {
                                            items: [
                                                Node {
                                                    node: Some(
                                                        String(
                                                            String {
                                                                sval: "\nDECLARE\n        local_thing_id BIGINT := 0;\nBEGIN\n        SELECT thing_id INTO local_thing_id FROM thing_map\n        WHERE\n                thing_map_field = parameter_thing\n        ORDER BY 1 LIMIT 1;\n\n        IF NOT FOUND THEN\n                local_thing_id = 0;\n        END IF;\n        RETURN local_thing_id;\nEND;\n",
                                                            },
                                                        ),
                                                    ),
                                                },
                                            ],
                                        },
                                    ),
                                ),
                            },
                        ),
                        defaction: DefelemUnspec,
                        location: 72,
                    },
                ),
            ),
        },
        Node {
            node: Some(
                DefElem(
                    DefElem {
                        defnamespace: "",
                        defname: "language",
                        arg: Some(
                            Node {
                                node: Some(
                                    String(
                                        String {
                                            sval: "plpgsql",
                                        },
                                    ),
                                ),
                            },
                        ),
                        defaction: DefelemUnspec,
                        location: 407,
                    },
                ),
            ),
        },
        Node {
            node: Some(
                DefElem(
                    DefElem {
                        defnamespace: "",
                        defname: "volatility",
                        arg: Some(
                            Node {
                                node: Some(
                                    String(
                                        String {
                                            sval: "stable",
                                        },
                                    ),
                                ),
                            },
                        ),
                        defaction: DefelemUnspec,
                        location: 424,
                    },
                ),
            ),
        },
    ],
    sql_body: None,
}"#
    );
}

#[test]
fn it_parses_table_functions() {
    let sql = "CREATE FUNCTION getfoo(int) RETURNS TABLE (f1 int) AS 'SELECT * FROM foo WHERE fooid = $1;' LANGUAGE SQL";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.functions(), ["getfoo"]);
    assert_eq!(result.ddl_functions(), ["getfoo"]);
    assert_eq!(result.call_functions().len(), 0);
    assert_eq!(result.statement_types(), ["CreateFunctionStmt"]);
}

#[test]
fn it_finds_called_functions() {
    let result = parse("SELECT testfunc(1);").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.functions(), ["testfunc"]);
    assert_eq!(result.ddl_functions().len(), 0);
    assert_eq!(result.call_functions(), ["testfunc"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_dropped_functions() {
    let result = parse("DROP FUNCTION IF EXISTS testfunc(x integer);").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.functions(), ["testfunc"]);
    assert_eq!(result.ddl_functions(), ["testfunc"]);
    assert_eq!(result.call_functions().len(), 0);
    assert_eq!(result.statement_types(), ["DropStmt"]);
}

#[test]
fn it_finds_renamed_functions() {
    let result = parse("ALTER FUNCTION testfunc(integer) RENAME TO testfunc2;").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    let functions: Vec<String> = sorted(result.functions()).collect();
    let ddl_functions: Vec<String> = sorted(result.functions()).collect();
    assert_eq!(functions, ["testfunc", "testfunc2"]);
    assert_eq!(ddl_functions, ["testfunc", "testfunc2"]);
    assert_eq!(result.call_functions().len(), 0);
    assert_eq!(result.statement_types(), ["RenameStmt"]);
}

// https://github.com/pganalyze/pg_query/issues/38
#[test]
fn it_finds_nested_tables_in_SELECT() {
    let sql = "select u.email, (select count(*) from enrollments e where e.user_id = u.id) as num_enrollments from users u";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["enrollments", "users"]);
    assert_eq!(select_tables, ["enrollments", "users"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

// https://github.com/pganalyze/pg_query/issues/52
#[test]
fn it_separates_CTE_names_from_table_names() {
    let sql = "WITH cte_name AS (SELECT 1) SELECT * FROM table_name, cte_name";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["table_name"]);
    assert_eq!(result.select_tables(), ["table_name"]);
    assert_eq!(result.cte_names, ["cte_name"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_nested_tables_in_FROM_clause() {
    let result = parse("select u.* from (select * from users) u").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["users"]);
    assert_eq!(result.select_tables(), ["users"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_nested_tables_in_WHERE_clause() {
    let result = parse("select users.id from users where 1 = (select count(*) from user_roles)").unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["user_roles", "users"]);
    assert_eq!(select_tables, ["user_roles", "users"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_tables_in_SELECT_with_subselects_without_FROM() {
    let query = "
        SELECT *
        FROM pg_catalog.pg_class c
        JOIN (
            SELECT 17650 AS oid
            UNION ALL
            SELECT 17663 AS oid
        ) vals ON c.oid = vals.oid
    ";
    let result = parse(query).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["pg_catalog.pg_class"]);
    assert_eq!(result.select_tables(), ["pg_catalog.pg_class"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
    // TODO: add filter_columns
    // expect(query.filter_columns).to eq [["pg_catalog.pg_class", "oid"], ["vals", "oid"]]
}

#[test]
fn it_finds_nested_tables_in_IN_clause() {
    let sql = "
        select users.*
        from users
        where users.id IN (select user_roles.user_id from user_roles)
            and (users.created_at between '2016-06-01' and '2016-06-30')
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["user_roles", "users"]);
    assert_eq!(select_tables, ["user_roles", "users"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_nested_tables_in_ORDER_BY_clause() {
    let sql = "
        select users.*
        from users
        order by (
            select max(user_roles.role_id)
            from user_roles
            where user_roles.user_id = users.id
        )
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["user_roles", "users"]);
    assert_eq!(select_tables, ["user_roles", "users"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_nested_tables_in_ORDER_BY_clause_with_multiple_entries() {
    let sql = "
        select users.*
        from users
        order by (
            select max(user_roles.role_id)
            from user_roles
            where user_roles.user_id = users.id
        ) asc, (
            select max(user_logins.role_id)
            from user_logins
            where user_logins.user_id = users.id
        ) desc
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["user_logins", "user_roles", "users"]);
    assert_eq!(select_tables, ["user_logins", "user_roles", "users"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_nested_tables_in_GROUP_BY_clause() {
    let sql = "
        select users.*
        from users
        group by (
            select max(user_roles.role_id)
            from user_roles
            where user_roles.user_id = users.id
        )
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["user_roles", "users"]);
    assert_eq!(select_tables, ["user_roles", "users"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_nested_tables_in_GROUP_BY_clause_with_multiple_entries() {
    let sql = "
        select users.*
        from users
        group by (
            select max(user_roles.role_id)
            from user_roles
            where user_roles.user_id = users.id
        ), (
            select max(user_logins.role_id)
            from user_logins
            where user_logins.user_id = users.id
        )
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["user_logins", "user_roles", "users"]);
    assert_eq!(select_tables, ["user_logins", "user_roles", "users"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_nested_tables_in_HAVING_clause() {
    let sql = "
        select users.*
        from users
        group by users.id
        having 1 > (
            select count(user_roles.role_id)
            from user_roles
            where user_roles.user_id = users.id
        )
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["user_roles", "users"]);
    assert_eq!(select_tables, ["user_roles", "users"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_nested_tables_in_HAVING_clause_with_boolean_expression() {
    let sql = "
        select users.*
        from users
        group by users.id
        having true and 1 > (
            select count(user_roles.role_id)
            from user_roles
            where user_roles.user_id = users.id
        )
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["user_roles", "users"]);
    assert_eq!(select_tables, ["user_roles", "users"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_nested_tables_in_a_subselect_on_a_JOIN() {
    let sql = "
        select foo.*
        from foo
        join ( select * from bar ) b
        on b.baz = foo.quux
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["bar", "foo"]);
    assert_eq!(select_tables, ["bar", "foo"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_nested_tables_in_a_subselect_in_a_JOIN_condition() {
    let sql = "
        SELECT *
        FROM foo
        INNER JOIN join_a ON foo.id = join_a.id AND join_a.id IN (
            SELECT id
            FROM sub_a
            INNER JOIN sub_b ON sub_a.id = sub_b.id AND sub_b.id IN (
                SELECT id
                FROM sub_c
                INNER JOIN sub_d ON sub_c.id IN (SELECT id from sub_e)
            )
        )
        INNER JOIN join_b ON foo.id = join_b.id AND join_b.id IN (
          SELECT id FROM sub_f
        )
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["foo", "join_a", "join_b", "sub_a", "sub_b", "sub_c", "sub_d", "sub_e", "sub_f"]);
    assert_eq!(select_tables, tables);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_correctly_categorizes_CTEs_after_UNION_SELECT() {
    let sql = "
        WITH cte_a AS (
            SELECT * FROM table_a
        ), cte_b AS (
            SELECT * FROM table_b
        )
        SELECT id FROM table_c
        LEFT JOIN cte_b ON table_c.id = cte_b.c_id
        UNION
        SELECT * FROM cte_a
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let cte_names: Vec<String> = sorted(result.cte_names.clone()).collect();
    assert_eq!(tables, ["table_a", "table_b", "table_c"]);
    assert_eq!(cte_names, ["cte_a", "cte_b"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_correctly_categorizes_CTEs_after_EXCEPT_SELECT() {
    let sql = "
        WITH cte_a AS (
            SELECT * FROM table_a
        ), cte_b AS (
            SELECT * FROM table_b
        )
        SELECT id FROM table_c
        LEFT JOIN cte_b ON table_c.id = cte_b.c_id
        EXCEPT
        SELECT * FROM cte_a
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let cte_names: Vec<String> = sorted(result.cte_names.clone()).collect();
    assert_eq!(tables, ["table_a", "table_b", "table_c"]);
    assert_eq!(cte_names, ["cte_a", "cte_b"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_correctly_categorizes_CTEs_after_INTERSECT_SELECT() {
    let sql = "
        WITH cte_a AS (
            SELECT * FROM table_a
        ), cte_b AS (
            SELECT * FROM table_b
        )
        SELECT id FROM table_c
        LEFT JOIN cte_b ON table_c.id = cte_b.c_id
        INTERSECT
        SELECT * FROM cte_a
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let cte_names: Vec<String> = sorted(result.cte_names.clone()).collect();
    assert_eq!(tables, ["table_a", "table_b", "table_c"]);
    assert_eq!(cte_names, ["cte_a", "cte_b"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_tables_inside_subselectes_in_MIN_MAX_COALESCE() {
    let sql = "
        SELECT GREATEST(
            date_trunc($1, $2::timestamptz) + $3::interval,
            COALESCE(
                (
                    SELECT first_aggregate_starts_at
                    FROM schema_aggregate_infos
                    WHERE base_table = $4 LIMIT $5
                ),
                now() + $6::interval
            )
        ) AS first_hourly_start_ts
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["schema_aggregate_infos"]);
    assert_eq!(result.select_tables(), ["schema_aggregate_infos"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_tables_inside_CASE_statements() {
    let sql = "
        SELECT
        CASE
            WHEN id IN (SELECT foo_id FROM when_a) THEN (SELECT MAX(id) FROM then_a)
            WHEN id IN (SELECT foo_id FROM when_b) THEN (SELECT MAX(id) FROM then_b)
            ELSE (SELECT MAX(id) FROM elsey)
        END
        FROM foo
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["elsey", "foo", "then_a", "then_b", "when_a", "when_b"]);
    assert_eq!(select_tables, tables);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_tables_inside_casts() {
    let sql = "
        SELECT 1
        FROM   foo
        WHERE  x = any(cast(array(SELECT a FROM bar) as bigint[]))
            OR x = any(array(SELECT a FROM baz)::bigint[])
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    assert_eq!(tables, ["bar", "baz", "foo"]);
    assert_eq!(result.functions().len(), 0);
    assert_eq!(result.call_functions().len(), 0);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_functions_in_FROM_clause() {
    let sql = "SELECT * FROM my_custom_func()";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.functions(), ["my_custom_func"]);
    assert_eq!(result.call_functions(), ["my_custom_func"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_functions_in_LATERAL_clause() {
    let sql = "
        SELECT *
        FROM unnest($1::text[]) AS a(x)
        LEFT OUTER JOIN LATERAL (
            SELECT json_build_object($2, z.z)
            FROM (
                SELECT *
                FROM (
                    SELECT row_to_json(
                        (SELECT * FROM (SELECT public.my_function(b) FROM public.c) d)
                    )
                ) e
            ) f
        ) AS g ON (1)
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["public.c"]);
    let functions: Vec<String> = sorted(result.functions()).collect();
    let call_functions: Vec<String> = sorted(result.call_functions()).collect();
    assert_eq!(functions, ["json_build_object", "public.my_function", "row_to_json", "unnest"]);
    assert_eq!(call_functions, functions);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_parses_INSERT() {
    let result = parse("insert into users(pk, name) values (1, 'bob');").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["users"]);

    let result = parse("insert into users(pk, name) select pk, name from other_users;").unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    assert_eq!(tables, ["other_users", "users"]);

    let sql = "
        with cte as (
            select pk, name from other_users
        )
        insert into users(pk, name) select * from cte;
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    assert_eq!(tables, ["other_users", "users"]);
    assert_eq!(result.select_tables(), ["other_users"]);
    assert_eq!(result.dml_tables(), ["users"]);
    assert_eq!(result.cte_names, ["cte"]);
    assert_eq!(result.statement_types(), ["InsertStmt"]);
}

#[test]
fn it_parses_UPDATE() {
    let result = parse("update users set name = 'bob';").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["users"]);
    assert_eq!(result.statement_types(), ["UpdateStmt"]);

    let result = parse("update users set name = (select name from other_users limit 1);").unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    assert_eq!(tables, ["other_users", "users"]);
    assert_eq!(result.statement_types(), ["UpdateStmt"]);

    let sql = "
        with cte as (
            select name from other_users limit 1
        )
        update users set name = (select name from cte);
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    assert_eq!(tables, ["other_users", "users"]);
    assert_eq!(result.select_tables(), ["other_users"]);
    assert_eq!(result.dml_tables(), ["users"]);
    assert_eq!(result.cte_names, ["cte"]);
    assert_eq!(result.statement_types(), ["UpdateStmt"]);

    let sql = "
        UPDATE users SET name = users_new.name
        FROM users_new
        INNER JOIN join_table ON join_table.user_id = new_users.id
        WHERE users.id = users_new.id
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["join_table", "users", "users_new"]);
    assert_eq!(select_tables, ["join_table", "users_new"]);
    assert_eq!(result.dml_tables(), ["users"]);
    assert_eq!(result.statement_types(), ["UpdateStmt"]);
}

#[test]
fn it_parses_DELETE() {
    let result = parse("DELETE FROM users;").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["users"]);
    assert_eq!(result.dml_tables(), ["users"]);
    assert_eq!(result.statement_types(), ["DeleteStmt"]);

    let sql = "
        DELETE FROM users USING foo
        WHERE foo_id = foo.id AND foo.action = 'delete';
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    assert_eq!(tables, ["foo", "users"]);
    assert_eq!(result.dml_tables(), ["users"]);
    assert_eq!(result.select_tables(), ["foo"]);
    assert_eq!(result.statement_types(), ["DeleteStmt"]);

    let sql = "
        DELETE FROM users
        WHERE foo_id IN (SELECT id FROM foo WHERE action = 'delete');
    ";
    let result = parse(sql).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    assert_eq!(tables, ["foo", "users"]);
    assert_eq!(result.dml_tables(), ["users"]);
    assert_eq!(result.select_tables(), ["foo"]);
    assert_eq!(result.statement_types(), ["DeleteStmt"]);
}

#[test]
fn it_parses_DROP_TYPE() {
    let result = parse("DROP TYPE IF EXISTS repack.pk_something").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.statement_types(), ["DropStmt"]);
    assert_debug_eq!(
        result.protobuf.nodes()[0].0,
        r#"DropStmt(
    DropStmt {
        objects: [
            Node {
                node: Some(
                    TypeName(
                        TypeName {
                            names: [
                                Node {
                                    node: Some(
                                        String(
                                            String {
                                                sval: "repack",
                                            },
                                        ),
                                    ),
                                },
                                Node {
                                    node: Some(
                                        String(
                                            String {
                                                sval: "pk_something",
                                            },
                                        ),
                                    ),
                                },
                            ],
                            type_oid: 0,
                            setof: false,
                            pct_type: false,
                            typmods: [],
                            typemod: -1,
                            array_bounds: [],
                            location: 20,
                        },
                    ),
                ),
            },
        ],
        remove_type: ObjectType,
        behavior: DropRestrict,
        missing_ok: true,
        concurrent: false,
    },
)"#
    );
}
