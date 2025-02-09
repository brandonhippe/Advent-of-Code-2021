use relative_path::RelativePath;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let turn_on: HashSet<i64> = HashSet::from_iter(
        contents
            .lines()
            .nth(0)
            .unwrap()
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(i, _)| i as i64),
    );

    let mut image: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in contents.lines().skip(2).enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                image.insert((x as i64, y as i64));
            }
        }
    }

    for iteration in 0..2 {
        image = enhancement(&image, &turn_on, turn_on.contains(&0) && iteration % 2 == 1);
    }

    return image.len() as i64;
}

fn part2(contents: String) -> i64 {

    let turn_on: HashSet<i64> = HashSet::from_iter(
        contents
            .lines()
            .nth(0)
            .unwrap()
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(i, _)| i as i64),
    );

    let mut image: HashSet<(i64, i64)> = HashSet::new();
    for (y, line) in contents.lines().skip(2).enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                image.insert((x as i64, y as i64));
            }
        }
    }

    for iteration in 0..50 {
        image = enhancement(&image, &turn_on, turn_on.contains(&0) && iteration % 2 == 1);
    }

    return image.len() as i64;
}

fn enhancement(
    image: &HashSet<(i64, i64)>,
    turn_on: &HashSet<i64>,
    outisde_lit: bool,
) -> HashSet<(i64, i64)> {
    let mut new_image: HashSet<(i64, i64)> = HashSet::new();
    let min_x: i64 = *image.iter().map(|(x, _)| x).min().unwrap();
    let max_x: i64 = *image.iter().map(|(x, _)| x).max().unwrap();
    let min_y: i64 = *image.iter().map(|(_, y)| y).min().unwrap();
    let max_y: i64 = *image.iter().map(|(_, y)| y).max().unwrap();

    for y in min_y - 1..=max_y + 1 {
        for x in min_x - 1..=max_x + 1 {
            let mut val: i64 = 0;
            for (dy, dx) in [-1, 0, 1]
                .iter()
                .flat_map(|x| [-1, 0, 1].iter().map(move |y| (x, y)))
            {
                val <<= 1;
                let nx = x + dx;
                let ny = y + dy;

                if outisde_lit && (nx < min_x || nx > max_x || ny < min_y || ny > max_y) {
                    val += 1;
                } else {
                    val += if image.contains(&(nx, ny)) { 1 } else { 0 };
                }
            }

            if turn_on.contains(&val) {
                new_image.insert((x, y));
            }
        }
    }

    return new_image;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 35);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 3351);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "20".to_string();
	
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
        "\nPart 1:\nPixels on after 2 iterations: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nPixels on after 50 iterations: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}