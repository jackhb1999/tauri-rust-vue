use std::fs;
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    let headers = reader.headers()?.clone();
    for reader in reader.records() {
        let record = reader?;
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<Value>();
        ret.push(json_value);
    }
    let json = serde_json::to_string(&ret)?;
    fs::write(output, json)?;
    Ok(())
}