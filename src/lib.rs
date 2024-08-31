use std::collections::HashMap;
use std::fs::{self};
use std::io;
use std::path::Path;

// Todo: better errors, better variable names
// add average map score
// add car names, stage names (for single leaderboards)

#[derive(Debug, Clone)]
pub struct Stage {
    pub name: String,
    pub time: u32,
    _car: u8,
    pub player_name: String,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub rankings: HashMap<String, u64>,
    pub stages: HashMap<String, Stage>,
}

impl Player {
    pub fn new(name: String, stages: HashMap<String, Stage>) -> Player {
        Player {
            name,
            score: 0,
            rankings: HashMap::new(),
            stages,
        }
    }
}
impl Stage {
    pub fn from_line(line: &str, player_name: &str) -> Option<Self> {
        let parts: Vec<&str> = line.split(':').collect();
        // check for daily, weekly events / for now filter out bonus cars / include Australia DLC?
        if parts.len() == 3
            && !parts[0].contains("Custom")
            && !parts[0].contains("Bonus")
            && !(parts[0].contains("daily") || parts[0].contains("weekly"))
        {
            let name = parts[0].to_string();
            let time = parts[1].parse().ok()?;
            let _car = parts[2].parse().ok()?;
            Some(Stage {
                name,
                time,
                _car,
                player_name: player_name.to_string(),
            })
        } else {
            None
        }
    }
}

pub fn read_stages_from_file(path: &Path, player_name: &str) -> io::Result<HashMap<String, Stage>> {
    let mut stages = HashMap::new();
    let content = fs::read_to_string(path)?;
    for line in content.lines() {
        if let Some(stage) = Stage::from_line(line, player_name) {
            stages.insert(stage.name.clone(), stage);
        }
    }
    Ok(stages)
}

pub fn load_users_from_dir(dir: &Path) -> io::Result<Vec<Player>> {
    let mut players: Vec<Player> = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|ext| ext.to_str()) == Some("txt") {
            let file_name = path
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("Unknown")
                .to_string();

            if let Ok(stages) = read_stages_from_file(&path, &file_name) {
                players.push(Player::new(file_name, stages))
            }
        }
    }
    Ok(players)
}

// best rust code ever
pub fn load_all_stages() -> Vec<String> {
    let contents = fs::read_to_string("Leaderboards.txt").unwrap();
    let lines: Vec<String> = contents.lines().map(String::from).collect();
    let mut stage_names: Vec<String> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        stage_names.push(parts[0].to_owned());
    }
    stage_names
}

pub fn collect_stages_from_players(players: &[Player]) -> HashMap<String, Vec<Stage>> {
    let mut every_stage: HashMap<String, Vec<Stage>> = HashMap::new();

    for player in players {
        for stage in player.stages.values() {
            every_stage
                .entry(stage.name.clone())
                .or_insert_with(Vec::new)
                .push(stage.clone());
        }
    }
    every_stage
}

pub fn rank_stages(every_stage: &mut HashMap<String, Vec<Stage>>, players: &mut Vec<Player>) {
    for (_stage_name, stages) in every_stage.iter_mut() {
        // Sort the stages by time (ascending)
        // how to manage the same time?
        stages.sort_by_key(|stage| stage.time);

        if let Some(fastest_stage) = stages.first() {
            let fastest_time = fastest_stage.time as f64;

            // update player scores based on their ranking
            for (pos, stage) in stages.iter().enumerate() {
                let rank: i32 = pos as i32 + 1;
                let player_time = stage.time as f64;

                // scoring algorithm
                let score =
                    1000.0 * (fastest_time / player_time) * 0.988f64.powf(rank as f64 - 1.0);

                // Find the corresponding player and update their score
                if let Some(player) = players.iter_mut().find(|p| p.name == stage.player_name) {
                    player.score += score as u32;
                    player
                        .rankings
                        .entry(_stage_name.clone())
                        .or_insert(score as u64);
                }
            }
        }
    }
}

pub fn build_leaderboard(players: &mut Vec<Player>) {
    // Sort players by score in descending order
    players.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    println!("\nFinal Leaderboard:");
    for (rank, player) in players.iter().enumerate() {
        println!("{}: {} \t{:.2} points", rank + 1, player.name, player.score);
    }
}

pub fn convert_ms_to_string(ms: u32) -> String {
    let total_seconds = ms / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    let milliseconds = ms % 1000;

    let string = format!("{minutes:02}:{seconds:02}:{milliseconds:03}");
    string
}
