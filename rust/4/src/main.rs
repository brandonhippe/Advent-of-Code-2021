use regex::Regex;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let nums_regex = Regex::new(r"(\d+)").unwrap();
    let lines: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            nums_regex
                .captures_iter(line)
                .map(|x| x[1].parse().unwrap())
                .collect()
        })
        .collect();

    let nums_dict: HashMap<i32, i32> =
        HashMap::from_iter(lines[0].iter().enumerate().map(|(i, x)| (*x, i as i32)));
    let mut first_win: i32 = (nums_dict.len() - 1) as i32;
    let mut score: i32 = 0;

    for i in (2..lines.len()).step_by(6) {
        let board_set: HashSet<i32> = HashSet::from_iter(
            lines[i]
                .iter()
                .chain(lines[i + 1].iter())
                .chain(lines[i + 2].iter())
                .chain(lines[i + 3].iter())
                .chain(lines[i + 4].iter())
                .cloned(),
        );


        for j in 0..5 {
            let row_win: i32 = lines[i + j]
                .iter()
                .map(|x| {
                    if nums_dict.contains_key(x) {
                        *nums_dict.get(x).unwrap()
                    } else {
                        nums_dict.len() as i32
                    }
                })
                .max()
                .expect("No max found");
            if row_win < first_win {
                first_win = row_win;
                let used_set: HashSet<i32> =
                    HashSet::from_iter((0..row_win + 1).map(|x| lines[0][x as usize]));
                let unused_set: HashSet<i32> = board_set.difference(&used_set).cloned().collect();

                score = unused_set.iter().map(|x| *x).sum::<i32>() * (lines[0][row_win as usize]);
            }

            let col_win: i32 = (0..5)
                .map(|x| {
                    if nums_dict.contains_key(&lines[i + x][j]) {
                        *nums_dict.get(&lines[i + x][j]).unwrap()
                    } else {
                        nums_dict.len() as i32
                    }
                })
                .max()
                .expect("No max found");
            if col_win < first_win {
                first_win = col_win;

                let used_set: HashSet<i32> =
                    HashSet::from_iter((0..col_win + 1).map(|x| lines[0][x as usize]));
                let unused_set: HashSet<i32> = board_set.difference(&used_set).cloned().collect();

                score = unused_set.iter().map(|x| *x).sum::<i32>() * (lines[0][col_win as usize]);
            }
        }
    }

    return score;
}

fn part2(contents: String) -> i32 {
    let nums_regex = Regex::new(r"(\d+)").unwrap();
    let lines: Vec<Vec<i32>> = contents
        .lines()
        .map(|line| {
            nums_regex
                .captures_iter(line)
                .map(|x| x[1].parse().unwrap())
                .collect()
        })
        .collect();

    let nums_dict: HashMap<i32, i32> =
        HashMap::from_iter(lines[0].iter().enumerate().map(|(i, x)| (*x, i as i32)));
    let mut last_win: i32 = 0;
    let mut score: i32 = 0;

    for i in (2..lines.len()).step_by(6) {
        let board_set: HashSet<i32> = HashSet::from_iter(
            lines[i]
                .iter()
                .chain(lines[i + 1].iter())
                .chain(lines[i + 2].iter())
                .chain(lines[i + 3].iter())
                .chain(lines[i + 4].iter())
                .cloned(),
        );
        let mut board_win: i32 = (nums_dict.len() - 1) as i32;
        let mut board_score: i32 = 0;

        for j in 0..5 {
            let row_win: i32 = lines[i + j]
                .iter()
                .map(|x| {
                    if nums_dict.contains_key(x) {
                        *nums_dict.get(x).unwrap()
                    } else {
                        0
                    }
                })
                .max()
                .expect("No max found");
            if row_win < board_win {
                board_win = row_win;
                let used_set: HashSet<i32> =
                    HashSet::from_iter((0..row_win + 1).map(|x| lines[0][x as usize]));
                let unused_set: HashSet<i32> = board_set.difference(&used_set).cloned().collect();

                board_score =
                    unused_set.iter().map(|x| *x).sum::<i32>() * (lines[0][row_win as usize]);
            }

            let col_win: i32 = (0..5)
                .map(|x| {
                    if nums_dict.contains_key(&lines[i + x][j]) {
                        *nums_dict.get(&lines[i + x][j]).unwrap()
                    } else {
                        0
                    }
                })
                .max()
                .expect("No max found");
            if col_win < board_win {
                board_win = col_win;

                let used_set: HashSet<i32> =
                    HashSet::from_iter((0..col_win + 1).map(|x| lines[0][x as usize]));
                let unused_set: HashSet<i32> = board_set.difference(&used_set).cloned().collect();

                board_score =
                    unused_set.iter().map(|x| *x).sum::<i32>() * (lines[0][col_win as usize]);
            }
        }

        if board_win > last_win {
            last_win = board_win;
            score = board_score;
        }
    }

    return score;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 4512);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 1924);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "4".to_string();
	
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
        "\nPart 1:\nScore of first winning board: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nScore of last winning board: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}