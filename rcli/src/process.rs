use csv::Reader;
use serde_json::Value;

// Name,Position,Jersey Number,Nationality,Age

pub fn process_csv(input_file: &str, output_file: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input_file)?;
    let mut record_list = vec![];
    let header = reader.headers()?.clone();
    // println!("{:?}", header);
    for record in reader.records() {
        let rec = record?;
        let zipped = header.iter().zip(rec.iter()).collect::<Value>();
        // println!("{:?}", zipped);
        record_list.push(zipped);
    }
    // let players = rdr
    //     .deserialize::<Player>()
    //     .map(|player| player.unwrap())
    //     .collect::<Vec<Player>>();
    let player_json = serde_json::to_string_pretty(&record_list)?;
    std::fs::write(output_file, player_json)?;
    Ok(())
}