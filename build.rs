// Custom build
use cfg_if::cfg_if;

fn main() {
	cfg_if! {
		if #[cfg(feature = "lovr-modules")] {
			use cmake::Config;

			// TODO: Recursively include all C files.
			println!("cargo:rerun-if-changed=lovr/CMakeLists.txt");
			// Use the `cc` crate to build a C file and statically link it.
			let lovr = Config::new("lovr")
							  .build_target("lovr")
							  .define("LOVR_ENABLE_PHYSICS", "NO")
							  .build();
		}
	}
}