use relative_path::RelativePath;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::rc::Rc;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut numbers: VecDeque<Rc<RefCell<Number>>> = Vec::new().into();
    let mut index: i64 = 0;

    for line in contents.lines() {
        numbers.push_front(parse_number(line, &mut index));
    }

    while numbers.len() > 1 {
        // Get the left and right numbers
        let left_number = numbers.pop_back().unwrap();
        let right_number = numbers.pop_back().unwrap();

        // Create a new number containing the left and right numbers
        let mut new_number = Rc::new(RefCell::new(Number::new()));
        new_number.borrow_mut().index = index;
        index += 1;
        new_number.borrow_mut().left = Some(left_number.clone());
        new_number.borrow_mut().right = Some(right_number.clone());

        // Set the left and right parents
        left_number.borrow_mut().parent = Some(new_number.clone());
        right_number.borrow_mut().parent = Some(new_number.clone());

        // Reduce new number
        reduce(&mut new_number, &mut index);
        numbers.push_back(new_number);

    }

    return get_magnitude(numbers.pop_back().unwrap());
}

fn part2(contents: String) -> i64 {
    let lines: Vec<&str> = contents.lines().collect();

    let mut max_val: i64 = 0;
    for l_ix in 0..lines.len() {
        for r_ix in 0..lines.len() {
            if l_ix == r_ix {
                continue;
            }
            let mut index: i64 = 0;

            // Get the left and right numbers
            let left_number = parse_number(lines[l_ix], &mut index);
            let right_number = parse_number(lines[r_ix], &mut index);

            // Create a new number containing the left and right numbers
            let mut new_number = Rc::new(RefCell::new(Number::new()));
            new_number.borrow_mut().index = index;
            index += 1;
            new_number.borrow_mut().left = Some(left_number.clone());
            new_number.borrow_mut().right = Some(right_number.clone());

            // Set the left and right parents
            left_number.borrow_mut().parent = Some(new_number.clone());
            right_number.borrow_mut().parent = Some(new_number.clone());

            reduce(&mut new_number, &mut index);

            let val = get_magnitude(new_number.clone());
            if val > max_val {
                max_val = val;
            }
        }
    }

    return max_val;
}

fn reduce(number: &mut Rc<RefCell<Number>>, index: &mut i64) {
    let mut changed = true;

    while changed {
        changed = false;

        let mut depth = 0;
        let mut next = Some(number.clone());
        let mut left_regular: Option<Rc<RefCell<Number>>> = None;
        while let Some(ref curr) = next {
            if curr.borrow().magnitude.is_some() {
                left_regular = Some(curr.clone());
            }

            if depth == 4 && curr.clone().borrow().left.is_some() {
                // Left Explosion
                if left_regular.is_some() {
                    let mut left_val = curr
                        .as_ref()
                        .borrow()
                        .left
                        .as_ref()
                        .unwrap()
                        .borrow()
                        .magnitude
                        .unwrap();
                    left_val += left_regular.as_ref().unwrap().borrow().magnitude.unwrap();
                    left_regular.as_ref().unwrap().borrow_mut().magnitude = Some(left_val);
                }

                // Right Explosion
                let mut right_regular: Option<Rc<RefCell<Number>>> =
                    dfs_traversal(curr.as_ref().borrow().right.as_ref().unwrap().clone(), 0).0;
                while let Some(ref new_right) = right_regular {
                    if new_right.borrow().magnitude.is_some() {
                        let mut right_val = new_right.borrow().magnitude.unwrap();
                        right_val += curr
                            .as_ref()
                            .borrow()
                            .right
                            .as_ref()
                            .unwrap()
                            .borrow()
                            .magnitude
                            .unwrap();
                        new_right.borrow_mut().magnitude = Some(right_val);
                        break;
                    }

                    right_regular = dfs_traversal(new_right.clone(), 0).0;
                }

                // Replace with regular 0
                let mut number = curr.as_ref().borrow_mut();
                number.magnitude = Some(0);
                number.left = None;
                number.right = None;

                changed = true;
                break;
            }

            let (new_next, new_depth) = dfs_traversal(curr.clone(), depth);

            next = new_next;
            depth = new_depth;
        }

        if changed {
            continue;
        }

        let mut depth = 0;
        let mut next = Some(number.clone());
        while let Some(ref curr) = next {
            if curr.borrow().magnitude.is_some() {
                let mag = curr.borrow().magnitude.unwrap();

                if mag >= 10 {
                    let left_val = mag / 2;
                    let right_val = mag - left_val;

                    let left = Rc::new(RefCell::new(Number::new()));
                    left.borrow_mut().index = *index;
                    *index += 1;
                    left.borrow_mut().magnitude = Some(left_val);
                    left.borrow_mut().parent = Some(curr.clone());

                    let right = Rc::new(RefCell::new(Number::new()));
                    right.borrow_mut().index = *index;
                    *index += 1;
                    right.borrow_mut().magnitude = Some(right_val);
                    right.borrow_mut().parent = Some(curr.clone());

                    curr.borrow_mut().magnitude = None;
                    curr.borrow_mut().left = Some(left.clone());
                    curr.borrow_mut().right = Some(right.clone());

                    changed = true;
                    break;
                }
            }

            let (new_next, new_depth) = dfs_traversal(curr.clone(), depth);

            next = new_next;
            depth = new_depth;
        }
    }
}

fn get_magnitude(number: Rc<RefCell<Number>>) -> i64 {
    if number.borrow().magnitude.is_some() {
        return number.borrow().magnitude.unwrap();
    }

    let left = get_magnitude(number.borrow().left.as_ref().unwrap().clone());
    let right = get_magnitude(number.borrow().right.as_ref().unwrap().clone());

    return 3 * left + 2 * right;
}

fn dfs_traversal(curr: Rc<RefCell<Number>>, depth: i64) -> (Option<Rc<RefCell<Number>>>, i64) {
    if curr.borrow().left.is_some() {
        return (
            Some(curr.borrow().left.as_ref().unwrap().clone()),
            depth + 1,
        );
    }

    let mut traversal = curr.clone();
    let mut new_depth = depth;

    while traversal.borrow().parent.is_some() {
        let from = traversal.clone();

        let next = traversal.borrow().parent.as_ref().unwrap().clone();
        traversal = next;
        new_depth -= 1;

        if traversal.borrow().left == Some(from.clone()) {
            return (
                Some(traversal.borrow().right.as_ref().unwrap().clone()),
                new_depth + 1,
            );
        }
    }

    return (None, new_depth);
}

fn parse_number(line: &str, index: &mut i64) -> Rc<RefCell<Number>> {
    let mut number_stack: Vec<Rc<RefCell<Number>>> = Vec::new();
    let mut val_string: String = String::new();

    for c in line.chars() {
        match c {
            '[' => {
                let new_number = Rc::new(RefCell::new(Number::new()));
                new_number.borrow_mut().index = *index;
                *index += 1;

                if let Some(parent) = number_stack.last() {
                    new_number.borrow_mut().parent = Some(parent.clone());
                }

                number_stack.push(new_number);
            }
            ']' => {
                if val_string.len() > 0 {
                    let val = val_string.parse::<i64>().unwrap();

                    let new_number = Rc::new(RefCell::new(Number::new()));
                    new_number.borrow_mut().index = *index;
                    *index += 1;

                    if let Some(parent) = number_stack.last() {
                        new_number.borrow_mut().parent = Some(parent.clone());
                    }

                    new_number.borrow_mut().magnitude = Some(val);
                    number_stack.push(new_number);

                    val_string = String::new();
                }

                let popped = number_stack.pop().unwrap();

                if let Some(parent) = number_stack.last() {
                    parent.borrow_mut().right = Some(popped.clone());
                }
            }
            ',' => {
                if val_string.len() > 0 {
                    let val = val_string.parse::<i64>().unwrap();

                    let new_number = Rc::new(RefCell::new(Number::new()));
                    new_number.borrow_mut().index = *index;
                    *index += 1;

                    if let Some(parent) = number_stack.last() {
                        new_number.borrow_mut().parent = Some(parent.clone());
                    }

                    new_number.borrow_mut().magnitude = Some(val);
                    number_stack.push(new_number);

                    val_string = String::new();
                }

                let popped = number_stack.pop().unwrap();

                if let Some(parent) = number_stack.last() {
                    parent.borrow_mut().left = Some(popped.clone());
                }
            }
            _ => {
                val_string.push(c);
            }
        }
    }

    return number_stack.pop().unwrap();
}

#[derive(Clone)]
struct Number {
    parent: Option<Rc<RefCell<Number>>>,
    left: Option<Rc<RefCell<Number>>>,
    right: Option<Rc<RefCell<Number>>>,
    magnitude: Option<i64>,
    index: i64,
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.magnitude.is_some() {
            return write!(f, "{:?}", self.magnitude.unwrap());
        } else {
            return write!(
                f,
                "[{:?},{:?}]",
                self.left.clone().unwrap(),
                self.right.clone().unwrap()
            );
        }
    }
}

impl Number {
    fn new() -> Number {
        Number {
            magnitude: None,
            left: None,
            right: None,
            parent: None,
            index: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 4140);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 3993);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "18".to_string();
	
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
        "\nPart 1:\nMagnitude of final sum: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLargest magnitude of any two sums: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}