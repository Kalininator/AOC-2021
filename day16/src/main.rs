use num_bigint::BigUint;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = utils::read_file(&args[1]);
    let mut binary = parse_hex(&lines[0]);

    let packets = parse_transmission(&mut binary);
    let version_sum: u128 = packets.iter().map(|p| p.version).sum();
    println!("Packets: {:?}", packets);
    println!("Part 1: {}", version_sum);
    println!("Outermost value: {}", packets[0].value);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Packet {
    version: u128,
    type_id: u128,
    value: u128,
}

fn parse_transmission(bools: &mut Vec<bool>) -> Vec<Packet> {
    let mut packets: Vec<Packet> = vec![];

    let version = binary_to_decimal(&bools.drain(0..3).collect());
    let type_id = binary_to_decimal(&bools.drain(0..3).collect());

    if type_id == 4 {
        println!("Literal packet");
        // Literal value
        let mut value_bools: Vec<bool> = vec![];
        while bools.len() >= 5 {
            let group_bools: Vec<bool> = bools.drain(0..5).collect();
            let is_last = !group_bools[0];
            for b in group_bools.iter().skip(1) {
                value_bools.push(*b);
            }
            if is_last {
                break;
            }
        }
        let padding_length = (value_bools.len() * 5) % 4;
        bools.drain(0..padding_length);
        let value = binary_to_decimal(&value_bools);
        packets.push(Packet {
            version,
            type_id,
            value,
        });
    } else {
        println!("Operator packet");
        let length_type_id = bools.drain(0..1).next().unwrap();
        let mut subpackets: Vec<Packet> = vec![];
        let mut subpacket_values: Vec<u128> = vec![];
        match length_type_id {
            false => {
                let mut total_length_in_bits =
                    binary_to_decimal(&bools.drain(0..15).collect()) as usize;
                println!("Total length in bits: {}", total_length_in_bits);
                while total_length_in_bits > 0 {
                    let len_before = bools.len();
                    let mut new_packets = parse_transmission(bools);
                    subpacket_values.push(new_packets[0].value);
                    subpackets.append(&mut new_packets);
                    total_length_in_bits -= len_before - bools.len();
                }
            }
            true => {
                let number_subpackets = binary_to_decimal(&bools.drain(0..11).collect());
                println!("Number of subpackets: {}", number_subpackets);
                for _ in 0..number_subpackets {
                    let mut new_packets = parse_transmission(bools);
                    subpacket_values.push(new_packets[0].value);
                    subpackets.append(&mut new_packets);
                }
            }
        }

        println!("subpacket values: {:?}", subpacket_values);
        println!("subpackets: {:?}", subpackets);
        let value: u128 = match type_id {
            0 => subpacket_values.iter().sum(),
            1 => subpacket_values.iter().product(),
            2 => *subpacket_values.iter().min().unwrap(),
            3 => *subpacket_values.iter().max().unwrap(),
            5 => {
                if subpacket_values[0] > subpacket_values[1] {
                    1
                } else {
                    0
                }
            }
            6 => {
                if subpacket_values[0] < subpacket_values[1] {
                    1
                } else {
                    0
                }
            }
            7 => {
                if subpacket_values[0] == subpacket_values[1] {
                    1
                } else {
                    0
                }
            }
            _ => panic!("Invalid type_id {}", type_id),
        };

        packets.push(Packet {
            version,
            type_id,
            value,
        });
        packets.append(&mut subpackets);
    }

    packets
}

fn binary_to_decimal(bools: &Vec<bool>) -> u128 {
    let mut acc: u128 = 0;
    for i in bools {
        match i {
            true => acc = (acc * 2) + 1,
            false => acc *= 2,
        }
    }
    acc
}

fn parse_hex(line: &str) -> Vec<bool> {
    let mut acc: Vec<bool> = vec![];
    for c in line.chars() {
        acc.append(&mut to_binary(c).chars().map(|c| c == '1').collect());
    }
    acc
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => panic!("Invalid hex character {}", c),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_value_packet() {
        let input = "D2FE28";
        let packets = parse_transmission(&mut parse_hex(input));
        assert_eq!(
            packets,
            vec![Packet {
                version: 6,
                type_id: 4,
                value: 2021
            }]
        );
    }

    #[test]
    fn number_subpackets_operator() {
        let input = "EE00D40C823060";
        let packets = parse_transmission(&mut parse_hex(input));
        assert_eq!(
            packets,
            vec![
                Packet {
                    version: 7,
                    type_id: 3,
                    value: 3
                },
                Packet {
                    version: 2,
                    type_id: 4,
                    value: 1
                },
                Packet {
                    version: 4,
                    type_id: 4,
                    value: 2
                },
                Packet {
                    version: 1,
                    type_id: 4,
                    value: 3
                },
            ]
        );
    }

    #[test]
    fn subpacket_length_operator() {
        let input = "38006F45291200";
        let packets = parse_transmission(&mut parse_hex(input));
        assert_eq!(
            packets,
            vec![
                Packet {
                    version: 1,
                    type_id: 6,
                    value: 1
                },
                Packet {
                    version: 6,
                    type_id: 4,
                    value: 10
                },
                Packet {
                    version: 2,
                    type_id: 4,
                    value: 20
                },
            ]
        );
    }

    #[test]
    fn nested_operators() {
        let input = "8A004A801A8002F478";
        let mut binary = parse_hex(input);
        println!("Binary: {:?}", binary);
        let packets = parse_transmission(&mut binary);
        assert_eq!(
            packets,
            vec![
                Packet {
                    version: 4,
                    type_id: 2,
                    value: 15
                },
                Packet {
                    version: 1,
                    type_id: 2,
                    value: 15
                },
                Packet {
                    version: 5,
                    type_id: 2,
                    value: 15
                },
                Packet {
                    version: 6,
                    type_id: 4,
                    value: 15
                },
            ]
        );
    }

    #[test]
    fn product() {
        let input = "04005AC33890";
        let mut binary = parse_hex(input);
        let packets = parse_transmission(&mut binary);
        assert_eq!(packets[0].value, 54);
    }

    #[test]
    fn product_2() {
        let input = "9C0141080250320F1802104A08";
        let mut binary = parse_hex(input);
        let packets = parse_transmission(&mut binary);
        assert_eq!(packets[0].value, 1);
    }
}
