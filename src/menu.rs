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
        ("  Browse files", GeneralAction::Browse),
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
        Some(idx) => {
            if matches!(items[idx].1, GeneralAction::Browse) {
                run_browse_menu()
            } else {
                actions::run_general(&items[idx].1)
            }
        }
        None => Ok(()),  // User pressed Esc or 'q'
    }
}

pub fn run_browse_menu() -> anyhow::Result<()> {
    let cwd = std::env::current_dir()?;

    let mut entries: Vec<PathBuf> = std::fs::read_dir(&cwd)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .collect();

    entries.sort_by(|a, b| {
        let a_dir = a.is_dir();
        let b_dir = b.is_dir();
        match (a_dir, b_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.file_name().cmp(&b.file_name()),
        }
    });

    if entries.is_empty() {
        println!("Directory is empty.");
        return Ok(());
    }

    // Build display labels with a [dir] indicator
    let labels: Vec<String> = entries
        .iter()
        .map(|p| {
            let name = p.file_name().unwrap_or_default().to_string_lossy();
            if p.is_dir() {
                format!("   {}", name)
            } else {
                format!("   {}", name)
            }
        })
        .collect();

    let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();

    let dir_name = cwd.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "/".to_string());
    let subtitle_text = format!(" {} ", dir_name);
    let subtitle: &str = Box::leak(subtitle_text.into_boxed_str());

    match pick_item(&label_refs, " browse ", subtitle)? {
        Some(idx) => run_file_menu(&entries[idx]),
        None => Ok(()),
    }
}

pub fn run_file_menu(path: &PathBuf) -> anyhow::Result<()> {
    let is_dir = path.is_dir();

    let mut items: Vec<(&str, FileAction)> = vec![
        ("  Rename", FileAction::Rename),
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

fn pick_item(items: &[&str], title: &str, subtitle: &str) -> anyhow::Result<Option<usize>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut state = ListState::default();
    state.select(Some(0));

    let result = loop {
        terminal.draw(|frame| {
            let area = frame.size();

            // Display a pop-up in the center of terminal
            let popup_width = 40u16.min(area.width.saturating_sub(4));
            let popup_height = (items.len() as u16 + 6).min(area.height.saturating_sub(4));
            let x = (area.width.saturating_sub(popup_width)) / 2;
            let y = (area.height.saturating_sub(popup_height)) / 2;

            let popup_area = ratatui::layout::Rect::new(x,y, popup_width, popup_height);

            // Outer block
            let block = Block::default()
                .title(Line::from(vec![
                        Span::styled(title, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                ]))
                .title_alignment(Alignment::Center)
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray));

            let inner = block.inner(popup_area);
            frame.render_widget(block, popup_area);

            // Layout - subtitle + list + hint
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(2),
                    Constraint::Min(1),
                    Constraint::Length(1),
                ])
                .split(inner);

            // Subtitle / context line
            let sub = Paragraph::new(Line::from(vec![Span::styled(
                    subtitle,
                    Style::default().fg(Color::Yellow),
                )]))
                .alignment(Alignment::Center);
            frame.render_widget(sub, chunks[0]);

            // Menu list
            let list_items: Vec<ListItem> = items
                .iter()
                .map(|label| {
                    ListItem::new(Line::from(vec![Span::raw(*label)]))
                        .style(Style::default().fg(Color::White))
                })
                .collect();

            let list = List::new(list_items)
                .highlight_style(
                    Style::default()
                        .bg(Color::Cyan)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("▶ ");

            frame.render_stateful_widget(list, chunks[1], &mut state);

            // Hint line
            let hint = Paragraph::new(Line::from(vec![Span::styled(
                    " ↑↓ navigate  enter select  esc cancel ",
                    Style::default().fg(Color::DarkGray),
                )]))
                .alignment(Alignment::Center);
            frame.render_widget(hint, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Esc | KeyCode::Char('q') => break Ok(None),
                KeyCode::Enter => {
                    break Ok(state.selected());
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    let i = state.selected().unwrap_or(0);
                    state.select(Some((i + 1).min(items.len() - 1)));
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    let i = state.selected().unwrap_or(0);
                    state.select(Some(i.saturating_sub(1)));
                }
                _ => {}
            }
        }
    };

    // Always restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

