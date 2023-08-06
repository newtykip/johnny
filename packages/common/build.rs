use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        johnny: { feature = "johnny" },
        tui: { feature = "tui" },

        autorole: { feature = "autorole" },
        sticky: { feature = "sticky" },

        db: { feature = "db" },
        db_events: { feature = "db-events" },

        mysql: { feature = "mysql" },
        postgres: { feature = "postgres" },
        sqlite: { feature = "sqlite" }
    }
}
