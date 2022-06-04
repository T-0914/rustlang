use std::collections::HashMap;

use crate::Match;
const MATCHES_DIVIDER: char = '\n';
const TEAM_DIVIDER: char = ';';
// const MATCH_RESULT_SIZE: usize = 3;

#[derive(Debug)]
enum MatchResult {
    Win,
    Draw,
    Loss,
}
#[derive(Debug)]
struct MatchReport<'a> {
    first_team: &'a str,
    second_team: &'a str,
    result: MatchResult,
}

/// .
pub fn format_matches<'a>(match_results: &'a str) -> Vec<Match> {
    let splitted_match_results: Vec<&str> = self::split(&match_results, MATCHES_DIVIDER);
    let mut result: HashMap<&'a str, Match> = HashMap::new();

    splitted_match_results.into_iter().for_each(|match_result| {
        let match_report: MatchReport = MatchReport {
            first_team: &self::split(match_result, TEAM_DIVIDER)[0],
            second_team: &self::split(match_result, TEAM_DIVIDER)[1],
            result: map_match_result(self::split(match_result, TEAM_DIVIDER)[2]),
        };
        println!("{:#?}", match_report);
        let result = &mut result;

        match match_report.result {
            MatchResult::Win => {
                if !result.contains_key(&match_report.first_team) {
                    result.insert(
                        match_report.first_team,
                        Match {
                            team: match_report.first_team,
                            matches_played: 1,
                            matches_won: 1,
                            matches_drawn: 0,
                            matches_lost: 0,
                            points: 3,
                        },
                    );
                } else {
                    let old_result = result.get(&match_report.first_team);
                    result.insert(
                        match_report.first_team,
                        Match {
                            team: match_report.first_team,
                            matches_played: old_result.unwrap().matches_played + 1,
                            matches_won: old_result.unwrap().matches_won + 1,
                            matches_drawn: old_result.unwrap().matches_drawn,
                            matches_lost: old_result.unwrap().matches_lost,
                            points: old_result.unwrap().points + 3,
                        },
                    );
                }
            }
            MatchResult::Draw => {
                if !result.contains_key(&match_report.first_team) {
                    result.insert(
                        match_report.first_team,
                        Match {
                            team: match_report.first_team,
                            matches_played: 1,
                            matches_won: 0,
                            matches_drawn: 1,
                            matches_lost: 0,
                            points: 1,
                        },
                    );
                } else {
                    let old_result = result.get(&match_report.first_team);
                    result.insert(
                        match_report.first_team,
                        Match {
                            team: match_report.first_team,
                            matches_played: old_result.unwrap().matches_played + 1,
                            matches_won: old_result.unwrap().matches_won,
                            matches_drawn: old_result.unwrap().matches_drawn + 1,
                            matches_lost: old_result.unwrap().matches_lost,
                            points: old_result.unwrap().points + 1,
                        },
                    );
                }
            }
            MatchResult::Loss => {
                if !result.contains_key(&match_report.first_team) {
                    result.insert(
                        match_report.first_team,
                        Match {
                            team: match_report.first_team,
                            matches_played: 1,
                            matches_won: 0,
                            matches_drawn: 0,
                            matches_lost: 1,
                            points: 0,
                        },
                    );
                } else {
                    let old_result = result.get(&match_report.first_team);
                    result.insert(
                        match_report.first_team,
                        Match {
                            team: match_report.first_team,
                            matches_played: old_result.unwrap().matches_played + 1,
                            matches_won: old_result.unwrap().matches_won,
                            matches_drawn: old_result.unwrap().matches_drawn,
                            matches_lost: old_result.unwrap().matches_lost + 1,
                            points: old_result.unwrap().points,
                        },
                    );
                }
            }
        }
    });

    println!("{:#?}", result);
    let match_1 = Match {
        team: "1111",
        matches_played: 1,
        matches_won: 1,
        matches_drawn: 0,
        matches_lost: 0,
        points: 3,
    };
    vec![match_1]
}

fn split(string: &str, divider: char) -> Vec<&str> {
    string.split(divider).collect()
}

// fn create_report_struct<'a>(data: &'a Vec<&str>) -> MatchReport<'a> {
//     MatchReport {
//         first_team: &data[0].to_string(),
//         second_team: &data[1].to_string(),
//         result: map_match_result(data[2]),
//     }
// }

fn map_match_result(result: &str) -> MatchResult {
    match result {
        "win" => MatchResult::Win,
        "draw" => MatchResult::Draw,
        "loss" => MatchResult::Loss,
        _ => panic!("Unexpected invalid token"),
    }
}
