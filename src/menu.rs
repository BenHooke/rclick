use std::io;
use std::path::PathBuf;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};

use crate::actions::{self, FileAction, GeneralAction};

// General right click
pub fn run_general_menu() -> anyhow::Result<()> {
    let items = vec![
        ("  New file", GeneralAction::NewFile),
        ("  New directory", GeneralAction::NewDir),
        ("  Open...", GeneralAction::Open),
        ("  Search...", GeneralAction::Search),
        ("  Copy path", GeneralAction::CopyPath),
        ("  History", GeneralAction::History),
    ];

    let labels: Vec<&str> = items.iter().map(|(L, _)| *L).collect();
    let title = " rclick ";
    let subtitle = " Current directory ";

    match pick_item(&labels, title, subtitle)? {
        Some(idx) => actions::run_general(&items[idx].1),
        None => Ok(()),  // User pressed Esc or 'q'
    }
}

pub fn run_file_menu(path: &PathBuf) -> anyhow::Result<()> {
    let is_dir = path.is_dir();

    let mut items: Vec<(&str, FileAction)> = vec![
        ("  Rename", FileAction::Renam),
        ("  Copy", FileAction::Copy),
        ("  Move", FileAction::Move),
        ("  View", FileAction::View),
    ];

    if is_dir {
        items.insert(0, ("  Open", FileAction::Open));
    }

    items.push(("  Permissions", FileAction::Permissions));
    items.push(("  Compress", FileAction::Compress));
    items.push(("  Delete", FileAction::Delete));

    let labels: Vec<&str> = items.iter().map(|(L, _)| *L).collect();

    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.to_string_lossy().into_owned());

    let title = " rcick ";
    let subtitle_text = format!(" {} ", name);
    let subtitle: &str = Box::leak(subtitle_text.into_boxed_str());

    match pick_item(&labels, title, subtitle)? {
        Some(idx) => actions::run_file(&items[idx].1, path),
        None => Ok(()),
    }
}

fn pick_item() {
    todo!()
}

