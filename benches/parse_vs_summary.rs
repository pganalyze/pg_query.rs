use brunch::Bench;
use pg_query;

brunch::benches!(
    Bench::new("parse").run_seeded_with(seed, |query| pg_query::parse(&query)),
    Bench::new("summary").run_seeded_with(seed, |query| pg_query::summary(&query, -1)),
    // I had to be less mean just so the parse+truncate one didn't crash.
    Bench::new("parse + truncate").run_seeded_with(less_mean_seed, |query| pg_query::parse(&query).unwrap().truncate(50).unwrap()),
    Bench::new("summary + truncate").run_seeded_with(less_mean_seed, |query| pg_query::summary(&query, 50)),
);

fn less_mean_seed() -> String {
    build_query(30)
}

fn seed() -> String {
    build_query(500)
}

fn build_query(table_references: i32) -> String {
    let mut query = "SELECT * FROM t".to_string();
    for i in 0..table_references {
        query = format!("{query} JOIN t{i} ON t.id = t{i}.t_id AND t{i}.k IN (1, 2, 3, 4) AND t{i}.f IN (SELECT o FROM p WHERE q = 'foo')");
    }
    query
}
