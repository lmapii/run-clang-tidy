# run-clang-tidy <!-- omit in toc -->

[![Build status](https://github.com/lmapii/run-clang-tidy/actions/workflows/ci.yml/badge.svg)](https://github.com/lmapii/run-clang-tidy/actions/workflows/ci.yml)

CLI application for running [`clang-tidy`](https://clang.llvm.org/extra/clang-tidy/) for an existing `.clang-tidy` file on a set of files, specified using globs in a `.json` configuration file.

# Quickstart <!-- omit in toc -->

The minimal command for executing this is the following:

```bash
$ run-clang-tidy path/to/tidy.json
```

Execute `run-clang-tidy --help` for more details, or `run-clang-tidy schema` for a complete schema description of the configuration file.

<img src="https://github.com/lmapii/run-clang-tidy/blob/main/screenshots/demo.gif?raw=true">

**Hints for the impatient user:**

- Hidden paths and files are excluded unless the setting is changed [in the configuration file](#pre-filtering).
- This tool assumes that `clang-tidy` is installed and in your path. The command can be specified in your [configuration file](#specifying-the-clang-tidy-command) or as a [command-line parameter](#specifying-an-alternative-tidy-file-and-command).
- Paths can be specified using [glob- or Unix-style path syntax](#glob--and-path-syntax).
- The analysis is [executed in parallel](#speeding-up-the-execution) if the `-j` option is specified.
- If your application doesn't compile the analysis will fail. Please get familiar with [`clang-tidy`](https://clang.llvm.org/extra/clang-tidy/).
- Most modern build systems support generating the [compilation database](https://clang.llvm.org/docs/JSONCompilationDatabase.html) that is required to execute `clang-tidy`. It can be created using `cmake` or you can build one yourself using plain `make` as demonstrated in the [example makefile](test-files/c-demo/project/makefile).
- Read [The build root and `compile_commands.json`](#the-build-root-and-compile_commandsjson) for details about the compilation database.

# Contents <!-- omit in toc -->

- [The JSON configuration file](#the-json-configuration-file)
  - [Adding paths](#adding-paths)
  - [The build root and `compile_commands.json`](#the-build-root-and-compile_commandsjson)
  - [Glob- and path syntax](#glob--and-path-syntax)
  - [Pre-filtering](#pre-filtering)
  - [Post-filtering](#post-filtering)
  - [Specifying a `.clang-tidy` file and a root directory](#specifying-a-clang-tidy-file-and-a-root-directory)
  - [Specifying the `clang-tidy` command](#specifying-the-clang-tidy-command)
- [Command-line Parameters](#command-line-parameters)
  - [Verbosity and `--quiet`](#verbosity-and---quiet)
  - [Speeding up the execution](#speeding-up-the-execution)
  - [Specifying an alternative tidy file and command](#specifying-an-alternative-tidy-file-and-command)
  - [Specifying an alternative build root](#specifying-an-alternative-build-root)
  - [Suppressing warnings](#suppressing-warnings)
  - [Applying fixes](#applying-fixes)
- [Use-cases](#use-cases)
- [Pitfalls](#pitfalls)
  - [Multiple `.clang-tidy` files](#multiple-clang-tidy-files)
  - [`clang-tidy` keeps on analyzing an excluded header files](#clang-tidy-keeps-on-analyzing-an-excluded-header-files)
  - [Some files do not seem to be not correctly analyzed](#some-files-do-not-seem-to-be-not-correctly-analyzed)
- [Roadmap](#roadmap)

# The JSON configuration file

The core of this CLI tool is a `.json` configuration file that specifies where all the files that should be analyzed can be found. We'll be using a demo file, building it up step by step to explain the individual fields. The structure of the `.json` file is also documented in the `schema` sub-command (execute `run-clang-tidy schema`). To get started, we create an empty `.json` file that contains an empty object.

```json
{
}
```

## Adding paths

The only fields that are really required in this configuration file are the **`paths`** and the **`buildRoot`**. The paths always need to be specified in the configuration file, whereas the build root folder can provided as command line parameter. The section [The build root and `compile_commands.json`](#the-build-root-and-compile_commandsjson) will provide the necessary information about the build root directory.

For now we're looking at the paths: The **`paths`** field contains paths or **globs**, relative to the parent directory of the **configuration file** `tidy.json`. Consider the following folder structure:

```
ProjectRoot
├── .clang-tidy
│
├── _bld_
│   └── compile_commands.json
│
├── Some
│   └── Path
│       ├── header h
│       └── source.c
│
└── Settings
    ├── tidy.json
    └── <...>
```

In the configuration file `tidy.json`, the paths to the two files and and build root directory would need to be specified as following:

```json
{
  "paths": [
    "../Some/Path/header h",
    "../Some/Path/source.c",
  ],
  "buildRoot": "../_bld"
}
```

> **Remark:** This tool is made for software developers, thus any user should know that paths by themselves can become fairly complex: Take links, throw in character encodings, you get the idea. So anyone using smileys or other surreal things in their paths can contribute to this repository in case of problems, but not all scenarios can or will be tested.

Clearly, no one wants to specify all paths manually, which is why this tool supports the use of Unix-style **globs**. The following patterns will all resolve to the same paths, but are just provided for reference:

```json
{
  "paths": [
    "../**/*.[ch]",
    "../Some/*/*.*",
  ],
  "buildRoot": "../_bld"
}
```

Assuming you have `clang-tidy` installed and a `.clang-tidy` file in one of the parent directories of your sources, e.g., in *ProjectRoot*, this is all you need::

```
$ run-clang-tidy path/to/tidy.json
```

Notice that the working directory of the tool is irrelevant since all paths are specified relative to the provided `tidy.json`. For now, this is all you need to know, we'll go into details about the supported scenarios later and will continue exploring the configuration options in the `.json` file.

## The build root and `compile_commands.json`

Modern build systems like [`cmake`](https://cmake.org) or [`clang`](https://clang.llvm.org/docs/) support the generation of a [compilation database][https://clang.llvm.org/docs/JSONCompilationDatabase.html]. This compilation database is simply a `.json` file (the convention is to name it `compile_commands.json`) placed in the root directory of your build that contains all the commands with all parameters that are invoked in the build process.

> **Remark:** Any static analyzer will need to know all compile options and files to analyze your code. It is therefore important that your codebase is buildable before trying to analyze it, though `clang-tidy` is amongst the more tolerating tools.

> **Remark:** In case you're using plain old `make`, a primitive way to generate this file is demonstrated in the [example makefile](test-files/c-demo/project/makefile) for the target `build-data`.

This compilation database is a major input for `clang-tidy`. Therefore, the path to the folder containing this file must be specified either in the configuration file using the **`buildRoot`**, or via the command line parameter `--build-root`. If specified in the configuration file, the path is resolved relative to the configuration file. As command line parameter an absolute or relative path to the invocation of `run-clang-tidy` must be provided.

## Glob- and path syntax

This tool uses the [globset](https://docs.rs/globset/latest/globset/index.html) rust crate to resolve globs. It therefore also relies on its [syntax](https://docs.rs/globset/latest/globset/index.html#syntax). We're borrowing the explanation here. When using globs, *standard Unix-style glob syntax* is supported:

- `?` matches any single character. It does not match path separators.
- `*` matches zero or more characters but does not match across directory boundaries, i.e., it does not match a path separator. You have to use `**` for that:
- `**` recursively matches directories and if used without a path separator it means "match everything".
- `{a,b}` matches `a` or `b` where `a` and `b` are arbitrary glob patterns. Nesting `{...}` is not supported.
- `[ab]` matches `a` or `b` where `a` and `b` are *characters*. Use `[!ab]` to match any character *except* for `a` and `b`.
- Metacharacters such as `*` and `?` can be escaped with the character class notation. e.g., `[*]` matches `*`.
- A backslash `\` will escape all metacharacters in a glob, but it must be specified as double backslash `\\` due to the fact that the glob is defined in a `.json` configuration file. If it precedes a non-meta character, then the slash is ignored.

For Windows paths, all globs are case insensitive.

> **Remark:** Due to the caveat that backslashes must be escaped in `.json` files, and that a backslash in a glob behaves differently depending on whether or not the following character is a metacharacter, it is highly recommended to use a forward slash `/` as path separator on **any** platform. On Windows it is possible to use `\\` as path separators, but only if it does not precede a metacharacter.

## Pre-filtering

By default, this tool will **exclude** all hidden files and folders from its search. This behaviour can be configured with the field **`filterPre`**. This field sets up a filter that is applied while recursively searching for files and therefore *before* matching files against the provided globs in the field `paths`. A typical pattern for such a filter is to exclude folders used by revision control systems, e.g., `.git` (or `.svn`) folders.

For this field, you can still use *globs*, but keep in mind that such a filter is applied on directories as well and thus if the filter matches then the directory will not even be searched, making it unnecessary to use, e.g., `**` after the name. The following example shows a pre-filter configured to exclude all files within the `.git` folder, and also excludes all hidden files and directories.

```json
{
  "paths": [
    "../**/*.[ch]",
    "../Some/*/*.*",
  ],
  "filterPre": ["**/.git", ".*"],
  "buildRoot": "../_bld"
}
```

If no hidden folders should be skipped simply set this field to an empty list `[]`.

## Post-filtering

With the previous configuration file, we matched all files and folders except for hidden files. Sometimes, however, it is useful to apply a filter *after* matching all paths, e.g., to exclude specific filenames that occur multiple times, or to simplify the patterns in the field `paths`. This can be achieved with **`filterPost`**:

```json
{
  "paths": [
    "../**/*.[ch]",
    "../Some/*/*.*",
  ],
  "filterPre": ["**/.git", ".*"],
  "filterPost": ["FreeRTOS.h", "**/Hal*/**"],
  "buildRoot": "../_bld"
}
```

In the above example, any `Hal*` folder within any of the paths will be filtered without having to create a complex glob for `paths`.

## Specifying a `.clang-tidy` file and a root directory

If no `.clang-tidy` file is placed in the root directory of your project (assuming there is one), executing `run-clang-tidy` without any additional command-line parameters (explained below) would not produce the desired results - quite the opposite since `clang-tidy` checks any root folder until it might encounter a `.clang-tidy` file. Therefore the configuration file allows to specify the tidy file using the field **`tidyFile`**, and the root common root directory of all paths using **`tidyRoot`**:

```
ProjectRoot
├── _bld_
│   └── compile_commands.json
│
├── Some
│   └── Path
│       ├── header.h
│       └── source.c
│
└── Settings
    ├── tidy.json
    ├── tidy.clang-tidy
    └── <...>
```

```json
{
  "paths": [
    "../**/*.[ch]",
    "../Some/*/*.*",
  ],
  "filterPre": ["**/.git", ".*"],
  "filterPost": ["FreeRTOS.h", "**/Hal*/**"],
  "tidyFile": "./tidy.clang-tidy",
  "tidyRoot": "../",
  "buildRoot": "../_bld"
}
```

The name *or the extension* of the `tidyFile` must be `.clang-tidy`. This allows you to store multiple `.clang-tidy` files in the same directory, e.g., `driver.clang-tidy` and `application.clang-tidy`.

When analyzing the files, `run-clang-tidy` will:
- Copy the provided tidy file to the specified root directory (renaming it to `.clang-tidy`, if necessary),
- execute `clang-tidy` for all resolved paths,
- and finally remove the temporary file.

Only if you kill the execution of the tool (e.g., via CTRL+C) it won't be able to delete the temporary file.

> **Remark:** Specifying a root directory is necessary since it is not feasible to determine a common denominator for all paths. Also, killing the execution of the tool will prevent deleting the temporary file and therefore might clutter your workspace with tidy files, since adding new globs or paths might result in a different root directory.

> **Remark:** The tool will check whether a `.clang-tidy` file *with different content* already exists in `tidyRoot` - and abort with an error if that is the case. If the contents match, the tool won't copy or delete any files and execute as if no `tidyRoot` and `tidyFile` were specified.

The `tidyFile` configuration will be replaced by the **`--tidy`** command-line parameter, if provided.

## Specifying the `clang-tidy` command

By default, the tool tries to use the command `clang-tidy` for analyzing all resolved paths. If this command is not in your path, or if you use a different name for your executable (e.g., `clang-tidy-10`), then you need to specify the command or full path to the executable either via the command-line parameter `--command` or using the `command` field in your configuration file:

```json
{
  "paths": [
    "../**/*.[ch]",
    "../Some/*/*.*",
  ],
  "filterPre": ["**/.git", ".*"],
  "filterPost": ["FreeRTOS.h", "**/Hal*/**"],
  "tidyFile": "./tidy.clang-tidy",
  "tidyRoot": "../",
  "buildRoot": "../_bld",
  "command": "/path/to/clang-tidy"
}
```

In contrast to the patterns in the field 'paths', the command can be specified as a path relative to the configuration file, as an absolute path, or as a simple executable name.

Similar to the `tidyFile` field, this configuration will be replaced by the **`--command`** command-line parameter, if provided. When specifying a relative path specified as command line parameter the path is resolved relative to the current *working directory*.

> **Notice:** It is important that your tidy file is compatible with the version of `clang-tidy` that you are using. This is the main reason why `clang-tidy` is not installed with this tool.

> **Notice:** Configuration files aim to be cross-platform as well. It is therefore **allowed to omit the `.exe` extension** for the `clang-tidy` executable. This also applies to the `--command` parameter.

# Command-line Parameters

All available command-line parameters should be sufficiently described by the tool itself, when providing any of the options `-h, --help, help`. Also, the JSON schema of the configuration file can be displayed by using the `schema` subcommand. This JSON schema also contains descriptions for each of the options described above:

```
$ run-clang-tidy --help
$ run-clang-tidy help
$ run-clang-tidy schema
```

In the following, the most important options are described briefly.

## Verbosity and `--quiet`

The verbosity is best configured by using the `-v` option:

* `-v` is the default option; the tool will provide a "pretty-print" output complete with progress bar (implemented by the rust crate [indicatif](https://github.com/mitsuhiko/indicatif)).

> The "pretty" output is only available for the `-v` log level, for any other log level the tool will switch to a debug-style output. This kind of output is not optimized for being redirected to a file since the progress bar will rewrite previous lines. Use the `-vv` debug option instead.

* `-vv` switches to the log level "debug", providing timestamps and a purely sequential output: No lines are being overwritten, and each message is logged to a new line.

* `-vvv` and above switch to the log level "trace", which can contain even more (probably irrelevant) messages. This is intended mainly for debugging the tool in case you find issues.

To turn off any kind of output except for error messages, use the `--quiet` option. This overwrites the `--verbosity` level.

## Speeding up the execution

By default, the tool will process each resolved path one by one. This can be rather slow for large projects. The command-line option `-j, --jobs` allows specifying the number of jobs that should be used the analysis.

* If specified without a value, e.g., `run-clang-tidy tidy.json -j`, then all available logical cores will be used the analysis.
* If specified *with* a value, e.g., `run-clang-tidy tidy.json -j 3`, then the tool will only spawn as many jobs as specified.

> **Remark:** On slower machines, when executed with normal log level, the progress bar might flicker since the terminal might not be able to re-draw the new line fast enough. Currently, there's no way around this.

## Specifying an alternative tidy file and command

The command-line options `--tidy` and `--command` allow specifying a `.clang-tidy` file and the command to use for executing `clang-tidy`. Please refer to the description of the `.json` configuration file for the [fields `tidyFile`](#specifying-a-clang-tidy-file-and-a-root-directory) and [`command`](#specifying-the-clang-tidy-command).

> **Remark:** Specifying `--tidy` requires the field `tidyRoot` to be configured.

## Specifying an alternative build root

The [build root](#the-build-root-and-compile_commandsjson) containing the compilation database is typically not fixed; each build might use a different output folder and tools may be installed in different directories (e.g., if executed as part of a CI chain).

Therefore the command-line option `--build-root` allows to specify the build directory when invoking this script, overriding, e.g., a default directory specified in the configuration `.json` file.

## Suppressing warnings

By default, warnings issued by `clang-tidy` are output on each run, unless the command-line option `--suppress-warnings` is used.

> **Remark:** `clang-tidy` warnings do not affect the return code of `run-clang-tidy`, regardless of whether or not they are part of the output. Use your `.clang-tidy` file to transform warnings into errors in case the execution should fail, e.g., by specifying `WarningsAsErrors`.

## Applying fixes

For some checks, `clang-tidy` supports applying fixes using the `-fix` option. The command-line option `--fix` of this wrapper enables both, `-fix` and `-fix-errors` to ensure that fixes are always applied.

> **Remark:** Also using `-fix-errors` ensures that compiler _warnings_ - which can be the annoying "system-header" warnings - don't prevent `clang-tidy` to apply fixes.

In case `clang-tidy` finds a problem and applies a fix, the execution will still report a failed execution. You'll need to execute `clang-tidy` again to be sure that there are no more findings or no more fixes to apply.

Notice that it can happen that multiple runs with the `--fix` option are necessary to really fix a problem since running `clang-tidy` only applies one "iteration" of a fix. E.g., the following definition in the test file [`module_fix.h`](test-files/c-demo/pkg_b/module_fix/module_fix.h) will only report success after the third execution:

```c
// This macro triggers "bugprone-macro-parentheses", which is fixable.
#define MODULE_FIX_EXPRESSION(a, b) a + b

// The first execution with '--fix' applies the following fix, which is still not correct.
#define MODULE_FIX_EXPRESSION(a, b) (a + b)

// A second execution with '--fix' applies the following fix. This second execution
// still reports an exit code != 0, since "bugprone-macro-parentheses" was still detected.
#define MODULE_FIX_EXPRESSION(a, b) ((a) + (b))

// Only after the third execution clang-tidy reports success.
```

# Use-cases

Due to the nature of this tool, i.e., the underlying `clang` tools, the use-cases are very similar when executing `clang-format`, for which a [dedicated wrapper](https://github.com/lmapii/run-clang-format) exists. Please refer to the matching section in the documentation of [`run-clang-format`](https://github.com/lmapii/run-clang-format#use-cases).


# Pitfalls

## Multiple `.clang-tidy` files

In case other `.clang-tidy` files exist in different folders, `.clang-tidy` will always use the first file that it finds when going back from the file to analyze. E.g., for `source.c` the file `ProjectRoot/Some/.clang-tidy` will be used.

```
ProjectRoot
│
├── Some
│   ├── .clang-tidy
│   └── Path
│       ├── header.h
│       └── source.c
│
└── .clang-tidy
```

When executing the tool with the following configuration, the files in `Some/Path` will be analyzed using `Some/.clang-tidy` and **not** with the configured tidy file, since this tool does not scan any paths for existing `.clang-tidy` files.

```json
{
  "paths": [
    "./Some/**/*.[ch]",
  ],
  "tidyFile": ".clang-tidy",
  "tidyRoot": "../"
}
```

## `clang-tidy` keeps on analyzing an excluded header files

There seems to be a catch when using `clang-tidy` to analyze sources, since it will also partly analyze included header files. E.g., assume the following `module_untidy.h` header file exists and is **excluded** from the analysis using filters and/or globs, i.e., `clang-tidy` is never invoked for the header file `module_untidy.h`:

```h
#pragma once

// This definition violates the
// readability-uppercase-literal-suffix rule
// The `u` suffix of `2uL` must be uppercase, see
// https://clang.llvm.org/extra/clang-tidy/checks/readability-uppercase-literal-suffix.html
#define MODULE_UNTIDY_SMTH 2uL
```

This header file might be used in the following `main.c`, a file for which `clang-tidy` is executed. Here `clang-tidy` will still produce an error since `MODULE_UNTIDY_SMTH` is expanded within `main.c`:

```c++
#include "module_untidy.h"

int main (int argc, const char *argv[]) // NOLINT : unused argument argv
{
    // MODULE_UNTIDY_SMTH is expanded here and violates
    // readability-uppercase-literal-suffix rule
    unsigned int some_variable = MODULE_UNTIDY_SMTH;
    return (int) some_variable;
}
```

For this exact usage, i.e., where a macro is expanded in a file that is part of the analysis, there is no way to tell `clang-tidy` to ignore this finding except by adding the `NOLINT` exclusion in your code (refer to the documentation for [suppressing undesired diagnostics](https://clang.llvm.org/extra/clang-tidy/#suppressing-undesired-diagnostics)).

For other scenarios it might be sufficient to **exclude** the `HeaderFilterRegex` setting from your `.clang-tidy` configuration file: This wrapper runs `clang-tidy` without the `-header-filter` parameter, therefore it will not be used except if specified in your `.clang-tidy` file. Also, please notice that `clang-tidy` uses the [posix ERE flavor](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap09.html#tag_09_04) in its regexes.

## Some files do not seem to be not correctly analyzed

When [adding paths](#adding-paths) using globs, `run-clang-tidy` will resolve the globs to all files it can find, regardless of whether they are part of your build ([`compile_commands.json`](#the-build-root-and-compile_commandsjson)) or not. It will therefore call `clang-tidy` even for files that are not in your build. This has two effects:

* `clang-tidy` will still try to compile the files, even if they are not part of your build. You'll get errors if the file doesn't compile, e.g., due to missing include paths or conflicting types for the function declaration and function definition.

* For file that can be successfully compiled by `clang-tidy` no errors or warning will be generated, since `clang-tidy` will skip the actual analysis step. The files will still be listed in the execution of `run-clang-tidy`.

> **Remark:** You may reproduce this by not excluding the [module_unused.h](test-files/c-demo/subfolder/pkg_c/module_unused/module_unused.h) in the test of this tool. This header file is not used in the build and the `e_module_unused_a_enum` violates the naming convention defined in the `.clang-tidy` files.

This might be fixed in a future version of `run-clang-tidy`, i.e., the tool might search for the compilation database too, and might try to find out which files are really part of the build. It should generate an error (or ignore it, see the roadmap below) if a glob resolved to a file that is not part of the build.

# Roadmap

* Update the default behaviour or add a `--strict` option to avoid running the analysis for files that are not part of the compilation database.
