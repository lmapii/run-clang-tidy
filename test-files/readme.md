
# Dependencies

This little `C` project is used for the integration tests of this CLI tool. To be able to run tests locally, the project must build and a `compile_commands.json` file must be generated (this can't be pre-generated due to tool dependencies). You'll need:

* `gcc`
* `make` version 4.0 or later
* `python3` with at least `argparse` installed

You can then run
`make -C test-files/c-demo/project build-data`

This will generate the following file, referenced by the integration tests:
`test-files/c-demo/_bld/out/compile_commands.json`

## Helpers

The GitHub actions need to perform the same steps so there's some scripts available in case you're running into troubles (or if you need some guideline for a docker image):

* For [macos](../.github/setup/load_artifacts_macos.sh) bash script is available
* For [Ubuntu](../.github/setup/load_artifacts_ubuntu.sh) a similar bash script is available
* For [Window](../.github/setup/load_artifacts_windows.bat) a batch script is used.
