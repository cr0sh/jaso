use std::os::unix::prelude::*;
use std::{fs, process::Command};

const DECOMPOSED: &str = "\u{110b}\u{1161}\u{11ab}\u{1102}\u{1167}\u{11bc}\u{1112}\u{1161}\u{1109}\u{1166}\u{110b}\u{116d}";
const COMPOSED: &str = "안녕하세요";

#[test]
fn test_ui() {
    let decomposed = format!("{DECOMPOSED}.txt");
    let composed = format!("{COMPOSED}.txt");
    for file in fs::read_dir(env!("CARGO_MANIFEST_DIR")).expect("cannot list the directory") {
        let filename = file.expect("cannot read a directory entry").file_name();
        if filename.as_bytes() == composed.as_bytes() {
            panic!("remove {composed} first");
        }
    }
    fs::write(&decomposed, []).expect("cannot create a file decomposed name");
    let code = Command::new("cargo")
        .args(["run", "--", &decomposed])
        .spawn()
        .expect("cannot spawn child")
        .wait()
        .expect("cannot wait child")
        .code()
        .unwrap();
    assert_eq!(code, 0, "cargo run failed");
    let mut fail = true;
    for file in fs::read_dir(env!("CARGO_MANIFEST_DIR")).expect("cannot list the directory") {
        let filename = file.expect("cannot read a directory entry").file_name();
        if filename.as_bytes() == decomposed.as_bytes() {
            panic!("{decomposed} still exists")
        }
        if filename.as_bytes() == composed.as_bytes() {
            fail = false;
            break;
        }
    }
    assert!(!fail, "jaso did not create the {composed}");
    fs::remove_file(&composed).expect("cannot remove the file");
}
