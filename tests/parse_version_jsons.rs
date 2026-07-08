use std::fs;
use std::path::Path;

#[test]
fn parse_all_version_jsons() {
    let test_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");

    let mut entries: Vec<_> = fs::read_dir(&test_dir)
        .expect("Failed to read tests directory")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
        .collect();

    entries.sort_by_key(|e| e.path());

    let mut failures = Vec::new();

    for entry in &entries {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("  ✗ FAILED: {} — could not read file: {}", file_name, e);
                failures.push(file_name);
                continue;
            }
        };

        match serde_json::from_str::<mine_data_structs::minecraft::Root>(&content) {
            Ok(_) => println!("  ✓ {} parsed successfully", file_name),
            Err(e) => {
                eprintln!("  ✗ FAILED: {}", file_name);
                eprintln!("    Error class: {:?}", e.classify());
                eprintln!("    Message:     {}", e);
                eprintln!("    Line:        {}", e.line());
                eprintln!("    Column:      {}", e.column());
                failures.push(file_name);
            }
        }
    }

    assert!(
        failures.is_empty(),
        "\n\n❌ The following version JSON(s) failed to parse:\n    {}\n",
        failures.join("\n    "),
    );
}
