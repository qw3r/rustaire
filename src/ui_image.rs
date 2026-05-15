use crate::card_img::{CardImages, CARD_IMG_H, CARD_IMG_W};
use crate::ui::AppState;
use image::DynamicImage;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};
use ratatui_image::{picker::Picker, protocol::StatefulProtocol, StatefulImage};

pub struct ImageRenderer {
    pub picker: Picker,
    pub card_images: CardImages,
}

impl ImageRenderer {
    pub fn new() -> Option<Self> {
        let mut picker = Picker::from_query_stdio().ok()?;
        picker.set_background_color(Some([0, 80, 40, 255]));
        let card_images = CardImages::generate();
        Some(ImageRenderer {
            picker,
            card_images,
        })
    }

    fn make_protocol(&mut self, img: &DynamicImage) -> StatefulProtocol {
        self.picker.new_resize_protocol(img.clone())
    }

    fn make_cropped_protocol(&mut self, img: &DynamicImage, height_fraction: f32) -> StatefulProtocol {
        let cropped_h = (CARD_IMG_H as f32 * height_fraction) as u32;
        let cropped = img.crop_imm(0, 0, CARD_IMG_W, cropped_h.min(CARD_IMG_H));
        self.picker.new_resize_protocol(cropped)
    }
}

pub fn render(frame: &mut Frame, state: &AppState, renderer: &mut ImageRenderer) {
    let area = frame.area();
    let theme = state.theme_id.theme();

    let bg = Style::default().bg(Color::Rgb(0, 80, 40));
    let bg_block = Block::default().style(bg);
    frame.render_widget(bg_block, area);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(9),
            Constraint::Min(12),
            Constraint::Length(3),
        ])
        .split(area);

    render_title_bar(frame, main_layout[0], state, &theme);
    render_top_row(frame, main_layout[1], state, renderer);
    render_tableau(frame, main_layout[2], state, renderer);
    render_status_bar(frame, main_layout[3], state, &theme);
}

fn render_title_bar(frame: &mut Frame, area: Rect, _state: &AppState, theme: &crate::theme::Theme) {
    let title = Paragraph::new(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  ♠ ♥ ♣ ♦  ", Style::default().fg(Color::from(theme.title_accent)).add_modifier(Modifier::BOLD)),
            Span::styled(
                "K L O N D I K E",
                Style::default().fg(Color::from(theme.title_text)).add_modifier(Modifier::BOLD),
            ),
            Span::styled("  ♦ ♣ ♥ ♠  ", Style::default().fg(Color::from(theme.title_accent)).add_modifier(Modifier::BOLD)),
            Span::styled("│ ", Style::default().fg(Color::from(theme.separator))),
            Span::styled(" H", Style::default().fg(Color::from(theme.hotkey)).add_modifier(Modifier::BOLD)),
            Span::styled("int ", Style::default().fg(Color::from(theme.label_bright))),
            Span::styled(" U", Style::default().fg(Color::from(theme.hotkey)).add_modifier(Modifier::BOLD)),
            Span::styled("ndo ", Style::default().fg(Color::from(theme.label_bright))),
            Span::styled(" N", Style::default().fg(Color::from(theme.hotkey)).add_modifier(Modifier::BOLD)),
            Span::styled("ew ", Style::default().fg(Color::from(theme.label_bright))),
            Span::styled(" D", Style::default().fg(Color::from(theme.hotkey)).add_modifier(Modifier::BOLD)),
            Span::styled("raw ", Style::default().fg(Color::from(theme.label_bright))),
            Span::styled(" T", Style::default().fg(Color::from(theme.hotkey)).add_modifier(Modifier::BOLD)),
            Span::styled("heme ", Style::default().fg(Color::from(theme.label_bright))),
            Span::styled(" I", Style::default().fg(Color::from(theme.hotkey)).add_modifier(Modifier::BOLD)),
            Span::styled("mg ", Style::default().fg(Color::from(theme.label_bright))),
            Span::styled(" Q", Style::default().fg(Color::from(theme.hotkey)).add_modifier(Modifier::BOLD)),
            Span::styled("uit", Style::default().fg(Color::from(theme.label_bright))),
        ]),
    ])
    .style(Style::default().bg(Color::from(theme.title_bar_bg)));
    frame.render_widget(title, area);
}

fn render_top_row(frame: &mut Frame, area: Rect, state: &AppState, renderer: &mut ImageRenderer) {
    let card_w = 14u16;
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(card_w),
            Constraint::Length(2),
            Constraint::Length(card_w),
            Constraint::Min(4),
            Constraint::Length(card_w),
            Constraint::Length(2),
            Constraint::Length(card_w),
            Constraint::Length(2),
            Constraint::Length(card_w),
            Constraint::Length(2),
            Constraint::Length(card_w),
            Constraint::Min(0),
        ])
        .split(area);

    // Stock
    let stock_area = chunks[1];
    if !state.game.stock.is_empty() {
        let mut proto = renderer.make_protocol(&renderer.card_images.back.clone());
        let img_widget = StatefulImage::default();
        frame.render_stateful_widget(img_widget, stock_area, &mut proto);
    } else if !state.game.waste.is_empty() {
        let mut proto = renderer.make_protocol(&renderer.card_images.empty.clone());
        let img_widget = StatefulImage::default();
        frame.render_stateful_widget(img_widget, stock_area, &mut proto);
    }

    // Waste
    let waste_area = chunks[3];
    if let Some(card) = state.game.waste.last() {
        let img = renderer.card_images.get_face(card).clone();
        let mut proto = renderer.make_protocol(&img);
        let img_widget = StatefulImage::default();
        frame.render_stateful_widget(img_widget, waste_area, &mut proto);
    }

    // Foundations
    for i in 0..4 {
        let found_area = chunks[5 + i * 2];
        if let Some(card) = state.game.foundations[i].last() {
            let img = renderer.card_images.get_face(card).clone();
            let mut proto = renderer.make_protocol(&img);
            let img_widget = StatefulImage::default();
            frame.render_stateful_widget(img_widget, found_area, &mut proto);
        } else {
            let mut proto = renderer.make_protocol(&renderer.card_images.empty.clone());
            let img_widget = StatefulImage::default();
            frame.render_stateful_widget(img_widget, found_area, &mut proto);
        }
    }
}

fn render_tableau(frame: &mut Frame, area: Rect, state: &AppState, renderer: &mut ImageRenderer) {
    let col_width = 14u16;
    let constraints: Vec<Constraint> = std::iter::once(Constraint::Length(2))
        .chain((0..7).map(|_| Constraint::Length(col_width)))
        .chain(std::iter::once(Constraint::Min(0)))
        .collect();

    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for col in 0..7 {
        render_tableau_column(frame, columns[col + 1], state, col, renderer);
    }
}

fn render_tableau_column(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
    col: usize,
    renderer: &mut ImageRenderer,
) {
    let pile = &state.game.tableau[col];
    if pile.is_empty() {
        return;
    }

    let mut y = 0u16;
    for (idx, card) in pile.iter().enumerate() {
        if y >= area.height {
            break;
        }

        let is_last = idx == pile.len() - 1;
        let card_h = if is_last {
            area.height.saturating_sub(y).min(7)
        } else if card.face_up {
            2
        } else {
            1
        };

        let card_area = Rect {
            x: area.x,
            y: area.y + y,
            width: area.width,
            height: card_h,
        };

        if card.face_up {
            let img = renderer.card_images.get_face(card).clone();
            let fraction = if is_last { 1.0 } else { 0.25 };
            let mut proto = renderer.make_cropped_protocol(&img, fraction);
            let img_widget = StatefulImage::default();
            frame.render_stateful_widget(img_widget, card_area, &mut proto);
        } else {
            let fraction = 0.12;
            let mut proto = renderer.make_cropped_protocol(&renderer.card_images.back.clone(), fraction);
            let img_widget = StatefulImage::default();
            frame.render_stateful_widget(img_widget, card_area, &mut proto);
        }

        y += card_h;
    }
}

fn render_status_bar(frame: &mut Frame, area: Rect, state: &AppState, theme: &crate::theme::Theme) {
    let hint_msg = state
        .hint_text
        .as_deref()
        .or(state.message.as_deref())
        .unwrap_or("");

    let status = Line::from(vec![
        Span::styled(" ♠ ", Style::default().fg(Color::from(theme.title_text))),
        Span::styled(
            format!("Score: {} ", state.game.score),
            Style::default().fg(Color::from(theme.score_color)).add_modifier(Modifier::BOLD),
        ),
        Span::styled("│ ", Style::default().fg(Color::from(theme.separator))),
        Span::styled(
            format!("Moves: {} ", state.game.moves_count),
            Style::default().fg(Color::from(theme.moves_color)),
        ),
        Span::styled("│ ", Style::default().fg(Color::from(theme.separator))),
        Span::styled(
            format!("[{}] [IMG] ", state.theme_id.name()),
            Style::default().fg(Color::from(theme.label_dim)),
        ),
        Span::styled("│ ", Style::default().fg(Color::from(theme.separator))),
        Span::styled(
            hint_msg.to_string(),
            Style::default().fg(Color::from(theme.hint_color)).add_modifier(Modifier::ITALIC),
        ),
    ]);

    let controls = Line::from(vec![
        Span::styled(" ←↑↓→", Style::default().fg(Color::from(theme.hotkey))),
        Span::styled(":Move  ", Style::default().fg(Color::from(theme.label_dim))),
        Span::styled("Space", Style::default().fg(Color::from(theme.hotkey))),
        Span::styled(":Select  ", Style::default().fg(Color::from(theme.label_dim))),
        Span::styled("Enter", Style::default().fg(Color::from(theme.hotkey))),
        Span::styled(":Auto-move  ", Style::default().fg(Color::from(theme.label_dim))),
        Span::styled("I", Style::default().fg(Color::from(theme.hotkey))),
        Span::styled(":Toggle text", Style::default().fg(Color::from(theme.label_dim))),
    ]);

    let separator = Line::from(Span::styled(
        "─".repeat(area.width as usize),
        Style::default().fg(Color::from(theme.separator)),
    ));

    let p = Paragraph::new(vec![separator, status, controls])
        .style(Style::default().bg(Color::from(theme.status_bar_bg)));
    frame.render_widget(p, area);
}
