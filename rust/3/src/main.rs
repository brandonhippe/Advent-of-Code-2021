use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let mut counts: Vec<i32> = Vec::new();

    for line in contents.lines() {
        if counts.len() == 0 {
            counts = vec![0; line.len()];
        }

        for (i, c) in line.chars().enumerate() {
            match c {
                '0' => counts[i] -= 1,
                '1' => counts[i] += 1,
                _ => println!("Invalid char"),
            }
        }
    }

    let gamma = counts
        .iter()
        .enumerate()
        .map(|(i, c)| if *c > 0 { 1 } else { 0 } * 2_i32.pow((counts.len() - i - 1) as u32))
        .sum::<i32>();
    let epsilon = counts
        .iter()
        .enumerate()
        .map(|(i, c)| if *c < 0 { 1 } else { 0 } * 2_i32.pow((counts.len() - i - 1) as u32))
        .sum::<i32>();

    return gamma * epsilon;
}


fn part2(contents: String) -> i32 {
    let mut oxy_strings: Vec<String> = Vec::from_iter(contents.lines().map(|l| l.to_string()));
    let mut co2_strings: Vec<String> = Vec::from_iter(contents.lines().map(|l| l.to_string()));

    for i in 0..contents.lines().next().unwrap().len() {
        if oxy_strings.len() != 1 {
            let oxy_char: char = if oxy_strings
                .iter()
                .map(|l| {
                    l.chars()
                        .nth(i)
                        .expect("No character to convert")
                        .to_string()
                        .parse::<i32>()
                        .unwrap()
                        * 2
                        - 1
                })
                .sum::<i32>()
                >= 0
            {
                '1'
            } else {
                '0'
            };

            for ix in (0..oxy_strings.len()).rev() {
                if oxy_strings[ix]
                    .chars()
                    .nth(i)
                    .expect("No character to convert")
                    != oxy_char
                {
                    oxy_strings.remove(ix);
                }
            }
        }

        if co2_strings.len() != 1 {
            let co2_char: char = if co2_strings
                .iter()
                .map(|l| {
                    l.chars()
                        .nth(i)
                        .expect("No character to convert")
                        .to_string()
                        .parse::<i32>()
                        .unwrap()
                        * 2
                        - 1
                })
                .sum::<i32>()
                >= 0
            {
                '0'
            } else {
                '1'
            };

            for ix in (0..co2_strings.len()).rev() {
                if co2_strings[ix]
                    .chars()
                    .nth(i)
                    .expect("No character to convert")
                    != co2_char
                {
                    co2_strings.remove(ix);
                }
            }
        }
    }

    return oxy_strings[0]
            .chars()
            .enumerate()
            .map(|(i, c)| {
                c.to_string().parse::<i32>().unwrap() * 2_i32.pow((oxy_strings[0].len() - i - 1) as u32)
            })
            .sum::<i32>()
        * co2_strings[0]
            .chars()
            .enumerate()
            .map(|(i, c)| {
                c.to_string().parse::<i32>().unwrap()
                    * 2_i32.pow((co2_strings[0].len() - i - 1) as u32)
            })
            .sum::<i32>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 198);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 230);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "3".to_string();
	
	let root = env::current_dir().unwrap();
	let path_str = if args.len() > 1 {
	    args[1].clone()
	} else if root.ends_with(format!("rust_{}_{}", year, day)) {
	    format!("../../../Inputs/{}_{}.txt", year, day)
	} else {
	    format!("/Inputs/{}_{}.txt", year, day)
	};

    let contents = fs::read_to_string(if args.len() > 1 {path_str} else {RelativePath::new(&path_str).to_path(&root).display().to_string()})
        .expect("Should have been able to read the file");

    let part1_timer = Instant::now();
    println!(
        "\nPart 1:\nPower consumption: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLife support rating: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}