use hex::FromHex;
use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    return contents
        .lines()
        .map(|packet| {
            version_sum({
                let packet_chars: Vec<u8> = packet
                    .chars()
                    .map(|x| <[u8; 1]>::from_hex(format!("0{}", x).to_string()).unwrap()[0])
                    .collect();
                let packet_bin: Vec<String> =
                    packet_chars.iter().map(|x| format!("{:04b}", x)).collect();
                packet_bin.join("")
            })
            .0
        })
        .sum::<i64>();
}

fn part2(contents: String) -> i64 {
    return contents
        .lines()
        .map(|packet| {
            eval_packet({
                let packet_chars: Vec<u8> = packet
                    .chars()
                    .map(|x| <[u8; 1]>::from_hex(format!("0{}", x).to_string()).unwrap()[0])
                    .collect();
                let packet_bin: Vec<String> =
                    packet_chars.iter().map(|x| format!("{:04b}", x)).collect();

                packet_bin.join("")
            })
            .0
        })
        .sum::<i64>();
}

fn version_sum(packet: String) -> (i64, String, bool) {
    if packet.len() < 6 {
        return (0, packet, false);
    }

    let mut version = i64::from_str_radix(&packet[0..3], 2).unwrap();
    let packet_type = i64::from_str_radix(&packet[3..6], 2).unwrap();

    if packet_type == 4 {
        let mut ix = 6;

        while packet.chars().nth(ix).unwrap() == '1' {
            ix += 5;
        }

        ix += 5;

        return (version, packet[ix..].to_string(), true);
    }

    let length_type: i64 = i64::from_str_radix(&packet[6..7], 2).unwrap();
    let mut subpacket: String;

    if length_type == 0 {
        let subpacket_bits = i64::from_str_radix(&packet[7..22], 2).unwrap();
        subpacket = packet[22..22 + subpacket_bits as usize].to_string();

        loop {
            let (subpacket_version_sum, after_subpacket, decode_success) =
                version_sum(subpacket.clone());
            subpacket = after_subpacket;

            if !decode_success {
                break;
            }

            version += subpacket_version_sum;
        }

        subpacket.push_str(&packet[22 + subpacket_bits as usize..]);
    } else {
        let subpacket_nums = i64::from_str_radix(&packet[7..18], 2).unwrap();
        subpacket = packet[18..].to_string();

        for _ in 0..subpacket_nums {
            let (subpacket_version_sum, after_subpacket, _) = version_sum(subpacket.clone());
            subpacket = after_subpacket;
            version += subpacket_version_sum;
        }
    }

    return (version, subpacket, true);
}

fn eval_packet(packet: String) -> (i64, String, bool) {
    if packet.len() < 6 {
        return (0, packet, false);
    }

    let packet_type = i64::from_str_radix(&packet[3..6], 2).unwrap();

    if packet_type == 4 {
        let mut ix: usize = 6;
        let mut val_string = String::new();

        while packet.chars().nth(ix).unwrap() == '1' {
            packet[ix + 1..ix + 5]
                .chars()
                .for_each(|x| val_string.push(x));
            ix += 5;
        }

        packet[ix + 1..ix + 5]
            .chars()
            .for_each(|x| val_string.push(x));
        ix += 5;

        let packet_val = i64::from_str_radix(&val_string, 2).unwrap();
        return (packet_val, packet[ix..].to_string(), true);
    }

    let length_type: i64 = i64::from_str_radix(&packet[6..7], 2).unwrap();

    let mut subpacket_vals: Vec<i64> = Vec::new();
    let mut subpacket: String;

    if length_type == 0 {
        let subpacket_bits = i64::from_str_radix(&packet[7..22], 2).unwrap();
        subpacket = packet[22..22 + subpacket_bits as usize].to_string();

        loop {
            let (subpacket_eval, after_subpacket, decode_success) = eval_packet(subpacket.clone());
            subpacket = after_subpacket;

            if !decode_success {
                break;
            }

            subpacket_vals.push(subpacket_eval);
        }

        subpacket.push_str(&packet[22 + subpacket_bits as usize..]);
    } else {
        let subpacket_nums = i64::from_str_radix(&packet[7..18], 2).unwrap();
        subpacket = packet[18..].to_string();

        for _ in 0..subpacket_nums {
            let (subpacket_eval, after_subpacket, _) = eval_packet(subpacket.clone());
            subpacket = after_subpacket;
            subpacket_vals.push(subpacket_eval);
        }
    }

    let packet_val: i64 = match packet_type {
        0 => subpacket_vals.iter().sum(),
        1 => subpacket_vals.iter().product(),
        2 => subpacket_vals.iter().min().unwrap().clone(),
        3 => subpacket_vals.iter().max().unwrap().clone(),
        5 => (subpacket_vals[0] > subpacket_vals[1]) as i64,
        6 => (subpacket_vals[0] < subpacket_vals[1]) as i64,
        7 => (subpacket_vals[0] == subpacket_vals[1]) as i64,
        _ => panic!("Invalid packet type"),
    };

    return (packet_val, subpacket, true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example_p1.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 88);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example_p2.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 2096);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "16".to_string();
	
	let root = env::current_dir().unwrap();
	let path_str = if args.len() > 1 {
	    args[1].clone()
    } else if root.ends_with(format!("{}", day)) {
	    format!("../../../Inputs/{}_{}.txt", year, day)
	} else {
	    format!("/Inputs/{}_{}.txt", year, day)
	};
    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nSum of version IDs: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nPacket value: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}