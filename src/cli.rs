use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "elo", 
    about = "Elo-based ranking tracker",
    group(
        clap::ArgGroup::new("storage")
        .required(true)
        .multiple(false)
        .args(["file", "fnew"])
    )
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(short, long, help = "Elo JSON file to use")]
    pub file: Option<String>,

    #[arg(long, help = "Create a new Elo JSON file")]
    pub fnew: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Player management
    Player {
        #[command(subcommand)]
        command: PlayerCommand,
    },

    /// Match recording and inspecting
    Match {
        #[command(subcommand)]
        command: MatchCommand,
    },

    /// Viewing rankings
    Ranking {
        #[arg(short, long, help = "list the top rated players")]
        top: Option<usize>,
    },
}

#[derive(Subcommand, Debug)]
pub enum PlayerCommand {
    /// Add a new player
    Add { name: String },
    /// List players
    List,
    /// Remove a player
    Remove { name: String },
    /// Manually update name and/or rating points
    ManualUpdate {
        id: u64,
        name: Option<String>,
        rating: Option<f64>,
    },
}

#[derive(Subcommand, Debug)]
pub enum MatchCommand {
    /// Record a match
    Record {
        player_a: String,
        player_b: String,

        #[command(flatten)]
        result: MatchResult,
    },
    /// List all matches
    List {
        #[arg(long, short, help = "Player to search filter matches for")]
        player: Option<String>
    },
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct MatchResult {
    #[arg(
        long,
        num_args = 2,
        help = "Input raw scores. The program will normalize them"
    )]
    pub score: Option<Vec<f64>>,

    #[arg(long, value_enum, help = "RESULT is from PLAYER_A's perspective")]
    pub result: Option<Outcome>,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Outcome {
    Win,
    Loss,
    Draw,
}
