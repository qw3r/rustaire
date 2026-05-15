use crate::card::Card;
use crate::game::Game;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cursor {
    Stock,
    Waste,
    Foundation(usize),
    Tableau(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Selection {
    Waste,
    Foundation(usize),
    Tableau(usize, usize),
}

pub struct AppState {
    pub game: Game,
    pub cursor: Cursor,
    pub selection: Option<Selection>,
    pub hint_text: Option<String>,
    pub message: Option<String>,
    pub auto_completing: bool,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            game: Game::new(),
            cursor: Cursor::Tableau(0),
            selection: Option::None,
            hint_text: None,
            message: None,
            auto_completing: false,
        }
    }

    pub fn new_game(&mut self) {
        self.game = Game::new();
        self.cursor = Cursor::Tableau(0);
        self.selection = None;
        self.hint_text = None;
        self.message = None;
        self.auto_completing = false;
    }
}

const CARD_WIDTH: u16 = 14;
const CARD_HEIGHT: u16 = 7;

pub fn render(frame: &mut Frame, state: &AppState) {
    let area = frame.area();

    let bg = Style::default().bg(Color::Rgb(0, 80, 40));
    let bg_block = Block::default().style(bg);
    frame.render_widget(bg_block, area);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(CARD_HEIGHT + 2),
            Constraint::Min(12),
            Constraint::Length(3),
        ])
        .split(area);

    render_title_bar(frame, main_layout[0], state);
    render_top_row(frame, main_layout[1], state);
    render_tableau(frame, main_layout[2], state);
    render_status_bar(frame, main_layout[3], state);
}

fn render_title_bar(frame: &mut Frame, area: Rect, _state: &AppState) {
    let title = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ♠ ♥ ♣ ♦  ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(
                "K L O N D I K E",
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("  ♦ ♣ ♥ ♠  ", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::styled(
                "│ ",
                Style::default().fg(Color::Rgb(60, 120, 80)),
            ),
            Span::styled(
                " H",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled("int ", Style::default().fg(Color::Rgb(180, 180, 180))),
            Span::styled(
                " U",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled("ndo ", Style::default().fg(Color::Rgb(180, 180, 180))),
            Span::styled(
                " N",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled("ew ", Style::default().fg(Color::Rgb(180, 180, 180))),
            Span::styled(
                " A",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled("uto ", Style::default().fg(Color::Rgb(180, 180, 180))),
            Span::styled(
                " Q",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled("uit", Style::default().fg(Color::Rgb(180, 180, 180))),
        ]),
    ])
    .style(Style::default().bg(Color::Rgb(20, 20, 40)));
    frame.render_widget(title, area);
}

fn render_top_row(frame: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(CARD_WIDTH + 2),
            Constraint::Length(2),
            Constraint::Length(CARD_WIDTH + 2),
            Constraint::Min(6),
            Constraint::Length(CARD_WIDTH + 2),
            Constraint::Length(2),
            Constraint::Length(CARD_WIDTH + 2),
            Constraint::Length(2),
            Constraint::Length(CARD_WIDTH + 2),
            Constraint::Length(2),
            Constraint::Length(CARD_WIDTH + 2),
            Constraint::Min(0),
        ])
        .split(area);

    let stock_selected = state.cursor == Cursor::Stock;
    render_stock(frame, chunks[1], state, stock_selected);

    let waste_selected = state.cursor == Cursor::Waste;
    let waste_picked = state.selection == Some(Selection::Waste);
    render_waste(frame, chunks[3], state, waste_selected, waste_picked);

    for i in 0..4 {
        let found_selected = state.cursor == Cursor::Foundation(i);
        let found_picked = state.selection == Some(Selection::Foundation(i));
        render_foundation(frame, chunks[5 + i * 2], state, i, found_selected, found_picked);
    }
}

fn render_stock(frame: &mut Frame, area: Rect, state: &AppState, selected: bool) {
    let border_style = if selected {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Rgb(60, 120, 80))
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(Span::styled(" Stock ", Style::default().fg(Color::Rgb(140, 140, 140))));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if state.game.stock.is_empty() && state.game.waste.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled("    ∅", Style::default().fg(Color::Rgb(80, 80, 80)))),
        ]);
        frame.render_widget(empty, inner);
    } else if state.game.stock.is_empty() {
        let recycle = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled("    ↺", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(Span::styled("  recycle", Style::default().fg(Color::Rgb(120, 120, 120)))),
        ]);
        frame.render_widget(recycle, inner);
    } else {
        let count = format!("     [{}]", state.game.stock.len());
        let card_back = Paragraph::new(vec![
            Line::from(Span::styled("┌────────────┐", Style::default().fg(Color::Rgb(70, 70, 200)))),
            Line::from(Span::styled("│▓▓▓▓▓▓▓▓▓▓▓▓│", Style::default().fg(Color::Rgb(70, 70, 200)))),
            Line::from(Span::styled("│▓▓▓░░░░░▓▓▓▓│", Style::default().fg(Color::Rgb(70, 70, 200)))),
            Line::from(Span::styled("│▓▓▓▓▓▓▓▓▓▓▓▓│", Style::default().fg(Color::Rgb(70, 70, 200)))),
            Line::from(Span::styled("└────────────┘", Style::default().fg(Color::Rgb(70, 70, 200)))),
            Line::from(Span::styled(count, Style::default().fg(Color::Rgb(120, 120, 120)))),
        ]);
        frame.render_widget(card_back, inner);
    }
}

fn render_waste(frame: &mut Frame, area: Rect, state: &AppState, selected: bool, picked: bool) {
    let border_style = if picked {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    } else if selected {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Rgb(60, 120, 80))
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(Span::styled(" Waste ", Style::default().fg(Color::Rgb(140, 140, 140))));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if let Some(card) = state.game.waste.last() {
        render_card_face_large(frame, inner, card, picked);
    } else {
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled("    —", Style::default().fg(Color::Rgb(80, 80, 80)))),
        ]);
        frame.render_widget(empty, inner);
    }
}

fn render_foundation(
    frame: &mut Frame,
    area: Rect,
    _state: &AppState,
    idx: usize,
    selected: bool,
    picked: bool,
) {
    let border_style = if picked {
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
    } else if selected {
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Rgb(60, 120, 80))
    };

    let suit_names = ["♥", "♦", "♣", "♠"];
    let title = format!(" {} ", suit_names[idx]);
    let title_color = if idx < 2 { Color::Red } else { Color::White };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(Span::styled(title, Style::default().fg(title_color)));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if let Some(card) = _state.game.foundations[idx].last() {
        render_card_face_large(frame, inner, card, picked);
    } else {
        let suit_symbols = ["♥", "♦", "♣", "♠"];
        let suit_colors = [Color::Rgb(180, 60, 60), Color::Rgb(180, 60, 60), Color::Rgb(120, 120, 120), Color::Rgb(120, 120, 120)];
        let placeholder = Paragraph::new(vec![
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled(
                format!("    {}", suit_symbols[idx]),
                Style::default().fg(suit_colors[idx]).add_modifier(Modifier::DIM),
            )),
        ]);
        frame.render_widget(placeholder, inner);
    }
}

fn render_card_face_large(frame: &mut Frame, area: Rect, card: &Card, highlighted: bool) {
    let color = if card.is_red() {
        Color::Rgb(255, 80, 80)
    } else {
        Color::White
    };
    let bg = if highlighted {
        Color::Rgb(40, 60, 40)
    } else {
        Color::Rgb(30, 30, 30)
    };
    let style = Style::default().fg(color).bg(bg);
    let rank = card.rank.symbol();
    let suit = card.suit.symbol();

    let lines = vec![
        Line::from(Span::styled(format!(" {:<2}           ", rank), style)),
        Line::from(Span::styled(format!(" {}            ", suit), style)),
        Line::from(Span::styled(format!("              "), style)),
        Line::from(Span::styled(format!("      {}       ", suit), style)),
        Line::from(Span::styled(format!("              "), style)),
        Line::from(Span::styled(format!("            {} ", suit), style)),
        Line::from(Span::styled(format!("           {:>2} ", rank), style)),
    ];

    let p = Paragraph::new(lines);
    frame.render_widget(p, area);
}

fn render_tableau(frame: &mut Frame, area: Rect, state: &AppState) {
    let col_width = CARD_WIDTH + 2;
    let constraints: Vec<Constraint> = std::iter::once(Constraint::Length(2))
        .chain((0..7).map(|_| Constraint::Length(col_width)))
        .chain(std::iter::once(Constraint::Min(0)))
        .collect();

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for col in 0..7 {
        let col_selected = state.cursor == Cursor::Tableau(col);
        render_tableau_column(frame, columns[col + 1], state, col, col_selected);
    }
}

fn render_tableau_column(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
    col: usize,
    selected: bool,
) {
    let pile = &state.game.tableau[col];
    let border_style = if selected {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::Rgb(60, 120, 80))
    };

    let col_label = format!(" {} ", col + 1);
    let block = Block::default()
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .border_style(border_style)
        .title(Span::styled(
            col_label,
            if selected {
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(100, 100, 100))
            },
        ));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if pile.is_empty() {
        if selected {
            let empty = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(
                    "   [  ]",
                    Style::default().fg(Color::Rgb(60, 120, 80)),
                )),
            ]);
            frame.render_widget(empty, inner);
        }
        return;
    }

    let mut y = 0u16;
    for (idx, card) in pile.iter().enumerate() {
        if y >= inner.height {
            break;
        }
        let is_selected_card = match state.selection {
            Some(Selection::Tableau(c, i)) => c == col && idx >= i,
            _ => false,
        };

        if card.face_up {
            let face_up_height = if idx == pile.len() - 1 { 5u16 } else { 2u16 };
            let card_area = Rect {
                x: inner.x,
                y: inner.y + y,
                width: inner.width.min(CARD_WIDTH),
                height: face_up_height.min(inner.height.saturating_sub(y)),
            };

            let color = if card.is_red() {
                Color::Rgb(255, 80, 80)
            } else {
                Color::White
            };
            let bg = if is_selected_card {
                Color::Rgb(40, 70, 40)
            } else {
                Color::Rgb(30, 30, 30)
            };
            let style = Style::default().fg(color).bg(bg);
            let rank = card.rank.symbol();
            let suit = card.suit.symbol();

            let mut lines = vec![Line::from(Span::styled(
                format!(" {:<2} {}         ", rank, suit),
                style,
            ))];
            if card_area.height >= 2 {
                lines.push(Line::from(Span::styled(
                    format!("      {}        ", suit),
                    style,
                )));
            }
            if card_area.height >= 3 {
                lines.push(Line::from(Span::styled(
                    format!("        {}      ", suit),
                    style,
                )));
            }
            if card_area.height >= 4 {
                lines.push(Line::from(Span::styled(
                    format!("          {}    ", suit),
                    style,
                )));
            }
            if card_area.height >= 5 {
                lines.push(Line::from(Span::styled(
                    format!("         {} {:>2} ", suit, rank),
                    style,
                )));
            }
            let p = Paragraph::new(lines);
            frame.render_widget(p, card_area);
            y += face_up_height;
        } else {
            let card_area = Rect {
                x: inner.x,
                y: inner.y + y,
                width: inner.width.min(CARD_WIDTH),
                height: 1u16.min(inner.height.saturating_sub(y)),
            };
            let back = Paragraph::new(Line::from(Span::styled(
                "┈┈┈┈┈┈┈┈┈┈┈┈┈┈",
                Style::default().fg(Color::Rgb(70, 70, 200)),
            )));
            frame.render_widget(back, card_area);
            y += 1;
        }
    }
}

fn render_status_bar(frame: &mut Frame, area: Rect, state: &AppState) {
    let hint_msg = state
        .hint_text
        .as_deref()
        .or(state.message.as_deref())
        .unwrap_or("");

    let status = Line::from(vec![
        Span::styled(" ♠ ", Style::default().fg(Color::White)),
        Span::styled(
            format!("Score: {} ", state.game.score),
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ),
        Span::styled("│ ", Style::default().fg(Color::Rgb(60, 120, 80))),
        Span::styled(
            format!("Moves: {} ", state.game.moves_count),
            Style::default().fg(Color::Cyan),
        ),
        Span::styled("│ ", Style::default().fg(Color::Rgb(60, 120, 80))),
        Span::styled(
            hint_msg.to_string(),
            Style::default().fg(Color::Green).add_modifier(Modifier::ITALIC),
        ),
    ]);

    let controls = Line::from(vec![
        Span::styled(
            " ←↑↓→",
            Style::default().fg(Color::Cyan),
        ),
        Span::styled(":Move  ", Style::default().fg(Color::Rgb(140, 140, 140))),
        Span::styled(
            "Space",
            Style::default().fg(Color::Cyan),
        ),
        Span::styled(":Select  ", Style::default().fg(Color::Rgb(140, 140, 140))),
        Span::styled(
            "Enter",
            Style::default().fg(Color::Cyan),
        ),
        Span::styled(":Auto-move  ", Style::default().fg(Color::Rgb(140, 140, 140))),
        Span::styled(
            "Dbl-click",
            Style::default().fg(Color::Cyan),
        ),
        Span::styled(":Auto-move", Style::default().fg(Color::Rgb(140, 140, 140))),
    ]);

    let separator = Line::from(Span::styled(
        "─".repeat(area.width as usize),
        Style::default().fg(Color::Rgb(60, 120, 80)),
    ));

    let p = Paragraph::new(vec![separator, status, controls])
        .style(Style::default().bg(Color::Rgb(20, 20, 40)));
    frame.render_widget(p, area);
}
