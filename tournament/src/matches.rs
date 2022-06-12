use crate::{Team, TeamName};
use std::collections::BTreeMap;

const MATCHES_DIVIDER: char = '\n';
const TEAM_DIVIDER: char = ';';
const WIN: &str = "win";
const DRAW: &str = "draw";
const LOSS: &str = "loss";

/// .
pub fn format_matches<'a>(match_results: &'a str) -> BTreeMap<TeamName, Team> {
    let mut teams: BTreeMap<TeamName, Team> = list_team(match_results);
    count_point_for_teams(&mut teams, match_results);
    teams
}

fn list_team(match_results: &str) -> BTreeMap<TeamName, Team> {
    let mut teams: BTreeMap<String, Team> = BTreeMap::new();
    let _match_results: String = replace_break_line(match_results);
    let splitted_match_result: Vec<&str> = split(&_match_results, TEAM_DIVIDER);
    let mut raw_teams: Vec<&str> = removed_wrong_value(splitted_match_result);

    raw_teams.sort();
    raw_teams.dedup();

    for team in raw_teams {
        let new_team = Team {
            matches_played: 0,
            matches_won: 0,
            matches_drawn: 0,
            matches_lost: 0,
            points: 0,
        };
        teams.insert(team.to_string(), new_team);
    }
    teams
}

fn count_point_for_teams(teams: &mut BTreeMap<TeamName, Team>, match_results: &str) {
    let _match_results: String = replace_break_line(match_results);
    let splitted_match_result: Vec<&str> = split(&_match_results, TEAM_DIVIDER);
    let chucks_of_matches = splitted_match_result.chunks(3);

    for chuck in chucks_of_matches.into_iter() {
        for (index, &seq) in chuck.iter().enumerate() {
            match seq {
                WIN => {
                    let won_team = teams.get_mut(chuck[index - 2]).unwrap();
                    won_team.update_for_win();

                    let loss_team = teams.get_mut(chuck[index - 1]).unwrap();
                    loss_team.update_for_loss();
                }
                DRAW => {
                    let first_team = teams.get_mut(chuck[index - 2]).unwrap();
                    first_team.update_for_draw();

                    let second_team = teams.get_mut(chuck[index - 1]).unwrap();
                    second_team.update_for_draw();
                }
                LOSS => {
                    let loss_team = teams.get_mut(chuck[index - 2]).unwrap();
                    loss_team.update_for_loss();

                    let won_team = teams.get_mut(chuck[index - 1]).unwrap();
                    won_team.update_for_win();
                }
                _ => continue,
            }
        }
    }
}

fn split(string: &str, divider: char) -> Vec<&str> {
    string.split(divider).collect::<Vec<&str>>()
}

fn replace_break_line(seq: &str) -> String {
    seq.replace(MATCHES_DIVIDER, &TEAM_DIVIDER.to_string())
}

fn removed_wrong_value(list_teams: Vec<&str>) -> Vec<&str> {
    list_teams
        .iter()
        .filter(|&&seq| is_not_a_team(seq))
        .cloned()
        .collect()
}

fn is_not_a_team(seq: &str) -> bool {
    seq != WIN && seq != DRAW && seq != LOSS
}
