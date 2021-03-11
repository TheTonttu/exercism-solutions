struct TeamStatistics {
    matches_played: u32,
    matches_won: u32,
    matches_drawn: u32,
    matches_lost: u32,
}

enum MatchResult {
    Win { home_team: String, visiting_team: String },
    Draw { home_team: String, visiting_team: String },
    Loss { home_team: String, visiting_team: String },
}

pub fn tally(match_results: &str) -> String {
    let results = parse_match_results(match_results);
    let team_stats = calculate_team_stats(results);
    format_table(team_stats)
}

fn parse_match_results(match_results: &str) -> Vec<MatchResult> {
    let lines : Vec<&str> = match_results.split('\n').collect();

    lines.iter().map(|l| {
        let elements: Vec<&str> = l.split(';').collect();
        match elements.as_slice() {
            [home_team, visiting_team, outcome] => {
                match *outcome {
                    "win" => Some(MatchResult::Win { home_team: home_team.to_string(), visiting_team: visiting_team.to_string() }),
                    "draw" => Some(MatchResult::Draw { home_team: home_team.to_string(), visiting_team: visiting_team.to_string() }),
                    "loss" => Some(MatchResult::Loss { home_team: home_team.to_string(), visiting_team: visiting_team.to_string() }),
                    _ => None
                }
            },
            _ => None
        }
    }).filter_map(|r| r)
    .collect()
}

fn calculate_team_stats(match_results: Vec<MatchResult>) -> Vec<TeamStatistics> {
    Vec::new()
}

fn format_table(team_stats: Vec<TeamStatistics>) -> String {
    let header = format_table_header();
    let content = format_team_stats(team_stats);

    match content.is_empty() {
        true => header,
        false => [header, content].join("\n")
    }
}

fn format_team_stats(team_stats: Vec<TeamStatistics>) -> String {
    String::new()
}

fn format_table_header() -> String {
    "Team                           | MP |  W |  D |  L |  P".to_string()
}
