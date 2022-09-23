#![warn(rust_2018_idioms)]

pub mod cli;
pub mod cmd;
mod lib;

fn main() -> eyre::Result<()> {
    // println!(
    //     "Executing \n{} from \n{}\n",
    //     std::env::current_exe().unwrap().to_string_lossy(),
    //     std::env::current_dir().unwrap().to_string_lossy()
    // );

    let data = cli::Builder::build().parse()?;
    lib::run(data)
}
