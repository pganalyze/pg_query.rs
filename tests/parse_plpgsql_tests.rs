#[test]
fn it_can_parse_a_simple_function() {
    let result = pg_query::parse_plpgsql(
        " \
        CREATE OR REPLACE FUNCTION cs_fmt_browser_version(v_name varchar, v_version varchar) \
        RETURNS varchar AS $$ \
        BEGIN \
            IF v_version IS NULL THEN \
                RETURN v_name; \
            END IF; \
            RETURN v_name || '/' || v_version; \
        END; \
        $$ LANGUAGE plpgsql;",
    );
    assert!(result.is_ok());
    let result = result.unwrap();
    let expected = include_str!("data/simple_plpgsql.json");
    assert_eq!(serde_json::to_string_pretty(&result).unwrap(), expected);
}

#[test]
fn it_will_error_on_invalid_input() {
    let result = pg_query::parse_plpgsql("CREATE RANDOM ix_test ON contacts.person;");
    assert!(result.is_err());
    assert_eq!(
        result.err().unwrap(),
        pg_query::Error::Parse("syntax error at or near \"RANDOM\"".into())
    );
}
