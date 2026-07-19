use std::path::PathBuf;
use std::process::{Command, Output};

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_kmeansn"))
}

fn example(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(name)
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn version_flag_works() {
    let output = bin().arg("--version").output().unwrap();
    assert!(output.status.success());
    assert!(stdout(&output).starts_with("kmeansn "));
}

#[test]
fn help_lists_subcommands() {
    let output = bin().arg("--help").output().unwrap();
    assert!(output.status.success());
    let text = stdout(&output);
    for cmd in ["fit", "assign", "cluster-neighbors"] {
        assert!(text.contains(cmd), "help should mention {cmd}");
    }
}

#[test]
fn fit_then_assign_roundtrip_csv() {
    let dir = std::env::temp_dir().join(format!("kmeansn-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let centroids = dir.join("centroids.json");

    let output = bin()
        .args(["fit", "-k", "2", "--seed", "42"])
        .args(["--input".as_ref(), example("data.csv").as_os_str()])
        .args(["--output".as_ref(), centroids.as_os_str()])
        .output()
        .unwrap();
    assert!(output.status.success(), "fit failed: {}", stderr(&output));

    let json: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(&centroids).unwrap()).unwrap();
    assert_eq!(json["version"], "kmeansn.centroids.v1");
    assert_eq!(json["centroids"].as_array().unwrap().len(), 2);

    let output = bin()
        .arg("assign")
        .args(["--centroids".as_ref(), centroids.as_os_str()])
        .args(["--input".as_ref(), example("data.csv").as_os_str()])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "assign failed: {}",
        stderr(&output)
    );

    let text = stdout(&output);
    let mut lines = text.lines();
    let header = lines.next().unwrap();
    assert!(header.contains("_cluster_id"));
    assert!(header.contains("_cluster_distance"));
    assert!(lines.count() >= 4, "expected assigned rows");

    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn fit_reads_ndjson_from_stdin_with_format_flag() {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = bin()
        .args(["fit", "-k", "2", "--seed", "1", "--input-format", "ndjson"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"{\"x\":1.0,\"y\":2.0}\n{\"x\":1.2,\"y\":2.1}\n{\"x\":8.5,\"y\":9.1}\n{\"x\":9.0,\"y\":9.2}\n")
        .unwrap();

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "fit failed: {}", stderr(&output));
    let json: serde_json::Value = serde_json::from_str(&stdout(&output)).unwrap();
    assert_eq!(json["dims"], 2);
}

#[test]
fn seeded_fits_are_reproducible() {
    let run = || {
        let output = bin()
            .args(["fit", "-k", "2", "--seed", "42"])
            .args(["--input".as_ref(), example("data.csv").as_os_str()])
            .output()
            .unwrap();
        assert!(output.status.success());
        stdout(&output)
    };
    assert_eq!(run(), run());
}

#[test]
fn invalid_k_fails_with_nonzero_exit() {
    let output = bin()
        .args(["fit", "-k", "0"])
        .args(["--input".as_ref(), example("data.csv").as_os_str()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(stderr(&output).contains("k must be greater than 0"));
}

#[test]
fn missing_input_format_on_stdin_fails_cleanly() {
    use std::process::Stdio;

    let output = bin()
        .args(["fit", "-k", "1"])
        .stdin(Stdio::null())
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(stderr(&output).contains("input-format"));
}
