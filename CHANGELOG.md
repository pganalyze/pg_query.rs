# Changelog

## 6.0.0   2024-11-26

* Upgrade to libpg_query 17-6.0.0
  - Updates to the Postgres 17 parser
  - Deparser improvements:
    - Add support for deparsing `JSON_TABLE`, `JSON_QUERY`, `JSON_EXISTS`, `JSON_VALUE`
    - Add support for deparsing `JSON`, `JSON_SCALAR`, `JSON_SERIALIZE`
    - Add support for deparsing `COPY ... FORCE_NULL(*)`
    - Add support for deparsing `ALTER COLUMN ... SET EXPRESSION AS`
    - Add support for deparsing `SET STATISTICS DEFAULT`
    - Add support for deparsing `SET ACCESS METHOD DEFAULT`
    - Add support for deparsing `... AT LOCAL`
    - Add support for deparsing `merge_action()`
    - Add support for deparsing `MERGE ... RETURNING`
    - Add support for deparsing `NOT MATCHED [ BY TARGET ]`

## 5.1.1    2024-10-30

* Make `ParseResult` struct public and implement `Debug`

## 5.1.0    2024-01-09

* Update to libpg_query 16-5.1.0
  - Add support for running on Windows
  - Add support for compiling on 32-bit systems
* Always build C library using "cc" crate
* Add `filter_columns` for getting columns that a query filters by
  - This returns the table name (if present) and column name for every
    column that's referenced in a JOIN or WHERE clause.


## 5.0.0    2023-12-22

* Align versioning scheme with that of other pg_query libraries
  (which is to generally aim to match the libpg_query version)
* Upgrade to libpg_query 5.0.0
  - Updates to the Postgres 16 parser
  - Multiple deparser improvements


## 0.8.2    2023-09-11

* Update bindgen to 0.66.1 to remove transitive dependency on atty and resolve build errors [#28](https://github.com/pganalyze/pg_query.rs/pull/28)

## 0.8.1    2023-08-07

* Upgrade to libpg_query 4.2.3
  - Fix builds when compiling with `glibc >=  2.38` [libpg_query#203](https://github.com/pganalyze/libpg_query/pull/203)
  - Deparser: Add support for COALESCE and other expressions in LIMIT clause [libpg_query#199](https://github.com/pganalyze/libpg_query/pull/199)

## 0.8.0    2023-07-25

* Upgrade to libpg_query 4.2.2 (Postgres 13 -> 15)
* Improve `ParseResult::tables()` to find tables in `cast` expressions

## 0.7.0     2022-07-19

* Adds ParseResult struct with convenience functions to get table and function references
* Adds ability to deparse a mutated query AST back into a string
* Adds context-aware query truncation
* Adds Ruby test suite to ensure feature parity
* Adds ability to split multi-query strings ([#6](https://github.com/pganalyze/pg_query.rs/pull/6))
* Fixes memory leaks in fingerprint and normalize ([#8](https://github.com/pganalyze/pg_query.rs/pull/8))

## 0.6.0 and earlier

This crate was previously maintained by @paupino, who now maintains a slimmed down crate: https://github.com/paupino/pg_parse
