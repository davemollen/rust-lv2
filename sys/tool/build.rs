use std::{fs, path::Path};

fn create_new_libmodla_header_without_pragmas(lib_path: &str) {
    let include_dir = Path::new(lib_path);
    let input_header = include_dir.join("libmodla.h");
    let output_header = include_dir.join("libmodla-clean.h");
    let content = fs::read_to_string(&input_header).expect("Failed to read libmodla.h");

    // Remove the visibility pragmas
    let cleaned = content
        .lines()
        .filter(|line| {
            let trimmed_line = line.trim();
            trimmed_line != "#pragma GCC visibility push(hidden)"
                && trimmed_line != "#pragma GCC visibility pop"
        })
        .collect::<Vec<_>>()
        .join("\n");

    // Write the output header
    fs::write(&output_header, cleaned).expect("Failed to write libmodla-clean.h");
}

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let mod_license_lib_path = format!("{}/../lv2/mod-license", manifest_dir);
    create_new_libmodla_header_without_pragmas(&mod_license_lib_path);
}
