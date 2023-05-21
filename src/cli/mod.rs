use std::{path, process};

mod handlers;
mod logging;
pub mod utils;

use clap::{arg, crate_authors, crate_description, crate_name, crate_version, Arg};
#[allow(unused_imports)]
use color_eyre::{eyre::eyre, eyre::WrapErr, Help};
use schemars::{schema_for, JsonSchema};
use serde::Deserialize;

#[derive(Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")] // removed: deny_unknown_fields
pub struct JsonModel {
    /// List of paths and/or globs.
    /// This list may contain paths or shell-style globs to define the files that should be
    /// filtered. Paths or globs that resolve to folders will be silently ignored. Any path
    /// contained in this list must be specified relative to the configuration file.
    pub paths: Vec<String>,
    /// Optional list of globs used for efficiently pre-filtering paths.
    /// In contrast to the post-filter, searching will completely skip all paths and its siblings
    /// for any match with any pattern. E.g., [".git"] will skip all ".git" folders completely.
    /// By default, i.e., if this field is not present in the configuration, the tool will skip all
    /// hidden paths and files. Set this entry to an empty list to prevent any kind of
    /// pre-filtering.
    pub filter_pre: Option<Vec<String>>,
    /// Optional list of globs to use for post-filtering.
    /// This filter will be applied for all paths _after_ they have been resolved. In contrast to
    /// the pre-filter, siblings of paths will not be filtered without the corresponding glob. E.g.,
    /// ".git" will not filter any files, only ".git/**" would. Notice that only
    pub filter_post: Option<Vec<String>>,
    /// Optional path to a `.clang-tidy` yaml file (can be specified via --tidy). If no such path
    /// is provided by neither this field nor the command-line option, `clang-tidy` will perform a
    /// search for the `compile_commands.json` through all parent paths of the file to analyze.
    pub tidy_file: Option<path::PathBuf>,
    // TODO: allow this config to be skipped for clang-tidy >= 12.0.0
    /// Optional path where the `.clang-tidy` file should be copied to while executing.
    pub tidy_root: Option<path::PathBuf>,
    /// Optional path to the folder that contains the `compile-commands.json` (can be specified
    /// via --build-root).
    pub build_root: Option<path::PathBuf>,
    /// Optional path to the `clang-tidy` executable or command name
    pub command: Option<path::PathBuf>,
    // TODO: allow to specify additional options
    #[serde(skip)]
    /// Parent directory of the Json file, used to resolve paths specified within
    pub root: path::PathBuf,
    #[serde(skip)]
    /// Lossy Json filename
    pub name: String,
}

// goal: have compatible .json configuration files for clang-format and clang-tidy
// it should be possible to specify all command line options and non-unit relative paths
// using the command line, such that they can be set using ENV variables

#[derive(Debug)]
pub struct Data {
    /// Json input data
    pub json: JsonModel,
    /// Command-line override for the tidy file
    pub tidy_file: Option<path::PathBuf>,
    // TODO: add tidy_root override
    /// Command-line override for the build root folder
    pub build_root: Option<path::PathBuf>,
    /// Command-line override for the clang-tidy executable
    pub command: Option<path::PathBuf>,
    /// Command-line parameter for the number of jobs to use for executing clang-tidy
    /// If `None` then all available jobs should be used, else the specified number of jobs.
    pub jobs: Option<u8>,
    /// Command-line option to suppress warnings issued by clang-tidy.
    pub ignore_warn: bool,
    /// Suppress all logging.
    pub quiet: bool,
}

pub struct Builder {
    pub matches: clap::ArgMatches,
}

impl Builder {
    fn app() -> clap::Command {
        clap::Command::new(crate_name!())
            .arg_required_else_help(true)
            .version(crate_version!())
            .author(crate_authors!())
            .about(crate_description!())
            .arg(
                arg!(<JSON>)
                    .help("Path/configuration as .json")
                    .value_parser(clap::value_parser!(std::path::PathBuf)),
            )
            .arg(
                arg!(-t --tidy ... "Optional path to the .clang-tidy configuration file. \
                                    Overrides <JSON> configuration. If no path is provided, \
                                    `clang-tidy` will attempt a search for the compile commands \
                                    through all parent paths of the file that is being analyzed.")
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .required(false)
                .action(clap::ArgAction::Set),
            )
            .arg(
                clap::Arg::new("build-root")
                    .short('b')
                    .long("build-root")
                    .help(
                        "Optional path to the build root folder which should \
                         contain the compile-commands.json file. Overrides <JSON> \
                         configuration.",
                    )
                    .value_parser(clap::value_parser!(std::path::PathBuf))
                    .action(clap::ArgAction::Set)
                    .required(false),
            )
            .arg(
                arg!(-c --command ... "Optional path to executable or clang-tidy command. \
                                       Overrides <JSON> configuration, defaults to `clang-tidy`")
                .value_parser(clap::value_parser!(std::path::PathBuf))
                .required(false)
                .action(clap::ArgAction::Set),
            )
            .arg(
                arg!(-j --jobs ... "Optional parameter to define the number of jobs to use. \
                                    If provided without value (e.g., '-j') all available logical \
                                    cores are used. Maximum value is 255")
                .required(false)
                .num_args(0..=1)
                .action(clap::ArgAction::Set),
            )
            .arg(arg!(-v --verbose ... "Verbosity, use -vv... for verbose output.").global(true))
            // .arg(
            //     arg!(--fix "Fix findings on the fly if available."),
            // )
            .arg(
                arg!(-q --quiet "Suppress all output except for errors; overrides -v")
                    .action(clap::ArgAction::SetTrue),
            )
            .arg(
                Arg::new("suppress-warnings")
                    .long("suppress-warnings")
                    .action(clap::ArgAction::SetTrue)
                    .help("Suppress warnings; overrides -v"),
            )
            .subcommand_negates_reqs(true)
            .subcommand(
                clap::Command::new("schema")
                    .about("Print the schema used for the <JSON> configuration file"),
            )
    }

    pub fn build() -> Builder {
        let cmd = Builder::app();
        let builder = Builder {
            matches: cmd.get_matches(),
        };
        logging::setup(&builder.matches);
        builder
    }

    pub fn parse(self) -> eyre::Result<Data> {
        if self.matches.subcommand_matches("schema").is_some() {
            println!("{}", JsonModel::schema(),);
            process::exit(0);
        }

        let json_path = self.path_for_key("JSON", true)?;
        let json = JsonModel::load(json_path).wrap_err("Invalid parameter for <JSON>")?;

        let tidy_file = match self.matches.contains_id("tidy") {
            false => None,
            true => {
                let tidy_path = self
                    .path_for_key("tidy", true)
                    .wrap_err("Invalid parameter for option --tidy")?;
                let path = utils::file_with_name_or_ext(tidy_path, ".clang-tidy")
                    .wrap_err("Invalid parameter for option --tidy")?;
                Some(path)
            }
        };

        let command = match self.matches.get_one::<std::path::PathBuf>("command") {
            None => None,
            Some(_) => Some(
                utils::executable_or_exists(self.path_for_key("command", false)?, None)
                    .wrap_err("Invalid parameter for option --command")
                    .suggestion(
                        "Please make sure that '--command' is either a valid absolute path, \
                            a valid path relative to the current working directory \
                            or a known application",
                    )?,
            ),
        };

        let build_root = match self.matches.get_one::<std::path::PathBuf>("build-root") {
            None => None,
            Some(_) => Some(
                utils::dir_or_err(self.path_for_key("build-root", false)?)
                    .wrap_err("Invalid parameter for option --build-root")
                    .suggestion(
                        "Please make sure that '--build-root' is either a valid absolute path or \
                            a valid path relative to the current working directory",
                    )?,
            ),
        };

        let jobs = {
            if let Some(val) = self.matches.get_one::<String>("jobs") {
                let val: u8 = val
                    .parse()
                    .map_err(|_| eyre!("Invalid parameter for option --jobs"))
                    .suggestion("Please provide a number in the range [0 .. 255]")?;
                Some(val)
            } else {
                None
            }
        };

        Ok(Data {
            json,
            tidy_file,
            build_root,
            command,
            jobs,
            ignore_warn: self.matches.get_flag("suppress-warnings"),
            // TODO: replace quiet flag with own logger implementation.
            quiet: self.matches.get_flag("quiet"),
        })
    }

    fn path_for_key(&self, key: &str, check_exists: bool) -> eyre::Result<path::PathBuf> {
        let path = self
            .matches
            .get_one::<std::path::PathBuf>(key)
            .map(std::path::PathBuf::from)
            .ok_or(eyre!(format!(
                "Could not convert parameter '{key}' to path"
            )))?;

        if check_exists {
            return utils::path_or_err(path);
        }
        Ok(path)
    }
}

impl JsonModel {
    fn schema() -> String {
        let schema = schema_for!(JsonModel);
        serde_json::to_string_pretty(&schema).unwrap()
    }

    fn load(path: impl AsRef<path::Path>) -> eyre::Result<JsonModel> {
        let json_path = utils::file_with_ext(path.as_ref(), "json", true)?;
        let json_name = json_path.to_string_lossy();

        let f = std::fs::File::open(path.as_ref())
            .wrap_err(format!("Failed to open provided JSON file '{json_name}'"))?;

        let mut json: JsonModel = serde_json::from_reader(std::io::BufReader::new(f))
            .wrap_err(format!("Validation failed for '{json_name}'"))
            .suggestion(format!(
        "Please make sure that '{json_name}' is a valid .json file and the contents match the required schema."))?;

        json.root = json_path
            .canonicalize()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();

        json.name = json_path.to_string_lossy().into();
        Ok(json)
    }
}
