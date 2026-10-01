# Changelog

## 18.0.0   2026-09-01

* Upgrade to Postgres 18.6 (libpg_query 18.1.0)
  - Performance: libpg_query now uses upb instead of protobuf-c for
    serialization, which is substantially faster thanks to its built-in arena
    allocation, resulting in 2x faster runtime.
  - Robustness: overly deep queries now return a "stack depth limit exceeded"
    error instead of crashing the process
    ([#348](https://github.com/pganalyze/libpg_query/pull/348)), and the
    deparser strictly checks unexpected pointer values in input parse trees
    instead of crashing
  - Parser: comments are now ignored when parsing queries (they are only
    significant for scanning), fixing parse errors for tokens like
    `NOT /* comment */ IN`
    ([#378](https://github.com/pganalyze/libpg_query/pull/378));
    PL/pgSQL statements without bodies now return an error instead of
    crashing ([#363](https://github.com/pganalyze/libpg_query/pull/363))
  - Parser: PL/pgSQL parsing reworked to use proper type definitions
    (Postgres 18.0.0), replacing the previous ad-hoc approach
  - Deparser: parentheses are now added based on operator precedence, fixing
    cases where deparsed SQL changed meaning or was invalid
    ([#371](https://github.com/pganalyze/libpg_query/pull/371))
  - Deparser: comment handling reworked for Postgres 18 multi-statement
    strings: comments between statements are no longer attributed to the
    following statement (relevant for `deparse_comments_for_query`)
  - `normalize`: support `NOTIFY` statements
    ([#340](https://github.com/pganalyze/libpg_query/pull/340)) and fix
    handling of `U&` special constants in `DefElem` nodes
    ([#347](https://github.com/pganalyze/libpg_query/pull/347))
  - `summary`: fix a relation going missing when a CTE shares its name
    ([#367](https://github.com/pganalyze/libpg_query/pull/367)) and fix a
    memory leak when the tree walk throws an error
* **Breaking**: `parse`, `fingerprint` and `summary` now require parser/fingerprint
  options arguments, corresponding to the C `pg_query_parse_opts`,
  `pg_query_fingerprint_opts` and `pg_query_summary` APIs. Options accept
  `ParserOptions`/`FingerprintOptions` constants or raw integers; pass `0`
  for the defaults:
  - `parse(statement, parser_options)`
  - `fingerprint(statement, parser_options, fingerprint_options)`
  - `summary(statement, parser_options, truncate_limit)`
* **Breaking**: `deparse` now requires a `DeparseOptions` argument, and all
  `deparse` convenience methods (`ParseResult::deparse`,
  `protobuf::ParseResult::deparse`, `Node::deparse`, `NodeEnum::deparse`,
  `NodeRef::deparse`, `NodeMut::deparse`) likewise require it, mirroring the
  C `PostgresDeparseOpts` struct (comments, pretty_print, indent_size,
  max_line_length, trailing_newline, commas_start_of_line); pass
  `Default::default()` for the previous plain behavior
* Fingerprint values changed: fingerprinting now follows Postgres 18
  query ID behavior. In SELECT/DML statements the relation alias (if present)
  replaces the relation name and schema names are ignored; sequences of 2+ digits
  in relation names remain ignored by default. Additionally, `BEGIN`/`START
  TRANSACTION` options now affect fingerprints
  ([#358](https://github.com/pganalyze/libpg_query/pull/358)), while
  `NOTIFY` payloads ([#353](https://github.com/pganalyze/libpg_query/pull/353))
  and role names (e.g. in `CREATE ROLE`/`GRANT`
  [#357](https://github.com/pganalyze/libpg_query/pull/357)) are now
  ignored. Set operation chains deeper than 100 levels are now cut off
  consistently with other deeply nested nodes. Use
  `FingerprintOptions::RANGEVAR_PG17_COMPAT` for Postgres 17 compatible
  fingerprints. See the "Fingerprinting a query" section in the README for
  details and examples.
* **Breaking**: protobuf message structure follows the Postgres 18 parse tree;
  this is an incomplete list of notable changes:
  - `SinglePartitionSpec` message removed (and its variants from `NodeEnum`,
    `NodeRef` and `NodeMut`)
  - `InsertStmt`/`UpdateStmt`/`DeleteStmt`/`MergeStmt`: `returning_list:
    Vec<Node>` replaced by `returning_clause: Option<ReturningClause>`; new
    `ReturningClause`, `ReturningOption` and `ReturningExpr` messages plus
    `ReturningOptionKind` and `VarReturningType` enums support the new PG18
    `RETURNING OLD AS ... NEW AS ...` syntax (`Query` gains
    `returning_old_alias`/`returning_new_alias`, `Var` gains `varreturningtype`)
  - `Constraint`: `inhcount` removed; `is_enforced`, `generated_kind` and
    `without_overlaps` added
  - `RowCompareType` enum replaced by `CompareType` (with new range comparison
    variants)
  - `DefElem` gains `arg_location` (constant locations are now recorded by the
    parser rather than searched in the query text)
  - Many field tags shifted as fields were added/removed; serialized protobuf
    messages are not compatible across Postgres major versions
* New: `deparse_comments_for_query` extracts comments from a query with
  insertion metadata, for re-insertion via `DeparseOptions::comments`
* New: `scan_tokens` exposes `pg_query_scan_tokens`, returning scanner tokens
  directly as `Vec<protobuf::ScanToken>` without protobuf serialization
* New: `is_utility_stmt` exposes `pg_query_is_utility_stmt`, returning a
  boolean per statement indicating whether it is a utility statement
  (via the new `Error::IsUtility` variant on parse errors)

## 6.2.1   2026-09-30

* Upgrade to libpg_query 17-6.2.5
* Security fix: Heap out-of-bounds write and read in pg_query_normalize ([GHSA-6ggm-xmc9-8ffg](https://github.com/pganalyze/libpg_query/security/advisories/GHSA-6ggm-xmc9-8ffg))
* Deparser:
  - Add strict checking for unexpected pointer values
  - Preserve parentheses around subscripted array constructors
    - This prevents `(ARRAY[...])[...]` from being deparsed as invalid SQL
  - Fix handling of constraint key named `value` in `ALTER TABLE`
* pg_query_normalize:
  - Add support for `NOTIFY` statements
  - Avoid undefined behaviour for overly large parameter references

## 6.2.0   2026-07-29

* Upgrade to libpg_query 17-6.2.2
* Add `pg_query::summary` function
  - This uses the new `pg_query_summary` C function that significantly improves performance when
    you need metadata (like a list of referenced tables) but don't need the full parse tree.
* `NodeEnum`: Improve performance when iterating over parse tree using `nodes` and `nodes_mut`
* Fix build caching issues in `build.rs` script

## 6.1.1   2025-08-22

* `NodeEnum`: Support `MERGE` queries
* `NodeEnum`: Support `CALL fn()` queries
* `NodeEnum`: Iterate over `IndexElem` nodes
* Derive `serde::Serialize` for `protobuf::ParseResult`

## 6.1.0   2025-04-02

* Upgrade to libpg_query 17-6.1.0
  - Update to Postgres 17.4, and add recent patches scheduled for Postgres 17.5 (not yet released)
    - Notably, this pulls in support for macOS 15.4 which defines strchrnul
      in its standard library, fixing builds on up-to-date macOS versions.
  - Deparser improvements
    - Add parenthesis around AT LOCAL / AT TIMEZONE if needed
    - Correctness improvements related to expressions and function calls
* Upgrade prost dependency to fix build issues on Linux
* `ParseResult`: make `tables` and `functions` fields public

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
