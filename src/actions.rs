use std::env;
use std::path::PathBuf;
use std::process::Command;

use dialoguer::{Confirm, Input, Select, theme::ColorfulTheme};

#[derive(Debug)]
pub enum GeneralAction {
    NewFile,
    NewDir,
    Open,
    Search,
    CopyPath,
    History,
}

#[derive(Debug)]
pub enum FileAction {
    Rename,
    Copy,
    Move,
    Open,
    View,
    Permissions,
    Compress,
    Delete,
}

// Menu when ran with no args
pub fn run_general() {
    println!();
    match action {
        GeneralAction::NewFile => action_new_file(),
        GeneralAction::NewDir => action_new_dir(),
        GeneralAction::Open => action_open_picker(),
        GeneralAction::Search => action_search(),
        GeneralAction::CopyPath => action_copy_path(),
        GeneralAction::History => action_history(),
    }
}

// Menu when user specifies a file
pub fn run_file() {
    println!();
    match action {
        FileAction::Rename => action_rename(path),
        FileAction::Copy => action_copy(path),
        FileAction::Move => action_move(path),
        FileAction::Open => action_open(path),
        FileAction::View => action_view(path),
        FileAction::Permissions => action_permissions(path),
        FileAction::Compress => action_compress(path),
        FileAction::Delete => action_delete(path),
    }
}

fn action_new_file() {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("New file name")
        .interact_text()?;

    let path = PathBuf::from(&name);
    if path.exists() {
        eprintln!("'{}' already exists.", name);
        return Ok(());
    }

    std::fs::File::create(&path)?;
    println!("Created '{}'", name);
    Ok(())
}

fn action_new_dir() {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("New folder name")
        .interact_text()?;

    std::fs::create_dir_all(&name)?;
    println!("Created directory '{}'", name);
    Ok(())
}

fn action_open_picker() {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Path to open")
        .interact_text()?;

    let path = PathBuf::from(&name);
    action_open(&path)
}

fn action_search() {
    todo!()
}

fn action_copy_path() {
    todo!()
}

fn action_history() {
    todo!()
}

fn action_rename() {
    todo!()
}

fn action_copy() {
    todo!()
}

fn action_move() {
    todo!()
}

fn action_open() {
    todo!()
}

fn action_view() {
    todo!()
}

fn action_permissions() {
    todo!()
}

fn action_compress() {
    todo!()
}

fn action_delete() {
    todo!()
}

// Helpers

fn is_text_file() {
    todo!()
}

fn try_copy_to_clipboard() {
    todo!()
}

fn shell_escape() {
    todo!()
}

