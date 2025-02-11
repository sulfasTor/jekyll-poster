use std::{path::PathBuf, process};

use clap::Parser;
use post::{
    post::launch_editor,
    publish::{self, commit_post},
};

pub mod post;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    post_filename: String,
    project_path: PathBuf,

    #[arg(short, long, default_value = "default")]
    layout: String,

    #[arg(short, long)]
    publish: bool,
}

fn main() {
    let args = Args::parse();

    let post_path = match launch_editor(&args.post_filename, &args.layout) {
        Ok(filename_path) => filename_path,
        Err(err) => {
            eprint!("{}", err);
            process::exit(1);
        }
    };

    match commit_post(&post_path, &args.project_path) {
        Ok(_) => println!("All fine."),
        Err(err) => println!("{err}"),
    }
}
