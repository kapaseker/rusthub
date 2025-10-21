use csv::Reader;
use serde::{Deserialize, Serialize};

// Name,Position,Jersey Number,Nationality,Age
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub name: String,
    pub position: String,
    #[serde(rename = "Jersey Number")]
    pub jersey_number: u8,
    pub nationality: String,
    pub age: u32,
}

pub fn process_csv(input_file: &str, output_file: &str) -> anyhow::Result<()> {
    let mut rdr = Reader::from_path(input_file)?;
    let players = rdr
        .deserialize::<Player>()
        .map(|player| player.unwrap())
        .collect::<Vec<Player>>();
    let player_json = serde_json::to_string_pretty(&players)?;
    std::fs::write(output_file, player_json)?;
    Ok(())
}