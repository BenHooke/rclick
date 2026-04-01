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

pub fn run_file_menu() {
    todo!()
}

fn pick_item() {
    todo!()
}

