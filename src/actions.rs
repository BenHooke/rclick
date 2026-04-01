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

pub fn run_general() {
    todo!()
}

pub fn run_file() {
    todo!()
}

fn action_new_file() {
    todo!()
}

fn action_new_dir() {
    todo!()
}

fn action_open_picker() {
    todo!()
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

