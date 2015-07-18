use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

#[derive(Debug)]
pub struct GameConfig {
    loadtime: i64,
    turntime: i64,
    rows: i64,
    cols: i64,
    turns: i64,
    viewradius2: i64,
    attackradius2: i64,
    spawnradius2: i64,
    player_seed : i64
}

pub fn read_config<R: Read>(reader: &mut BufReader<R>) -> GameConfig {
    let fallback_value: i64 = 0;
    let config_params = read_lines_until_go(reader);

    GameConfig {
        loadtime: config_params.get("loadtime").unwrap_or(&fallback_value).clone(),
        turntime: config_params.get("turntime").unwrap_or(&fallback_value).clone(),
        rows: config_params.get("rows").unwrap_or(&fallback_value).clone(),
        cols: config_params.get("cols").unwrap_or(&fallback_value).clone(),
        turns: config_params.get("turns").unwrap_or(&fallback_value).clone(),
        viewradius2: config_params.get("viewradius2").unwrap_or(&fallback_value).clone(),
        attackradius2: config_params.get("attackradius2").unwrap_or(&fallback_value).clone(),
        spawnradius2: config_params.get("spawnradius2").unwrap_or(&fallback_value).clone(),
        player_seed : config_params.get("player_seed").unwrap_or(&fallback_value).clone()
    }
}

fn parse_string_int_pair(line: String) -> (String, i64) {
    let mut pair = line.trim().split(" ");
    let key = pair.next().unwrap();
    let value = pair.next().unwrap();
    (key.to_string(), value.parse::<i64>().unwrap())
}

fn read_lines_until_go<R: Read>(reader: &mut BufReader<R>) ->  HashMap<String,i64> {
    let mut config_params: HashMap<String,i64> = HashMap::new();

    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        line = line.trim().to_string();
        if line.is_empty() {
            continue;
        } else if line.eq("ready") {
            break;
        } else {
            let (key, value) = parse_string_int_pair(line);
            config_params.insert(key,value);
        }
    }
    config_params
}

#[cfg(test)]
mod test;

