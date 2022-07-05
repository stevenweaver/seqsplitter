use serde_json::Value;
use std::fs;
use std::process::Command;

#[test]
fn test_report() {
    assert!(Command::new("bash")
        .arg("-c")
        .arg("target/debug/seqsplitter -q tests/resources/example.fastqsanger > /tmp/report.json")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success());

    let result = fs::read_to_string("/tmp/report.json").unwrap();
    let expected = include_str!("expected/report.json");

    let v: Value = serde_json::from_str(&result).unwrap();
    let w: Value = serde_json::from_str(&expected).unwrap();

    assert_eq!(v["complexity"]["value"], w["complexity"]["value"]);
}
