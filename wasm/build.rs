use std::{fs, path::Path};

fn main() {
    let pkg = Path::new("../package.json"); // 必要なら調整
    let json = fs::read_to_string(pkg).expect("package.json not found");

    let v: serde_json::Value = serde_json::from_str(&json).expect("invalid package.json");

    let version = v["version"].as_str().expect("version field not found");

    println!("cargo:rustc-env=PKG_VERSION={}", version);
}
