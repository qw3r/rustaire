use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeId {
    Classic,
    Midnight,
    Dracula,
    Solarized,
    Retro,
    Nord,
}

impl ThemeId {
    pub fn all() -> &'static [ThemeId] {
        &[
            ThemeId::Classic,
            ThemeId::Midnight,
            ThemeId::Dracula,
            ThemeId::Solarized,
            ThemeId::Retro,
            ThemeId::Nord,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            ThemeId::Classic => "Classic",
            ThemeId::Midnight => "Midnight",
            ThemeId::Dracula => "Dracula",
            ThemeId::Solarized => "Solarized",
            ThemeId::Retro => "Retro",
            ThemeId::Nord => "Nord",
        }
    }

    pub fn next(&self) -> ThemeId {
        let all = Self::all();
        let idx = all.iter().position(|t| t == self).unwrap_or(0);
        all[(idx + 1) % all.len()]
    }

    pub fn theme(&self) -> Theme {
        match self {
            ThemeId::Classic => Theme::classic(),
            ThemeId::Midnight => Theme::midnight(),
            ThemeId::Dracula => Theme::dracula(),
            ThemeId::Solarized => Theme::solarized(),
            ThemeId::Retro => Theme::retro(),
            ThemeId::Nord => Theme::nord(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub bg: Color,
    pub title_bar_bg: Color,
    pub title_text: Color,
    pub title_accent: Color,
    pub status_bar_bg: Color,
    pub separator: Color,
    pub border_normal: Color,
    pub border_selected: Color,
    pub border_picked: Color,
    pub card_red: Color,
    pub card_black: Color,
    pub card_bg: Color,
    pub card_bg_selected: Color,
    pub card_back: Color,
    pub card_back_pattern: Color,
    pub label_dim: Color,
    pub label_bright: Color,
    pub hotkey: Color,
    pub score_color: Color,
    pub moves_color: Color,
    pub hint_color: Color,
    pub empty_slot: Color,
    pub recycle_color: Color,
}

impl Theme {
    pub fn classic() -> Self {
        Theme {
            bg: Color::Rgb(0, 80, 40),
            title_bar_bg: Color::Rgb(20, 20, 40),
            title_text: Color::White,
            title_accent: Color::Yellow,
            status_bar_bg: Color::Rgb(20, 20, 40),
            separator: Color::Rgb(60, 120, 80),
            border_normal: Color::Rgb(60, 120, 80),
            border_selected: Color::Cyan,
            border_picked: Color::Green,
            card_red: Color::Rgb(255, 80, 80),
            card_black: Color::White,
            card_bg: Color::Rgb(30, 30, 30),
            card_bg_selected: Color::Rgb(40, 70, 40),
            card_back: Color::Rgb(70, 70, 200),
            card_back_pattern: Color::Rgb(70, 70, 200),
            label_dim: Color::Rgb(100, 100, 100),
            label_bright: Color::Rgb(180, 180, 180),
            hotkey: Color::Cyan,
            score_color: Color::Yellow,
            moves_color: Color::Cyan,
            hint_color: Color::Green,
            empty_slot: Color::Rgb(80, 80, 80),
            recycle_color: Color::Yellow,
        }
    }

    pub fn midnight() -> Self {
        Theme {
            bg: Color::Rgb(10, 10, 30),
            title_bar_bg: Color::Rgb(20, 10, 40),
            title_text: Color::Rgb(200, 180, 255),
            title_accent: Color::Rgb(180, 120, 255),
            status_bar_bg: Color::Rgb(20, 10, 40),
            separator: Color::Rgb(60, 40, 100),
            border_normal: Color::Rgb(50, 40, 90),
            border_selected: Color::Rgb(150, 100, 255),
            border_picked: Color::Rgb(100, 255, 150),
            card_red: Color::Rgb(255, 100, 120),
            card_black: Color::Rgb(200, 200, 240),
            card_bg: Color::Rgb(20, 15, 40),
            card_bg_selected: Color::Rgb(40, 30, 70),
            card_back: Color::Rgb(80, 50, 150),
            card_back_pattern: Color::Rgb(100, 70, 180),
            label_dim: Color::Rgb(80, 70, 120),
            label_bright: Color::Rgb(160, 150, 200),
            hotkey: Color::Rgb(150, 100, 255),
            score_color: Color::Rgb(255, 200, 100),
            moves_color: Color::Rgb(150, 100, 255),
            hint_color: Color::Rgb(100, 255, 180),
            empty_slot: Color::Rgb(50, 40, 80),
            recycle_color: Color::Rgb(255, 200, 100),
        }
    }

    pub fn dracula() -> Self {
        Theme {
            bg: Color::Rgb(40, 42, 54),
            title_bar_bg: Color::Rgb(30, 31, 41),
            title_text: Color::Rgb(248, 248, 242),
            title_accent: Color::Rgb(255, 121, 198),
            status_bar_bg: Color::Rgb(30, 31, 41),
            separator: Color::Rgb(68, 71, 90),
            border_normal: Color::Rgb(68, 71, 90),
            border_selected: Color::Rgb(139, 233, 253),
            border_picked: Color::Rgb(80, 250, 123),
            card_red: Color::Rgb(255, 85, 85),
            card_black: Color::Rgb(248, 248, 242),
            card_bg: Color::Rgb(30, 31, 41),
            card_bg_selected: Color::Rgb(50, 52, 68),
            card_back: Color::Rgb(98, 114, 164),
            card_back_pattern: Color::Rgb(118, 134, 184),
            label_dim: Color::Rgb(98, 114, 164),
            label_bright: Color::Rgb(190, 190, 210),
            hotkey: Color::Rgb(139, 233, 253),
            score_color: Color::Rgb(241, 250, 140),
            moves_color: Color::Rgb(139, 233, 253),
            hint_color: Color::Rgb(80, 250, 123),
            empty_slot: Color::Rgb(68, 71, 90),
            recycle_color: Color::Rgb(241, 250, 140),
        }
    }

    pub fn solarized() -> Self {
        Theme {
            bg: Color::Rgb(0, 43, 54),
            title_bar_bg: Color::Rgb(7, 54, 66),
            title_text: Color::Rgb(238, 232, 213),
            title_accent: Color::Rgb(181, 137, 0),
            status_bar_bg: Color::Rgb(7, 54, 66),
            separator: Color::Rgb(88, 110, 117),
            border_normal: Color::Rgb(88, 110, 117),
            border_selected: Color::Rgb(38, 139, 210),
            border_picked: Color::Rgb(133, 153, 0),
            card_red: Color::Rgb(220, 50, 47),
            card_black: Color::Rgb(238, 232, 213),
            card_bg: Color::Rgb(7, 54, 66),
            card_bg_selected: Color::Rgb(20, 70, 85),
            card_back: Color::Rgb(42, 161, 152),
            card_back_pattern: Color::Rgb(52, 171, 162),
            label_dim: Color::Rgb(88, 110, 117),
            label_bright: Color::Rgb(147, 161, 161),
            hotkey: Color::Rgb(38, 139, 210),
            score_color: Color::Rgb(181, 137, 0),
            moves_color: Color::Rgb(38, 139, 210),
            hint_color: Color::Rgb(133, 153, 0),
            empty_slot: Color::Rgb(88, 110, 117),
            recycle_color: Color::Rgb(203, 75, 22),
        }
    }

    pub fn retro() -> Self {
        Theme {
            bg: Color::Rgb(20, 20, 20),
            title_bar_bg: Color::Rgb(40, 40, 0),
            title_text: Color::Rgb(0, 255, 0),
            title_accent: Color::Rgb(255, 255, 0),
            status_bar_bg: Color::Rgb(40, 40, 0),
            separator: Color::Rgb(0, 150, 0),
            border_normal: Color::Rgb(0, 120, 0),
            border_selected: Color::Rgb(0, 255, 0),
            border_picked: Color::Rgb(255, 255, 0),
            card_red: Color::Rgb(255, 60, 60),
            card_black: Color::Rgb(0, 255, 0),
            card_bg: Color::Rgb(0, 0, 0),
            card_bg_selected: Color::Rgb(0, 40, 0),
            card_back: Color::Rgb(0, 180, 0),
            card_back_pattern: Color::Rgb(0, 220, 0),
            label_dim: Color::Rgb(0, 100, 0),
            label_bright: Color::Rgb(0, 200, 0),
            hotkey: Color::Rgb(255, 255, 0),
            score_color: Color::Rgb(255, 255, 0),
            moves_color: Color::Rgb(0, 255, 0),
            hint_color: Color::Rgb(0, 255, 128),
            empty_slot: Color::Rgb(0, 80, 0),
            recycle_color: Color::Rgb(255, 255, 0),
        }
    }

    pub fn nord() -> Self {
        Theme {
            bg: Color::Rgb(46, 52, 64),
            title_bar_bg: Color::Rgb(36, 40, 50),
            title_text: Color::Rgb(236, 239, 244),
            title_accent: Color::Rgb(235, 203, 139),
            status_bar_bg: Color::Rgb(36, 40, 50),
            separator: Color::Rgb(76, 86, 106),
            border_normal: Color::Rgb(76, 86, 106),
            border_selected: Color::Rgb(136, 192, 208),
            border_picked: Color::Rgb(163, 190, 140),
            card_red: Color::Rgb(191, 97, 106),
            card_black: Color::Rgb(236, 239, 244),
            card_bg: Color::Rgb(59, 66, 82),
            card_bg_selected: Color::Rgb(67, 76, 94),
            card_back: Color::Rgb(94, 129, 172),
            card_back_pattern: Color::Rgb(114, 149, 192),
            label_dim: Color::Rgb(76, 86, 106),
            label_bright: Color::Rgb(180, 190, 210),
            hotkey: Color::Rgb(136, 192, 208),
            score_color: Color::Rgb(235, 203, 139),
            moves_color: Color::Rgb(136, 192, 208),
            hint_color: Color::Rgb(163, 190, 140),
            empty_slot: Color::Rgb(76, 86, 106),
            recycle_color: Color::Rgb(208, 135, 112),
        }
    }
}
