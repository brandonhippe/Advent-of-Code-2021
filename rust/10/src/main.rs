use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let syntax_map: HashMap<char, i32> = HashMap::from([
        ('(', 3),
        (')', 3),
        ('[', 57),
        (']', 57),
        ('{', 1197),
        ('}', 1197),
        ('<', 25137),
        ('>', 25137),
    ]);
    let mut syntax_score: i32 = 0;

    for line in contents.lines() {
        let mut opened: VecDeque<char> = VecDeque::new();

        for c in line.chars() {
            if ['(', '[', '{', '<'].contains(&c) {
                opened.push_back(c);
            } else if [')', ']', '}', '>'].contains(&c) {
                let last = opened.pop_back().unwrap();

                if syntax_map.get(&last) != syntax_map.get(&c) {
                    syntax_score += syntax_map.get(&c).unwrap();
                    break;
                }
            }
        }

    }

    return syntax_score;
}

fn part2(contents: String) -> i64 {
    let syntax_map: HashMap<char, i32> = HashMap::from([
        ('(', 3),
        (')', 3),
        ('[', 57),
        (']', 57),
        ('{', 1197),
        ('}', 1197),
        ('<', 25137),
        ('>', 25137),
    ]);

    let autocomplete_map: HashMap<char, i64> =
        HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let mut autocomplete_scores: Vec<i64> = Vec::new();

    for line in contents.lines() {
        let mut opened: VecDeque<char> = VecDeque::new();
        let mut valid: bool = true;

        for c in line.chars() {
            if ['(', '[', '{', '<'].contains(&c) {
                opened.push_back(c);
            } else if [')', ']', '}', '>'].contains(&c) {
                let last = opened.pop_back().unwrap();

                if syntax_map.get(&last) != syntax_map.get(&c) {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            let mut score: i64 = 0;

            while let Some(c) = opened.pop_back() {
                score *= 5;
                score += *autocomplete_map.get(&c).unwrap();
            }

            autocomplete_scores.push(score);
        }
    }

    autocomplete_scores.sort();
    return autocomplete_scores
        .iter()
        .skip(autocomplete_scores.len() / 2)
        .nth(0)
        .unwrap()
        .clone();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 26397);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 288957);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "10".to_string();
	
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
        "\nPart 1:\nTotal syntax error: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nMiddle autocomplete score: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}