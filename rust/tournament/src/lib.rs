use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;

struct Tournament<'a> {
    wins: HashMap<&'a str, u32>,
    losses: HashMap<&'a str, u32>,
    draws: HashMap<&'a str, u32>,
    played: HashMap<&'a str, u32>,
    points: HashMap<&'a str, u32>,
}

impl<'a> Tournament<'a> {
    fn new() -> Self {
        Self {
            wins: HashMap::new(),
            losses: HashMap::new(),
            draws: HashMap::new(),
            played: HashMap::new(),
            points: HashMap::new(),
        }
    }

    fn parse_result(&mut self, result: &'a str) {
        let res: Vec<&str> = result.split(";").collect();
        if let (Some(&team1), Some(&team2), Some(&outcome)) = (res.get(0), res.get(1), res.get(2)) {
            match outcome {
                "win" => {
                    self.add_win(team1);
                    self.add_loss(team2);
                }
                "loss" => {
                    self.add_loss(team1);
                    self.add_win(team2);
                }
                "draw" => {
                    self.add_draw(team1);
                    self.add_draw(team2);
                }
                _ => panic!("Invalid result"),
            }
        }
    }

    fn add_win(&mut self, team: &'a str) {
        let num_wins = self.wins.entry(team).or_insert(0);
        *num_wins += 1;

        let num_played = self.played.entry(team).or_insert(0);
        *num_played += 1;

        let num_points = self.points.entry(team).or_insert(0);
        *num_points += 3;
    }

    fn add_loss(&mut self, team: &'a str) {
        let num_losses = self.losses.entry(team).or_insert(0);
        *num_losses += 1;

        let num_played = self.played.entry(team).or_insert(0);
        *num_played += 1;
    }

    fn add_draw(&mut self, team: &'a str) {
        let num_draws = self.draws.entry(team).or_insert(0);
        *num_draws += 1;

        let num_played = self.played.entry(team).or_insert(0);
        *num_played += 1;

        let num_points = self.points.entry(team).or_insert(0);
        *num_points += 1;
    }

    fn get_played(&self, team: &str) -> u32 {
        match self.played.get(team) {
            None => 0,
            Some(&n) => n,
        }
    }

    fn get_wins(&self, team: &str) -> u32 {
        match self.wins.get(team) {
            None => 0,
            Some(&n) => n,
        }
    }

    fn get_losses(&self, team: &str) -> u32 {
        match self.losses.get(team) {
            None => 0,
            Some(&n) => n,
        }
    }

    fn get_draws(&self, team: &str) -> u32 {
        match self.draws.get(team) {
            None => 0,
            Some(&n) => n,
        }
    }

    fn get_points(&self, team: &str) -> u32 {
        match self.points.get(team) {
            None => 0,
            Some(&n) => n,
        }
    }
}

impl<'a> fmt::Display for Tournament<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let heading = format!("{:<31}| MP |  W |  D |  L |  P", "Team");

        let teams = self
            .played
            .keys()
            .sorted()
            .rev()
            .sorted_by_key(|&x| self.get_points(x))
            .rev();
        let table = teams.fold(heading, |mut output, team| {
            let row = format!(
                "\n{:<31}| {:2} | {:2} | {:2} | {:2} | {:2}",
                team,
                self.get_played(team),
                self.get_wins(team),
                self.get_draws(team),
                self.get_losses(team),
                self.get_points(team)
            );
            output.push_str(&row);
            output
        });
        write!(f, "{}", table)
    }
}

pub fn tally(match_results: &str) -> String {
    let mut tournament = Tournament::new();
    for result in match_results.split("\n") {
        tournament.parse_result(result);
    }
    tournament.to_string()
}
