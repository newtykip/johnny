let build = variable::get("build");

if build {
	file::delete("Cargo.toml");
	file::rename("Cargo.toml.build", "Cargo.toml");
} else {
	file::delete("Cargo.toml.build");
}
