use std::fmt::Display;

#[derive(Debug)]
pub struct Network {
    pub in_use: bool,
    pub ssid: String,
    pub signal: u8,
}

impl Network {
    pub fn from_nmcli_stdout(line: String, header_positions: &Vec<u8>) -> Self {
        let mut network = Self {
            in_use: false,
            ssid: "".to_string(),
            signal: 0,
        };

        let ssid_pos = header_positions
            .get(1)
            .expect("header_positions has no index for SSID")
            .to_owned() as usize;
        let signal_pos = header_positions
            .get(2)
            .expect("header_positions has no index for SSID")
            .to_owned() as usize;

        if line
            .chars()
            .nth(
                header_positions
                    .get(0)
                    .expect("header_positions has no index 0")
                    .to_owned() as usize,
            )
            .expect("Given line has no index 0")
            == '*'
        {
            network.in_use = true;
        }

        let mut current_spaces = 0;
        for char in line.chars().skip(ssid_pos) {
            if char == ' ' {
                current_spaces += 1;
                if current_spaces == 2 {
                    break;
                }
            } else {
                if current_spaces == 1 {
                    network.ssid.push(' ');
                }
                current_spaces = 0;
                network.ssid.push(char);
            }
        }

        let mut signal_as_string: String = line
            .chars()
            .nth(signal_pos)
            .expect("Line has no Signal pos")
            .to_string();

        signal_as_string.push(
            line.chars()
                .nth(signal_pos + 1)
                .expect("Line has  no Signal pos"),
        );

        network.signal = signal_as_string
            .parse::<u8>()
            .expect("Could not parse signal");

        return network;
    }
}

impl Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{} - {} - {} ", self.in_use, self.ssid, self.signal);
    }
}
