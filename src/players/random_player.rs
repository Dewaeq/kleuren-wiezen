use crate::{
    card::{Card, Cards},
    game::Game,
};

use super::Player;

#[derive(Default)]
pub struct RandomPlayer {
    cards: Cards,
    index: usize,
}

impl Player for RandomPlayer {
    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn cards(&self) -> Cards {
        self.cards
    }

    fn cards_mut(&mut self) -> &mut Cards {
        &mut self.cards
    }

    fn decide(&self, game: &Game) -> Card {
        loop {
            let card = self.cards.pick_random_card();

            if game.is_legal(card, self.index) {
                return card;
            }
        }
    }
}
