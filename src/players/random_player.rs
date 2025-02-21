use rand::{seq::SliceRandom, thread_rng, Rng};

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
        let mut rng = thread_rng();
        let my_cards = self.cards.into_array_13();

        loop {
            let mut card = my_cards[rng.gen_range(0..my_cards.len())];
            card.set_player(self.index);

            if game.is_legal(card) {
                return card;
            }
        }
    }
}
