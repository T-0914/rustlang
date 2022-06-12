use std::collections::BTreeMap;

mod matches;

#[derive(Debug)]
pub struct Team {
    matches_played: u64,
    matches_won: u64,
    matches_drawn: u64,
    matches_lost: u64,
    points: u64,
}

pub type TeamName = String;

impl Team {
    fn update_for_win(&mut self) {
        self.matches_played += 1;
        self.matches_won += 1;
        self.points += 3;
    }

    fn update_for_loss(&mut self) {
        self.matches_played += 1;
        self.matches_lost += 1;
    }

    fn update_for_draw(&mut self) {
        self.matches_played += 1;
        self.matches_drawn += 1;
        self.points += 1
    }
}

pub fn tally(match_results: &str) -> String {
    if match_results == "" {
        format!(
            "{0: <30} | {1} |  {2} |  {3} |  {4} |  {5}",
            "Team", "MP", "W", "D", "L", "P"
        )
    } else {
        let report: BTreeMap<TeamName, Team> = matches::format_matches(&match_results);
        let _report: Vec<(&TeamName, &Team)> = sort_report_by_point(&report);

        let mut result: String = String::from("");
        let header_row = format!(
            "{0: <30} | {1} |  {2} |  {3} |  {4} |  {5}\n",
            "Team", "MP", "W", "D", "L", "P"
        );

        result.push_str(&header_row);

        for (_team_name, _team) in _report.iter() {
            let row = _format(
                _team_name,
                _team.matches_played,
                _team.matches_won,
                _team.matches_drawn,
                _team.matches_lost,
                _team.points,
            );

            result.push_str(&row)
        }

        result.pop();
        result
    }
}

fn sort_report_by_point<'a>(
    hashmap: &'a BTreeMap<TeamName, Team>,
) -> Vec<(&'a TeamName, &'a Team)> {
    let mut _hashmap: Vec<(&TeamName, &Team)> = hashmap.iter().collect::<Vec<(&TeamName, &Team)>>();
    _hashmap.sort_by(|a: &(&String, &Team), b: &(&String, &Team)| b.1.points.cmp(&a.1.points));
    _hashmap
}

fn _format(
    team_name: &str,
    matches_played: u64,
    matches_won: u64,
    matches_drawn: u64,
    matches_lost: u64,
    points: u64,
) -> String {
    format!(
        "{0: <30} |  {1} |  {2} |  {3} |  {4} |  {5}\n",
        team_name.to_string(),
        matches_played,
        matches_won,
        matches_drawn,
        matches_lost,
        points
    )
}
