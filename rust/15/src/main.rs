use relative_path::RelativePath;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut positions: HashMap<(i64, i64), i64> = HashMap::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            positions.insert((x as i64, y as i64), c.to_string().parse::<i64>().unwrap());
        }
    }

    let max_x = positions.keys().map(|(x, _)| x).max().unwrap();
    let max_y = positions.keys().map(|(_, y)| y).max().unwrap();

    return a_star((0, 0), (*max_x, *max_y), &positions);
}

fn part2(contents: String) -> i64 {
    let mut positions: HashMap<(i64, i64), i64> = HashMap::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            positions.insert((x as i64, y as i64), c.to_string().parse::<i64>().unwrap());
        }
    }

    let x_dim = positions.keys().map(|(x, _)| x).max().unwrap() + 1;
    let y_dim = positions.keys().map(|(_, y)| y).max().unwrap() + 1;

    let max_x = x_dim * 5 - 1;

    let max_y = y_dim * 5 - 1;

    for y in 0..=max_y {
        for x in 0..=max_x {
            if positions.contains_key(&(x, y)) {
                continue;
            }

            let prev_x;
            let prev_y;

            if x >= x_dim {
                prev_x = x - x_dim;
                prev_y = y;
            } else {
                prev_x = x;
                prev_y = y - y_dim;
            }

            let new_val = positions.get(&(prev_x, prev_y)).unwrap() % 9 + 1;

            positions.insert((x, y), new_val);
        }
    }

    return a_star((0, 0), (max_x, max_y), &positions);
}

fn a_star(start: (i64, i64), end: (i64, i64), positions: &HashMap<(i64, i64), i64>) -> i64 {
    let mut open_set: BinaryHeap<(i64, i64, (i64, i64))> = BinaryHeap::from(vec![(0, 0, start)]);
    let mut open_dict: HashMap<(i64, i64), i64> = HashMap::from([(start, 0)]);
    let mut closed_dict: HashMap<(i64, i64), i64> = HashMap::new();

    while let Some((neg_f, g, pos)) = open_set.pop() {
        if pos == end {
            return g;
        }

        let f = -neg_f;

        if open_dict.contains_key(&pos) {
            open_dict.remove(&pos);
        }

        for (x, y) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_pos = (pos.0 + x, pos.1 + y);
            if !positions.contains_key(&new_pos) {
                continue;
            }

            let new_g = g + positions.get(&new_pos).unwrap();
            let new_f = new_g + manhattan_distance(new_pos, end);

            if open_dict.contains_key(&new_pos) && new_f >= *open_dict.get(&new_pos).unwrap() {
                continue;
            }

            if closed_dict.contains_key(&new_pos) && new_f >= *closed_dict.get(&new_pos).unwrap() {
                continue;
            }

            open_set.push((-new_f, new_g, new_pos));
            open_dict.insert(new_pos, new_f);
        }

        closed_dict.insert(pos, f);
    }

    return -1;
}

fn manhattan_distance(start: (i64, i64), end: (i64, i64)) -> i64 {
    return (start.0 - end.0).abs() + (start.1 - end.1).abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 40);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 315);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "15".to_string();
	
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
        "\nPart 1:\nLowest total risk: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLowest total risk: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}