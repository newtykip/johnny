use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        // db
        db: { feature = "db"},
        mysql: { all(db, feature = "mysql") },
        postgres: { all(db, feature = "postgres") },
        sqlite: { all(feature = "sqlite") },
    }
}
