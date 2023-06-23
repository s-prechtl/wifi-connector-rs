use wifi_connector::network::Network;
use anyhow::*;
use std::{process::Command, vec, io::Write};

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

fn get_all_networks() -> Vec<Network> {
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

    return all_networks;
}

fn connect_to_network(network: &Network, password: &str) -> Result<()> {
    let output = Command::new("nmcli")
        .arg("device")
        .arg("wifi")
        .arg("connect")
        .arg(&network.ssid.trim())
        .arg("password")
        .arg(password)
        .output()
        .expect("Failed to execute command");

    println!("{:?}", password);

    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow!("Failed to connect to network"))
    }
}

fn main() {
    let all_networks = get_all_networks();

    for (idx, network) in all_networks.iter().enumerate() {
       println!("{} - {}", idx, network);
    }

    println!("Which network do you choose? (0-{})", all_networks.len() - 1);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let network = all_networks.get(input.trim().parse::<usize>().unwrap()).expect("Given number does not reference a network");
    let mut password = String::new();
    print!("Now enter the password for the selected wifi '{}': ", network.ssid);
    std::io::stdout().flush();
    std::io::stdin().read_line(&mut password).unwrap();
    let result = connect_to_network(&network, password.trim());
    if  result.is_ok() {
        println!("Successfully connected to network '{}'", network.ssid);
    } else {
        println!("Connection failed");
    }
}
