use crate::matches::Match;
use crate::player::Player;
use crate::scoring;

pub fn expected(a: &Player, b: &Player) -> f64 {
    1.0 / (1.0 + 10.0_f64.powf((b.rating - a.rating) / 400.0))
}

pub fn update(a: &mut Player, b: &mut Player, game: &Match, k: f64) {
    if a.id != game.player_a || b.id != game.player_b {
        panic!("player IDs don't match.");
    }

    let expected_a = expected(a, b);
    let expected_b = 1.0 - expected_a;

    let (actual_a, actual_b) = scoring::normalize(game.score_a, game.score_b);

    let new_a = a.rating + k * (actual_a - expected_a);
    let new_b = b.rating + k * (actual_b - expected_b);

    a.rating = new_a;
    b.rating = new_b;
}
