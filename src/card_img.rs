use crate::card::{Card, Rank, Suit};
use image::{DynamicImage, Rgba, RgbaImage};
use std::collections::HashMap;

pub const CARD_IMG_W: u32 = 120;
pub const CARD_IMG_H: u32 = 168;

const CARD_BG: Rgba<u8> = Rgba([250, 250, 245, 255]);
const RED: Rgba<u8> = Rgba([200, 30, 30, 255]);
const BLACK: Rgba<u8> = Rgba([20, 20, 20, 255]);
const BACK_BLUE: Rgba<u8> = Rgba([40, 60, 160, 255]);
const BACK_PATTERN: Rgba<u8> = Rgba([60, 80, 200, 255]);
const BORDER: Rgba<u8> = Rgba([180, 180, 180, 255]);
const EMPTY_GRAY: Rgba<u8> = Rgba([60, 60, 60, 255]);
const FOUND_BG: Rgba<u8> = Rgba([30, 80, 50, 255]);

pub struct CardImages {
    pub faces: HashMap<(Suit, Rank), DynamicImage>,
    pub back: DynamicImage,
    pub empty: DynamicImage,
}

impl CardImages {
    pub fn generate() -> Self {
        let mut faces = HashMap::new();
        for suit in Suit::all() {
            for rank in Rank::all() {
                let img = render_card_face(suit, rank);
                faces.insert((suit, rank), DynamicImage::ImageRgba8(img));
            }
        }
        let back = DynamicImage::ImageRgba8(render_card_back());
        let empty = DynamicImage::ImageRgba8(render_empty_slot());
        CardImages { faces, back, empty }
    }

    pub fn get_face(&self, card: &Card) -> &DynamicImage {
        &self.faces[&(card.suit, card.rank)]
    }
}

fn render_card_face(suit: Suit, rank: Rank) -> RgbaImage {
    let mut img = RgbaImage::from_pixel(CARD_IMG_W, CARD_IMG_H, CARD_BG);
    draw_border(&mut img, BORDER);

    let color = if suit.is_red() { RED } else { BLACK };

    let rank_str = rank.symbol();

    // Top-left rank and suit
    draw_text_small(&mut img, 8, 8, rank_str, color);
    draw_suit_symbol(&mut img, 10, 28, suit, color, false);

    // Center large suit
    draw_suit_symbol(&mut img, 45, 65, suit, color, true);

    // Bottom-right rank and suit (rotated 180°)
    let rw = if rank_str.len() > 1 { 22 } else { 14 };
    draw_text_small_flipped(&mut img, CARD_IMG_W - 8 - rw, CARD_IMG_H - 22, rank_str, color);
    draw_suit_symbol_flipped(&mut img, CARD_IMG_W - 26, CARD_IMG_H - 44, suit, color);

    img
}

fn render_card_back() -> RgbaImage {
    let mut img = RgbaImage::from_pixel(CARD_IMG_W, CARD_IMG_H, BACK_BLUE);
    draw_border(&mut img, Rgba([30, 40, 120, 255]));

    // Diamond pattern
    for y in (8..CARD_IMG_H - 8).step_by(12) {
        for x in (8..CARD_IMG_W - 8).step_by(12) {
            let offset = if (y / 12) % 2 == 0 { 6 } else { 0 };
            draw_diamond(&mut img, x + offset, y, BACK_PATTERN);
        }
    }
    img
}

fn render_empty_slot() -> RgbaImage {
    let mut img = RgbaImage::from_pixel(CARD_IMG_W, CARD_IMG_H, FOUND_BG);
    draw_border_dashed(&mut img, EMPTY_GRAY);
    img
}

fn draw_border(img: &mut RgbaImage, color: Rgba<u8>) {
    let (w, h) = (img.width(), img.height());
    let r = 4u32;
    for x in r..w - r {
        img.put_pixel(x, 0, color);
        img.put_pixel(x, 1, color);
        img.put_pixel(x, h - 1, color);
        img.put_pixel(x, h - 2, color);
    }
    for y in r..h - r {
        img.put_pixel(0, y, color);
        img.put_pixel(1, y, color);
        img.put_pixel(w - 1, y, color);
        img.put_pixel(w - 2, y, color);
    }
    // Rounded corners
    for &(cx, cy) in &[(r, r), (w - r - 1, r), (r, h - r - 1), (w - r - 1, h - r - 1)] {
        for dx in 0..r {
            for dy in 0..r {
                let dist = ((dx * dx + dy * dy) as f32).sqrt();
                if dist <= r as f32 && dist >= (r - 2) as f32 {
                    let px = if cx < w / 2 { cx - dx } else { cx + dx };
                    let py = if cy < h / 2 { cy - dy } else { cy + dy };
                    if px < w && py < h {
                        img.put_pixel(px, py, color);
                    }
                }
            }
        }
    }
}

fn draw_border_dashed(img: &mut RgbaImage, color: Rgba<u8>) {
    let (w, h) = (img.width(), img.height());
    for x in (4..w - 4).step_by(8) {
        for dx in 0..4 {
            if x + dx < w - 4 {
                img.put_pixel(x + dx, 2, color);
                img.put_pixel(x + dx, h - 3, color);
            }
        }
    }
    for y in (4..h - 4).step_by(8) {
        for dy in 0..4 {
            if y + dy < h - 4 {
                img.put_pixel(2, y + dy, color);
                img.put_pixel(w - 3, y + dy, color);
            }
        }
    }
}

fn draw_diamond(img: &mut RgbaImage, cx: u32, cy: u32, color: Rgba<u8>) {
    let size = 4u32;
    for dy in 0..size {
        let width = if dy < size / 2 { dy } else { size - 1 - dy };
        for dx in 0..=width {
            let x1 = cx + size / 2 + dx;
            let x2 = cx + size / 2 - dx;
            let y = cy + dy;
            if x1 < img.width() && y < img.height() {
                img.put_pixel(x1, y, color);
            }
            if x2 < img.width() && y < img.height() {
                img.put_pixel(x2, y, color);
            }
        }
    }
}

// Simple bitmap font for ranks — each char is 8px wide, 12px tall
fn draw_text_small(img: &mut RgbaImage, x: u32, y: u32, text: &str, color: Rgba<u8>) {
    let mut cx = x;
    for ch in text.chars() {
        let bitmap = get_char_bitmap(ch);
        draw_bitmap(img, cx, y, &bitmap, color, false);
        cx += 10;
    }
}

fn draw_text_small_flipped(img: &mut RgbaImage, x: u32, y: u32, text: &str, color: Rgba<u8>) {
    let mut cx = x;
    // Draw in reverse order for flipped text
    for ch in text.chars().rev() {
        let bitmap = get_char_bitmap(ch);
        draw_bitmap(img, cx, y, &bitmap, color, true);
        cx += 10;
    }
}

fn draw_bitmap(img: &mut RgbaImage, x: u32, y: u32, bitmap: &[u16; 12], color: Rgba<u8>, flip: bool) {
    for row in 0..12u32 {
        let src_row = if flip { 11 - row } else { row };
        let bits = bitmap[src_row as usize];
        for col in 0..8u32 {
            let src_col = if flip { 7 - col } else { col };
            if (bits >> (7 - src_col)) & 1 == 1 {
                let px = x + col;
                let py = y + row;
                if px < img.width() && py < img.height() {
                    img.put_pixel(px, py, color);
                }
            }
        }
    }
}

fn draw_suit_symbol(img: &mut RgbaImage, x: u32, y: u32, suit: Suit, color: Rgba<u8>, large: bool) {
    let scale = if large { 3 } else { 1 };
    let bitmap = get_suit_bitmap(suit);
    for row in 0..12u32 {
        let bits = bitmap[row as usize];
        for col in 0..12u32 {
            if (bits >> (11 - col)) & 1 == 1 {
                for sy in 0..scale {
                    for sx in 0..scale {
                        let px = x + col * scale + sx;
                        let py = y + row * scale + sy;
                        if px < img.width() && py < img.height() {
                            img.put_pixel(px, py, color);
                        }
                    }
                }
            }
        }
    }
}

fn draw_suit_symbol_flipped(img: &mut RgbaImage, x: u32, y: u32, suit: Suit, color: Rgba<u8>) {
    let bitmap = get_suit_bitmap(suit);
    for row in 0..12u32 {
        let bits = bitmap[11 - row as usize];
        for col in 0..12u32 {
            if (bits >> col) & 1 == 1 {
                let px = x + col;
                let py = y + row;
                if px < img.width() && py < img.height() {
                    img.put_pixel(px, py, color);
                }
            }
        }
    }
}

// 12x12 bitmaps for suit symbols
fn get_suit_bitmap(suit: Suit) -> [u16; 12] {
    match suit {
        Suit::Hearts => [
            0b0000_0000_0000,
            0b0110_0110_0000,
            0b1111_1111_0000,
            0b1111_1111_0000,
            0b1111_1111_0000,
            0b1111_1111_0000,
            0b0111_1110_0000,
            0b0011_1100_0000,
            0b0001_1000_0000,
            0b0000_0000_0000,
            0b0000_0000_0000,
            0b0000_0000_0000,
        ],
        Suit::Diamonds => [
            0b0000_0000_0000,
            0b0001_1000_0000,
            0b0011_1100_0000,
            0b0111_1110_0000,
            0b1111_1111_0000,
            0b0111_1110_0000,
            0b0011_1100_0000,
            0b0001_1000_0000,
            0b0000_0000_0000,
            0b0000_0000_0000,
            0b0000_0000_0000,
            0b0000_0000_0000,
        ],
        Suit::Clubs => [
            0b0001_1000_0000,
            0b0011_1100_0000,
            0b0011_1100_0000,
            0b1111_1111_0000,
            0b1111_1111_0000,
            0b1111_1111_0000,
            0b0111_1110_0000,
            0b0001_1000_0000,
            0b0001_1000_0000,
            0b0111_1110_0000,
            0b0000_0000_0000,
            0b0000_0000_0000,
        ],
        Suit::Spades => [
            0b0001_1000_0000,
            0b0011_1100_0000,
            0b0111_1110_0000,
            0b1111_1111_0000,
            0b1111_1111_0000,
            0b1111_1111_0000,
            0b0110_0110_0000,
            0b0001_1000_0000,
            0b0001_1000_0000,
            0b0111_1110_0000,
            0b0000_0000_0000,
            0b0000_0000_0000,
        ],
    }
}

// 8x12 bitmap font for card rank characters
fn get_char_bitmap(ch: char) -> [u16; 12] {
    match ch {
        'A' => [
            0b0001_1000,
            0b0010_0100,
            0b0100_0010,
            0b0100_0010,
            0b0111_1110,
            0b0100_0010,
            0b0100_0010,
            0b0100_0010,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ],
        '2' => [
            0b0011_1100,
            0b0100_0010,
            0b0000_0010,
            0b0000_0100,
            0b0000_1000,
            0b0001_0000,
            0b0010_0000,
            0b0111_1110,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ],
        '3' => [
            0b0011_1100,
            0b0100_0010,
            0b0000_0010,
            0b0001_1100,
            0b0000_0010,
            0b0000_0010,
            0b0100_0010,
            0b0011_1100,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ],
        '4' => [
            0b0000_0100,
            0b0000_1100,
            0b0001_0100,
            0b0010_0100,
            0b0100_0100,
            0b0111_1110,
            0b0000_0100,
            0b0000_0100,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ],
        '5' => [
            0b0111_1110,
            0b0100_0000,
            0b0100_0000,
            0b0111_1100,
            0b0000_0010,
            0b0000_0010,
            0b0100_0010,
            0b0011_1100,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ],
        '6' => [
            0b0011_1100,
            0b0100_0000,
            0b0100_0000,
            0b0111_1100,
            0b0100_0010,
            0b0100_0010,
            0b0100_0010,
            0b0011_1100,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ],
        '7' => [
            0b0111_1110,
            0b0000_0010,
            0b0000_0100,
            0b0000_1000,
            0b0001_0000,
            0b0001_0000,
            0b0001_0000,
            0b0001_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ],
        '8' => [
            0b0011_1100,
            0b0100_0010,
            0b0100_0010,
            0b0011_1100,
            0b0100_0010,
            0b0100_0010,
            0b0100_0010,
            0b0011_1100,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ],
        '9' => [
            0b0011_1100,
            0b0100_0010,
            0b0100_0010,
            0b0011_1110,
            0b0000_0010,
            0b0000_0010,
            0b0000_0010,
            0b0011_1100,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ],
        '1' => [
            0b0000_1000,
            0b0001_1000,
            0b0010_1000,
            0b0000_1000,
            0b0000_1000,
            0b0000_1000,
            0b0000_1000,
            0b0011_1110,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ],
        '0' => [
            0b0011_1100,
            0b0100_0010,
            0b0100_0110,
            0b0100_1010,
            0b0101_0010,
            0b0110_0010,
            0b0100_0010,
            0b0011_1100,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ],
        'J' => [
            0b0001_1110,
            0b0000_0100,
            0b0000_0100,
            0b0000_0100,
            0b0000_0100,
            0b0100_0100,
            0b0100_0100,
            0b0011_1000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ],
        'Q' => [
            0b0011_1100,
            0b0100_0010,
            0b0100_0010,
            0b0100_0010,
            0b0100_0010,
            0b0100_1010,
            0b0100_0100,
            0b0011_1010,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ],
        'K' => [
            0b0100_0010,
            0b0100_0100,
            0b0100_1000,
            0b0101_0000,
            0b0110_0000,
            0b0101_0000,
            0b0100_1000,
            0b0100_0100,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ],
        _ => [0; 12],
    }
}
