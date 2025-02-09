use itertools::Itertools;
use relative_path::RelativePath;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let mut dots: HashSet<(i32, i32)> = HashSet::new();
    for line in contents.lines() {
        if line.len() == 0 {
            break;
        }

        dots.insert(
            line.split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap(),
        );
    }

    let instruction = contents.lines().skip(dots.len() + 1).next().unwrap();
    let axis = instruction.chars().nth(11).unwrap();
    let num = instruction[13..].parse::<i32>().unwrap();

    let mut new_dots: HashSet<(i32, i32)> = HashSet::new();
    for (x, y) in dots.iter() {
        if axis == 'x' {
            new_dots.insert((num - (num - *x).abs(), *y));
        } else {
            new_dots.insert((*x, num - (num - *y).abs()));
        }
    }


    return new_dots.len() as i32;
}

fn part2(contents: String) -> String {
    let mut dots: HashSet<(i32, i32)> = HashSet::new();
    for line in contents.lines() {
        if line.len() == 0 {
            break;
        }

        dots.insert(
            line.split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap(),
        );
    }

    for instruction in contents.lines().skip(dots.len() + 1) {
        let axis = instruction.chars().nth(11).unwrap();
        let num = instruction[13..].parse::<i32>().unwrap();

        let mut new_dots: HashSet<(i32, i32)> = HashSet::new();
        for (x, y) in dots.iter() {
            if axis == 'x' {
                new_dots.insert((num - (num - *x).abs(), *y));
            } else {
                new_dots.insert((*x, num - (num - *y).abs()));
            }
        }

        dots = new_dots;
    }

    let x_min = dots.iter().min_by_key(|(x, _)| x).unwrap().0;
    let x_max = dots.iter().max_by_key(|(x, _)| x).unwrap().0;
    let y_min = dots.iter().min_by_key(|(_, y)| y).unwrap().1;
    let y_max = dots.iter().max_by_key(|(_, y)| y).unwrap().1;

    let mut return_string: String = "".to_string();

    for y in y_min..=y_max {
        return_string.push('\n');
        for x in x_min..=x_max {
            if dots.contains(&(x, y)) {
                return_string.push('█');
            } else {
                return_string.push(' ');
            }
        }
    }

    return return_string;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 17);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(
            part2(contents),
            "\n#####\n#   #\n#   #\n#   #\n#####".to_string()
        );
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "13".to_string();
	
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
        "\nPart 1:\nNumber of dots visible after 1 fold: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMesssage: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}