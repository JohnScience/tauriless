use cargo_emit::convenience::compare_and_set_contents_hash;
use which::which;

const TRACKED_FRONTEND_PATHS: &[&str] = &[
    "package.json",
    "package-lock.json",
    "tsconfig.json",
    "tsconfig.node.json",
    "vite.config.ts",
    "index.html",
    "public",
    "src",
];

fn build_frontend() {
    let npm = which("npm").expect("npm not found");

    std::process::Command::new(&npm)
        .args(&["install"])
        .current_dir("../front")
        .output()
        .unwrap();
    std::process::Command::new(&npm)
        .args(&["run", "build"])
        .current_dir("../front")
        .output()
        .unwrap();
}

pub fn main() {
    use compare_and_set_contents_hash::HashFileOutcome::{Changed, Created, Unchanged};

    for p in TRACKED_FRONTEND_PATHS.iter() {
        println!("cargo:rerun-if-changed=../front/{}", p);
    }

    let hash_file_outcome = compare_and_set_contents_hash("../front/package-lock.json");
    match hash_file_outcome {
        Changed | Created => build_frontend(),
        Unchanged => (),
    }
}
