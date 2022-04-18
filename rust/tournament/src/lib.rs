use std::collections::HashMap;
use std::cmp::Ordering;

pub fn tally(match_results: &str) -> String {
    
    let matches: Vec<Match> = match_results
        .split("\n")
        .filter_map(|x| Match::from_string(x))
        .collect();

    let mut standings: HashMap<String, TeamStats> = HashMap::new();

    for _match in matches {
        let mut home_team_stats = standings.entry(_match.home_team.to_owned()).or_insert(TeamStats::new()).to_owned();
        let mut away_team_stats = standings.entry(_match.away_team.to_owned()).or_insert(TeamStats::new()).to_owned();
        home_team_stats.match_played += 1;
        away_team_stats.match_played += 1;
        if _match.result == MatchResult::Win {
            home_team_stats.win += 1;
            away_team_stats.loss += 1;
        } else if _match.result == MatchResult::Loss {
            away_team_stats.win += 1;
            home_team_stats.loss += 1;
        } else if _match.result == MatchResult::Draw {
            home_team_stats.draw += 1;
            away_team_stats.draw += 1;
        }
        standings.insert(_match.home_team.to_owned(), home_team_stats);
        standings.insert(_match.away_team.to_owned(), away_team_stats);
    }

    let mut team_table_rows: Vec<TableRow> = Vec::new();
    for (team_name, team_stats) in standings {
        team_table_rows.push(TableRow { 
            team_name: team_name,
            match_played: team_stats.match_played,
            win: team_stats.win,
            draw: team_stats.draw,
            loss: team_stats.loss,
            points: team_stats.win * 3 + team_stats.draw
        })
    }

    team_table_rows.sort();

    return build_table(team_table_rows);
}

fn build_table(team_rows: Vec<TableRow>) -> String {
    println!("{:?}", team_rows);
    let total_team_name_string_lenght: usize = 31;
    let mut header = pad_str("Team".to_string(), total_team_name_string_lenght);
    header += "| MP ";
    header += "|  W ";
    header += "|  D ";
    header += "|  L ";
    header += "|  P";
    if team_rows.len() == 0 {
        return header;
    }
    let team_rows_strings: Vec<String> = team_rows
        .iter()
        .map(|team_row| {
            let mut team_row_string = "".to_string();
            team_row_string += &pad_str(team_row.team_name.to_string(), total_team_name_string_lenght);
            team_row_string += &format!("|  {:} ", team_row.match_played);
            team_row_string += &format!("|  {:} ", team_row.win);
            team_row_string += &format!("|  {:} ", team_row.draw);
            team_row_string += &format!("|  {:} ", team_row.loss);
            team_row_string += &format!("|  {:}", team_row.points);
            return team_row_string;
        })
        .collect();
    let team_rows_string = team_rows_strings.join("\n");
    return [header, team_rows_string].join("\n");
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct TableRow {
    pub team_name: String,
    pub match_played: u8,
    pub win: u8,
    pub draw: u8,
    pub loss: u8,
    pub points: u8
}

impl PartialOrd for TableRow {
    
    fn partial_cmp(&self, other: &TableRow) -> Option<Ordering> {
        if other.points == self.points {
            return self.team_name.partial_cmp(&other.team_name);
        } else {
            return other.points.partial_cmp(&self.points);
        }
    }
}

#[derive(Debug, Clone)]
struct TeamStats {
    pub match_played: u8,
    pub win: u8,
    pub draw: u8,
    pub loss: u8
}

impl TeamStats {
    fn new() -> TeamStats {
        return TeamStats {
            match_played: 0,
            win: 0,
            draw: 0,
            loss: 0
        }
    }
}

#[derive(Debug)]
struct Match {
    pub home_team: String,
    pub away_team: String,
    pub result: MatchResult
}

#[derive(Debug, PartialEq)]
enum MatchResult {
    Win,
    Loss,
    Draw
}

impl Match {

    fn from_string(string: &str) -> Option<Match> {
        let components: Vec<&str> = string.split(";").collect();
        if components.len() != 3 {
            return Option::None;
        } else {
            let match_result = MatchResult::from_string(components[2]);
            if let Some(match_result) = match_result {
                return Option::Some(Match {
                    home_team: components[0].to_string(),
                    away_team: components[1].to_string(), 
                    result: match_result
                })
            } else {
                return Option::None;
            }
        }
    }
}

impl MatchResult {

    fn from_string(string: &str) -> Option<MatchResult> {
        return match string {
            "win" => Option::Some(MatchResult::Win),
            "loss" => Option::Some(MatchResult::Loss),
            "draw" => Option::Some(MatchResult::Draw),
            _ => { return Option::None; }
        }
    }
}

fn pad_str(string: String, total_length: usize) -> String {
    let mut padded_str = string;
    while padded_str.len() < total_length {
        padded_str += " ";
    }
    return padded_str;
}