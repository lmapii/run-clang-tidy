// input
// https://github.com/mattgathu/duma/blob/master/tests/
// https://crates.io/crates/assert_cmd

use std::{fs::File, io::Read, path, thread, time};

use assert_cmd::Command;
use clap::crate_name;

fn cmd() -> Command {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.env_clear();
    cmd.env_remove("PATH");
    cmd
}

fn cmd_with_path() -> Command {
    let mut cmd = cmd();
    cmd.env("PATH", crate_root().join("artifacts/clang"));
    cmd
}

fn crate_root() -> path::PathBuf {
    path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn crate_root_rel(path: &str) -> path::PathBuf {
    let path = crate_root().join(path);
    assert!(
        path.exists(),
        "Path {} does not exist",
        path.to_string_lossy()
    );
    path
}

#[test]
fn invoke_subs() {
    // an empty command fails since <JSON> is required
    cmd().assert().failure();

    // with and without proper PATH setup
    cmd_with_path().assert().failure();

    // these sub-commands need no parameters or even correct clang-tidy setup
    let empty_ok = vec!["help", "schema", "--version"];
    for arg in empty_ok.into_iter() {
        cmd().arg(arg).assert().success();
    }
}

fn run_cmd_and_assert(cmd: &mut Command, should_pass: bool) {
    println!("command {:?} should pass {}", cmd, should_pass);
    let output = cmd.output().unwrap();

    // if output.status.success() != should_pass {
    println!(
        "status.success: {}, should_pass: {}",
        output.status.success(),
        should_pass
    );
    println!("status: {}", output.status);
    println!("{}", String::from_utf8(output.stdout).unwrap());
    println!("{}", String::from_utf8(output.stderr).unwrap());
    // }

    if cfg!(windows) {
        // on windows deleting files (the temporary clang-tidy file) can take some time
        thread::sleep(time::Duration::from_millis(500));
    }

    assert_eq!(output.status.success(), should_pass);
}

#[test]
fn invoke_json_and_bin() {
    // empty .json file is not accepted
    let json = crate_root_rel("test-files/json/test-err-empty.json");
    run_cmd_and_assert(cmd().arg(json.as_os_str()), false);

    // even if paths[] is specified, buildRoot is required if not passed as parameter
    let json = crate_root_rel("test-files/json/test-err-missing-build-root.json");
    run_cmd_and_assert(cmd().arg(json.as_os_str()), false);

    // paired with an valid --build-root parameter, passes for a command in $PATH
    run_cmd_and_assert(
        cmd_with_path().arg(json.as_os_str()).arg(format!(
            "--build-root={}",
            crate_root_rel("test-files/c-demo/_bld/out").to_string_lossy()
        )),
        true,
    );

    // paired with an valid --build-root parameter, fails if the command is not in $PATH
    run_cmd_and_assert(
        cmd().arg(json.as_os_str()).arg(format!(
            "--build-root={}",
            crate_root_rel("test-files/c-demo/_bld/out").to_string_lossy()
        )),
        false,
    );

    let json = crate_root_rel("test-files/json/test-err-missing-build-root.json");
    run_cmd_and_assert(cmd().arg(json.as_os_str()), false);

    let json = crate_root_rel("test-files/json/test-ok-empty-paths.json");
    // .json file with empty paths is accepted, but clang-tidy is not in the $PATH
    run_cmd_and_assert(cmd().arg(json.as_os_str()), false);

    // as soon as we add the path to clang-tidy to $PATH the execution is successful
    run_cmd_and_assert(cmd_with_path().arg(json.as_os_str()), true);
}

#[test]
fn invoke_json_tidy() {
    let combinations = vec![
        // path to tidyFile does not exist
        ("test-files/json/test-err-invalid-tidy-path.json", false),
        // path to tidyFile exists, but this is not a tidy file
        ("test-files/json/test-err-invalid-tidy-file.json", false),
        // path to tidyFile exists, file has name ".clang-tidy", but no 'tidyRoot' exists
        ("test-files/json/test-err-no-tidy-root.json", false),
        // path to tidyFile exists, file has name ".clang-tidy", but 'tidyRoot' is an invalid path
        ("test-files/json/test-err-invalid-tidy-root.json", false),
        // path to tidyFile exists, file has name ".clang-tidy", and 'tidyRoot' exists
        ("test-files/json/test-ok-tidy.json", true),
        // path to tidyFile exists, file has name "named.clang-tidy", and 'tidyRoot' exists
        ("test-files/json/test-ok-tidy-named.json", true),
    ];

    for test in combinations.into_iter() {
        println!("checking {}", test.0);
        let json = crate_root_rel(test.0);
        run_cmd_and_assert(cmd_with_path().arg(json.as_os_str()), test.1);
    }
}

#[test]
fn invoke_json_command() {
    let combinations = vec![
        // path to command does not exist
        ("test-files/json/test-err-invalid-command.json", false),
        // path to command exists, but it is not an executable
        ("test-files/json/test-err-invalid-command-file.json", false),
        // command is not a path and an invalid executable name
        ("test-files/json/test-err-invalid-command-name.json", false),
        // no build-root specified, all other options are ok
        ("test-files/json/test-err-root-missing.json", false),
        // all fields exist and are valid, except buildRoot contains an invalid path
        ("test-files/json/test-err-root-invalid.json", false),
        // valid command has been provided as path
        ("test-files/json/test-ok-tidy-and-command.json", true),
    ];

    for test in combinations.into_iter() {
        println!("checking {}", test.0);
        let json = crate_root_rel(test.0);
        // using command WITHOUT path
        run_cmd_and_assert(cmd().arg(json.as_os_str()), test.1);
    }

    // test that also a valid executable name can be provided as command field (requires $PATH)
    let json = crate_root_rel("test-files/json/test-ok-tidy-and-command-name.json");
    run_cmd_and_assert(cmd_with_path().arg(json.as_os_str()), true);
}

#[test]
fn invoke_json_glob() {
    // test that an invalid glob leads to an error
    let json = crate_root_rel("test-files/json/test-err-invalid-glob.json");
    run_cmd_and_assert(cmd_with_path().arg(json.as_os_str()), false);
}

#[test]
fn invoke_arg_tidy() {
    // given: a valid .json configuration file
    let json = crate_root_rel("test-files/json/test-ok-tidy.json");

    // paired with an invalid --tidy parameter, leads to an error (overrides valid .json)
    run_cmd_and_assert(
        cmd_with_path()
            .arg(json.as_os_str())
            .arg("--tidy=i/do/not/exist.clang-tidy"),
        false,
    );

    // paired with an valid --tidy parameter, success
    run_cmd_and_assert(
        cmd_with_path().arg(json.as_os_str()).arg(format!(
            "--tidy={}",
            crate_root_rel("test-files/clang-tidy/named.clang-tidy").to_string_lossy()
        )),
        true,
    );

    let json = crate_root_rel("test-files/json/test-err-invalid-tidy-file.json");
    // a valid --tidy parameter even overrides an invalid json configuration file
    run_cmd_and_assert(
        cmd_with_path().arg(json.as_os_str()).arg(format!(
            "--tidy={}",
            crate_root_rel("test-files/clang-tidy/named.clang-tidy").to_string_lossy()
        )),
        true,
    );
}

#[test]
fn invoke_arg_command() {
    // given: a valid .json configuration file
    let json = crate_root_rel("test-files/json/test-ok-tidy-and-command.json");

    // paired with an invalid --command parameter, leads to an error (overrides valid .json)
    run_cmd_and_assert(
        cmd().arg(json.as_os_str()).arg("--command=i/do/not/exist"),
        false,
    );

    // paired with an valid path as --command parameter, success
    run_cmd_and_assert(
        cmd().arg(json.as_os_str()).arg(format!(
            "--command={}",
            crate_root()
                .join("artifacts/clang/clang-tidy")
                .to_string_lossy()
        )),
        true,
    );

    // paired with an valid command in $PATH as --command parameter, success
    run_cmd_and_assert(
        cmd_with_path()
            .arg(json.as_os_str())
            .arg("--command=clang-tidy"),
        true,
    );

    let json = crate_root_rel("test-files/json/test-err-invalid-command.json");
    // a valid --command parameter even overrides an invalid json configuration file
    run_cmd_and_assert(
        cmd_with_path()
            .arg(json.as_os_str())
            .arg("--command=clang-tidy"),
        true,
    );
}

#[test]
fn invoke_arg_root() {
    // given: a valid .json configuration file
    let json = crate_root_rel("test-files/json/test-ok-tidy.json");

    // paired with an invalid --build-root parameter, leads to an error (overrides valid .json)
    run_cmd_and_assert(
        cmd_with_path()
            .arg(json.as_os_str())
            .arg("--build-root=i/do/not/exist"),
        false,
    );

    // paired with an valid --build-root parameter, success
    run_cmd_and_assert(
        cmd_with_path().arg(json.as_os_str()).arg(format!(
            "--build-root={}",
            crate_root_rel("test-files/c-demo/_bld/out").to_string_lossy()
        )),
        true,
    );

    let json = crate_root_rel("test-files/json/test-err-root-invalid.json");
    // a valid --build-root parameter even overrides an invalid json configuration file
    run_cmd_and_assert(
        cmd_with_path().arg(json.as_os_str()).arg(format!(
            "--build-root={}",
            crate_root_rel("test-files/c-demo/_bld/out").to_string_lossy()
        )),
        true,
    );
}

#[test]
fn invoke_quiet() {
    fn assert_quiet(cmd: &mut Command, expect_quiet: bool) {
        let output = cmd.output().unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();
        let stderr = String::from_utf8(output.stderr).unwrap();

        println!("status: {}", output.status);
        println!("{stdout}");
        println!("{stderr}");

        if expect_quiet {
            assert_eq!(0, stdout.len());
            assert_eq!(0, stderr.len());
        } else {
            assert_ne!(0, stderr.len());
        }
    }

    assert_quiet(
        cmd_with_path()
            .arg(crate_root_rel("test-files/json/test-ok-tidy.json").as_os_str())
            .arg("-vvvv")
            .arg("--quiet"),
        true,
    );

    assert_quiet(
        cmd_with_path()
            .arg(crate_root_rel("test-files/json/test-err-empty.json").as_os_str())
            .arg("-vvvv")
            .arg("--quiet"),
        false,
    );
}

// cargo test --test invoke 'invoke_arg_fix' -- --test-threads=1 --nocapture

// On Windows, a simple compilation / warnings will be treated as errors and --fix would
// not apply any patch unless -fix-errors is specified.

// 1 warning generated.
// Found compiler errors, but -fix-errors was not specified.
// Fixes have NOT been applied.

#[test]
fn invoke_arg_fix() {
    let json = crate_root_rel("test-files/json/test-err-fix.json");
    let fix_file = crate_root_rel("test-files/c-demo/pkg_b/module_fix/module_fix.h");

    fn dump_file(file: impl Into<std::path::PathBuf>) {
        let file_path: std::path::PathBuf = file.into();
        let mut file_content = String::new();
        File::open(&file_path)
            .unwrap()
            .read_to_string(&mut file_content)
            .unwrap();
        println!(
            "File contents of {}: \n{}",
            file_path.to_string_lossy(),
            file_content
        );
    }

    let mut file_content = String::new();
    File::open(&fix_file)
        .expect("failed to open module_fix.h test file")
        .read_to_string(&mut file_content)
        .expect("failed to read module_fix.h test file");

    // Use scopeguard::guard to create a guard that restores the content when it goes out of scope
    let _guard = scopeguard::guard(file_content, |content| {
        println!("restoring {}", &fix_file.to_string_lossy());
        std::fs::write(&fix_file, content).expect("failed to restore module_fix.h");
    });

    println!("test-err-fix has a fixable bugprone error, so the execution fails");
    run_cmd_and_assert(cmd_with_path().arg(json.as_os_str()), false);

    println!("FIX #1 will apply parenthesis around the expression, but still fails.");
    run_cmd_and_assert(cmd_with_path().arg(json.as_os_str()).arg("--fix"), false);
    dump_file(&fix_file);

    println!("FIX #2 will apply parenthesis around the parameters, but still fails.");
    run_cmd_and_assert(cmd_with_path().arg(json.as_os_str()).arg("--fix"), false);
    dump_file(&fix_file);

    println!("after all fixes have been applied, the check should pass");
    run_cmd_and_assert(cmd_with_path().arg(json.as_os_str()), true);
}
