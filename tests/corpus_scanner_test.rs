use mao_agent::CorpusScanner;
use std::fs;

#[test]
fn test_scanner_excludes_raw_subdir() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();

    // Create directory structure:
    //   hist/01.md          ← should be scanned
    //   hist/raw/ignored.md ← should NOT be scanned (raw/ exclusion)
    let hist_dir = root.join("hist");
    let raw_dir = hist_dir.join("raw");
    fs::create_dir_all(&raw_dir).unwrap();

    fs::write(hist_dir.join("01.md"), "# Test\ncontent").unwrap();
    fs::write(raw_dir.join("ignored.md"), "# Raw\nshould be excluded").unwrap();

    let result = CorpusScanner::scan_dir(root).unwrap();

    assert_eq!(
        result.len(),
        1,
        "Expected 1 file (only 01.md), got {}: {:?}",
        result.len(),
        result
    );
    let name = result[0].file_name().unwrap().to_str().unwrap();
    assert_eq!(name, "01.md");
}
