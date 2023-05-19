fn main() {
    let pre_release = if let Ok(pre_release) = std::env::var("CI_PKG_VERSION_PRE_RELEASE") {
        "-".to_owned() + &pre_release
    } else {
        "".to_owned()
    };

    let build = if let Ok(build) = std::env::var("CI_PKG_VERSION_BUILD") {
        "+".to_owned() + &build
    } else {
        "".to_owned()
    };

    // If we set CARGO_PKG_VERSION this way, then it will override the default value, which is
    // taken from the `version` in Cargo.toml or from an override in .cargo/config.toml
    let version_override = format!("{}{}{}", env!("CARGO_PKG_VERSION"), pre_release, build);
    if !version_override.is_empty() {
        println!("cargo:rustc-env=CARGO_PKG_VERSION={}", version_override);
    }

    println!("cargo:rerun-if-env-changed=CI_PKG_VERSION_PRE_RELEASE");
    println!("cargo:rerun-if-env-changed=CI_PKG_VERSION_BUILD");
    println!("cargo:rerun-if-changed=.cargo/config.toml");
}
