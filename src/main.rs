use std::{process::Command, vec};
use anyhow::*;

struct Network {
    in_use: bool,
    ssid: String,
    signal: u8,
}

impl Network {
    fn from_nmcli_stdout(line: String) {
        for element in line.split(' ') {
            println!("{}", element);
        }
    }
}

fn get_available_wifis() -> Result<String> {
    let output = Command::new("nmcli")
        .arg("device")
        .arg("wifi")
        .arg("list")
        .output()
        .expect("Failed to execute command");

   Ok(String::from_utf8_lossy(output.stdout.as_slice()).into())
}

fn get_descriptor_positions(header_line: &String) -> Vec<u8> {
    let mut positions: Vec<u8> = vec![];

    positions.push(0);
        positions.push(header_line.find(" SSID").expect("SSID not found in header string") as u8 +1);
        positions.push(header_line.find("SIGNAL").expect("SIGNAL not found in header string") as u8);

    return positions;
}

fn main() {
    let all_wifis: String = get_available_wifis().expect("Wifi fetching exploded");
    let positions = get_descriptor_positions(&all_wifis);
    for position in (positions) {
        println!("{}", position);
    }
    // for line in all_wifis.split("\n") {
    //     Network::from_nmcli_stdout(line.to_owned());
    // }
}
