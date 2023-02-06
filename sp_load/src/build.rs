mod pack;

fn get_path(path: &str) -> String {
    let path = std::fs::canonicalize(path).expect(&format!("Could not find path {:?}", path));
    path.as_os_str().to_string_lossy().to_string()
}

fn main() {
    // Relative path issues:
    // https://internals.rust-lang.org/t/relative-paths-in-cargo-rerun-if-changed-are-not-properly-resolved-in-depfiles/14563/3
    // Tell Cargo to rerun if any file in assets folder or build script changes
    println!("cargo:rerun-if-changed={}", get_path("../../assets"));
    println!(
        "cargo:rerun-if-changed={}",
        get_path("../sp_load/src/build.rs")
    );
    println!(
        "cargo:rerun-if-changed={}",
        get_path("../sp_load/src/pack.rs")
    );
    //println!("cargo:warning=Assets path: {}", path);
    pack::run("../../assets", "assets.zip");
}
