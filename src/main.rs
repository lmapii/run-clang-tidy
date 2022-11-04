#![warn(rust_2018_idioms)]

fn main() -> eyre::Result<()> {
    let data = run_clang_tidy::cli::Builder::build().parse()?;
    run_clang_tidy::run(data)
}
