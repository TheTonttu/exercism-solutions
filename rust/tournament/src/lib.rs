use std::collections::HashMap;

const WIN_POINTS: u32 = 3;
const DRAW_POINTS: u32 = 1;

enum MatchResult<'a> {
    Win {
        home_team: &'a str,
        visiting_team: &'a str,
    },
    Draw {
        home_team: &'a str,
        visiting_team: &'a str,
    },
    Loss {
        home_team: &'a str,
        visiting_team: &'a str,
    },
}

#[derive(Copy, Clone)]
struct TeamStatistics<'a> {
    team_name: &'a str,
    matches_played: u32,
    matches_won: u32,
    matches_drawn: u32,
    matches_lost: u32,
    score: u32,
}

impl<'a> TeamStatistics<'a> {
    fn new(team_name: &'a str) -> Self {
        Self {
            team_name,
            matches_played: 0,
            matches_won: 0,
            matches_drawn: 0,
            matches_lost: 0,
            score: 0,
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let results = parse_match_results(match_results);
    let team_stats = calculate_team_stats(results);
    format_table(team_stats)
}

fn parse_match_results(match_results: &str) -> Vec<MatchResult> {
    match_results
        .split('\n')
        .map(|l| {
            let elements: Vec<&str> = l.split(';').collect();
            match elements.as_slice() {
                [home_team, visiting_team, outcome] => match *outcome {
                    "win" => Some(MatchResult::Win {
                        home_team,
                        visiting_team,
                    }),
                    "draw" => Some(MatchResult::Draw {
                        home_team,
                        visiting_team,
                    }),
                    "loss" => Some(MatchResult::Loss {
                        home_team,
                        visiting_team,
                    }),
                    _ => None,
                },
                _ => None,
            }
        })
        .filter_map(|r| r)
        .collect()
}

fn calculate_team_stats(match_results: Vec<MatchResult>) -> Vec<TeamStatistics> {
    let mut team_stats: HashMap<&str, TeamStatistics> = HashMap::new();

    match_results.iter().for_each(|res| match res {
        MatchResult::Win {
            home_team,
            visiting_team,
        } => {
            let ht = team_stats
                .entry(home_team)
                .or_insert_with(|| TeamStatistics::new(home_team));
            ht.matches_played += 1;
            ht.matches_won += 1;
            ht.score += WIN_POINTS;

            let vt = team_stats
                .entry(visiting_team)
                .or_insert_with(|| TeamStatistics::new(visiting_team));
            vt.matches_played += 1;
            vt.matches_lost += 1;
        }
        MatchResult::Draw {
            home_team,
            visiting_team,
        } => {
            let ht = team_stats
                .entry(home_team)
                .or_insert_with(|| TeamStatistics::new(home_team));
            ht.matches_played += 1;
            ht.matches_drawn += 1;
            ht.score += DRAW_POINTS;

            let vt = team_stats
                .entry(visiting_team)
                .or_insert_with(|| TeamStatistics::new(visiting_team));
            vt.matches_played += 1;
            vt.matches_drawn += 1;
            vt.score += DRAW_POINTS;
        }
        MatchResult::Loss {
            home_team,
            visiting_team,
        } => {
            let ht = team_stats
                .entry(home_team)
                .or_insert_with(|| TeamStatistics::new(home_team));
            ht.matches_played += 1;
            ht.matches_lost += 1;

            let vt = team_stats
                .entry(visiting_team)
                .or_insert_with(|| TeamStatistics::new(visiting_team));
            vt.matches_played += 1;
            vt.matches_won += 1;
            vt.score += WIN_POINTS;
        }
    });

    let mut stats: Vec<TeamStatistics> = team_stats.values().copied().collect();
    stats.sort_by(|ts1, ts2| {
        ts2.score
            .cmp(&ts1.score)
            .then(ts1.team_name.cmp(&ts2.team_name))
    });
    stats
}

fn format_table(team_stats: Vec<TeamStatistics>) -> String {
    let header = format_table_header();
    let content = format_team_stats(team_stats);

    match content.is_empty() {
        true => header,
        false => [header, content].join("\n"),
    }
}

fn format_team_stats(team_stats: Vec<TeamStatistics>) -> String {
    team_stats
        .iter()
        .map(|ts| {
            format!(
                "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
                ts.team_name,
                ts.matches_played,
                ts.matches_won,
                ts.matches_drawn,
                ts.matches_lost,
                ts.score
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_table_header() -> String {
    "Team                           | MP |  W |  D |  L |  P".to_string()
}
