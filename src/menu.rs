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

pub fn run_general_menu() {
    todo!()
}

pub fn run_file_menu() {
    todo!()
}

fn pick_item() {
    todo!()
}

