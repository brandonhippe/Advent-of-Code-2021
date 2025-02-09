use itertools::Itertools;
use regex::Regex;
use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let nums_re = Regex::new(r"(-?\d+)").unwrap();
    let (x_min, x_max, y_min, y_max) = nums_re
        .captures_iter(&contents)
        .map(|cap| cap[1].parse::<i64>().unwrap())
        .collect_tuple()
        .expect("Failed");

    let mut y_v_max: i64 = 0;
    for steps in 1..=500 {
        let mut x_v = 1;
        while final_x(x_v, steps) < x_min {
            x_v += 1;
        }

        let x_v_start = x_v;

        while final_x(x_v, steps) <= x_max {
            x_v += 1;
        }

        if x_v == x_v_start {
            continue;
        }

        let mut y_v = y_min;

        while final_y(y_v, steps) < y_min {
            y_v += 1;
        }

        let y_v_start = y_v;

        while final_y(y_v, steps) <= y_max {
            y_v += 1;
        }

        if y_v != y_v_start {
            if y_v - 1 > y_v_max {
                y_v_max = y_v - 1;
            }
        }
    }

    return triangle(y_v_max);
}

fn part2(contents: String) -> i64 {
    let nums_re = Regex::new(r"(-?\d+)").unwrap();
    let (x_min, x_max, y_min, y_max) = nums_re
        .captures_iter(&contents)
        .map(|cap| cap[1].parse::<i64>().unwrap())
        .collect_tuple()
        .expect("Failed");

    let mut y_vels: HashMap<i64, (i64, i64)> = HashMap::new();
    for steps in 1..=500 {
        let mut y_v = y_min;
        while final_y(y_v, steps) < y_min {
            y_v += 1;
        }

        y_vels.insert(steps, (y_v, 1));

        while final_y(y_v, steps) <= y_max {
            y_v += 1;
        }

        if y_v == y_vels[&steps].0 {
            y_vels.remove(&steps);
        } else {
            y_vels.get_mut(&steps).unwrap().1 = y_v;
        }
    }

    let mut x_vels: HashMap<i64, (i64, i64)> = HashMap::new();
    for s in y_vels.keys() {
        let steps = *s;
        let mut x_v = 1;
        while final_x(x_v, steps) < x_min {
            x_v += 1;
        }

        x_vels.insert(steps, (x_v, 1));

        while final_x(x_v, steps) <= x_max {
            x_v += 1;
        }

        if x_v == x_vels[&steps].0 {
            x_vels.remove(&steps);
        } else {
            x_vels.get_mut(&steps).unwrap().1 = x_v;
        }
    }

    let mut valid: HashSet<(i64, i64)> = HashSet::new();
    for (y_steps, (y_v_min, y_v_max)) in y_vels.iter() {
        let (x_v_min, x_v_max) = x_vels.get(y_steps).unwrap();
        for y_v in *y_v_min..*y_v_max {
            for x_v in *x_v_min..*x_v_max {
                valid.insert((x_v, y_v));
            }
        }
    }

    return valid.len() as i64;
}

fn final_x(x_v: i64, steps: i64) -> i64 {
    if x_v <= steps {
        return triangle(x_v);
    }

    return triangle(x_v) - triangle(x_v - steps);
}

fn final_y(y_v: i64, steps: i64) -> i64 {
    if y_v <= 0 {
        return -(triangle(y_v - steps) - triangle(y_v));
    }

    if y_v >= steps {
        return triangle(y_v) - triangle(y_v - steps);
    }

    return triangle(y_v) - triangle(steps - y_v - 1);
}

fn triangle(n: i64) -> i64 {
    return n * (n + 1) / 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 45);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 112);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "17".to_string();
	
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
        "\nPart 1:\nMaximum height reached: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nTrajectories that hit target: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}