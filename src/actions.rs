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
pub fn run_general(action: &GeneralAction) -> anyhow::Result<()> {
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
pub fn run_file(action: &FileAction, path: &PathBuf) -> anyhow::Result<()> {
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

fn action_new_file() -> anyhow::Result<()> {
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

fn action_new_dir() -> anyhow::Result<()> {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("New folder name")
        .interact_text()?;

    std::fs::create_dir_all(&name)?;
    println!("Created directory '{}'", name);
    Ok(())
}

fn action_open_picker() -> anyhow::Result<()> {
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Path to open")
        .interact_text()?;

    let path = PathBuf::from(&name);
    action_open(&path)
}

fn action_search() -> anyhow::Result<()> {
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
            Command::new("find")
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
                .default(".".into())
                .interact_text()?;
            Command::new("grep")
                .args(["-rIn", "--color=auto", &query, &dir])
                .status()?;
        }
        _ => {}
    }
    Ok(())
}

fn action_copy_path() -> anyhow::Result<()> {
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

fn action_history() -> anyhow::Result<()> {
    println!("Shell history is managed by your shell (bash/zsh/fish).");
    println!("To jump to a recent directory, try:");
    println!("  cd -          (go back one directory)");
    println!("  pushd / popd  (directory stack)");
    println!("  history | grep cd   (search your history)");
    Ok(())
}

fn action_rename(path: &PathBuf) -> anyhow::Result<()> {
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

fn action_copy(path: &PathBuf) -> anyhow::Result<()> {
    let dest: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Copy to (path or directory)")
        .interact_text()?;

    let status = if path.is_dir() {
        Command::new("cp").args(["-r", &path.to_string_lossy(), &dest]).status()?
    } else {
        Command::new("cp").args([&path.to_string_lossy() as &str, &dest]).status()?
    };

    if status.success() {
        println!("Copied to '{}'.", dest);
    }
    Ok(())
}

fn action_move(path: &PathBuf) -> anyhow::Result<()> {
    let dest: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Move to (path or directory)")
        .interact_text()?;

    let status = Command::new("mv")
        .args([&path.to_string_lossy() as &str, &dest])
        .status()?;

    if status.success() {
        println!("Moved to '{}'.", dest);
    }
    Ok(())
}

fn action_open(path: &PathBuf) -> anyhow::Result<()> {
    if path.is_dir() {
        // We can't cd the parent shell, so we peint the command and advise the user
        let abs = path.canonicalize().unwrap_or_else(|_| path.clone());
        // Write the cd target to a temp file the shell wrapper can read
        if let Ok(tmp) = std::env::var("RCLICK_CD_FILE") {
            std::fs::write(&tmp, abs.to_string_lossy().as_bytes())?;
        } else {
            println!("To open this directory, run:");
            println!("  cd {}", shell_escape(&abs.to_string_lossy()));
        }
        return Ok(());
    }

    // Detect text files by extension and open in $EDITOR
    if is_text_file(path) {
        let editor = env::var("EDITOR").unwrap_or_else(|_| "nano".into());
        Command::new(&editor).arg(path).status()?;
        return Ok(());
    }

    // Fall back to xdg-open (Linux) or open (macOS)
    let opener = if cfg!(target_os = "macos") { "open" } else { "xdg-open" };
    Command::new(opener).arg(path).status()?;
    Ok(())
}

fn action_view(path: &PathBuf) -> anyhow::Result<()> {
    if path.is_dir() {
        Command::new("ls").args(["-lah", &path.to_string_lossy() as &str]).status()?;
        return Ok(());
    }

    // Try bat first, fall back to less
    let pager = if Command::new("bat").arg("--version").output().is_ok() {
        "bat"
    } else {
        "less"
    };

    Command::new(pager).arg(path).status()?;
    Ok(())
}

fn action_permissions(path: &PathBuf) -> anyhow::Result<()> {
    // Show current permissions
    Command::new("ls").args(["-la", &path.to_string_lossy() as &str]).status()?;
    println!();

    let presets = vec![
        ("644  Owner: read/write   Others: read only  (typical file)", "644"),
        ("755  Owner: read/write/exec   Others: read/exec  (typical program)", "755"),
        ("600  Owner: read/write   Others: none  (private file)", "600"),
        ("777  Everyone: full access  (not recommended)", "777"),
        ("Enter a custom value", "custom"),
    ];

    let labels: Vec<&str> = presets.iter().map(|(L, _)| *L).collect();
    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("New permissions")
        .items(&labels)
        .default(0)
        .interact()?;

    let mode = if presets[choice].1 == "custom" {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter octal permissions (e.g. 744)")
            .interact_text()?
    } else {
        presets[choice].1.to_string()
    };

    Command::new("chmod")
        .args([&mode, &path.to_string_lossy() as &str])
        .status()?;
    println!("Permissions set to {}.", mode);
    Ok(())
}

fn action_compress(path: &PathBuf) -> anyhow::Result<()> {
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let default_archive = format!("{}.tar.gz", name);

    let archive_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Archive name")
        .with_initial_text(&default_archive)
        .interact_text()?;

    let status = Command::new("tar")
        .args(["-czf", &archive_name, &path.to_string_lossy() as &str])
        .status()?;

    if status.success() {
        println!("Created '{}'.", archive_name);
    }
    Ok(())
}

fn action_delete(path: &PathBuf) -> anyhow::Result<()> {
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let kind = if path.is_dir() { "folder" } else { "file" };

    println!("!!! This will permanently delete the {} '{}' !!!", kind, name);

    let confirmed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Are you sure?")
        .default(false)
        .interact()?;

    if !confirmed {
        println!("Cancelled.");
        return Ok(());
    }

    if path.is_dir() {
        std::fs::remove_dir_all(path)?;
    } else {
        std::fs::remove_file(path)?;
    }

    println!("Deleted '{}'", name);
    Ok(())
}

// Helper funtions

fn is_text_file(path: &PathBuf) -> bool {
    let text_extensions = [
        "txt", "md", "rs", "py", "js", "ts", "jsx", "tsx", "html", "css",
        "json", "toml", "yaml", "yml", "sh", "bash", "zsh", "fish", "env",
        "gitignore", "dockerfile", "makefile", "c", "cpp", "h", "go", "rb",
        "java", "kt", "swift", "xml", "csv", "log", "conf", "ini", "cfg",
    ];

    match path.extension() {
        Some(ext) => text_extensions.contains(&ext.to_string_lossy().to_lowercase().as_str()),
        None => {
            // No extension - check the filename itself
            let name = path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase();
            matches!(name.as_str(), "makefile" | "dockerfile" | "readme" | "license")
        }
    }
}

fn try_copy_to_clipboard(text: &str) -> bool {
    // macOS
    if let Ok(mut child) = Command::new("pbcopy").stdin(std::process::Stdio::piped()).spawn() {
        if let Some(stdin) = child.stdin.as_mut() {
            use std::io::Write;
            let _ = stdin.write_all(text.as_bytes());
        }
        return child.wait().map(|s| s.success()).unwrap_or(false);
    }
    // Linux (xclip)
    if let Ok(mut child) = Command::new("xclip")
        .args(["-selection", "clipboard"])
        .stdin(std::process::Stdio::piped())
        .spawn()
    {
        if let Some(stdin) = child.stdin.as_mut() {
            use std::io::Write;
            let _ = stdin.write_all(text.as_bytes());
        }
        return child.wait().map(|s| s.success()).unwrap_or(false);
    }
    // Linux (xsel)
    if let Ok(mut child) = Command::new("xsel")
        .args(["--clipboard", "--input"])
        .stdin(std::process::Stdio::piped())
        .spawn()
    {
        if let Some(stdin) = child.stdin.as_mut() {
            use std::io::Write;
            let _ = stdin.write_all(text.as_bytes());
        }
        return child.wait().map(|s| s.success()).unwrap_or(false);
    }
    false
}

fn shell_escape(s: &str) -> String {
    if s.contains(' ') || s.contains('\'') {
        format!("'{}'", s.replace('\'', r"'\''"))
    } else {
        s.to_string()
    }
}

