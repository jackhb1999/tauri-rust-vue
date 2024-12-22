use std::fs;
use csv::Reader;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input:&str,output:&str)->anyhow::Result<()>{
    // let records = reader
    //     .deserialize()
    //     .map(|record| record.unwrap())
    //     .collect::<Vec<Player>>();
    // println!("{:?}", records);
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for reader in reader.deserialize() {
        let record: Player = reader?;
        ret.push(record);
    }
    let json = serde_json::to_string(&ret)?;
    fs::write(output, json)?;
    Ok(())
}