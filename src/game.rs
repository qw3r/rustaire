use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Location {
    Stock,
    Waste,
    Foundation(usize),
    Tableau(usize),
}

#[derive(Debug, Clone)]
pub struct Move {
    pub from: Location,
    pub to: Location,
    pub cards: Vec<Card>,
    pub revealed_card: bool,
    pub stock_recycled: bool,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub stock: Vec<Card>,
    pub waste: Vec<Card>,
    pub foundations: [Vec<Card>; 4],
    pub tableau: [Vec<Card>; 7],
    pub history: Vec<Move>,
    pub moves_count: u32,
    pub score: i32,
}

impl Game {
    pub fn new() -> Self {
        let mut deck: Vec<Card> = Vec::with_capacity(52);
        for suit in Suit::all() {
            for rank in Rank::all() {
                deck.push(Card::new(suit, rank));
            }
        }

        let mut rng = thread_rng();
        deck.shuffle(&mut rng);

        let mut tableau: [Vec<Card>; 7] = Default::default();
        let mut idx = 0;
        for col in 0..7 {
            for row in 0..=col {
                let mut card = deck[idx];
                if row == col {
                    card.face_up = true;
                }
                tableau[col].push(card);
                idx += 1;
            }
        }

        let stock: Vec<Card> = deck[idx..].to_vec();

        Game {
            stock,
            waste: Vec::new(),
            foundations: Default::default(),
            tableau,
            history: Vec::new(),
            moves_count: 0,
            score: 0,
        }
    }

    pub fn draw_from_stock(&mut self) {
        if self.stock.is_empty() {
            if self.waste.is_empty() {
                return;
            }
            let cards_moved: Vec<Card> = self.waste.drain(..).collect();
            self.history.push(Move {
                from: Location::Stock,
                to: Location::Stock,
                cards: cards_moved.clone(),
                revealed_card: false,
                stock_recycled: true,
            });
            self.stock = cards_moved;
            self.stock.reverse();
            for card in &mut self.stock {
                card.face_up = false;
            }
            self.score = (self.score - 100).max(0);
        } else {
            if let Some(mut card) = self.stock.pop() {
                card.face_up = true;
                self.waste.push(card);
                self.history.push(Move {
                    from: Location::Stock,
                    to: Location::Waste,
                    cards: vec![card],
                    revealed_card: false,
                    stock_recycled: false,
                });
            }
        }
        self.moves_count += 1;
    }

    pub fn move_waste_to_foundation(&mut self) -> bool {
        if let Some(card) = self.waste.last() {
            for i in 0..4 {
                let suit = Suit::all()[i];
                let top = self.foundations[i].last();
                if card.can_stack_on_foundation(top, suit) {
                    let card = self.waste.pop().unwrap();
                    self.foundations[i].push(card);
                    self.history.push(Move {
                        from: Location::Waste,
                        to: Location::Foundation(i),
                        cards: vec![card],
                        revealed_card: false,
                        stock_recycled: false,
                    });
                    self.score += 10;
                    self.moves_count += 1;
                    return true;
                }
            }
        }
        false
    }

    pub fn move_waste_to_tableau(&mut self, col: usize) -> bool {
        if let Some(card) = self.waste.last() {
            if self.can_place_on_tableau(*card, col) {
                let card = self.waste.pop().unwrap();
                self.tableau[col].push(card);
                self.history.push(Move {
                    from: Location::Waste,
                    to: Location::Tableau(col),
                    cards: vec![card],
                    revealed_card: false,
                    stock_recycled: false,
                });
                self.score += 5;
                self.moves_count += 1;
                return true;
            }
        }
        false
    }

    pub fn move_tableau_to_foundation(&mut self, from_col: usize) -> bool {
        if let Some(card) = self.tableau[from_col].last() {
            for i in 0..4 {
                let suit = Suit::all()[i];
                let top = self.foundations[i].last();
                if card.can_stack_on_foundation(top, suit) {
                    let card = self.tableau[from_col].pop().unwrap();
                    let revealed = self.reveal_top(from_col);
                    self.foundations[i].push(card);
                    self.history.push(Move {
                        from: Location::Tableau(from_col),
                        to: Location::Foundation(i),
                        cards: vec![card],
                        revealed_card: revealed,
                        stock_recycled: false,
                    });
                    self.score += 10;
                    self.moves_count += 1;
                    return true;
                }
            }
        }
        false
    }

    pub fn move_tableau_to_tableau(&mut self, from_col: usize, card_idx: usize, to_col: usize) -> bool {
        if from_col == to_col {
            return false;
        }
        if card_idx >= self.tableau[from_col].len() {
            return false;
        }
        let card = self.tableau[from_col][card_idx];
        if !card.face_up {
            return false;
        }
        if !self.can_place_on_tableau(card, to_col) {
            return false;
        }

        let cards: Vec<Card> = self.tableau[from_col].split_off(card_idx);
        let revealed = self.reveal_top(from_col);
        self.tableau[to_col].extend_from_slice(&cards);
        self.history.push(Move {
            from: Location::Tableau(from_col),
            to: Location::Tableau(to_col),
            cards,
            revealed_card: revealed,
            stock_recycled: false,
        });
        self.moves_count += 1;
        true
    }

    pub fn move_foundation_to_tableau(&mut self, found_idx: usize, to_col: usize) -> bool {
        if let Some(card) = self.foundations[found_idx].last() {
            if self.can_place_on_tableau(*card, to_col) {
                let card = self.foundations[found_idx].pop().unwrap();
                self.tableau[to_col].push(card);
                self.history.push(Move {
                    from: Location::Foundation(found_idx),
                    to: Location::Tableau(to_col),
                    cards: vec![card],
                    revealed_card: false,
                    stock_recycled: false,
                });
                self.score = (self.score - 15).max(0);
                self.moves_count += 1;
                return true;
            }
        }
        false
    }

    fn can_place_on_tableau(&self, card: Card, col: usize) -> bool {
        if let Some(top) = self.tableau[col].last() {
            card.can_stack_on_tableau(top)
        } else {
            card.rank == Rank::King
        }
    }

    fn reveal_top(&mut self, col: usize) -> bool {
        if let Some(card) = self.tableau[col].last_mut() {
            if !card.face_up {
                card.face_up = true;
                return true;
            }
        }
        false
    }

    pub fn undo(&mut self) -> bool {
        if let Some(mv) = self.history.pop() {
            if mv.stock_recycled {
                for mut card in mv.cards {
                    card.face_up = true;
                    self.waste.push(card);
                }
                self.stock.clear();
                self.score = (self.score + 100).min(9999);
            } else {
                match (&mv.from, &mv.to) {
                    (Location::Stock, Location::Waste) => {
                        if let Some(mut card) = self.waste.pop() {
                            card.face_up = false;
                            self.stock.push(card);
                        }
                    }
                    (Location::Waste, Location::Foundation(i)) => {
                        if let Some(card) = self.foundations[*i].pop() {
                            self.waste.push(card);
                            self.score -= 10;
                        }
                    }
                    (Location::Waste, Location::Tableau(col)) => {
                        if self.tableau[*col].pop().is_some() {
                            let card = mv.cards[0];
                            self.waste.push(card);
                            self.score -= 5;
                        }
                    }
                    (Location::Tableau(from_col), Location::Foundation(i)) => {
                        if mv.revealed_card {
                            if let Some(card) = self.tableau[*from_col].last_mut() {
                                card.face_up = false;
                            }
                        }
                        if let Some(card) = self.foundations[*i].pop() {
                            self.tableau[*from_col].push(card);
                            self.score -= 10;
                        }
                    }
                    (Location::Tableau(from_col), Location::Tableau(to_col)) => {
                        if mv.revealed_card {
                            if let Some(card) = self.tableau[*from_col].last_mut() {
                                card.face_up = false;
                            }
                        }
                        let count = mv.cards.len();
                        let len = self.tableau[*to_col].len();
                        let cards: Vec<Card> = self.tableau[*to_col].split_off(len - count);
                        self.tableau[*from_col].extend(cards);
                    }
                    (Location::Foundation(i), Location::Tableau(col)) => {
                        if let Some(card) = self.tableau[*col].pop() {
                            self.foundations[*i].push(card);
                            self.score += 15;
                        }
                    }
                    _ => {}
                }
            }
            self.moves_count = self.moves_count.saturating_sub(1);
            true
        } else {
            false
        }
    }

    pub fn get_hint(&self) -> Option<String> {
        // Check waste to foundation
        if let Some(card) = self.waste.last() {
            for i in 0..4 {
                let suit = Suit::all()[i];
                let top = self.foundations[i].last();
                if card.can_stack_on_foundation(top, suit) {
                    return Some(format!("Move {} from waste to foundation", card));
                }
            }
        }

        // Check tableau to foundation
        for col in 0..7 {
            if let Some(card) = self.tableau[col].last() {
                if card.face_up {
                    for i in 0..4 {
                        let suit = Suit::all()[i];
                        let top = self.foundations[i].last();
                        if card.can_stack_on_foundation(top, suit) {
                            return Some(format!(
                                "Move {} from column {} to foundation",
                                card,
                                col + 1
                            ));
                        }
                    }
                }
            }
        }

        // Check tableau to tableau
        for from_col in 0..7 {
            for (idx, card) in self.tableau[from_col].iter().enumerate() {
                if !card.face_up {
                    continue;
                }
                for to_col in 0..7 {
                    if from_col == to_col {
                        continue;
                    }
                    if self.can_place_on_tableau(*card, to_col) {
                        // Don't suggest moving a King to an empty column if it's already at the bottom
                        if card.rank == Rank::King && idx == 0 {
                            continue;
                        }
                        if self.tableau[to_col].is_empty() && card.rank != Rank::King {
                            continue;
                        }
                        return Some(format!(
                            "Move {} from column {} to column {}",
                            card,
                            from_col + 1,
                            to_col + 1
                        ));
                    }
                }
            }
        }

        // Check waste to tableau
        if let Some(card) = self.waste.last() {
            for col in 0..7 {
                if self.can_place_on_tableau(*card, col) {
                    return Some(format!("Move {} from waste to column {}", card, col + 1));
                }
            }
        }

        // Suggest drawing from stock
        if !self.stock.is_empty() {
            return Some("Draw from stock".to_string());
        }

        if !self.waste.is_empty() {
            return Some("Recycle waste pile".to_string());
        }

        None
    }

    pub fn is_won(&self) -> bool {
        self.foundations.iter().all(|f| f.len() == 13)
    }

    pub fn auto_complete_available(&self) -> bool {
        if self.stock.is_empty() && self.waste.is_empty() {
            return self.tableau.iter().all(|col| col.iter().all(|c| c.face_up));
        }
        false
    }

    pub fn auto_complete_step(&mut self) -> bool {
        for col in 0..7 {
            if self.move_tableau_to_foundation(col) {
                return true;
            }
        }
        false
    }
}
