use relative_path::RelativePath;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let nums: Vec<i32> = contents
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let min: i32 = *nums.iter().min().unwrap();
    let max: i32 = *nums.iter().max().unwrap();

    return (min..max + 1)
        .map(|c| nums.iter().map(|n| (n - c).abs()).sum::<i32>())
        .min()
        .unwrap();
}

fn part2(contents: String) -> i32 {
    let nums: Vec<i32> = contents
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let min: i32 = *nums.iter().min().unwrap();
    let max: i32 = *nums.iter().max().unwrap();

    return (min..max + 1)
        .map(|c| {
            nums.iter()
                .map(|n| ((n - c).abs()) * ((n - c).abs() + 1) / 2)
                .sum::<i32>()
        })

        .min()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 37);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 168);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "7".to_string();
	
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
        "\nPart 1:\nMinimum fuel: {}\nRan in {:.5?}",
        part1(contents.clone().lines().next().unwrap().to_string()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMinimun fuel: {}\nRan in {:.5?}",
        part2(contents.clone().lines().next().unwrap().to_string()),
        part2_timer.elapsed()
    );
}