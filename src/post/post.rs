use crate::post::errors::PostError;
use chrono::Local;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

const DEFAULT_EDITOR: &str = "vi";
const TEMP_FILE_POST: &str = ".JEKYLL_EDITPOST";

fn write_temp_file(layout: &str, date: &str) -> io::Result<()> {
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
    fs::write(TEMP_FILE_POST, fm.as_bytes())
}

pub fn launch_editor(filename: Option<PathBuf>, layout: &str) -> Result<String, PostError> {
    let date_now = Local::now();
    write_temp_file(layout, &date_now.format("%Y-%m-%d %H:%M:%S").to_string())
        .map_err(|_| PostError)?;

    let editor = match env::var("EDITOR") {
        Ok(editor) => editor,
        Err(_) => DEFAULT_EDITOR.to_string(),
    };

    Command::new(editor)
        .arg(TEMP_FILE_POST)
        .status()
        .map_err(|_| PostError)?;

    Ok(format!(
        "{}-{}.md",
        date_now.format("%Y-%m-%d"),
        filename.unwrap().into_os_string().into_string().unwrap()
    ))
}
