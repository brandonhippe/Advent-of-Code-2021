use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut polymer: HashMap<String, i64> = HashMap::new();
    let mut letters: HashMap<char, i64> = HashMap::new();

    for i in 1..contents.lines().nth(0).unwrap().len() {
        let pair: String = contents
            .lines()
            .nth(0)
            .unwrap()
            .get(i - 1..i + 1)
            .unwrap()
            .to_string();
        let num: &mut i64 = polymer.entry(pair).or_insert(0);
        *num += 1;
    }

    for c in contents.lines().nth(0).unwrap().chars() {
        let num: &mut i64 = letters.entry(c).or_insert(0);
        *num += 1;
    }

    let mut mapping: HashMap<String, String> = HashMap::new();
    for line in contents.lines().skip(2) {
        let key: String = line.get(0..=1).unwrap().to_string();
        let value: String = line.chars().nth(6).unwrap().to_string();
        mapping.insert(key, value);
    }

    for _ in 0..10 {

        let mut new_polymer: HashMap<String, i64> = HashMap::new();
        for (key, value) in &polymer {
            if mapping.contains_key(key) {
                let new_key: String = [
                    key.chars().nth(0).unwrap().to_string(),
                    mapping.get(key).unwrap().to_string(),
                ]
                .join("");
                let num: &mut i64 = new_polymer.entry(new_key).or_insert(0);
                *num += value;

                let new_key: String = [
                    mapping.get(key).unwrap().to_string(),
                    key.chars().nth(1).unwrap().to_string(),
                ]
                .join("");
                let num: &mut i64 = new_polymer.entry(new_key).or_insert(0);
                *num += value;

                let num: &mut i64 = letters
                    .entry(mapping.get(key).unwrap().chars().nth(0).unwrap())
                    .or_insert(0);
                *num += value;
            }
        }

        polymer = new_polymer;
    }

    return *letters.iter().max_by_key(|&(_, v)| v).unwrap().1
        - *letters.iter().min_by_key(|&(_, v)| v).unwrap().1;
}

fn part2(contents: String) -> i64 {
    let mut polymer: HashMap<String, i64> = HashMap::new();
    let mut letters: HashMap<char, i64> = HashMap::new();

    for i in 1..contents.lines().nth(0).unwrap().len() {
        let pair: String = contents
            .lines()
            .nth(0)
            .unwrap()
            .get(i - 1..i + 1)
            .unwrap()
            .to_string();
        let num: &mut i64 = polymer.entry(pair).or_insert(0);
        *num += 1;
    }

    for c in contents.lines().nth(0).unwrap().chars() {
        let num: &mut i64 = letters.entry(c).or_insert(0);
        *num += 1;
    }

    let mut mapping: HashMap<String, String> = HashMap::new();
    for line in contents.lines().skip(2) {
        let key: String = line.get(0..=1).unwrap().to_string();
        let value: String = line.chars().nth(6).unwrap().to_string();
        mapping.insert(key, value);
    }

    for _ in 0..40 {
        let mut new_polymer: HashMap<String, i64> = HashMap::new();
        for (key, value) in &polymer {
            if mapping.contains_key(key) {
                let new_key: String = [
                    key.chars().nth(0).unwrap().to_string(),
                    mapping.get(key).unwrap().to_string(),
                ]
                .join("");
                let num: &mut i64 = new_polymer.entry(new_key).or_insert(0);
                *num += value;

                let new_key: String = [
                    mapping.get(key).unwrap().to_string(),
                    key.chars().nth(1).unwrap().to_string(),
                ]
                .join("");
                let num: &mut i64 = new_polymer.entry(new_key).or_insert(0);
                *num += value;

                let num: &mut i64 = letters
                    .entry(mapping.get(key).unwrap().chars().nth(0).unwrap())
                    .or_insert(0);
                *num += value;
            }
        }

        polymer = new_polymer;
    }

    return *letters.iter().max_by_key(|&(_, v)| v).unwrap().1
        - *letters.iter().min_by_key(|&(_, v)| v).unwrap().1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 1588);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 2188189693529);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "14".to_string();
	
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
        "\nPart 1:\nMost common - least common: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMost common - least common: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}