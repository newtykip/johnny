use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        tui: { feature = "tui" },
        verbose: { feature = "verbose" }
    }
}
