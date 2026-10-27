#![allow(non_snake_case)]
#![cfg(test)]

use pg_query::{fingerprint, Error, FingerprintOptions, ParserOptions};

#[test]
fn it_can_fingerprint_a_simple_statement() {
    let result = fingerprint("SELECT * FROM contacts.person WHERE id IN (1, 2, 3, 4);", 0, 0).unwrap();
    assert_eq!(result.hex, "5735f5c64dd9f68e");
}

#[test]
fn it_will_error_on_invalid_input() {
    let error = fingerprint("CREATE RANDOM ix_test ON contacts.person;", 0, 0).err().unwrap();
    assert_eq!(error, Error::Parse("syntax error at or near \"RANDOM\"".into()));
}

#[test]
fn it_works_for_multi_statement_queries() {
    let q1 = "SET x=$1; SELECT A";
    let q2 = "SET x=$1; SELECT a";
    assert_eq!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);

    let q1 = "SET x=$1; SELECT A";
    let q2 = "SELECT a";
    assert_ne!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);
}

#[test]
fn it_ignores_column_aliases() {
    let q1 = "SELECT a AS b";
    let q2 = "SELECT a AS c";
    assert_eq!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);

    let q1 = "SELECT a";
    let q2 = "SELECT a AS c";
    assert_eq!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);

    // When a table alias is used, Postgres 18 uses that for the fingerprint instead of the table name
    let q1 = "SELECT * FROM a AS b";
    let q2 = "SELECT * FROM a AS c";
    let q3 = "SELECT * FROM other AS c";
    assert_ne!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);
    assert_eq!(fingerprint(q2, 0, 0).unwrap().hex, fingerprint(q3, 0, 0).unwrap().hex);

    let q1 = "SELECT * FROM a";
    let q2 = "SELECT * FROM a AS c";
    assert_ne!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);

    let q1 = "SELECT * FROM (SELECT * FROM x AS y) AS a";
    let q2 = "SELECT * FROM (SELECT * FROM x AS z) AS b";
    assert_ne!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);

    let q1 = "SELECT a AS b UNION SELECT x AS y";
    let q2 = "SELECT a AS c UNION SELECT x AS z";
    assert_eq!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);
}

// (pending in the Ruby test suite)
// #[test]
// fn it_ignores_aliases_referenced_in_query() {
//     let q1 = "SELECT s1.id FROM snapshots s1";
//     let q2 = "SELECT s2.id FROM snapshots s2";
//     assert_eq!(fingerprint(q1).unwrap().hex, fingerprint(q2).unwrap().hex);
//     let q1 = "SELECT a AS b ORDER BY b";
//     let q2 = "SELECT a AS c ORDER BY c";
//     assert_eq!(fingerprint(q1).unwrap().hex, fingerprint(q2).unwrap().hex);
// }

#[test]
fn it_ignores_param_references() {
    let q1 = "SELECT $1";
    let q2 = "SELECT $2";
    assert_eq!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);
}

#[test]
fn it_ignores_SELECT_target_list_ordering() {
    let q1 = "SELECT a, b FROM x";
    let q2 = "SELECT b, a FROM x";
    assert_eq!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);
    let q1 = "SELECT $1, b FROM x";
    let q2 = "SELECT b, $1 FROM x";
    assert_eq!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);
    let q1 = "SELECT $1, $2, b FROM x";
    let q2 = "SELECT $1, b, $2 FROM x";
    assert_eq!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);

    // Testing uniqueness
    let q1 = "SELECT a, c FROM x";
    let q2 = "SELECT b, a FROM x";
    assert_ne!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);
    let q1 = "SELECT b FROM x";
    let q2 = "SELECT b, a FROM x";
    assert_ne!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);
}

#[test]
fn it_ignores_INSERT_col_ordering() {
    let q1 = "INSERT INTO test (a, b) VALUES ($1, $2)";
    let q2 = "INSERT INTO test (b, a) VALUES ($1, $2)";
    assert_eq!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);

    // Testing uniqueness
    let q1 = "INSERT INTO test (a, c) VALUES ($1, $2)";
    let q2 = "INSERT INTO test (b, a) VALUES ($1, $2)";
    assert_ne!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);
    let q1 = "INSERT INTO test (b) VALUES ($1, $2)";
    let q2 = "INSERT INTO test (b, a) VALUES ($1, $2)";
    assert_ne!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);
}

#[test]
fn it_ignores_IN_list_size() {
    let q1 = "SELECT * FROM x WHERE y IN ($1, $2, $3)";
    let q2 = "SELECT * FROM x WHERE y IN ($1)";
    assert_eq!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);

    let q1 = "SELECT * FROM x WHERE y IN ( $1::uuid, $2::uuid, $3::uuid )";
    let q2 = "SELECT * FROM x WHERE y IN ( $1::uuid )";
    assert_eq!(fingerprint(q1, 0, 0).unwrap().hex, fingerprint(q2, 0, 0).unwrap().hex);
}

#[test]
fn it_works() {
    let result = fingerprint("SELECT 1", 0, 0).unwrap();
    assert_eq!(result.hex, "50fde20626009aba");

    let result = fingerprint("SELECT 2", 0, 0).unwrap();
    assert_eq!(result.hex, "50fde20626009aba");

    let result = fingerprint("SELECT $1", 0, 0).unwrap();
    assert_eq!(result.hex, "50fde20626009aba");

    let result = fingerprint("SELECT 1; SELECT a FROM b", 0, 0).unwrap();
    assert_eq!(result.hex, "3efa3b10d558d06d");

    let result = fingerprint("SELECT COUNT(DISTINCT id), * FROM targets WHERE something IS NOT NULL AND elsewhere::interval < now()", 0, 0).unwrap();
    assert_eq!(result.hex, "26b6553101185d22");

    let result = fingerprint("INSERT INTO test (a, b) VALUES ($1, $2)", 0, 0).unwrap();
    assert_eq!(result.hex, "51e63b8083b48bdd");

    let result = fingerprint("INSERT INTO test (b, a) VALUES ($1, $2)", 0, 0).unwrap();
    assert_eq!(result.hex, "51e63b8083b48bdd");

    let result = fingerprint(
        "INSERT INTO test (a, b) VALUES (ARRAY[$1, $2, $3, $4], $5::timestamptz), (ARRAY[$6, $7, $8, $9], $10::timestamptz), ($11, $12::timestamptz)",
        0,
        0,
    )
    .unwrap();
    assert_eq!(result.hex, "4dfdd5260cac5acf");

    let result = fingerprint("SELECT b AS x, a AS y FROM z", 0, 0).unwrap();
    assert_eq!(result.hex, "1a8bf5d7614de3a5");

    let result = fingerprint("SELECT * FROM x WHERE y = $1", 0, 0).unwrap();
    assert_eq!(result.hex, "4ff39426bd074231");

    let result = fingerprint("SELECT * FROM x WHERE y = ANY ($1)", 0, 0).unwrap();
    assert_eq!(result.hex, "4ff39426bd074231");

    let result = fingerprint("SELECT * FROM x WHERE y IN ($1)", 0, 0).unwrap();
    assert_eq!(result.hex, "4ff39426bd074231");

    let result = fingerprint("SELECT * FROM x WHERE y IN ($1, $2, $3)", 0, 0).unwrap();
    assert_eq!(result.hex, "4ff39426bd074231");

    let result = fingerprint("SELECT * FROM x WHERE y IN ( $1::uuid )", 0, 0).unwrap();
    assert_eq!(result.hex, "4ff39426bd074231");

    let result = fingerprint("SELECT * FROM x WHERE y IN ( $1::uuid, $2::uuid, $3::uuid )", 0, 0).unwrap();
    assert_eq!(result.hex, "4ff39426bd074231");

    let result = fingerprint("PREPARE a123 AS SELECT a", 0, 0).unwrap();
    assert_eq!(result.hex, "9b5e6ead8be993e8");

    let result = fingerprint("EXECUTE a123", 0, 0).unwrap();
    assert_eq!(result.hex, "44ef1d2beabd53e8");

    let result = fingerprint("DEALLOCATE a123", 0, 0).unwrap();
    assert_eq!(result.hex, "d8a65a814fbc5f95");

    let result = fingerprint("DEALLOCATE ALL", 0, 0).unwrap();
    assert_eq!(result.hex, "2debfb8745df64a7");

    let result = fingerprint("EXPLAIN ANALYZE SELECT a", 0, 0).unwrap();
    assert_eq!(result.hex, "82845c1b5c6102e5");

    let result = fingerprint("WITH a AS (SELECT * FROM x WHERE x.y = $1 AND x.z = 1) SELECT * FROM a", 0, 0).unwrap();
    assert_eq!(result.hex, "6831e38bbb3dd18c");

    let result = fingerprint(
        "CREATE TABLE types (a float(2), b float(49), c NUMERIC(2, 3), d character(4), e char(5), f varchar(6), g character varying(7))",
        0,
        0,
    )
    .unwrap();
    assert_eq!(result.hex, "008d6ba4aa0f4c6e");

    let result =
        fingerprint("CREATE VIEW view_a (a, b) AS WITH RECURSIVE view_a (a, b) AS (SELECT * FROM a(1)) SELECT \"a\", \"b\" FROM \"view_a\"", 0, 0)
            .unwrap();
    assert_eq!(result.hex, "c6ef6b9f498feda4");

    let result = fingerprint("VACUUM FULL my_table", 0, 0).unwrap();
    assert_eq!(result.hex, "fdf2f4127644f4d8");

    let result = fingerprint("SELECT * FROM x AS a, y AS b", 0, 0).unwrap();
    assert_eq!(result.hex, "675cc4043bec7035");

    let result = fingerprint("SELECT * FROM y AS a, x AS b", 0, 0).unwrap();
    assert_eq!(result.hex, "675cc4043bec7035");

    let result = fingerprint("SELECT x AS a, y AS b FROM x", 0, 0).unwrap();
    assert_eq!(result.hex, "65dff5f5e9a643ad");

    let result = fingerprint("SELECT y AS a, x AS b FROM x", 0, 0).unwrap();
    assert_eq!(result.hex, "65dff5f5e9a643ad");

    let result = fingerprint("SELECT x, y FROM z", 0, 0).unwrap();
    assert_eq!(result.hex, "330267237da5535f");

    let result = fingerprint("SELECT y, x FROM z", 0, 0).unwrap();
    assert_eq!(result.hex, "330267237da5535f");

    let result = fingerprint("INSERT INTO films (code, title, did) VALUES ('UA502', 'Bananas', 105), ('T_601', 'Yojimbo', DEFAULT)", 0, 0).unwrap();
    assert_eq!(result.hex, "459fdc70778b841e");

    let result = fingerprint("INSERT INTO films (code, title, did) VALUES ($1, $2, $3)", 0, 0).unwrap();
    assert_eq!(result.hex, "459fdc70778b841e");

    let result = fingerprint("SELECT * FROM a", 0, 0).unwrap();
    assert_eq!(result.hex, "fcf44da7b597ef43");

    let result = fingerprint("SELECT * FROM a AS b", 0, 0).unwrap();
    assert_eq!(result.hex, "579efc630b5991eb");

    let result = fingerprint("UPDATE users SET one_thing = $1, second_thing = $2 WHERE users.id = $1", 0, 0).unwrap();
    assert_eq!(result.hex, "a0ea386c1cfd1e69");

    let result = fingerprint("UPDATE users SET something_else = $1 WHERE users.id = $1", 0, 0).unwrap();
    assert_eq!(result.hex, "3172bc3e0d631d55");

    let result = fingerprint("UPDATE users SET something_else = (SELECT a FROM x WHERE uid = users.id LIMIT 1) WHERE users.id = $1", 0, 0).unwrap();
    assert_eq!(result.hex, "f1127a8b91fbecbf");

    let result = fingerprint("SAVEPOINT some_id", 0, 0).unwrap();
    assert_eq!(result.hex, "8ebd566ea1bf947b");

    let result = fingerprint("RELEASE some_id", 0, 0).unwrap();
    assert_eq!(result.hex, "60d618658252d2af");

    let result = fingerprint("PREPARE TRANSACTION 'some_id'", 0, 0).unwrap();
    assert_eq!(result.hex, "d993959a33d627d4");

    let result = fingerprint("START TRANSACTION READ WRITE", 0, 0).unwrap();
    assert_eq!(result.hex, "d2bd82412b9a616d");

    let result = fingerprint("DECLARE cursor_123 CURSOR FOR SELECT * FROM test WHERE id = 123", 0, 0).unwrap();
    assert_eq!(result.hex, "d2bec62d2a7ec7cb");

    let result = fingerprint("FETCH 1000 FROM cursor_123", 0, 0).unwrap();
    assert_eq!(result.hex, "37f4d2f6a957ae48");

    let result = fingerprint("CLOSE cursor_123", 0, 0).unwrap();
    assert_eq!(result.hex, "2c7963684fc2bad9");

    let result = fingerprint("-- nothing", 0, 0).unwrap();
    assert_eq!(result.hex, "d8d13f8b2da6c9ad");

    let result = fingerprint("CREATE FOREIGN TABLE ft1 () SERVER no_server", 0, 0).unwrap();
    assert_eq!(result.hex, "74481c4af7c76be1");

    let result = fingerprint("UPDATE x SET a = 1, b = 2, c = 3", 0, 0).unwrap();
    assert_eq!(result.hex, "fd5c248c0e642ce4");

    let result = fingerprint("UPDATE x SET z = now()", 0, 0).unwrap();
    assert_eq!(result.hex, "a222eaabaa1e7cb1");

    let result = fingerprint("CREATE TEMPORARY TABLE my_temp_table (test_id integer NOT NULL) ON COMMIT DROP", 0, 0).unwrap();
    assert_eq!(result.hex, "a6d58d968d06bbad");

    let result = fingerprint("CREATE TEMPORARY TABLE my_temp_table AS SELECT 1", 0, 0).unwrap();
    assert_eq!(result.hex, "695ebe73a3abc45c");

    let result = fingerprint("SELECT INTERVAL (0) $2", 0, 0).unwrap();
    assert_eq!(result.hex, "50fde20626009aba");

    let result = fingerprint("SELECT INTERVAL (2) $2", 0, 0).unwrap();
    assert_eq!(result.hex, "50fde20626009aba");

    let result = fingerprint("SELECT * FROM t WHERE t.a IN (1, 2) AND t.b = 3", 0, 0).unwrap();
    assert_eq!(result.hex, "346aea01be9173b6");

    let result = fingerprint("SELECT * FROM t WHERE t.b = 3 AND t.a IN (1, 2)", 0, 0).unwrap();
    assert_eq!(result.hex, "346aea01be9173b6");

    let result = fingerprint("SELECT * FROM t WHERE a && '[1,2]'", 0, 0).unwrap();
    assert_eq!(result.hex, "673f199f13dfe665");

    let result = fingerprint("SELECT * FROM t WHERE a && '[1,2]'::int4range", 0, 0).unwrap();
    assert_eq!(result.hex, "673f199f13dfe665");

    let result = fingerprint("SELECT * FROM t_20210301_x", 0, 0).unwrap();
    assert_eq!(result.hex, "6f8169980cd70a25");

    let result = fingerprint("SELECT * FROM t_20210302_x", 0, 0).unwrap();
    assert_eq!(result.hex, "6f8169980cd70a25");

    let result = fingerprint("SELECT * FROM t_20210302_y", 0, 0).unwrap();
    assert_eq!(result.hex, "d357dac4a24fcf1b");

    let result = fingerprint("SELECT * FROM t_1", 0, 0).unwrap();
    assert_eq!(result.hex, "018bd9230646143e");

    let result = fingerprint("SELECT * FROM t_2", 0, 0).unwrap();
    assert_eq!(result.hex, "3f1444da570c1a66");
}

#[test]
fn it_matches_the_readme_pg17_compat_example() {
    // mirrors the example in the libpg_query README
    let q1 = "SELECT * FROM public.users u";
    let q2 = "SELECT * FROM myschema.users u";
    assert_eq!(fingerprint(q1, 0, 0).unwrap().hex, "6640d8be64880eed");
    assert_eq!(fingerprint(q2, 0, 0).unwrap().hex, "6640d8be64880eed");
    assert_eq!(fingerprint(q1, 0, FingerprintOptions::RANGEVAR_PG17_COMPAT).unwrap().hex, "d3198777453aef12");
    assert_eq!(fingerprint(q2, 0, FingerprintOptions::RANGEVAR_PG17_COMPAT).unwrap().hex, "91e880cf13f4f219");
}

#[test]
fn it_fingerprints_using_parse_modes() {
    let cases = [
        ("SELECT 1", ParserOptions::DEFAULT, "50fde20626009aba"),
        ("integer", ParserOptions::TYPE_NAME, "c71927729d707de5"),
        ("character varying(32)", ParserOptions::TYPE_NAME, "453ab4df8fd3eea9"),
        ("EXISTS(SELECT 1)", ParserOptions::PLPGSQL_EXPR, "976a797c8ca2985b"),
        ("v_version IS NULL", ParserOptions::PLPGSQL_EXPR, "6b292a26fcb78b1c"),
        ("pos:= instr($1, $2, 1)", ParserOptions::PLPGSQL_ASSIGN1, "786552659cc61f6f"),
        ("temp_str := substring(string FROM beg_index)", ParserOptions::PLPGSQL_ASSIGN1, "d6671d73f1654866"),
        ("v3.c1 := 4", ParserOptions::PLPGSQL_ASSIGN2, "a8c86658ce26a653"),
        ("NEW.name = upper(cleanString(NEW.name))", ParserOptions::PLPGSQL_ASSIGN2, "bb52450e4d46f7a1"),
        ("NEW.author.first_name = upper(cleanString(NEW.author.first_name))", ParserOptions::PLPGSQL_ASSIGN3, "a148e3f78b53c252"),
    ];
    for (query, parser_options, expected) in cases {
        let result = fingerprint(query, parser_options, 0).unwrap();
        assert_eq!(result.hex, expected, "unexpected fingerprint for {:?}", query);
    }
}

#[test]
fn it_fingerprints_using_fingerprint_options() {
    // By default, 2+ consecutive digits in the relation name are ignored (these two match)
    let query = "SELECT * FROM orders_2024_01";
    let expected = "0e612f391ad711b8";
    assert_eq!(fingerprint(query, 0, 0).unwrap().hex, expected);
    assert_eq!(fingerprint("SELECT * FROM orders_2024_02", 0, 0).unwrap().hex, expected);

    // With FULL_RELNAME the full relation name is fingerprinted (these two differ)
    assert_eq!(fingerprint(query, 0, FingerprintOptions::FULL_RELNAME).unwrap().hex, "3cc2d1ca3f22c9bf");
    assert_eq!(fingerprint("SELECT * FROM orders_2024_02", 0, FingerprintOptions::FULL_RELNAME).unwrap().hex, "291f96ac98cf4c38");

    // By default (Postgres 18+ behaviour) the alias replaces the relation name and schema names are ignored
    assert_eq!(fingerprint("SELECT * FROM public.sales", 0, 0).unwrap().hex, "4d93c901b91cb364");
    assert_eq!(fingerprint("SELECT * FROM sales s", 0, 0).unwrap().hex, "93a3bbe18171c380");

    // With RANGEVAR_IGNORE_ALIASES aliases are ignored, matching the unqualified reference above
    assert_eq!(fingerprint("SELECT * FROM sales s", 0, FingerprintOptions::RANGEVAR_IGNORE_ALIASES).unwrap().hex, "4d93c901b91cb364");

    // With RANGEVAR_INCLUDE_SCHEMA the schema name is fingerprinted, whilst aliases still replace the relation name
    assert_eq!(fingerprint("SELECT * FROM public.sales", 0, FingerprintOptions::RANGEVAR_INCLUDE_SCHEMA).unwrap().hex, "78d676e53f612747");
    assert_eq!(fingerprint("SELECT * FROM public.sales s", 0, FingerprintOptions::RANGEVAR_INCLUDE_SCHEMA).unwrap().hex, "9cf22829ca3b350b");

    // RANGEVAR_PG17_COMPAT matches the fingerprint from libpg_query 17
    let query = "MERGE into measurement m USING new_measurement nm ON (m.city_id = nm.city_id and m.logdate=nm.logdate) WHEN MATCHED AND nm.peaktemp IS NULL THEN DELETE WHEN MATCHED THEN UPDATE SET peaktemp = greatest(m.peaktemp, nm.peaktemp), unitsales = m.unitsales + coalesce(nm.unitsales, 0) WHEN NOT MATCHED THEN INSERT (city_id, logdate, peaktemp, unitsales) VALUES (city_id, logdate, peaktemp, unitsales)";
    assert_eq!(fingerprint(query, 0, FingerprintOptions::RANGEVAR_PG17_COMPAT).unwrap().hex, "fe086f143f4c7ed9");

    // All flags combined
    let all = FingerprintOptions::RANGEVAR_PG17_COMPAT | FingerprintOptions::FULL_RELNAME;
    assert_eq!(all.bits(), 19);
    assert_eq!(fingerprint("SELECT * FROM public.orders_2024_01 o", 0, all).unwrap().hex, "115077f8a9c3c10d");
}
