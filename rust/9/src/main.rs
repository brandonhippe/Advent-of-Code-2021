use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let mut heights: HashMap<(i32, i32), i32> = HashMap::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            heights.insert((x as i32, y as i32), c.to_digit(10).unwrap() as i32);
        }
    }

    let mut total_risk: i32 = 0;
    for (pos, height) in heights.iter() {
        let mut small_point: bool = true;
        for i in (-1..=1).step_by(2) {
            if heights.get(&(pos.0 + i, pos.1)).unwrap_or(&10) <= height {
                small_point = false;
            }

            if heights.get(&(pos.0, pos.1 + i)).unwrap_or(&10) <= height {
                small_point = false;
            }
        }

        if small_point {
            total_risk += height + 1;
        }
    }


    return total_risk;
}

fn part2(contents: String) -> i32 {
    let mut heights: HashMap<(i32, i32), i32> = HashMap::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            heights.insert((x as i32, y as i32), c.to_digit(10).unwrap() as i32);
        }
    }

    let mut basins: Vec<i32> = Vec::new();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    for (pos, height) in heights.iter() {
        if visited.contains(pos) {
            continue;
        }

        if height == &9 {
            visited.insert(*pos);
            continue;
        }

        let mut basin_size: i32 = 0;
        let mut open_list: VecDeque<(i32, i32)> = VecDeque::from(vec![*pos]);
        while !open_list.is_empty() {
            let current = open_list.pop_front().unwrap();
            if visited.contains(&current) {
                continue;
            }

            basin_size += 1;
            visited.insert(current);

            for i in (-1..=1).step_by(2) {
                let next = (current.0 + i, current.1);
                if heights.get(&next).unwrap_or(&9) != &9 && !visited.contains(&next) {
                    open_list.push_back(next);
                }

                let next = (current.0, current.1 + i);
                if heights.get(&next).unwrap_or(&9) != &9 && !visited.contains(&next) {
                    open_list.push_back(next);
                }
            }
        }

        basins.push(basin_size);
    }

    basins.sort();
    return basins.iter().skip(basins.len() - 3).product();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 15);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 1134);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "9".to_string();
	
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
        "\nPart 1:\nTotal risk level of low points: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nProduct of sizes of 3 largest basins: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}