// #[warn(dead_code)]
// use tabled::{Table, Tabled};

// #[derive(Tabled)]
#[derive(Debug)]
pub struct Match<'a> {
    team: &'a str,
    matches_played: u64,
    matches_won: u64,
    matches_drawn: u64,
    matches_lost: u64,
    points: u64,
}

mod matches;

pub fn tally(match_results: &str) -> String {
    // unimplemented!(
    //     "Given the result of the played matches '{}' return a properly formatted tally table string.",
    //     match_results
    // );
    let matches: Vec<Match> = matches::format_matches(&match_results);
    // Table::new(matches).to_string()

    "debug".to_string()
}
