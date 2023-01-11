# Changelog

## Unreleased

* Upgrade to PostgreSQL 15.1 via libpg_query 4.0.0
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
