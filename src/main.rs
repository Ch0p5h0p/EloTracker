mod cli;
mod config;
mod matches;
mod player;
mod rating;
mod scoring;
mod storage;

use clap::Parser;

use cli::*;

use crate::matches::Match;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut data = match (&cli.file, &cli.fnew) {
        (Some(path), None) => match storage::Data::load(path) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("failed to load data: {e}");
                std::process::exit(1);
            }
        },
        (None, Some(path)) => {
            if std::path::Path::new(path).exists() {
                return Err(format!("File {path} already exists.").into());
            }
            storage::Data::new(path)
        }
        _ => unreachable!(),
    };

    match cli.command {
        Some(Command::Player { command }) => match command {
            PlayerCommand::Add { name } => {
                data.new_player(name.as_str())?;
            }
            PlayerCommand::List => {
                data.display_players();
            }
            PlayerCommand::Remove { name } => {
                data.remove_player(name.as_str())?;
            }
            PlayerCommand::ManualUpdate { id, name, rating } => {
                data.update_player(id, (name, rating))?;
            }
        },

        Some(Command::Match { command }) => match command {
            MatchCommand::Record {
                player_a,
                player_b,
                result,
            } => {
                let player_a_id = data.get_player_id(player_a.as_str());
                let player_b_id = data.get_player_id(player_b.as_str());
                let scores = match result {
                    MatchResult {
                        score: Some(scores),
                        ..
                    } => scores,
                    MatchResult {
                        result: Some(outcome),
                        ..
                    } => match outcome {
                        cli::Outcome::Win => vec![1.0, 0.0],
                        cli::Outcome::Loss => vec![0.0, 1.0],
                        cli::Outcome::Draw => vec![0.5, 0.5],
                    },
                    _ => unreachable!(),
                };

                let m = Match::new(player_a_id, player_b_id, scores);
                let k = data.config.k;
                data.add_match(&m);
                let (player_a, player_b) = data.get_mut_pair(player_a_id, player_b_id);

                rating::update(player_a, player_b, &m, k);

                println!(
                    "{} -> {:.2}, {} -> {:.2}",
                    player_a.name, player_a.rating, player_b.name, player_b.rating
                );
            }
            MatchCommand::List { player } => {
                match player {
                    Some(name) => data.display_matches_by_player(name.as_str()),
                    None => data.display_matches(),
                }
            }
        },

        Some(Command::Ranking { top }) => match top {
            Some(n) => data.display_top(n),
            None => data.display_top(0),
        },

        None => {}
    }

    data.save()?;

    Ok(())
}
