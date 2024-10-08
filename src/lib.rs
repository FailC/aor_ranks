use std::collections::HashMap;
use std::fs::{self, DirBuilder, File};
use std::io::{self, Write};
use std::num::NonZeroUsize;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};

mod game;
use game::locations;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Stage {
    name: String,
    pub time: u32,
    car: u8,
    player_name: String,
    location: String,
    stage_number: usize,
    stage_name: String,
    direction: String,
    weather: String,
    group: String,
}

#[derive(Debug)]
pub struct Player {
    name: String,
    score: u32,
    pub rankings: HashMap<String, u64>,
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
    pub fn get_average_score(&self) -> u64 {
        let mut score: u64 = 0;
        let mut len = 0;
        for (_name, stage_score) in &self.rankings {
            score += stage_score;
            len += 1;
        }
        if len == 0 {
            return 0;
        }
        score / len
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
            let location = location_parts[0].to_string();
            let stage_number: NonZeroUsize = location_parts[2].parse().ok()?;
            let group = location_parts[5].to_string();
            // most optimized Rust code ever
            let locations = &game::locations::LOCATIONS;
            let stage_name = match locations::get_name(&locations, &location, stage_number.into()) {
                Some(name) => name.to_string(),
                None => return None,
            };

            let direction = location_parts[3].to_string();
            let weather = location_parts[4].to_string();

            Some(Stage {
                name,
                time,
                car,
                player_name: player_name.to_string(),
                location,
                stage_number: stage_number.into(),
                stage_name,
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
                .unwrap_or("Unknown");

            if let Ok(stages) = read_stages_from_file(&path, player_name) {
                players.push(Player::new(player_name.to_string(), stages))
            }
        }
    }
    Ok(players)
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
    for stage in every_stage.values_mut() {
        stage.sort_by(|a, b| a.time.cmp(&b.time));
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
                }
            }
        }
    }
    ranked_stages
}

pub fn create_single_leaderboards(single_leaderboards: &HashMap<String, Vec<String>>) {
    for (k, v) in single_leaderboards.iter() {
        let mut text: Vec<String> = Vec::new();
        text.push(format!("{}", k));
        for y in v {
            text.push(format!("{}", y));
        }
        create_file("./Leaderboards/all_stages", text, k).unwrap();
    }
}

pub fn get_leaderboard(players: &mut Vec<Player>) -> Vec<String> {
    // Sort players by score in descending order
    players.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut leaderboard = Vec::new();
    for (rank, player) in players.iter().enumerate() {
        let s = format!(
            "{}: {} \t{:.2} points\t({})",
            rank + 1,
            player.name,
            player.score,
            player.get_average_score()
        );
        leaderboard.push(s);
    }
    leaderboard
}

// shitty code to test stuff
// create one leaderboard for every group
pub fn create_group_leaderboards(players: &Vec<Player>) {
    let mut groups: HashMap<&str, HashMap<&str, u64>> = HashMap::new();

    for player in players {
        for (stage_name, points) in &player.rankings {
            let parts: Vec<&str> = stage_name.split("_").collect();
            let group = parts[5];
            let country_group = groups.entry(group).or_insert(HashMap::new());
            country_group
                .entry(&player.name)
                .and_modify(|score| *score += *points)
                .or_insert(*points);
        }
    }
    // sort player score in each group
    for (group, players) in groups {
        let mut file_content = Vec::new();
        let file_name: &str = group;

        let mut sorted_vec: Vec<(&str, u64)> = players.iter().map(|(&k, &v)| (k, v)).collect();
        sorted_vec.sort_by(|a, b| b.1.cmp(&a.1));

        for x in sorted_vec {
            file_content.push(format!("{}: {}", x.0, x.1));
        }
        create_file("./Leaderboards/groups", file_content, file_name)
            .expect("failed to create file in create_group_leaderboards");
    }
}

pub fn create_folder(path: &str) {
    //let path = "./Leaderboards";
    match DirBuilder::new().create(path) {
        Ok(()) => println!("creating directory: {}", path),
        Err(_) => println!("directory exists: {}", path),
    };
}
// Counter for some debugging i guess
pub static COUNTER: AtomicUsize = AtomicUsize::new(0);
pub fn create_file<T: AsRef<str>>(
    dir_path: &str,
    text: Vec<T>,
    file_name: &str,
) -> std::io::Result<()> {
    //let dir_path = "./Leaderboards";
    let file_path = Path::new(dir_path).join(file_name);
    let mut file = File::create(&file_path)?;

    for x in text {
        file.write_all(x.as_ref().as_bytes())?;
        file.write_all("\n".as_bytes())?;
    }
    COUNTER.fetch_add(1, Ordering::SeqCst);
    //println!("created: {}", file_path.display());
    Ok(())
}
