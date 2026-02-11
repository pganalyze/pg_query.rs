use honggfuzz::fuzz;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            // Convert bytes to string, skipping invalid UTF-8
            let Ok(input) = std::str::from_utf8(data) else {
                return;
            };

            // Fuzz the main parse function
            let _ = pg_query::parse(input);

            // Fuzz normalize
            let _ = pg_query::normalize(input);

            // Fuzz fingerprint
            let _ = pg_query::fingerprint(input);

            // Fuzz scanner
            let _ = pg_query::scan(input);

            // Fuzz statement splitting
            let _ = pg_query::split_with_parser(input);
            let _ = pg_query::split_with_scanner(input);

            // Fuzz summary
            let _ = pg_query::summary(input, 100);

            // If parse succeeds, also test deparse round-trip
            if let Ok(result) = pg_query::parse(input) {
                let _ = result.deparse();
                let _ = result.tables();
                let _ = result.functions();
                let _ = result.statement_types();
                let _ = result.truncate(50);
            }
        });
    }
}
