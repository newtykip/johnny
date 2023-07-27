clean:
	cargo clean-recursive

format-toml:
	taplo fmt *.toml template/*.toml packages/**/*.toml config.toml.example

format-code:
	cargo fmt --all

format: format-toml format-code

new-package:
	cd packages && cargo generate --path ../template

new-migration name:
	sea-orm-cli migrate generate {{name}}

build-all:
	cargo build-all-features
