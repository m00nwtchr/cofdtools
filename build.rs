use std::process::Command;

fn pnpm_installed() -> bool {
	return Command::new("pnpm").arg("-v").status().unwrap().success();
}

fn main() {
	println!("cargo:rerun-if-changed=./src");
	println!("cargo:rerun-if-changed=input.css");
	println!("cargo:rerun-if-changed=tailwind.config.js");

	if pnpm_installed() {
		// Compile TailwindCSS .css file
		Command::new("pnpm")
			.args([
				"dlx",
				"tailwindcss",
				"-i",
				"input.css",
				"-c",
				"tailwind.config.js",
				"-o",
				"public/tailwind.css",
				"--minify",
			])
			.current_dir(std::env!("CARGO_MANIFEST_DIR"))
			.env("NODE_ENV", "production")
			.spawn()
			.unwrap();
	}
}
