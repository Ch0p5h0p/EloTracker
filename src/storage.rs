use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::EloConfig;
use crate::matches::Match;
use crate::player::Player;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Data {
    pub config: EloConfig,
    players: Vec<Player>,
    matches: Vec<Match>,
    #[serde(skip)]
    pub path: PathBuf,
}

impl Data {
    pub fn new(filepath: &str) -> Self {
        Data {
            config: EloConfig { k: 32.0 },
            players: Vec::new(),
            matches: Vec::new(),
            path: PathBuf::from(filepath),
        }
    }

    pub fn get_player_id(&self, playername: &str) -> u64 {
        self.players
            .iter()
            .find(|x| x.name == playername)
            .ok_or(format!("Player {playername} not found"))
            .unwrap()
            .id
    }

    pub fn get_player_name(&self, id: u64) -> String {
        self.players
            .iter()
            .find(|p| p.id == id)
            .ok_or(format!("Player {id} not found"))
            .unwrap()
            .name
            .clone()
    }

    pub fn get_mut_pair(&mut self, id_a: u64, id_b: u64) -> (&mut Player, &mut Player) {
        let index_a = self.players.iter().position(|p| p.id == id_a).unwrap();
        let index_b = self.players.iter().position(|p| p.id == id_b).unwrap();

        assert_ne!(index_a, index_b, "Cannot get the same player twice");

        let (first, second) = if index_a < index_b {
            let (left, right) = self.players.split_at_mut(index_b);
            (&mut left[index_a], &mut right[0])
        } else {
            let (left, right) = self.players.split_at_mut(index_a);
            (&mut right[0], &mut left[index_b])
        };

        (first, second)
    }

    pub fn update_player(
        &mut self,
        id: u64,
        new_data: (Option<String>, Option<f64>),
    ) -> Result<(), Box<dyn std::error::Error>> {
        let player = self
            .players
            .iter_mut()
            .find(|p| p.id == id)
            .ok_or(format!("Player not found: {id}"))?;
        let (new_name, new_rating) = new_data;
        if let Some(name) = new_name {
            player.name = name;
        };
        if let Some(rating) = new_rating {
            player.rating = rating;
        };

        Ok(())
    }

    pub fn next_id(&self) -> Option<u64> {
        self.players
            .iter()
            .map(|p| p.id)
            .max()
            .unwrap_or(0)
            .checked_add(1)
    }

    pub fn new_player(&mut self, playername: &str) -> Result<(), Box<dyn std::error::Error>> {
        let id = match self.next_id() {
            Some(n) => n,
            None => panic!("u64 overflow"),
        };

        if let Some(p) = self.players.iter().find(|p| p.name == playername) {
            return Err(format!("Player {} already exists.", p.name).into());
        }

        self.players.push(Player {
            id,
            name: playername.to_string(),
            rating: 1000.0,
        });

        Ok(())
    }

    pub fn remove_player(&mut self, playername: &str) -> Result<(), Box<dyn std::error::Error>> {
        let player_index = self
            .players
            .iter_mut()
            .position(|p| p.name == playername)
            .ok_or(format!("Player {playername} not found"))?;

        self.players.remove(player_index);

        Ok(())
    }

    pub fn add_match(&mut self, m: &Match) {
        self.matches.push(m.clone());
    }

    pub fn display_players(&self) {
        println!("┌{:─<18}┬{:─<18}┬{:─<12}┐", "", "", "");
        println!("│ {:16} │ {:<16} │ {:^10} │", "ID", "Name", "Rating");
        println!("├{:─<18}┼{:─<18}┼{:─<12}┤", "", "", "");
        for p in self.players.iter() {
            println!("│ {:^16x} │ {:<16} │ {:^10.2} │", p.id, p.name, p.rating);
        }
        println!("└{:─<18}┴{:─<18}┴{:─<12}┘", "", "", "");
    }

    pub fn display_matches(&self) {
        println!("┌{:─<18}┬{:─<18}┬{:─<18}┬{:─<18}┬{:─<22}┐", "", "", "", "", "");
        println!(
            "│ {:<16} │ {:<16} │ {:<16} │ {:<16} │ {:<20} │",
            "Player 1", "Player 2", "Score", "Adjustment", "Timestamp"
        );
        println!("├{:─<18}┼{:─<18}┼{:─<18}┼{:─<18}┼{:─<22}┤", "", "", "", "", "");
        for m in self.matches.iter() {
            println!(
                "│ {:<16} │ {:<16} │ {:^16} │ {:^16} │ {:^20} │",
                self.get_player_name(m.player_a),
                self.get_player_name(m.player_b),
                format_scores(m.score_a, m.score_b, self.get_player_name(m.player_a)),
                format!("{:+.2}, {:+.2}", m.adjustment_a, m.adjustment_b),
                m.timestamp
            );
        }
        println!("└{:─<18}┴{:─<18}┴{:─<18}┴{:─<18}┴{:─<22}┘", "", "", "", "", "");
    }

    pub fn display_matches_by_player(&self, playername: &str) {
        println!("┌{:─<18}┬{:─<18}┬{:─<18}┬{:─<18}┐", "", "", "", "");
        println!(
            "│ {:<16} │ {:<16} │ {:<16} │ {:<16} │",
            "Player 1", "Player 2", "Score", "Adjustment"
        );
        println!("├{:─<18}┼{:─<18}┼{:─<18}┼{:─<18}┤", "", "", "", "");
        for m in self.matches.iter() {
            let player_a = self.get_player_name(m.player_a);
            let player_b = self.get_player_name(m.player_b);
            if player_a != playername && player_b != playername {
                continue;
            }

            println!(
                "│ {:<16} │ {:<16} │ {:^16} │ {:^16} │",
                self.get_player_name(m.player_a),
                self.get_player_name(m.player_b),
                format_scores(m.score_a, m.score_b, self.get_player_name(m.player_a)),
                format!("{:+.2}, {:+.2}", m.adjustment_a, m.adjustment_b)
            );
        }
        println!("└{:─<18}┴{:─<18}┴{:─<18}┴{:─<18}┘", "", "", "", "");
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path: &Path = self.path.as_ref();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = serde_json::to_string_pretty(self)?;
        fs::write(path, contents)?;

        Ok(())
    }

    pub fn load(filepath: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(filepath)?;
        let mut data: Self = serde_json::from_str(&contents)?;

        data.path = PathBuf::from(filepath);

        Ok(data)
    }

    pub fn display_top(&self, n: usize) {
        let mut sorted_players = self.players.clone();
        if sorted_players.len() == 0 {
            eprintln!("No players to rank");
            std::process::exit(1);
        }
        sorted_players.sort_by(|pa, pb| {
            pa.rating
                .partial_cmp(&pb.rating)
                .unwrap_or(std::cmp::Ordering::Greater)
        });

        sorted_players.reverse();
        let score_size = usize::try_from(sorted_players.len().ilog10() + 1).unwrap() + 2;

        println!(
            "┌{:─<size$}─{:─<18}─{:─<12}┐",
            "",
            "",
            "",
            size = score_size + 3
        );
        if n == 0 {
            println!("│ {:^size$} │", "RANKING", size = score_size + 33);
            println!(
                "├{:─<size$}┬{:─<18}┬{:─<12}┤",
                "",
                "",
                "",
                size = score_size + 3
            );
            for (i, p) in sorted_players.iter().enumerate() {
                println!(
                    "│ #{:<size$} │ {:<16} │ {:^10.2} │",
                    i + 1,
                    p.name,
                    p.rating,
                    size = score_size
                );
            }
            println!(
                "└{:─<size$}┴{:─<18}┴{:─<12}┘",
                "",
                "",
                "",
                size = score_size + 3
            );
        } else {
            println!(
                "│ {:^size$} │",
                format!("TOP {}", n),
                size = score_size + 33
            );
            println!(
                "├{:─<size$}┬{:─<18}┬{:─<12}┤",
                "",
                "",
                "",
                size = score_size + 3
            );
            for (i, p) in sorted_players.iter().enumerate() {
                if i == n {
                    break;
                }
                println!(
                    "│ #{:<size$} │ {:<16} │ {:^10.2} │",
                    i + 1,
                    p.name,
                    p.rating,
                    size = score_size
                );
            }
            println!(
                "└{:─<size$}┴{:─<18}┴{:─<12}┘",
                "",
                "",
                "",
                size = score_size + 3
            );
        }
    }
}

fn format_scores(a: f64, b: f64, player_a: String) -> String {
    if (a + b - 1.0).abs() < f64::EPSILON {
        match (a, b) {
            (1.0, 0.0) => format!("{} WIN", player_a),
            (0.0, 1.0) => format!("{} LOSS", player_a),
            (0.5, 0.5) => format!("{} DRAW", player_a),
            _ => format!(
                "{} - {}",
                format!("{a:.2}")
                    .trim_end_matches('0')
                    .trim_end_matches('.'),
                format!("{b:.2}")
                    .trim_end_matches('0')
                    .trim_end_matches('.'),
            ),
        }
    } else {
        format!(
            "{} - {}",
            format!("{a:.2}")
                .trim_end_matches('0')
                .trim_end_matches('.'),
            format!("{b:.2}")
                .trim_end_matches('0')
                .trim_end_matches('.'),
        )
    }
}
