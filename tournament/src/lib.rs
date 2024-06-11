use std::collections::HashMap;

// tuple for stats
pub struct Stats {
    mp: u32, // matches played
    w: u32,  // wins
    d: u32,  // draws
    l: u32,  // losses
    p: u32,  // points
}
pub struct Team {
    name: String,
    stats: Stats,
}

impl Team {
    pub fn new(
        name: &str,
        mp: Option<u32>,
        w: Option<u32>,
        d: Option<u32>,
        l: Option<u32>,
        p: Option<u32>,
    ) -> Self {
        Team {
            name: name.to_string(),
            stats: Stats {
                mp: mp.unwrap_or(0),
                w: w.unwrap_or(0),
                d: d.unwrap_or(0),
                l: l.unwrap_or(0),
                p: p.unwrap_or(0),
            },
        }
    }

    pub fn update_stats(&mut self, result: &str) {
        match result {
            "win" => {
                self.stats.mp += 1;
                self.stats.w += 1;
                self.stats.p += 3;
            }
            "draw" => {
                self.stats.mp += 1;
                self.stats.d += 1;
                self.stats.p += 1;
            }
            "loss" => {
                self.stats.mp += 1;
                self.stats.l += 1;
            }
            _ => panic!("Invalid result"),
        }
    }

    pub fn print_results(&self) -> String {
        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.name, self.stats.mp, self.stats.w, self.stats.d, self.stats.l, self.stats.p
        )
    }

    pub fn print_name(&self) -> String {
        self.name.clone()
    }

    pub fn compare(&self, other: &Self) -> std::cmp::Ordering {
        match self.stats.p.cmp(&other.stats.p) {
            std::cmp::Ordering::Equal => self.name.cmp(&other.name).reverse(),
            ordering => ordering,
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let header = String::from("Team                           | MP |  W |  D |  L |  P");
    let mut teams: HashMap<String, Team> = HashMap::new();
    // split by newline and iterate over each line
    match_results
        .split('\n')
        .filter(|val| !val.is_empty())
        .for_each(|line| {
            let parts: Vec<&str> = line.split(';').collect();
            let team1 = parts[0];
            let team2 = parts[1];
            let result = parts[2];
            let inverse_result = match result {
                "win" => "loss",
                "loss" => "win",
                "draw" => "draw",
                _ => panic!("Invalid result"),
            };

            // update/create team1
            let team1 = teams
                .entry(team1.to_string())
                .or_insert(Team::new(team1, None, None, None, None, None));
            team1.update_stats(result);

            // update/create team2
            let team2 = teams
                .entry(team2.to_string())
                .or_insert(Team::new(team2, None, None, None, None, None));
            team2.update_stats(inverse_result);
        });

    let mut teams: Vec<Team> = teams.into_values().collect();
    teams.sort_by(|a, b| b.compare(a));

    let mut output = vec![header];
    teams.iter().for_each(|team| {
        output.push(team.print_results());
    });

    output.join("\n")
}
