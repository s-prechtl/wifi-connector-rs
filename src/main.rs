use anyhow::*;
use std::{collections::HashMap, io::Write, process::Command, vec};
use wifi_connector::network::Network;

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

    let mut all_wifi_lines: Vec<String> =
        nmcli_output.split('\n').map(|s| String::from(s)).collect();
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
        .arg(password.trim())
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow!("Failed to connect to network"))
    }
}

fn try_connect_to_network_without_password(network: &Network) -> Result<()> {
    let output = Command::new("nmcli")
        .arg("device")
        .arg("wifi")
        .arg("connect")
        .arg(&network.ssid.trim())
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow!("Couldn't connect to wifi without password"))
    }
}

fn print_networks(networks: &Vec<Network>) {
    for (idx, network) in networks.iter().enumerate() {
        println!("{} - {}", idx, network);
    }
}

fn password_prompt_for_network(network: &Network) -> String {
    let mut password = String::new();
    print!(
        "Now enter the password for the selected wifi '{}': ",
        network.ssid
    );
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut password).unwrap();
    password
}

fn choose_network(max: usize) -> String {
    println!("Which network do you choose? (0-{})", max);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

fn print_header() {
    println!("Index - In use - SSID - Signal");
}

fn uniquify_networks(networks: Vec<Network>) -> Vec<Network> {
    let mut unique_networks: HashMap<String, Network> = HashMap::new();

    for network in &networks {
        let current_ssid = &network.ssid;
        if unique_networks.keys().any(|key| key == current_ssid) {
            if unique_networks[current_ssid].signal < network.signal {
                unique_networks.insert(current_ssid.to_owned(), network.to_owned());
            }
        } else {
            unique_networks.insert(current_ssid.to_owned(), network.to_owned());
        }
    }

    return unique_networks.into_values().collect();
}

fn main() {
    let all_networks = uniquify_networks(get_all_networks());

    print_header();
    print_networks(&all_networks);

    let chosen_network_index = choose_network(all_networks.len() - 1);

    let network = all_networks
        .get(chosen_network_index.trim().parse::<usize>().unwrap())
        .expect("Given number does not reference a network");
    let mut connected = try_connect_to_network_without_password(network);

    if !connected.is_ok() {
        let password = password_prompt_for_network(network);
        connected = connect_to_network(&network, password.trim());
    }

    if connected.is_ok() {
        println!("Successfully connected to network '{}'", network.ssid);
    } else {
        println!("Connection failed");
    }
}
