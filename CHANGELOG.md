# Changelog

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
