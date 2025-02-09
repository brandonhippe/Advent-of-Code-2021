use itertools::Itertools;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let mut point_counts: HashMap<(i32, i32), i32> = HashMap::new();

    for line in contents.lines() {
        let point_1: (i32, i32) = line
            .split(" -> ")
            .nth(0)
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        let point_2: (i32, i32) = line
            .split(" -> ")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        if point_1.0 == point_2.0 {
            let min_y = std::cmp::min(point_1.1, point_2.1);
            let max_y = std::cmp::max(point_1.1, point_2.1);

            for y in min_y..=max_y {
                let point = (point_1.0, y);
                let count = point_counts.entry(point).or_insert(0);

                *count += 1;
            }
        } else if point_1.1 == point_2.1 {
            let min_x = std::cmp::min(point_1.0, point_2.0);
            let max_x = std::cmp::max(point_1.0, point_2.0);

            for x in min_x..=max_x {
                let point = (x, point_1.1);
                let count = point_counts.entry(point).or_insert(0);
                *count += 1;
            }
        }
    }

    return point_counts.values().filter(|x| **x >= 2).count() as i32;
}

fn part2(contents: String) -> i32 {
    let mut point_counts: HashMap<(i32, i32), i32> = HashMap::new();

    for line in contents.lines() {
        let point_1: (i32, i32) = line
            .split(" -> ")
            .nth(0)
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        let point_2: (i32, i32) = line
            .split(" -> ")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        if point_1.0 == point_2.0 {
            let min_y = std::cmp::min(point_1.1, point_2.1);
            let max_y = std::cmp::max(point_1.1, point_2.1);

            for y in min_y..=max_y {
                let point = (point_1.0, y);
                let count = point_counts.entry(point).or_insert(0);
                *count += 1;
            }
        } else if point_1.1 == point_2.1 {
            let min_x = std::cmp::min(point_1.0, point_2.0);
            let max_x = std::cmp::max(point_1.0, point_2.0);

            for x in min_x..=max_x {
                let point = (x, point_1.1);
                let count = point_counts.entry(point).or_insert(0);
                *count += 1;
            }
        } else {
            let x_step = if point_1.0 < point_2.0 { 1 } else { -1 };
            let y_step = if point_1.1 < point_2.1 { 1 } else { -1 };
            let min_x = std::cmp::min(point_1.0, point_2.0);
            let max_x = std::cmp::max(point_1.0, point_2.0);

            for offset in 0..=max_x - min_x {
                let point = (point_1.0 + x_step * offset, point_1.1 + y_step * offset);
                let count = point_counts.entry(point).or_insert(0);
                *count += 1;
            }
        }
    }

    return point_counts.values().filter(|x| **x >= 2).count() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 5);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 12);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "5".to_string();
	
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
        "\nPart 1:\nPoints where two lines intersect: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nPoints where two lines intersect: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}