use itertools::Itertools;
use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let mut pos = vec![0, 0];

    for line in contents.lines() {
        let split_line: Vec<_> = line.split(" ").collect();
        let dir = split_line[0].to_string();
        let dist = split_line[1].parse::<i32>().unwrap();

        match dir.as_str() {
            "forward" => {
                pos[0] += dist;
            }
            "down" => {
                pos[1] += dist;
            }
            "up" => {
                pos[1] -= dist;
            }
            &_ => {
                println!("Unknown direction: {}", dir);
            }
        }
    }

    return pos.into_iter().product();
}

fn part2(contents: String) -> i32 {
    let mut pos = vec![0, 0];

    let mut aim = 0;

    for line in contents.lines() {
        let split_line: Vec<_> = line.split(" ").collect();
        let dir = split_line[0].to_string();
        let dist = split_line[1].parse::<i32>().unwrap();

        match dir.as_str() {
            "forward" => {
                pos[0] += dist;
                pos[1] += aim * dist;
            }
            "down" => {
                aim += dist;
            }
            "up" => {
                aim -= dist;
            }
            &_ => {
                println!("Unknown direction: {}", dir);
            }
        }
    }

    return pos.into_iter().product();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 150);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 900);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let year = "2021".to_string();
    let day = "2".to_string();

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
        "\nPart 1:\nHorizontal position * depth: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nHorizontal position * depth: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}