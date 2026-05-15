mod card;
mod card_img;
mod game;
mod theme;
mod ui;
mod ui_image;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::fs::File;
use std::io;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::time::{Duration, Instant};
use ui::{AppState, Cursor, Selection};
use ui_image::ImageRenderer;

fn open_tty() -> io::Result<File> {
    File::options().read(true).write(true).open("/dev/tty")
}

fn main() -> io::Result<()> {
    let tty = open_tty()?;
    let fd = tty.as_raw_fd();
    let write_fd = unsafe { libc::dup(fd) };
    if write_fd < 0 {
        return Err(io::Error::last_os_error());
    }
    let tty_write: File = unsafe { File::from_raw_fd(write_fd) };
    let _tty_hold = tty;

    enable_raw_mode()?;
    execute!(&tty_write, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(tty_write);
    let mut terminal = Terminal::new(backend)?;

    let result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = result {
        eprintln!("Error: {}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<File>>) -> io::Result<()> {
    let mut state = AppState::new();
    let mut last_click: Option<(u16, u16, Instant)> = None;
    let mut image_renderer = ImageRenderer::new();
    let mut use_image_mode = image_renderer.is_some();

    loop {
        terminal.draw(|frame| {
            if use_image_mode {
                if let Some(ref mut renderer) = image_renderer {
                    ui_image::render(frame, &state, renderer);
                } else {
                    ui::render(frame, &state);
                }
            } else {
                ui::render(frame, &state);
            }
        })?;

        if state.game.is_won() {
            state.message = Some("Congratulations! You won! Press N for new game, Q to quit.".to_string());
            terminal.draw(|frame| {
                if use_image_mode {
                    if let Some(ref mut renderer) = image_renderer {
                        ui_image::render(frame, &state, renderer);
                    } else {
                        ui::render(frame, &state);
                    }
                } else {
                    ui::render(frame, &state);
                }
            })?;
            loop {
                if event::poll(Duration::from_millis(100))? {
                    if let Event::Key(key) = event::read()? {
                        if key.kind != KeyEventKind::Press {
                            continue;
                        }
                        match key.code {
                            KeyCode::Char('n') | KeyCode::Char('N') => {
                                state.new_game();
                                break;
                            }
                            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                                return Ok(());
                            }
                            _ => {}
                        }
                    }
                }
            }
            continue;
        }

        if state.auto_completing {
            if state.game.auto_complete_available() {
                if !state.game.auto_complete_step() {
                    state.auto_completing = false;
                    state.message = Some("Auto-complete finished!".to_string());
                }
                std::thread::sleep(Duration::from_millis(120));
                continue;
            } else {
                state.auto_completing = false;
            }
        }

        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }
                    state.hint_text = None;

                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
                            return Ok(());
                        }
                        KeyCode::Char('n') | KeyCode::Char('N') => {
                            state.new_game();
                        }
                        KeyCode::Char('u') | KeyCode::Char('U') => {
                            if state.game.undo() {
                                state.selection = None;
                                state.message = Some("Undo!".to_string());
                            } else {
                                state.message = Some("Nothing to undo.".to_string());
                            }
                        }
                        KeyCode::Char('h') | KeyCode::Char('H') => {
                            state.hint_text = state.game.get_hint().or(Some("No hints available.".to_string()));
                        }
                        KeyCode::Char('a') | KeyCode::Char('A') => {
                            if state.game.auto_complete_available() {
                                state.auto_completing = true;
                                state.message = Some("Auto-completing...".to_string());
                            } else {
                                state.message = Some("Cannot auto-complete yet (all cards must be face-up).".to_string());
                            }
                        }
                        KeyCode::Char('t') | KeyCode::Char('T') => {
                            state.cycle_theme();
                        }
                        KeyCode::Char('d') | KeyCode::Char('D') => {
                            state.game.draw_from_stock();
                        }
                        KeyCode::Char('i') | KeyCode::Char('I') => {
                            if image_renderer.is_some() {
                                use_image_mode = !use_image_mode;
                                state.message = Some(if use_image_mode {
                                    "Image mode".to_string()
                                } else {
                                    "Text mode".to_string()
                                });
                            } else {
                                state.message = Some("Image mode not available (terminal doesn't support it)".to_string());
                            }
                        }
                        KeyCode::Left => {
                            state.cursor = move_cursor_left(&state.cursor);
                            state.message = None;
                        }
                        KeyCode::Right => {
                            state.cursor = move_cursor_right(&state.cursor);
                            state.message = None;
                        }
                        KeyCode::Up => {
                            state.cursor = move_cursor_up(&state.cursor);
                            state.message = None;
                        }
                        KeyCode::Down => {
                            state.cursor = move_cursor_down(&state.cursor);
                            state.message = None;
                        }
                        KeyCode::Char(' ') => {
                            handle_space(&mut state);
                        }
                        KeyCode::Enter => {
                            handle_enter(&mut state);
                        }
                        KeyCode::Char('0') => {
                            state.cursor = Cursor::Waste;
                            handle_space(&mut state);
                        }
                        KeyCode::Char(c @ '1'..='7') => {
                            let col = (c as usize) - ('1' as usize);
                            state.cursor = Cursor::Tableau(col);
                            handle_space(&mut state);
                        }
                        _ => {}
                    }
                }
                Event::Mouse(mouse) => {
                    handle_mouse(&mut state, mouse, &mut last_click);
                }
                _ => {}
            }
        }
    }
}

fn move_cursor_left(cursor: &Cursor) -> Cursor {
    match cursor {
        Cursor::Stock => Cursor::Stock,
        Cursor::Waste => Cursor::Stock,
        Cursor::Foundation(0) => Cursor::Waste,
        Cursor::Foundation(i) => Cursor::Foundation(i - 1),
        Cursor::Tableau(0) => Cursor::Tableau(0),
        Cursor::Tableau(col) => Cursor::Tableau(col - 1),
    }
}

fn move_cursor_right(cursor: &Cursor) -> Cursor {
    match cursor {
        Cursor::Stock => Cursor::Waste,
        Cursor::Waste => Cursor::Foundation(0),
        Cursor::Foundation(3) => Cursor::Foundation(3),
        Cursor::Foundation(i) => Cursor::Foundation(i + 1),
        Cursor::Tableau(6) => Cursor::Tableau(6),
        Cursor::Tableau(col) => Cursor::Tableau(col + 1),
    }
}

fn move_cursor_up(cursor: &Cursor) -> Cursor {
    match cursor {
        Cursor::Tableau(col) => {
            if *col < 2 {
                if *col == 0 {
                    Cursor::Stock
                } else {
                    Cursor::Waste
                }
            } else {
                Cursor::Foundation((*col).min(5) - 2)
            }
        }
        _ => *cursor,
    }
}

fn move_cursor_down(cursor: &Cursor) -> Cursor {
    match cursor {
        Cursor::Stock => Cursor::Tableau(0),
        Cursor::Waste => Cursor::Tableau(1),
        Cursor::Foundation(i) => Cursor::Tableau((*i + 2).min(6)),
        _ => *cursor,
    }
}

fn handle_space(state: &mut AppState) {
    state.message = None;

    match state.cursor {
        Cursor::Stock => {
            state.selection = None;
            state.game.draw_from_stock();
        }
        Cursor::Waste => {
            if state.game.waste.is_empty() {
                return;
            }
            if state.selection == Some(Selection::Waste) {
                state.selection = None;
            } else {
                state.selection = Some(Selection::Waste);
            }
        }
        Cursor::Foundation(i) => {
            if let Some(sel) = &state.selection {
                match sel {
                    Selection::Waste => {
                        if state.game.move_waste_to_foundation() {
                            state.selection = None;
                        } else {
                            state.message = Some("Invalid move.".to_string());
                        }
                    }
                    Selection::Tableau(col, _) => {
                        let col = *col;
                        if state.game.move_tableau_to_foundation(col) {
                            state.selection = None;
                        } else {
                            state.message = Some("Invalid move.".to_string());
                        }
                    }
                    _ => {
                        state.selection = None;
                    }
                }
            } else if !state.game.foundations[i].is_empty() {
                state.selection = Some(Selection::Foundation(i));
            }
        }
        Cursor::Tableau(col) => {
            if let Some(sel) = state.selection.clone() {
                let success = match sel {
                    Selection::Waste => state.game.move_waste_to_tableau(col),
                    Selection::Foundation(i) => state.game.move_foundation_to_tableau(i, col),
                    Selection::Tableau(from_col, card_idx) => {
                        state.game.move_tableau_to_tableau(from_col, card_idx, col)
                    }
                };
                if success {
                    state.selection = None;
                } else {
                    if let Selection::Tableau(from_col, _) = sel {
                        if from_col == col {
                            select_tableau_card(state, col);
                            return;
                        }
                    }
                    state.message = Some("Invalid move.".to_string());
                }
            } else {
                select_tableau_card(state, col);
            }
        }
    }
}

fn select_tableau_card(state: &mut AppState, col: usize) {
    let pile = &state.game.tableau[col];
    if pile.is_empty() {
        return;
    }
    for (idx, card) in pile.iter().enumerate() {
        if card.face_up {
            state.selection = Some(Selection::Tableau(col, idx));
            return;
        }
    }
}

fn handle_enter(state: &mut AppState) {
    state.message = None;

    match state.cursor {
        Cursor::Stock => {
            state.game.draw_from_stock();
        }
        Cursor::Waste => {
            if state.game.move_waste_to_foundation() {
                return;
            }
            for col in 0..7 {
                if state.game.move_waste_to_tableau(col) {
                    return;
                }
            }
            state.message = Some("No valid move for this card.".to_string());
        }
        Cursor::Tableau(col) => {
            if state.game.move_tableau_to_foundation(col) {
                return;
            }
            let pile = &state.game.tableau[col];
            if pile.is_empty() {
                state.message = Some("Empty column.".to_string());
                return;
            }
            let first_face_up = pile.iter().position(|c| c.face_up);
            if let Some(card_idx) = first_face_up {
                for to_col in 0..7 {
                    if to_col == col {
                        continue;
                    }
                    if state.game.move_tableau_to_tableau(col, card_idx, to_col) {
                        return;
                    }
                }
            }
            state.message = Some("No valid auto-move.".to_string());
        }
        Cursor::Foundation(i) => {
            for col in 0..7 {
                if state.game.move_foundation_to_tableau(i, col) {
                    return;
                }
            }
            state.message = Some("No valid move.".to_string());
        }
    }
}

fn handle_mouse(
    state: &mut AppState,
    mouse: crossterm::event::MouseEvent,
    last_click: &mut Option<(u16, u16, Instant)>,
) {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            let x = mouse.column;
            let y = mouse.row;
            let now = Instant::now();

            state.hint_text = None;
            state.message = None;

            let is_double_click = if let Some((lx, ly, lt)) = last_click {
                now.duration_since(*lt) < Duration::from_millis(400)
                    && (*lx as i16 - x as i16).unsigned_abs() <= 2
                    && (*ly as i16 - y as i16).unsigned_abs() <= 1
            } else {
                false
            };
            *last_click = Some((x, y, now));

            let col_width = 16u16;
            let tableau_start_y = 11u16;
            let top_row_y_start = 2u16;
            let top_row_y_end = 10u16;

            if y >= top_row_y_start && y < top_row_y_end {
                let offset_x = x.saturating_sub(1);
                if offset_x < col_width {
                    state.cursor = Cursor::Stock;
                    if is_double_click {
                        handle_enter(state);
                    } else {
                        handle_space(state);
                    }
                    return;
                } else if offset_x < col_width * 2 + 1 {
                    state.cursor = Cursor::Waste;
                    if is_double_click {
                        handle_enter(state);
                    } else {
                        handle_space(state);
                    }
                    return;
                } else {
                    let found_start = col_width * 2 + 5;
                    if x >= found_start {
                        let idx = ((x - found_start) / col_width) as usize;
                        if idx < 4 {
                            state.cursor = Cursor::Foundation(idx);
                            if is_double_click {
                                handle_enter(state);
                            } else {
                                handle_space(state);
                            }
                            return;
                        }
                    }
                }
            }

            if y >= tableau_start_y {
                let offset_x = x.saturating_sub(1);
                let col = (offset_x / col_width) as usize;
                if col < 7 {
                    state.cursor = Cursor::Tableau(col);
                    if is_double_click {
                        state.selection = None;
                        handle_enter(state);
                    } else {
                        handle_space(state);
                    }
                }
            }
        }
        MouseEventKind::Down(MouseButton::Right) => {
            state.selection = None;
            state.message = None;
        }
        _ => {}
    }
}
