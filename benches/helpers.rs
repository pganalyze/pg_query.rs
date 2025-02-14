use std::ffi::CString;

pub fn build_query(table_references: i32) -> String {
    let mut query = "SELECT * FROM t".to_string();
    for i in 0..table_references {
        query = format!("{query} JOIN t{i} ON t.id = t{i}.t_id AND t{i}.k IN (1, 2, 3, 4) AND t{i}.f IN (SELECT o FROM p WHERE q = 'foo')");
    }
    query
}

pub fn seed() -> String {
    build_query(100)
}

pub fn c_seed() -> CString {
    CString::new(seed()).unwrap()
}
