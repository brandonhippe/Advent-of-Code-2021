use relative_path::RelativePath;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i32 {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    let mut max_visit: HashMap<String, i32> = HashMap::new();

    for line in contents.lines() {
        let from = line.split("-").nth(0).unwrap().to_string();
        let to = line.split("-").nth(1).unwrap().to_string();

        connections
            .entry(from.clone())
            .or_insert(Vec::new())
            .push(to.clone());
        connections
            .entry(to.clone())
            .or_insert(Vec::new())
            .push(from.clone());

        max_visit.insert(
            from.clone(),
            if from == from.to_ascii_lowercase() {
                1
            } else {
                i32::MAX
            },
        );
        max_visit.insert(
            to.clone(),
            if to == to.to_ascii_lowercase() {
                1

            } else {
                i32::MAX
            },
        );
    }

    return dfs(
        "start".to_string(),
        "end".to_string(),
        connections,
        max_visit,
        false,
    );
}

fn part2(contents: String) -> i32 {
    let mut connections: HashMap<String, Vec<String>> = HashMap::new();
    let mut max_visit: HashMap<String, i32> = HashMap::new();

    for line in contents.lines() {
        let from = line.split("-").nth(0).unwrap().to_string();
        let to = line.split("-").nth(1).unwrap().to_string();

        connections
            .entry(from.clone())
            .or_insert(Vec::new())
            .push(to.clone());
        connections
            .entry(to.clone())
            .or_insert(Vec::new())
            .push(from.clone());

        max_visit.insert(
            from.clone(),
            if from == from.to_ascii_lowercase() {
                2
            } else {
                i32::MAX
            },
        );
        max_visit.insert(
            to.clone(),
            if to == to.to_ascii_lowercase() {
                2
            } else {
                i32::MAX
            },
        );
    }

    max_visit.insert("start".to_string(), 1);
    max_visit.insert("end".to_string(), 1);

    return dfs(
        "start".to_string(),
        "end".to_string(),
        connections,
        max_visit,
        true,
    );
}

fn dfs(
    start: String,
    end: String,
    connections: HashMap<String, Vec<String>>,
    max_visit: HashMap<String, i32>,
    p2: bool,
) -> i32 {
    if start == end {
        return 1;
    }

    let mut new_max_visit = max_visit.clone();
    new_max_visit.insert(start.clone(), max_visit.get(&start).unwrap() - 1);

    if p2
        && new_max_visit.get(&start).unwrap() == &0
        && new_max_visit.values().filter(|&x| x == &0).count() > 2
    {
        return 0;
    }

    let mut paths: i32 = 0;
    for connection in connections.get(&start).unwrap() {
        if max_visit.get(connection).unwrap() == &0 {
            continue;
        }

        paths += dfs(
            connection.clone(),
            end.clone(),
            connections.clone(),
            new_max_visit.clone(),
            p2,
        );
    }

    return paths;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 10);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 36);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "12".to_string();
	
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
        "\nPart 1:\nNumber of paths: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nNumber of paths: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}