use std::path;

#[allow(unused_imports)]
use color_eyre::{eyre::eyre, eyre::WrapErr, Help};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Deserialize;

use crate::cli;
use crate::cmd;

mod globs;
mod resolve;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct JsonModel {
    pub paths: Vec<String>,
    pub filter_post: Option<Vec<String>>,
    pub style: Option<path::PathBuf>,
}

fn log_pretty() -> bool {
    // fancy logging using indicatif is only done for log level "info". when debugging we
    // do not use a progress bar, if info is not enabled at all ("quiet") then the progress
    // is also not shown
    !log::log_enabled!(log::Level::Debug) && log::log_enabled!(log::Level::Info)
}

struct LogStep(u8);

impl LogStep {
    fn new() -> LogStep {
        LogStep(1)
    }

    fn next(&mut self) -> String {
        // TODO: the actual number of steps could be determined by a macro?
        let str = format!(
            "{}",
            console::style(format!("[ {:1}/5 ]", self.0)).bold().dim()
        );
        self.0 += 1;
        if log_pretty() {
            str
        } else {
            "".to_string()
        }
    }
}

fn get_command(data: &cli::Data) -> eyre::Result<cmd::Runner> {
    let cmd_path = resolve::command(data)?;
    let mut cmd = cmd::Runner::new(&cmd_path);

    cmd.validate()
        .wrap_err(format!(
            "Failed to execute the specified command '{}'",
            cmd_path.to_string_lossy()
        ))
        .suggestion(format!(
            "Please make sure that the command '{}' exists or is in your search path",
            cmd_path.to_string_lossy()
        ))?;

    Ok(cmd)
}

fn setup_jobs(jobs: Option<u8>) -> eyre::Result<()> {
    // configure rayon to use the specified number of threads (globally)
    if let Some(jobs) = jobs {
        let jobs = if jobs == 0 { 1u8 } else { jobs };
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(jobs.into())
            .build_global();

        if let Err(err) = pool {
            return Err(err)
                .wrap_err(format!("Failed to create thread pool of size {}", jobs))
                .suggestion("Please try to decrease the number of jobs");
        }
    };
    Ok(())
}

pub fn run(data: cli::Data) -> eyre::Result<()> {
    let start = std::time::Instant::now();

    log::info!(" ");
    let mut step = LogStep::new();

    let build_root = resolve::build_root(&data)?;
    let tidy_file = resolve::tidy_file(&data)?;

    if let Some(tidy_file) = &tidy_file {
        log::info!(
            "{} Found configuration file {}",
            step.next(),
            console::style(tidy_file.to_string_lossy()).bold(),
        );
    } else {
        log::info!(
            "{} No configuration file specified, assuming .clang-tidy exists in the project tree",
            step.next()
        );
    }

    let candidates =
        globs::build_matchers_from(&data.json.paths, &data.json.root, "paths", &data.json.name)?;
    let filter_pre =
        globs::build_glob_set_from(&data.json.filter_pre, "preFilter", &data.json.name)?;
    let filter_post =
        globs::build_glob_set_from(&data.json.filter_post, "postFilter", &data.json.name)?;

    let (paths, filtered) = globs::match_paths(candidates, filter_pre, filter_post);
    let paths = paths.into_iter().map(|p| p.canonicalize().unwrap());

    let filtered = if filtered.is_empty() {
        "".to_string()
    } else {
        format!(" (filtered {} paths)", filtered.len())
    };

    log::info!(
        "{} Found {} files for the provided path patterns{}",
        step.next(),
        console::style(paths.len()).bold(),
        filtered
    );

    let cmd = get_command(&data)?;
    let cmd_path = match cmd.get_path().canonicalize() {
        Ok(path) => path,
        Err(_) => cmd.get_path(),
    };
    log::info!(
        "{} Found clang-tidy version {} using command {}",
        step.next(),
        console::style(cmd.get_version().unwrap()).bold(),
        console::style(cmd_path.to_string_lossy()).bold(),
    );

    let strip_root = None; // build_root;

    setup_jobs(data.jobs)?;
    log::info!("{} Executing clang-tidy ...\n", step.next(),);

    let pb = indicatif::ProgressBar::new(paths.len() as u64);
    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template(if console::Term::stdout().size().1 > 80 {
                "{prefix:>12.cyan.bold} [{bar:26}] {pos}/{len} {wide_msg}"
            } else {
                "{prefix:>12.cyan.bold} [{bar:26}] {pos}/{len}"
            })
            .progress_chars("=> "),
    );

    // preparation for indicatif 0.17
    // pb.set_style(
    //     indicatif::ProgressStyle::with_template(if console::Term::stdout().size().1 > 80 {
    //         "{prefix:>12.cyan.bold} [{bar:26}] {pos}/{len} {wide_msg}"
    //     } else {
    //         "{prefix:>12.cyan.bold} [{bar:26}] {pos}/{len}"
    //     })
    //     .unwrap()
    //     .progress_chars("=> "),
    // );

    if log_pretty() {
        pb.set_prefix("Running");
    }
    let paths: Vec<_> = paths.collect();

    let result: eyre::Result<()> = {
        let failures: Vec<_> = paths
            .into_par_iter()
            .map(|path| {
                // TODO: specify --fix
                let result = match cmd.run_tidy(&path, &tidy_file, &build_root, false) {
                    Ok(_) => None,
                    Err(err) => {
                        let print_path = match &strip_root {
                            None => path.clone(),
                            Some(strip) => path.strip_prefix(strip).unwrap().to_path_buf(),
                        };
                        Some((print_path, format!("{}", err)))
                    }
                };
                let (prefix, style) = match result {
                    Some(_) => ("Error", console::Style::new().red().bold()),
                    None => ("Match", console::Style::new().green().bold()),
                };
                log_step(prefix, path.as_path(), &strip_root, &pb, style);
                if let Some(err) = &result {
                    if !log_pretty() {
                        log::error!("{}", err.1);
                    }
                }
                result
            })
            .flatten()
            .collect();

        if !failures.is_empty() {
            Err(eyre::eyre!(format!(
                "Execution failed for the following files:\n{}",
                failures
                    .into_iter()
                    .map(|result| format!("{}", result.0.to_string_lossy()))
                    .collect::<Vec<_>>()
                    .join("\n")
            )))
        } else {
            Ok(())
        }
    };
    result?;

    let duration = start.elapsed();
    if log_pretty() {
        pb.finish();

        println!(
            "{:>12} in {}",
            console::Style::new().green().bold().apply_to("Finished"),
            indicatif::HumanDuration(duration)
        );
    } else {
        log::info!("{} Finished in {:#?}", step.next(), duration);
    }

    log::info!(" "); // just an empty newline
    Ok(())
}

fn log_step(
    prefix: &str,
    path: &path::Path,
    strip_path: &Option<path::PathBuf>,
    progress: &indicatif::ProgressBar,
    style: console::Style,
) {
    // let style = console::Style::new().green().bold();
    let print_path = match strip_path {
        None => path,
        Some(strip) => path.strip_prefix(strip).unwrap(),
    };

    if log_pretty() {
        progress.println(format!(
            "{:>12} {}",
            style.apply_to(prefix),
            print_path.to_string_lossy(),
        ));
        progress.inc(1);
    } else {
        log::info!("  + {}", path.to_string_lossy());
    }
}
