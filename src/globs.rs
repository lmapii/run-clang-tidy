use std::path;

#[allow(unused_imports)]
use color_eyre::{eyre::eyre, eyre::WrapErr, Help};

fn wrap_result<T>(result: eyre::Result<T>, field: &str, file: &str) -> eyre::Result<T> {
    result
        .wrap_err(format!("Error while parsing '{field}'"))
        .suggestion(format!(
            "Check the format of the field '{field}' in the provided file '{file}'."
        ))
}

pub fn build_matchers_from<'a, P>(
    globs: &'a [String],
    root: P,
    field: &str,
    file: &str,
) -> eyre::Result<Vec<globmatch::Matcher<'a, path::PathBuf>>>
where
    P: AsRef<path::Path>,
{
    // https://stackoverflow.com/a/33217302/7281683
    let globs: Vec<_> = globs.iter().map(|s| &**s).collect();

    wrap_result(
        globmatch::wrappers::build_matchers(&globs, root).map_err(|err| eyre!(err)),
        field,
        file,
    )
}

pub fn build_glob_set_from<'a>(
    filter: &'a Option<Vec<String>>,
    field: &str,
    file: &str,
) -> eyre::Result<Option<Vec<globmatch::GlobSet<'a>>>> {
    let filter = filter
        .as_ref()
        .map(|filter| filter.iter().map(|s| &**s).collect());

    wrap_result(
        globmatch::wrappers::build_glob_set(&filter, !cfg!(windows)).map_err(|err| eyre!(err)),
        field,
        file,
    )
}

pub fn match_paths<P>(
    candidates: Vec<globmatch::Matcher<'_, P>>,
    filter: Option<Vec<globmatch::GlobSet<'_>>>,
    filter_post: Option<Vec<globmatch::GlobSet<'_>>>,
) -> (Vec<path::PathBuf>, Vec<path::PathBuf>)
where
    P: AsRef<path::Path>,
{
    let (paths, filtered) = globmatch::wrappers::match_paths(candidates, filter, filter_post);

    log::debug!(
        "paths \n{}",
        paths
            .iter()
            .map(|p| format!("{}", p.canonicalize().unwrap().to_string_lossy()))
            .collect::<Vec<_>>()
            .join("\n")
    );

    if !filtered.is_empty() {
        log::debug!(
            "filtered \n{}",
            filtered
                .iter()
                .map(|p| format!("{}", p.canonicalize().unwrap().to_string_lossy()))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    let paths = paths
        .into_iter()
        .filter(|path| path.as_path().is_file())
        .collect(); // accept only files
    (paths, filtered)
}
