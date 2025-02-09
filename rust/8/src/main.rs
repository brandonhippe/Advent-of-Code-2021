use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    return contents
        .lines()
        .map(|line| {
            line.split(" | ")
                .nth(1)
                .unwrap()
                .split(" ")
                .map(|x| x.to_string())
                .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
                .count() as i32
        })
        .sum::<i32>();
}

fn part2(contents: String) -> i32 {
    let digit_map: HashMap<&str, i32> = HashMap::from([
        ("abcefg", 0),
        ("cf", 1),
        ("acdeg", 2),
        ("acdfg", 3),
        ("bcdf", 4),
        ("abdfg", 5),
        ("abdefg", 6),
        ("acf", 7),
        ("abcdefg", 8),
        ("abcdfg", 9),
    ]);
    let segment_freq_lens: HashMap<i32, char> = HashMap::from([

        (344, 'a'),
        (204, 'b'),
        (304, 'c'),
        (266, 'd'),
        (96, 'e'),
        (396, 'f'),
        (280, 'g'),
    ]);

    let mut output_sum: i32 = 0;
    for line in contents.lines() {
        let mut segment_map: HashMap<char, char> = HashMap::new();
        let inputs: Vec<String> = line
            .split(" | ")
            .nth(0)
            .unwrap()
            .split(" ")
            .map(|x| x.to_string())
            .collect();

        for c in segment_freq_lens.values() {
            let freq = inputs.iter().filter(|x| x.contains(*c)).count() as i32;
            let len = inputs
                .iter()
                .filter(|x| x.contains(*c))
                .map(|x| x.len())
                .sum::<usize>() as i32;

            segment_map.insert(*c, *segment_freq_lens.get(&(freq * len)).unwrap());
        }

        let mut output_val: i32 = 0;
        for n in line.split(" | ").nth(1).unwrap().split(" ") {
            let mut converted_string_vec: Vec<char> =
                n.chars().map(|x| *segment_map.get(&x).unwrap()).collect();
            converted_string_vec.sort();
            output_val = output_val * 10
                + *digit_map
                    .get(&converted_string_vec.iter().collect::<String>()[..])
                    .unwrap();
        }

        output_sum += output_val;
    }

    return output_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 26);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 61229);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "8".to_string();
	
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
        "\nPart 1:\nOccurrances of unique output values: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nSum of output values: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}