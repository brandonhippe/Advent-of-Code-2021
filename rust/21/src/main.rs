use cached::proc_macro::cached;
use itertools::Itertools;
use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let (mut p1, mut p2): (i64, i64) = contents
        .lines()
        .map(|line| line.split(": ").nth(1).unwrap().parse::<i64>().unwrap())
        .collect_tuple()
        .unwrap();

    let mut p1_score: i64 = 0;
    let mut p2_score: i64 = 0;
    let mut rolls: i64 = 0;
    let mut curr_roll: i64 = 6;

    loop {
        p1 += curr_roll - 1;
        p1 %= 10;
        p1 += 1;
        p1_score += p1;
        curr_roll += 9;
        rolls += 3;

        if p1_score >= 1000 {
            return p2_score * rolls;
        }

        p2 += curr_roll - 1;
        p2 %= 10;
        p2 += 1;
        p2_score += p2;

        curr_roll += 9;
        rolls += 3;

        if p2_score >= 1000 {
            return p1_score * rolls;
        }
    }
}

fn part2(contents: String) -> i64 {
    let (p1, p2): (i64, i64) = contents
        .lines()
        .map(|line| line.split(": ").nth(1).unwrap().parse::<i64>().unwrap())
        .collect_tuple()
        .unwrap();

    let (p1_wins, p2_wins) = universe_wins(p1, p2, 0, 0, true);

    return p1_wins.max(p2_wins);
}

#[cached]
fn universe_wins(p1: i64, p2: i64, p1_score: i64, p2_score: i64, turn: bool) -> (i64, i64) {
    let mut p1_wins: i64 = 0;
    let mut p2_wins: i64 = 0;

    for (roll_val, occurrances) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)].iter() {
        if turn {
            let mut new_p1 = p1 + roll_val - 1;
            new_p1 %= 10;
            new_p1 += 1;
            let new_p1_score = p1_score + new_p1;
            if new_p1_score >= 21 {
                p1_wins += occurrances;
            } else {
                let (new_p1, new_p2) = universe_wins(new_p1, p2, new_p1_score, p2_score, !turn);
                p1_wins += occurrances * new_p1;
                p2_wins += occurrances * new_p2;
            }
        } else {
            let mut new_p2 = p2 + roll_val - 1;
            new_p2 %= 10;
            new_p2 += 1;

            let new_p2_score = p2_score + new_p2;
            if new_p2_score >= 21 {
                p2_wins += occurrances;
            } else {
                let (new_p1, new_p2) = universe_wins(p1, new_p2, p1_score, new_p2_score, !turn);
                p1_wins += occurrances * new_p1;
                p2_wins += occurrances * new_p2;
            }
        }
    }

    return (p1_wins, p2_wins);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 739785);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 444356092776315);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "21".to_string();
	
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
        "\nPart 1:\nLosing Score * Total Die Rolls: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMost universes won: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}