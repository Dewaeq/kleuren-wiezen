use std::time::Instant;

use crate::game::Game;

pub fn bench(size: Option<usize>) {
    let size = size.unwrap_or(80_000);
    println!("Simulating {size} random games...");

    let mut games = Vec::with_capacity(size);

    for _ in 0..size {
        games.push(Game::new());
    }

    let start = Instant::now();

    for game in &mut games {
        for _ in 0..13 {
            game.play_trick();
        }
    }

    println!(
        "{}ms, {} games/s",
        start.elapsed().as_millis(),
        (size as f64) / start.elapsed().as_secs_f64()
    );

    let mut score = [0; 4];
    for game in &mut games {
        for i in 0..4 {
            score[i] += game.score[i];
        }
    }

    println!("{score:?}");
}