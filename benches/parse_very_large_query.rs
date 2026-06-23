use brunch::Bench;
use std::hint::black_box;
use std::time::Duration;

brunch::benches! {
    Bench::new("parse large query")
        .with_timeout(Duration::from_secs(180))
        .run_seeded_with(build_large_query, |sql| pg_query::parse(black_box(&*sql)).unwrap()),
}

fn build_large_query() -> String {
    let ids = (0..1_300_000).map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    format!("SELECT COUNT(*) FROM users WHERE id IN ({ids})")
}
