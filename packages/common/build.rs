use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        johnny: { feature = "johnny" },
        db: { feature = "db" },
        tui: { feature = "tui" }
    }
}
