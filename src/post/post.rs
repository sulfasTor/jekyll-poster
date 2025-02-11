use anyhow::{anyhow, Result};
use chrono::Local;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

const DEFAULT_EDITOR: &str = "vi";

fn write_temp_file(filename: &Path, layout: &str, date: &str) -> io::Result<()> {
    let fm = format!(
        "---
layout: {}
title: TITLE
excerpt_separator: <!--more-->
date: {}
tag:
---
# TL;DR
<!--more-->
# TITLE
",
        layout, date,
    );
    fs::write(filename, fm.as_bytes())
}

fn get_or_create_post_path(filename: &str) -> Result<PathBuf> {
    let filename_path = PathBuf::from(format!("{}-{}.md", Local::now().format("%Y-%m-%d"), filename));
    let post_dir_path = PathBuf::from("_posts");
    if !post_dir_path.is_dir() {
        fs::create_dir(&post_dir_path)?
    }
    let filename_path = post_dir_path.join(filename_path);
    Ok(filename_path)
}

pub fn launch_editor(filename: &str, layout: &str) -> Result<PathBuf> {
    let filename_path = get_or_create_post_path(&filename)?;
    write_temp_file(
        &filename_path,
        &layout,
        &Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
    )
    .map_err(|err| anyhow!("Error creating post").context(err))?;

    let editor = match env::var("EDITOR") {
        Ok(editor) => editor,
        Err(_) => DEFAULT_EDITOR.to_string(),
    };

    Command::new(editor)
        .arg(&filename)
        .status()
        .map_err(|err| anyhow!("Error creating post").context(err))?;

    Ok(filename_path)
}
