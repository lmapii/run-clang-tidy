
# Dependencies

When trying to execute the integration tests locally,

* Please check the dependencies in the [test-files](../test-files/readme.md)
* Place a valid `clang-tidy` executable into `<repo-root>/artifacts/clang/clang-tidy[.exe]`
* Please check the [ci setup](../.github/setup/) for the `clang-tidy` version that is currently used for testing.

This setup is required to test most of the possible combinations and/or valid fields. The CI integrates this workflow in the test steps.

# Invocation

Use `cargo test -- --test-threads 1` since conflicts might be generated when the `.clang-tidy` file is placed into the root directory and if tests are executed in parallel.
