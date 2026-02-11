# Fuzzing pg_query

This directory contains fuzzing harnesses using [honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs).

## Prerequisites

Install honggfuzz:

```bash
cargo install honggfuzz
```

You also need honggfuzz system dependencies. On Ubuntu/Debian:

```bash
sudo apt-get install build-essential binutils-dev libunwind-dev libblocksruntime-dev liblzma-dev
```

## Running the fuzzer

From the repository root:

```bash
cargo hfuzz run fuzz_parse
```

The fuzzer will run indefinitely, saving crashes to `hfuzz_workspace/fuzz_parse/`.

## Fuzzing targets

The `fuzz_parse` harness tests the following functions with random byte input:

- `pg_query::parse()` - Main SQL parser
- `pg_query::normalize()` - Query normalization
- `pg_query::fingerprint()` - Query fingerprinting
- `pg_query::scan()` - SQL tokenizer
- `pg_query::split_with_parser()` - Statement splitting (strict)
- `pg_query::split_with_scanner()` - Statement splitting (lenient)
- `pg_query::summary()` - Fast summary extraction

When parsing succeeds, it also tests:
- `ParseResult::deparse()` - Round-trip back to SQL
- `ParseResult::tables()` - Table extraction
- `ParseResult::functions()` - Function extraction
- `ParseResult::statement_types()` - Statement classification
- `ParseResult::truncate()` - Query truncation

## Reproducing crashes

To reproduce a crash from a saved input file:

```bash
cargo hfuzz run-debug fuzz_parse hfuzz_workspace/fuzz_parse/SIGABRT.PC.*.fuzz
```
