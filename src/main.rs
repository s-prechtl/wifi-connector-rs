use wifi_connector::network::Network;
use anyhow::*;
use std::{process::Command, vec};

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
    positions.push(
        header_line
            .find(" SSID")
            .expect("SSID not found in header string") as u8
            + 1,
    );
    positions.push(
        header_line
            .find("SIGNAL")
            .expect("SIGNAL not found in header string") as u8,
    );

    return positions;
}

fn main() {
    let nmcli_output: String = get_available_wifis().expect("Wifi fetching exploded");
    let positions = get_descriptor_positions(&nmcli_output);

    let mut all_wifi_lines: Vec<String> = nmcli_output.split('\n').map(|s| String::from(s)).collect();
    let mut all_networks: Vec<Network> = vec![];
    all_wifi_lines.remove(0);

    for line in all_wifi_lines {
        if line.is_empty() {
            continue;
        }
        all_networks.push(Network::from_nmcli_stdout(line.to_owned(), &positions));
    }

    for network in all_networks {
       dbg!("{}", network);
    }
}
