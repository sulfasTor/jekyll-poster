use std::{path::PathBuf, process};

use clap::Parser;
use post::{edit::launch_editor, publish::add_and_commit_post};

pub mod post;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    post_id_name: String,
    project_path: PathBuf,

    #[arg(short, long, default_value = "default")]
    layout: String,

    #[arg(short, long)]
    publish: bool,
}

fn main() {
    let args = Args::parse();

    let post_path = match launch_editor(&args.post_id_name, &args.project_path, &args.layout) {
        Ok(filename_path) => filename_path,
        Err(err) => {
            eprint!("{}", err);
            process::exit(1);
        }
    };

    if args.publish {
        match add_and_commit_post(&post_path, &args.project_path) {
            Ok(_) => println!("Succesfully published new post."),
            Err(err) => println!("{err}"),
        }
    }
}
