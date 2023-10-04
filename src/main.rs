use clap::{Parser, Subcommand};
use std::path::PathBuf;
use directories::UserDirs;


#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short = 'p', long ,env)]
    garden_path: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Commands,
}
#[derive(Subcommand, Debug)]
enum Commands {
    /// write something in your garden
    ///
    /// This command will open your $EDITOR, wait for you
    /// to write something, and then save the file to your garden
    Write {
        #[clap(short, long)]
        title: Option<String>,
    },
}

fn get_default_garden_dir() -> Option<PathBuf> {
UserDirs::new()
    .map(|dirs| dirs.home_dir().join("garden"))
}

fn main() {
    let args = Args::parse();
    dbg!(args);
}
