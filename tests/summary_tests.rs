#![allow(non_snake_case)]
#![cfg(test)]

#[cfg(test)]
use itertools::sorted;

use pg_query::{
    protobuf::{self, a_const::Val},
    summary, Error, NodeEnum, NodeRef, SummaryResult, TriggerType,
};

#[macro_use]
mod support;
use support::*;

#[test]
fn it_parses_simple_query() {
    let result = summary("SELECT * FROM test WHERE a = 1", 0, -1).unwrap();

    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 1);
    assert_eq!(result.aliases.is_empty(), true);
    assert_eq!(result.cte_names.len(), 0);
    assert_eq!(result.functions.len(), 0);
    assert_eq!(result.filter_columns.len(), 0);
    assert_eq!(result.truncated_query.is_none(), true);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_handles_errors() {
    let error = summary("CREATE RANDOM ix_test ON contacts.person;", 0, -1).err().unwrap();
    assert_eq!(error, Error::Parse("syntax error at or near \"RANDOM\"".into()));

    let error = summary("SELECT 'ERR", 0, -1).err().unwrap();
    assert_eq!(error, Error::Parse("unterminated quoted string at or near \"'ERR\"".into()));
}

#[test]
fn it_handles_basic_query() {
    let query = r#"SELECT * FROM "t0""#;
    let result = summary(query, 0, -1).unwrap();
    assert_eq!(result.tables().len(), 1);
    assert_eq!(result.tables()[0], "t0");
    assert_eq!(result.warnings, Vec::<String>::new());
    assert_eq!(result.aliases, std::collections::HashMap::new());
    assert_eq!(result.cte_names, Vec::<String>::new());
    assert_eq!(result.functions, Vec::<pg_query::Function>::new());
    assert_eq!(result.filter_columns, Vec::<pg_query::FilterColumn>::new());
    assert_eq!(result.truncated_query, None);
}

#[test]
fn it_handles_join_expression() {
    let query = r#"SELECT * FROM "t0" JOIN "t1" ON (1)"#;
    let result = summary(query, 0, -1).unwrap();
    assert_eq!(result.tables().len(), 2);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_handles_recursion_without_error() {
    // The Ruby version of pg_query fails here because of Ruby protobuf limitations
    let query = r#"SELECT * FROM "t0"
        JOIN "t1" ON (1) JOIN "t2" ON (1) JOIN "t3" ON (1) JOIN "t4" ON (1) JOIN "t5" ON (1)
        JOIN "t6" ON (1) JOIN "t7" ON (1) JOIN "t8" ON (1) JOIN "t9" ON (1) JOIN "t10" ON (1)
        JOIN "t11" ON (1) JOIN "t12" ON (1) JOIN "t13" ON (1) JOIN "t14" ON (1) JOIN "t15" ON (1)
        JOIN "t16" ON (1) JOIN "t17" ON (1) JOIN "t18" ON (1) JOIN "t19" ON (1) JOIN "t20" ON (1)
        JOIN "t21" ON (1) JOIN "t22" ON (1) JOIN "t23" ON (1) JOIN "t24" ON (1) JOIN "t25" ON (1)
        JOIN "t26" ON (1) JOIN "t27" ON (1) JOIN "t28" ON (1) JOIN "t29" ON (1)"#;
    let result = summary(query, 0, -1).unwrap();
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
    let result = summary(query, 0, -1).unwrap();
    eprintln!("{:?}", result);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["snapshots", "system_snapshots"]);
    assert_eq!(select_tables, ["snapshots", "system_snapshots"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_parses_empty_queries() {
    let result = summary("-- nothing", 0, -1).unwrap();
    //assert_eq!(result.protobuf.nodes().len(), 0);
    assert_eq!(result.statement_types().len(), 0);
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.aliases.is_empty(), true);
    assert_eq!(result.cte_names.len(), 0);
    assert_eq!(result.functions.len(), 0);
    assert_eq!(result.filter_columns.len(), 0);
    assert_eq!(result.truncated_query.is_none(), true);
}

#[test]
fn it_parses_floats_with_leading_dot() {
    let result = summary("SELECT .1", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
}

#[test]
fn it_parses_bit_strings_hex_notation() {
    let result = summary("SELECT X'EFFF'", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
}

#[test]
fn it_parses_ALTER_TABLE() {
    let result = summary("ALTER TABLE test ADD PRIMARY KEY (gid)", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["test"]);
    assert_eq!(result.ddl_tables(), ["test"]);
    assert_eq!(result.statement_types(), ["AlterTableStmt"]);
}

#[test]
fn it_parses_SET() {
    let result = summary("SET statement_timeout=1", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.ddl_tables().len(), 0);
    assert_eq!(result.statement_types(), ["VariableSetStmt"]);
}

#[test]
fn it_parses_SHOW() {
    let result = summary("SHOW work_mem", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.statement_types(), ["VariableShowStmt"]);
}

#[test]
fn it_parses_COPY() {
    let result = summary("COPY test (id) TO stdout", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["test"]);
    assert_eq!(result.statement_types(), ["CopyStmt"]);
}
/*
#[test]
fn it_parses_DROP_TABLE() {
    let result = summary("drop table abc.test123 cascade", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["abc.test123"]);
    assert_eq!(result.ddl_tables(), ["abc.test123"]);
    assert_eq!(result.statement_types(), ["DropStmt"]);

    let result = summary("drop table abc.test123, test", 0, -1).unwrap();
    let tables: Vec<String> = sorted(result.tables()).collect();
    let ddl_tables: Vec<String> = sorted(result.ddl_tables()).collect();
    assert_eq!(tables, ["abc.test123", "test"]);
    assert_eq!(ddl_tables, ["abc.test123", "test"]);
}
*/

#[test]
fn it_parses_COMMIT() {
    let result = summary("COMMIT", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.statement_types(), ["TransactionStmt"]);
}

#[test]
fn it_parses_CHECKPOINT() {
    let result = summary("CHECKPOINT", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.statement_types(), ["CheckPointStmt"]);
}

#[test]
fn it_parses_VACUUM() {
    let result = summary("VACUUM my_table", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["my_table"]);
    assert_eq!(result.ddl_tables(), ["my_table"]);
    assert_eq!(result.statement_types(), ["VacuumStmt"]);
}

/*
#[test]
fn it_parses_EXPLAIN() {
    let result = summary("EXPLAIN DELETE FROM test", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["test"]);
    assert_eq!(result.statement_types(), ["ExplainStmt"]);
}

#[test]
fn it_parses_SELECT_INTO() {
    let result = summary("CREATE TEMP TABLE test AS SELECT 1", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["test"]);
    assert_eq!(result.ddl_tables(), ["test"]);
    assert_eq!(result.statement_types(), ["CreateTableAsStmt"]);
}

#[test]
fn it_parses_LOCK() {
    let result = summary("LOCK TABLE public.schema_migrations IN ACCESS SHARE MODE", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["public.schema_migrations"]);
    assert_eq!(result.statement_types(), ["LockStmt"]);
}

#[test]
fn it_parses_CREATE_TABLE() {
    let result = summary("CREATE TABLE test (a int4, 0, -1)").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["test"]);
    assert_eq!(result.ddl_tables(), ["test"]);
    assert_eq!(result.statement_types(), ["CreateStmt"]);
}

#[test]
fn it_parses_CREATE_TABLE_AS() {
    let result = summary("CREATE TABLE foo AS SELECT * FROM bar;", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    assert_eq!(tables, ["bar", "foo"]);
    assert_eq!(result.ddl_tables(), ["foo"]);
    assert_eq!(result.select_tables(), ["bar"]);
    assert_eq!(result.statement_types(), ["CreateTableAsStmt"]);

    let sql = "CREATE TABLE foo AS SELECT id FROM bar UNION SELECT id from baz;";
    let result = summary(sql, 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let select_tables: Vec<String> = sorted(result.select_tables()).collect();
    assert_eq!(tables, ["bar", "baz", "foo"]);
    assert_eq!(result.ddl_tables(), ["foo"]);
    assert_eq!(select_tables, ["bar", "baz"]);
    assert_eq!(result.statement_types(), ["CreateTableAsStmt"]);
}
*/
#[test]
fn it_fails_to_parse_CREATE_TABLE_WITH_OIDS() {
    let error = summary("CREATE TABLE test (a int4) WITH OIDS", 0, -1).err().unwrap();
    assert_eq!(error, Error::Parse("syntax error at or near \"OIDS\"".to_string()));
}

#[test]
fn it_parses_CREATE_INDEX() {
    let result = summary("CREATE INDEX testidx ON test USING btree (a, (lower(b) || upper(c))) WHERE pow(a, 2) > 25", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["test"]);
    assert_eq!(result.ddl_tables(), ["test"]);
    assert_eq!(result.statement_types(), ["IndexStmt"]);
    let call_functions: Vec<String> = sorted(result.call_functions()).collect();
    assert_eq!(call_functions, ["lower", "pow", "upper"]);
}

/*
#[test]
fn it_parses_CREATE_SCHEMA() {
    let result = summary("CREATE SCHEMA IF NOT EXISTS test AUTHORIZATION joe", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.statement_types(), ["CreateSchemaStmt"]);
}

#[test]
fn it_parses_CREATE_VIEW() {
    let result = summary("CREATE VIEW myview AS SELECT * FROM mytab", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    assert_eq!(tables, ["mytab", "myview"]);
    assert_eq!(result.ddl_tables(), ["myview"]);
    assert_eq!(result.select_tables(), ["mytab"]);
    assert_eq!(result.statement_types(), ["ViewStmt"]);
}

#[test]
fn it_parses_REFRESH_MATERIALIZED_VIEW() {
    let result = summary("REFRESH MATERIALIZED VIEW myview", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["myview"]);
    assert_eq!(result.ddl_tables(), ["myview"]);
    assert_eq!(result.statement_types(), ["RefreshMatViewStmt"]);
}

#[test]
fn it_parses_CREATE_RULE() {
    let sql = "CREATE RULE shoe_ins_protect AS ON INSERT TO shoe DO INSTEAD NOTHING";
    let result = summary(sql, 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["shoe"]);
    assert_eq!(result.ddl_tables(), ["shoe"]);
    assert_eq!(result.statement_types(), ["RuleStmt"]);
}

#[test]
fn it_parses_CREATE_TRIGGER() {
    let sql = "CREATE TRIGGER check_update BEFORE UPDATE ON accounts FOR EACH ROW EXECUTE PROCEDURE check_account_update()";
    let result = summary(sql, 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["accounts"]);
    assert_eq!(result.ddl_tables(), ["accounts"]);
    assert_eq!(result.statement_types(), ["CreateTrigStmt"]);
}

#[test]
fn it_parses_DROP_SCHEMA() {
    let result = summary("DROP SCHEMA myschema", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.statement_types(), ["DropStmt"]);
}

#[test]
fn it_parses_DROP_VIEW() {
    let result = summary("DROP VIEW myview, myview2", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.statement_types(), ["DropStmt"]);
}

#[test]
fn it_parses_DROP_INDEX() {
    let result = summary("DROP INDEX CONCURRENTLY myindex", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.statement_types(), ["DropStmt"]);
}

#[test]
fn it_parses_DROP_RULE() {
    let result = summary("DROP RULE myrule ON mytable CASCADE", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["mytable"]);
    assert_eq!(result.ddl_tables(), ["mytable"]);
    assert_eq!(result.statement_types(), ["DropStmt"]);
}

#[test]
fn it_parses_DROP_TRIGGER() {
    let result = summary("DROP TRIGGER IF EXISTS mytrigger ON mytable RESTRICT", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["mytable"]);
    assert_eq!(result.ddl_tables(), ["mytable"]);
    assert_eq!(result.statement_types(), ["DropStmt"]);
}
*/

#[test]
fn it_parses_GRANT() {
    let result = summary("GRANT INSERT, UPDATE ON mytable TO myuser", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["mytable"]);
    assert_eq!(result.ddl_tables(), ["mytable"]);
    assert_eq!(result.statement_types(), ["GrantStmt"]);
}

/*
#[test]
fn it_parses_REVOKE() {
    let result = summary("REVOKE admins FROM joe", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.statement_types(), ["GrantRoleStmt"]);
}

#[test]
fn it_parses_TRUNCATE() {
    let result = summary(r#"TRUNCATE bigtable, "fattable" RESTART IDENTITY"#, 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    let ddl_tables: Vec<String> = sorted(result.ddl_tables()).collect();
    assert_eq!(tables, ["bigtable", "fattable"]);
    assert_eq!(ddl_tables, ["bigtable", "fattable"]);
    assert_eq!(result.statement_types(), ["TruncateStmt"]);
}

#[test]
fn it_parses_WITH() {
    let result = summary("WITH a AS (SELECT * FROM x WHERE x.y = $1 AND x.z = 1, 0, -1) SELECT * FROM a").unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["x"]);
    assert_eq!(result.cte_names, ["a"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}
*/

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
    let result = summary(sql, 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.functions(), ["thing"]);
    assert_eq!(result.ddl_functions(), ["thing"]);
    assert_eq!(result.call_functions().len(), 0);
    assert_eq!(result.statement_types(), ["CreateFunctionStmt"]);
}

#[test]
fn it_parses_table_functions() {
    let sql = "CREATE FUNCTION getfoo(int) RETURNS TABLE (f1 int) AS 'SELECT * FROM foo WHERE fooid = $1;' LANGUAGE SQL";
    let result = summary(sql, 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.functions(), ["getfoo"]);
    assert_eq!(result.ddl_functions(), ["getfoo"]);
    assert_eq!(result.call_functions().len(), 0);
    assert_eq!(result.statement_types(), ["CreateFunctionStmt"]);
}

#[test]
fn it_finds_called_functions() {
    let result = summary("SELECT testfunc(1);", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.functions(), ["testfunc"]);
    assert_eq!(result.ddl_functions().len(), 0);
    assert_eq!(result.call_functions(), ["testfunc"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_dropped_functions() {
    let result = summary("DROP FUNCTION IF EXISTS testfunc(x integer);", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.functions(), ["testfunc"]);
    assert_eq!(result.ddl_functions(), ["testfunc"]);
    assert_eq!(result.call_functions().len(), 0);
    assert_eq!(result.statement_types(), ["DropStmt"]);
}

#[test]
fn it_finds_renamed_functions() {
    let result = summary("ALTER FUNCTION testfunc(integer) RENAME TO testfunc2;", 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["table_name"]);
    assert_eq!(result.select_tables(), ["table_name"]);
    assert_eq!(result.cte_names, ["cte_name"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_nested_tables_in_FROM_clause() {
    let result = summary("select u.* from (select * from users) u", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["users"]);
    assert_eq!(result.select_tables(), ["users"]);
    assert_eq!(result.statement_types(), ["SelectStmt"]);
}

#[test]
fn it_finds_nested_tables_in_WHERE_clause() {
    let result = summary("select users.id from users where 1 = (select count(*) from user_roles)", 0, -1).unwrap();
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
    let result = summary(query, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary("insert into users(pk, name) values (1, 'bob');", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["users"]);

    let result = summary("insert into users(pk, name) select pk, name from other_users;", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    assert_eq!(tables, ["other_users", "users"]);

    let sql = "
        with cte as (
            select pk, name from other_users
        )
        insert into users(pk, name) select * from cte;
    ";
    let result = summary(sql, 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    assert_eq!(tables, ["other_users", "users"]);
    assert_eq!(result.select_tables(), ["other_users"]);
    assert_eq!(result.dml_tables(), ["users"]);
    assert_eq!(result.cte_names, ["cte"]);
    assert_eq!(result.statement_types(), ["InsertStmt", "SelectStmt"]);
}

#[test]
fn it_parses_UPDATE() {
    let result = summary("update users set name = 'bob';", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["users"]);
    assert_eq!(result.statement_types(), ["UpdateStmt"]);

    let result = summary("update users set name = (select name from other_users limit 1);", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    assert_eq!(tables, ["other_users", "users"]);
    assert_eq!(result.statement_types(), ["UpdateStmt", "SelectStmt"]);

    let sql = "
        with cte as (
            select name from other_users limit 1
        )
        update users set name = (select name from cte);
    ";
    let result = summary(sql, 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    assert_eq!(tables, ["other_users", "users"]);
    assert_eq!(result.select_tables(), ["other_users"]);
    assert_eq!(result.dml_tables(), ["users"]);
    assert_eq!(result.cte_names, ["cte"]);
    assert_eq!(result.statement_types(), ["UpdateStmt", "SelectStmt"]);

    let sql = "
        UPDATE users SET name = users_new.name
        FROM users_new
        INNER JOIN join_table ON join_table.user_id = new_users.id
        WHERE users.id = users_new.id
    ";
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary("DELETE FROM users;", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.tables(), ["users"]);
    assert_eq!(result.dml_tables(), ["users"]);
    assert_eq!(result.statement_types(), ["DeleteStmt"]);

    let sql = "
        DELETE FROM users USING foo
        WHERE foo_id = foo.id AND foo.action = 'delete';
    ";
    let result = summary(sql, 0, -1).unwrap();
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
    let result = summary(sql, 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    let tables: Vec<String> = sorted(result.tables()).collect();
    assert_eq!(tables, ["foo", "users"]);
    assert_eq!(result.dml_tables(), ["users"]);
    assert_eq!(result.select_tables(), ["foo"]);
    assert_eq!(result.statement_types(), ["DeleteStmt", "SelectStmt"]);
}

#[test]
fn it_parses_DROP_TYPE() {
    let result = summary("DROP TYPE IF EXISTS repack.pk_something", 0, -1).unwrap();
    assert_eq!(result.warnings.len(), 0);
    assert_eq!(result.statement_types(), ["DropStmt"]);

    // TODO: VERIFY THIS IS CORRECT.
    assert_eq!(result.tables().len(), 0);
    assert_eq!(result.aliases.len(), 0);
    assert_eq!(result.cte_names.len(), 0);
    assert_eq!(result.functions.len(), 0);
    assert_eq!(result.filter_columns.len(), 0);
    assert_eq!(result.truncated_query, None);
}
