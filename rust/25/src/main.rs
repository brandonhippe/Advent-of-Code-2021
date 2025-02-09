use relative_path::RelativePath;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut east_facing: HashSet<(i64, i64)> = HashSet::new();
    let mut south_facing: HashSet<(i64, i64)> = HashSet::new();

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '>' => east_facing.insert((x as i64, y as i64)),
                'v' => south_facing.insert((x as i64, y as i64)),
                _ => false,
            };
        }
    }

    let x_max: i64 = contents.lines().nth(0).unwrap().len() as i64;
    let y_max: i64 = contents.lines().collect::<Vec<&str>>().len() as i64;
    let mut t: i64 = 0;
    loop {
        let (new_east, new_south) =
            move_heards(east_facing.clone(), south_facing.clone(), x_max, y_max);
        t += 1;

        if new_east == east_facing && new_south == south_facing {
            break;
        }

        east_facing = new_east;
        south_facing = new_south;
    }


    return t;
}

fn part2(_contents: String) -> String {
    return "Christmas has been saved!".to_string();
}

fn move_heards(
    east_facing: HashSet<(i64, i64)>,
    south_facing: HashSet<(i64, i64)>,
    x_max: i64,
    y_max: i64,
) -> (HashSet<(i64, i64)>, HashSet<(i64, i64)>) {
    let mut new_east: HashSet<(i64, i64)> = HashSet::new();
    let mut new_south: HashSet<(i64, i64)> = HashSet::new();

    for (x, y) in east_facing.iter() {
        let new_x = (x + 1) % x_max;
        if !east_facing.contains(&(new_x, *y)) && !south_facing.contains(&(new_x, *y)) {
            new_east.insert((new_x, *y));
        } else {
            new_east.insert((*x, *y));
        }
    }

    for (x, y) in south_facing.iter() {
        let new_y = (y + 1) % y_max;
        if !new_east.contains(&(*x, new_y)) && !south_facing.contains(&(*x, new_y)) {
            new_south.insert((*x, new_y));
        } else {
            new_south.insert((*x, *y));
        }
    }

    return (new_east, new_south);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 58);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "25".to_string();
	
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
        "\nPart 1:\nFirst step where no sea cucumbers move: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\n{}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}