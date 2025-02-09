use relative_path::RelativePath;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut group_lines: Vec<&str> = Vec::new();
    let mut groups: Vec<DigitProgram> = Vec::new();

    for line in contents.lines() {
        if line.chars().nth(0).unwrap() == 'i' {
            if group_lines.len() != 0 {
                groups.push(DigitProgram::new(group_lines.clone()));
            }

            group_lines.clear();
        }

        group_lines.push(line);
    }

    groups.push(DigitProgram::new(group_lines.clone()));

    return get_input(groups, 0, 0, (1..=9).rev().collect())
        .iter()
        .fold(0, |acc, x| acc * 10 + x);
}

fn part2(contents: String) -> i64 {
    let mut group_lines: Vec<&str> = Vec::new();
    let mut groups: Vec<DigitProgram> = Vec::new();

    for line in contents.lines() {
        if line.chars().nth(0).unwrap() == 'i' {

            if group_lines.len() != 0 {
                groups.push(DigitProgram::new(group_lines.clone()));
            }

            group_lines.clear();
        }

        group_lines.push(line);
    }

    groups.push(DigitProgram::new(group_lines.clone()));

    return get_input(groups, 0, 0, (1..=9).collect())
        .iter()
        .fold(0, |acc, x| acc * 10 + x);
}

#[derive(Clone, Debug)]
struct DigitProgram {
    dec_type: bool,
    x_offset: i64,
    z_mod26_range: Vec<i64>,
}

impl DigitProgram {
    fn new(lines: Vec<&str>) -> DigitProgram {
        let dec = lines[4].split(" ").last().unwrap() == "26";
        let x = if dec {
            lines[5].split(" ").last().unwrap().parse::<i64>().unwrap()
        } else {
            lines[15].split(" ").last().unwrap().parse::<i64>().unwrap()
        };
        let z_mod_range: Vec<i64> = if dec {
            Vec::from_iter((1..=9).map(|n| n - x))
        } else {
            Vec::new()
        };

        let dig_prog = DigitProgram {
            dec_type: dec,
            x_offset: x,
            z_mod26_range: z_mod_range,
        };

        return dig_prog;
    }

    fn z_output(self, w: i64, z: i64) -> i64 {
        if self.dec_type {
            if z % 26 + self.x_offset == w {
                return z / 26;
            } else {
                return -1;
            }
        } else {
            return 26 * z + self.x_offset + w;
        }
    }
}

fn get_input(
    groups: Vec<DigitProgram>,
    z: i64,
    digit_index: usize,
    digit_iter: Vec<i64>,
) -> VecDeque<i64> {
    let g = &groups[digit_index];

    if g.dec_type {
        if g.z_mod26_range.contains(&(z % 26)) {
            let w = g.z_mod26_range.iter().position(|v| *v == z % 26).unwrap() as i64 + 1;
            let next_z = g.clone().z_output(w, z);

            if digit_index == groups.len() - 1 {
                return if next_z == 0 {
                    vec![w].into()
                } else {
                    vec![].into()
                };
            }

            let mut next_inputs =
                get_input(groups.clone(), next_z, digit_index + 1, digit_iter.clone());

            if next_inputs.len() != 0 {
                next_inputs.push_front(w);
                return next_inputs;
            }
        }
    } else {
        for w in digit_iter.iter() {
            let next_z = g.clone().z_output(*w, z);
            if digit_index == groups.len() - 1 {
                return if next_z == 0 {
                    vec![*w].into()
                } else {
                    vec![].into()
                };
            }

            let mut next_inputs =
                get_input(groups.clone(), next_z, digit_index + 1, digit_iter.clone());

            if next_inputs.len() != 0 {
                next_inputs.push_front(*w);
                return next_inputs;
            }
        }
    }

    return vec![].into();
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "24".to_string();
	
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
        "\nPart 1:\nLargest valid number: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSmallest valid number: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}