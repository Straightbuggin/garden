use clap::{error::ErrorKind, CommandFactory, Parser, Subcommand};
use directories::UserDirs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short = 'p', long, env)]
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
    UserDirs::new().map(|dirs| dirs.home_dir().join("garden"))
}

fn main()->Result<(), std::io::Error> {
    let args = Args::parse();

    let Some(garden_path) =
        args.garden_path.or_else(get_default_garden_dir)
        else {
            let mut cmd = Args::command();
            cmd.error(
                ErrorKind::ValueValidation,
                format!(
                    "garden directory not provided and home directory unavailable for default garden directory"
                ),
            )
                .exit()
        };
    if !garden_path.exists() {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!(
                "garden directory `{}` doesn't exist, or is inaccessible",
                garden_path.display()
            ),
        )
        .exit()
    };

    match args.cmd {
        Commands::Write { title }  => {
            garden::write(garden_path, title)
        }
    }
}
