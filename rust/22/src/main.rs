use regex::Regex;
use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let num_re = Regex::new(r"(-?\d+)").unwrap();
    let mut cubes_instructions: Vec<(i64, i64, i64, i64, i64, i64, bool)> = vec![];
    for line in contents.lines() {
        let nums = num_re
            .captures_iter(line)
            .map(|x| x[0].parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        if nums.iter().map(|x| x.abs()).max().unwrap() <= 50 {
            cubes_instructions.push((
                nums[0],
                nums[1],
                nums[2],
                nums[3],
                nums[4],
                nums[5],
                line.chars().nth(1).unwrap() == 'n',
            ));
        }
    }

    return count_lit(&cubes_instructions);
}

fn part2(contents: String) -> i64 {
    let num_re = Regex::new(r"(-?\d+)").unwrap();
    let mut cubes_instructions: Vec<(i64, i64, i64, i64, i64, i64, bool)> = vec![];
    for line in contents.lines() {
        let nums = num_re

            .captures_iter(line)
            .map(|x| x[0].parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        cubes_instructions.push((
            nums[0],
            nums[1],
            nums[2],
            nums[3],
            nums[4],
            nums[5],
            line.chars().nth(1).unwrap() == 'n',
        ));
    }

    return count_lit(&cubes_instructions);
}

fn count_lit(instructions: &Vec<(i64, i64, i64, i64, i64, i64, bool)>) -> i64 {
    let mut total: i64 = 0;
    let mut finished: Vec<(i64, i64, i64, i64, i64, i64)> = vec![];

    for ins in instructions.iter().rev() {
        let box1 = (ins.0, ins.1, ins.2, ins.3, ins.4, ins.5);
        let op = ins.6;

        if op {
            let mut intersections: Vec<(i64, i64, i64, i64, i64, i64, bool)> = vec![];
            for box2 in finished.iter() {
                if let Some(intersection) = find_intersection(box1, *box2) {
                    intersections.push((intersection.0, intersection.1, intersection.2, intersection.3, intersection.4, intersection.5, true));
                }
            }

            total += (box1.1 - box1.0 + 1) * (box1.3 - box1.2 + 1) * (box1.5 - box1.4 + 1);
            if !intersections.is_empty() {
                total -= count_lit(&intersections);
            }
        }

        finished.push(box1);
    }

    return total;
}

fn find_intersection(box1: (i64, i64, i64, i64, i64, i64), box2: (i64, i64, i64, i64, i64, i64)) -> Option<(i64, i64, i64, i64, i64, i64)> {
    let max_x = if box1.0 > box2.0 { box1.0 } else { box2.0 };
    let min_x = if box1.1 < box2.1 { box1.1 } else { box2.1 };
    let max_y = if box1.2 > box2.2 { box1.2 } else { box2.2 };
    let min_y = if box1.3 < box2.3 { box1.3 } else { box2.3 };
    let max_z = if box1.4 > box2.4 { box1.4 } else { box2.4 };
    let min_z = if box1.5 < box2.5 { box1.5 } else { box2.5 };

    if min_x >= max_x && min_y >= max_y && min_z >= max_z {
        return Some((max_x, min_x, max_y, min_y, max_z, min_z));
    } else {
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("p1_1_example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 39);

        let contents =
            fs::read_to_string("p1_2_example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 590784);

        let contents =
            fs::read_to_string("p2_example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 474140);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("p2_example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 2758514936282235);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "22".to_string();
	
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
        "\nPart 1:\nTotal cubes on: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nTotal cubes on: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}