use crate::card::Card;
use crate::game::Game;
use crate::theme::{Theme, ThemeId};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
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
    pub theme_id: ThemeId,
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
            theme_id: ThemeId::ClassicLight,
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

    pub fn cycle_theme(&mut self) {
        self.theme_id = self.theme_id.next();
        self.message = Some(format!("Theme: {}", self.theme_id.name()));
    }
}

const CARD_WIDTH: u16 = 14;
const CARD_HEIGHT: u16 = 7;

pub fn render(frame: &mut Frame, state: &AppState) {
    let area = frame.area();
    let theme = state.theme_id.theme();

    let bg = Style::default().bg(theme.bg);
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

    render_title_bar(frame, main_layout[0], state, &theme);
    render_top_row(frame, main_layout[1], state, &theme);
    render_tableau(frame, main_layout[2], state, &theme);
    render_status_bar(frame, main_layout[3], state, &theme);
}

fn render_title_bar(frame: &mut Frame, area: Rect, _state: &AppState, theme: &Theme) {
    let title = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ♠ ♥ ♣ ♦  ", Style::default().fg(theme.title_accent).add_modifier(Modifier::BOLD)),
            Span::styled(
                "K L O N D I K E",
                Style::default()
                    .fg(theme.title_text)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("  ♦ ♣ ♥ ♠  ", Style::default().fg(theme.title_accent).add_modifier(Modifier::BOLD)),
            Span::styled("│ ", Style::default().fg(theme.separator)),
            Span::styled(" H", Style::default().fg(theme.hotkey).add_modifier(Modifier::BOLD)),
            Span::styled("int ", Style::default().fg(theme.label_bright)),
            Span::styled(" U", Style::default().fg(theme.hotkey).add_modifier(Modifier::BOLD)),
            Span::styled("ndo ", Style::default().fg(theme.label_bright)),
            Span::styled(" N", Style::default().fg(theme.hotkey).add_modifier(Modifier::BOLD)),
            Span::styled("ew ", Style::default().fg(theme.label_bright)),
            Span::styled(" A", Style::default().fg(theme.hotkey).add_modifier(Modifier::BOLD)),
            Span::styled("uto ", Style::default().fg(theme.label_bright)),
            Span::styled(" D", Style::default().fg(theme.hotkey).add_modifier(Modifier::BOLD)),
            Span::styled("raw ", Style::default().fg(theme.label_bright)),
            Span::styled(" T", Style::default().fg(theme.hotkey).add_modifier(Modifier::BOLD)),
            Span::styled("heme ", Style::default().fg(theme.label_bright)),
            Span::styled(" Q", Style::default().fg(theme.hotkey).add_modifier(Modifier::BOLD)),
            Span::styled("uit", Style::default().fg(theme.label_bright)),
        ]),
    ])
    .style(Style::default().bg(theme.title_bar_bg));
    frame.render_widget(title, area);
}

fn render_top_row(frame: &mut Frame, area: Rect, state: &AppState, theme: &Theme) {
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
    render_stock(frame, chunks[1], state, stock_selected, theme);

    let waste_selected = state.cursor == Cursor::Waste;
    let waste_picked = state.selection == Some(Selection::Waste);
    render_waste(frame, chunks[3], state, waste_selected, waste_picked, theme);

    for i in 0..4 {
        let found_selected = state.cursor == Cursor::Foundation(i);
        let found_picked = state.selection == Some(Selection::Foundation(i));
        render_foundation(frame, chunks[5 + i * 2], state, i, found_selected, found_picked, theme);
    }
}

fn render_stock(frame: &mut Frame, area: Rect, state: &AppState, selected: bool, theme: &Theme) {
    let border_style = if selected {
        Style::default().fg(theme.border_selected).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.border_normal)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(Span::styled(" Stock ", Style::default().fg(theme.label_dim)));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if state.game.stock.is_empty() && state.game.waste.is_empty() {
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled("      ∅", Style::default().fg(theme.empty_slot))),
        ]);
        frame.render_widget(empty, inner);
    } else if state.game.stock.is_empty() {
        let recycle = Paragraph::new(vec![
            Line::from(""),
            Line::from(Span::styled("      ↺", Style::default().fg(theme.recycle_color).add_modifier(Modifier::BOLD))),
            Line::from(""),
            Line::from(Span::styled("    recycle", Style::default().fg(theme.label_dim))),
        ]);
        frame.render_widget(recycle, inner);
    } else {
        let count = format!("     [{}]", state.game.stock.len());
        let card_back = Paragraph::new(vec![
            Line::from(Span::styled("┌────────────┐", Style::default().fg(theme.card_back))),
            Line::from(Span::styled("│▓▓▓▓▓▓▓▓▓▓▓▓│", Style::default().fg(theme.card_back_pattern))),
            Line::from(Span::styled("│▓▓▓░░░░░▓▓▓▓│", Style::default().fg(theme.card_back_pattern))),
            Line::from(Span::styled("│▓▓▓▓▓▓▓▓▓▓▓▓│", Style::default().fg(theme.card_back_pattern))),
            Line::from(Span::styled("└────────────┘", Style::default().fg(theme.card_back))),
            Line::from(Span::styled(count, Style::default().fg(theme.label_dim))),
        ]);
        frame.render_widget(card_back, inner);
    }
}

fn render_waste(frame: &mut Frame, area: Rect, state: &AppState, selected: bool, picked: bool, theme: &Theme) {
    let border_style = if picked {
        Style::default().fg(theme.border_picked).add_modifier(Modifier::BOLD)
    } else if selected {
        Style::default().fg(theme.border_selected).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.border_normal)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(Span::styled(" Waste ", Style::default().fg(theme.label_dim)));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if let Some(card) = state.game.waste.last() {
        render_card_face_large(frame, inner, card, picked, theme);
    } else {
        let empty = Paragraph::new(vec![
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled("      —", Style::default().fg(theme.empty_slot))),
        ]);
        frame.render_widget(empty, inner);
    }
}

fn render_foundation(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
    idx: usize,
    selected: bool,
    picked: bool,
    theme: &Theme,
) {
    let border_style = if picked {
        Style::default().fg(theme.border_picked).add_modifier(Modifier::BOLD)
    } else if selected {
        Style::default().fg(theme.border_selected).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(theme.border_normal)
    };

    let suit_names = ["♥", "♦", "♣", "♠"];
    let title = format!(" {} ", suit_names[idx]);
    let title_color = if idx < 2 { theme.card_red } else { theme.card_black };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(Span::styled(title, Style::default().fg(title_color)));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if let Some(card) = state.game.foundations[idx].last() {
        render_card_face_large(frame, inner, card, picked, theme);
    } else {
        let suit_symbols = ["♥", "♦", "♣", "♠"];
        let suit_color = if idx < 2 { theme.card_red } else { theme.empty_slot };
        let placeholder = Paragraph::new(vec![
            Line::from(""),
            Line::from(""),
            Line::from(""),
            Line::from(Span::styled(
                format!("      {}", suit_symbols[idx]),
                Style::default().fg(suit_color).add_modifier(Modifier::DIM),
            )),
        ]);
        frame.render_widget(placeholder, inner);
    }
}

fn render_card_face_large(frame: &mut Frame, area: Rect, card: &Card, highlighted: bool, theme: &Theme) {
    let color = if card.is_red() { theme.card_red } else { theme.card_black };
    let bg = if highlighted { theme.card_bg_selected } else { theme.card_bg };
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

fn render_tableau(frame: &mut Frame, area: Rect, state: &AppState, theme: &Theme) {
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
        render_tableau_column(frame, columns[col + 1], state, col, col_selected, theme);
    }
}

fn render_tableau_column(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
    col: usize,
    selected: bool,
    theme: &Theme,
) {
    let pile = &state.game.tableau[col];
    let border_style = if selected {
        Style::default().fg(theme.border_selected)
    } else {
        Style::default().fg(theme.border_normal)
    };

    let col_label = format!(" {} ", col + 1);
    let block = Block::default()
        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
        .border_style(border_style)
        .title(Span::styled(
            col_label,
            if selected {
                Style::default().fg(theme.border_selected).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(theme.label_dim)
            },
        ));
    let inner = block.inner(area);
    frame.render_widget(block, area);

    if pile.is_empty() {
        if selected {
            let empty = Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(
                    "     [  ]",
                    Style::default().fg(theme.empty_slot),
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

        let prev_face_up = idx > 0 && pile[idx - 1].face_up;

        if card.face_up {
            let is_last = idx == pile.len() - 1;
            let show_border = prev_face_up;
            let face_up_height = if is_last { 4u16 } else if show_border { 2u16 } else { 1u16 };
            let card_area = Rect {
                x: inner.x,
                y: inner.y + y,
                width: inner.width.min(CARD_WIDTH),
                height: face_up_height.min(inner.height.saturating_sub(y)),
            };

            let color = if card.is_red() { theme.card_red } else { theme.card_black };
            let bg = if is_selected_card { theme.card_bg_selected } else { theme.card_bg };
            let style = Style::default().fg(color).bg(bg);
            let border_style = if is_selected_card {
                Style::default().fg(theme.border_picked).bg(bg)
            } else {
                Style::default().fg(theme.border_normal).bg(bg)
            };
            let rank = card.rank.symbol();
            let suit = card.suit.symbol();

            let mut lines = Vec::new();
            if show_border {
                lines.push(Line::from(Span::styled(
                    "┄┄┄┄┄┄┄┄┄┄┄┄┄┄",
                    border_style,
                )));
            }
            lines.push(Line::from(Span::styled(
                format!(" {:<2} {}         ", rank, suit),
                style,
            )));
            if is_last {
                if card_area.height >= 3 {
                    lines.push(Line::from(Span::styled(
                        format!("      {}        ", suit),
                        style,
                    )));
                }
                if card_area.height >= 4 {
                    lines.push(Line::from(Span::styled(
                        format!("         {} {:>2} ", suit, rank),
                        style,
                    )));
                }
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
                Style::default().fg(theme.card_back),
            )));
            frame.render_widget(back, card_area);
            y += 1;
        }
    }
}

fn render_status_bar(frame: &mut Frame, area: Rect, state: &AppState, theme: &Theme) {
    let hint_msg = state
        .hint_text
        .as_deref()
        .or(state.message.as_deref())
        .unwrap_or("");

    let status = Line::from(vec![
        Span::styled(" ♠ ", Style::default().fg(theme.title_text)),
        Span::styled(
            format!("Score: {} ", state.game.score),
            Style::default().fg(theme.score_color).add_modifier(Modifier::BOLD),
        ),
        Span::styled("│ ", Style::default().fg(theme.separator)),
        Span::styled(
            format!("Moves: {} ", state.game.moves_count),
            Style::default().fg(theme.moves_color),
        ),
        Span::styled("│ ", Style::default().fg(theme.separator)),
        Span::styled(
            format!("[{}] ", state.theme_id.name()),
            Style::default().fg(theme.label_dim),
        ),
        Span::styled("│ ", Style::default().fg(theme.separator)),
        Span::styled(
            hint_msg.to_string(),
            Style::default().fg(theme.hint_color).add_modifier(Modifier::ITALIC),
        ),
    ]);

    let controls = Line::from(vec![
        Span::styled(" ←↑↓→", Style::default().fg(theme.hotkey)),
        Span::styled(":Move  ", Style::default().fg(theme.label_dim)),
        Span::styled("Space", Style::default().fg(theme.hotkey)),
        Span::styled(":Select  ", Style::default().fg(theme.label_dim)),
        Span::styled("Enter", Style::default().fg(theme.hotkey)),
        Span::styled(":Auto-move  ", Style::default().fg(theme.label_dim)),
        Span::styled("Dbl-click", Style::default().fg(theme.hotkey)),
        Span::styled(":Auto-move", Style::default().fg(theme.label_dim)),
    ]);

    let separator = Line::from(Span::styled(
        "─".repeat(area.width as usize),
        Style::default().fg(theme.separator),
    ));

    let p = Paragraph::new(vec![separator, status, controls])
        .style(Style::default().bg(theme.status_bar_bg));
    frame.render_widget(p, area);
}
