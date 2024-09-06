use std::collections::HashMap;
use std::fs::{self, DirBuilder, File};
use std::io::{self, Write};
use std::path::Path;

mod game;
use game::locations::*;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Stage {
    name: String,
    pub time: u32,
    car: u8,
    player_name: String,
    location: String,
    stage_number: u8,
    stage_name: String,
    direction: String,
    weather: String,
    group: String,
}

#[derive(Debug, Clone)]
pub struct Player {
    name: String,
    score: u32,
    rankings: HashMap<String, u64>,
    stages: HashMap<String, Stage>,
}

impl Player {
    fn new(name: String, stages: HashMap<String, Stage>) -> Self {
        Player {
            name,
            score: 0,
            rankings: HashMap::new(),
            stages,
        }
    }
    pub fn get_average_score(&self) -> String {
        let mut score: u64 = 0;
        let mut len = 0;
        for (_h, v) in &self.rankings {
            score += v;
            len += 1;
        }
        format!("average score {}: {score}", self.name, score = score / len)
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
            let time = parts[1].parse().ok()?;
            // return early because of DNF time
            if time >= 356400000 {
                return None;
            }
            let name = parts[0].to_string();
            let car = parts[2].parse().ok()?;
            let location_parts: Vec<&str> = parts[0].split("_").collect();
            let location = location_parts[0];
            let stage_number: u8 = location_parts[2].parse().ok()?;
            let group = location_parts[5].to_string();
            let location_enum = Countries::from_str(location)?;
            let stage_name = match location_enum {
                Countries::Finland => Finland::from_number(stage_number),
                Countries::Sardinia => Sardinia::from_number(stage_number),
                Countries::Japan => Japan::from_number(stage_number),
                Countries::Norway => Norway::from_number(stage_number),
                Countries::Germany => Germany::from_number(stage_number),
                Countries::Kenya => Kenya::from_number(stage_number),
                Countries::Indonesia => Indonesia::from_number(stage_number),
                Countries::Australia => Australia::from_number(stage_number),
            }?;
            let direction = location_parts[3].to_string();
            let weather = location_parts[4].to_string();

            Some(Stage {
                name,
                time,
                car,
                player_name: player_name.to_string(),
                location: location.to_string(),
                stage_number,
                stage_name: stage_name.to_string(),
                direction,
                weather,
                group,
            })
        } else {
            None
        }
    }
    pub fn time_to_string(&self) -> String {
        let total_seconds = self.time / 1000;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        let milliseconds = self.time % 1000;
        return format!("{minutes:02}:{seconds:02}:{milliseconds:03}");
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
            // player_name
            let player_name = path
                .file_stem()
                .and_then(|name| name.to_str())
                .unwrap_or("Unknown")
                .to_string();

            if let Ok(stages) = read_stages_from_file(&path, &player_name) {
                players.push(Player::new(player_name, stages))
            }
        }
    }
    Ok(players)
}

// best rust code ever
pub fn _load_all_stages() -> Vec<String> {
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

pub fn get_ranked_stages(
    every_stage: &HashMap<String, Vec<Stage>>,
    players: &mut Vec<Player>,
) -> HashMap<String, Vec<String>> {
    let mut ranked_stages: HashMap<String, Vec<String>> = HashMap::new();

    for (stage_name, stages) in every_stage.iter() {
        if let Some(fastest_stage) = stages.first() {
            let fastest_time = fastest_stage.time as f64;
            // update player scores
            for (pos, stage) in stages.iter().enumerate() {
                let rank: i32 = pos as i32 + 1;
                let player_time = stage.time as f64;
                // scoring algorithm
                let score =
                    1000.0 * (fastest_time / player_time) * 0.988f64.powf(rank as f64 - 1.0);
                // Find player and update score
                if let Some(player) = players.iter_mut().find(|p| p.name == stage.player_name) {
                    player.score += score as u32;
                    player
                        .rankings
                        .entry(stage_name.clone())
                        .or_insert(score as u64);

                    let player_value = format!(
                        "{} {} {} {}",
                        player.name,
                        stage.time_to_string(),
                        score as i64,
                        stage.car
                    );
                    let stage_key = format!(
                        "{}: {} {} {} {}",
                        stage.location,
                        stage.stage_name,
                        stage.group,
                        stage.direction,
                        stage.weather
                    );
                    ranked_stages
                        .entry(stage_key)
                        .or_insert(Vec::new())
                        .push(player_value);
                    // file for every single stage here? -> extra function
                }
            }
        }
    }
    ranked_stages
}

pub fn create_single_leaderboards(single_leaderboards: HashMap<String, Vec<String>>) {
    for (k, v) in single_leaderboards.iter() {
        let mut text: Vec<String> = Vec::new();
        text.push(format!("{}", k));
        for y in v {
            text.push(format!("{}", y));
        }
        let _ = create_file("./Leaderboards/all_stages", text.clone(), k).unwrap();
    }
}

pub fn get_leaderboard(players: &mut Vec<Player>) -> Vec<String> {
    // Sort players by score in descending order
    players.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut string: Vec<String> = Vec::new();
    for (rank, player) in players.iter().enumerate() {
        let s = format!("{}: {} \t{:.2} points", rank + 1, player.name, player.score);
        string.push(s);
    }
    string
}

pub fn create_folder(path: &str) {
    //let path = "./Leaderboards";
    match DirBuilder::new().create(path) {
        Ok(()) => println!("creating directory.."),
        Err(_) => println!("directory exists"),
    };
}

// create files for each leaderboard?
pub fn create_file(dir_path: &str, text: Vec<String>, file_name: &str) -> std::io::Result<()> {
    //let dir_path = "./Leaderboards";
    let file_path = Path::new(dir_path).join(file_name);
    let mut file = File::create(&file_path)?;

    for x in text {
        file.write_all(x.as_bytes())?;
        file.write_all("\n".as_bytes())?;
    }
    println!("created: {}", file_path.display());
    Ok(())
}
