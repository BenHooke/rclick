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
    let options = vec!["Search for a file (by name)", "Search for text (inside files)"];
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What are you looking for?")
        .items(&options)
        .default(0)
        .interact()?;

    match choice {
        0 => {
            // find
            let query: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("File name to search for (supports wildcards, e.g. *.txt)")
                .interact_text()?;
            println!("\nSearching from current directory...\n");
            Command::new("fing")
                .args([".", "-name", &query])
                .status()?;
        }
        1 => {
            // grep
            let query: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Text to search for")
                .interact_text()?;
            let dir: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Search in (directory -default is current dir-)")
                .default("."into())
                .interact_text()?;
            Command::new("grep")
                .args(["-r", "--color=auto", &query, &dir])
                .status()?;
        }
        _ => {}
    }
    Ok(())
}

fn action_copy_path() {
    let cwd = env::current_dir()?;
    let path_str = cwd.to_string_lossy().into_owned();

    // Try pbcopy (macOS), then xclip, then xsel, then just print
    let copied =try_copy_to_clipboard(&path_str);
    if copied {
        println!("Copied to clipboard:\n  {}", path_str);
    } else {
        println!("Error: failed to copy, copy manually:\n  {}", path_str);
    }
    Ok(())
}

fn action_history() {
    println!("Shell history is managed by your shell (bash/zsh/fish).");
    println!("To jump to a recent directory, try:");
    println!("  cd -          (go back one directory)");
    println!("  pushd / popd  (directory stack)");
    println!("  history | grep cd   (search your history)");
    Ok(())
}

fn action_rename() {
    let old_name = path.file_name().unwrap_or_default().to_string_lossy();

    let new_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("New name")
        .with_initial_text(old_name.as_ref())
        .interact_text()?;

    if new_name == old_name.as_ref() {
        println!("Name unchanged.");
        return Ok(());
    }

    let new_path = path.with_file_name(&new_name);
    if new_path.exists() {
        let overwrite = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("'{}' already exists. Overwrite?", new_name))
            .default(false)
            .interact()?;
        if !overwrite {
            println!("Cancelled");
            return Ok(());
        }
    }

    std::fs::rename(path, &new_path)?;
    println!("Renamed '{}' to '{}'.", old_name, new_name);
    Ok(())
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

