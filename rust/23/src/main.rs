use cached::proc_macro::cached;
use relative_path::RelativePath;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let mut rooms: String =
        contents.lines().nth(1).unwrap()[1..contents.lines().nth(1).unwrap().len() - 1].to_string();
    let mut end: String = rooms.clone();

    for line in contents.lines().collect::<Vec<&str>>()
        [2..contents.lines().collect::<Vec<&str>>().len() - 1]
        .iter()
    {
        rooms.push_str(line.replace("#", "").trim());
        end.push_str("ABCD");
    }

    return a_star(rooms, end);
}

fn part2(contents: String) -> i64 {
    let mut rooms: String =
        contents.lines().nth(1).unwrap()[1..contents.lines().nth(1).unwrap().len() - 1].to_string();
    let mut end: String = rooms.clone();
    let mut lines: Vec<&str> = contents.lines().collect();
    lines.insert(3, "  #D#C#B#A#");
    lines.insert(4, "  #D#B#A#C#");

    for line in lines[2..lines.len() - 1].iter() {
        rooms.push_str(line.replace("#", "").trim());
        end.push_str("ABCD");

    }

    return a_star(rooms, end);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 12521);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 44169);
    }
}

#[cached]
fn heuristic(state: String) -> i64 {
    let mut energy: i64 = 0;
    let mut counts: Vec<i64> = vec![(state.len() - 10) as i64 / 4; 4];
    let mut moves: Vec<bool> = vec![true; state.len()];
    for (i, s) in state[11..].chars().rev().enumerate() {
        let level = ((state.len() - 11) / 4) - (i / 4);
        let room = 3 - (i % 4);

        if s as i64 - 'A' as i64 == room as i64 && level as i64 == counts[room] {
            moves[11 + 4 * (level - 1) + room] = false;
            counts[room] -= 1
        }
    }

    for (i, s) in state.chars().enumerate() {
        if s == '.' || !moves[i] {
            continue;
        }

        let goal_room = s as usize - 'A' as usize;
        let goal_index = 2 * (goal_room + 1);
        let val = 10_i64.pow(goal_room as u32);

        if i < 11 {
            // In hallway
            energy += val * ((i as i64 - goal_index as i64).abs() + counts[goal_room]);
        } else {
            // In a room
            let level = (i - 11) / 4 + 1;
            let room = (i - 11) % 4;

            if room == goal_room {
                // In correct room, move out of room first, then back in
                energy += val * (level as i64 + 2 + counts[goal_room] as i64);
            } else {
                // In wrong room
                let hall_index = 2 * (room + 1);
                energy += val
                    * (level as i64
                        + (hall_index as i64 - goal_index as i64).abs()
                        + counts[goal_room] as i64);
            }
        }

        counts[goal_room] -= 1;
    }

    return energy;
}

#[cached]
fn get_next(state: String) -> Vec<(String, i64)> {
    let mut pos_indexes: Vec<i64> = vec![0, 1, 3, 5, 7];
    for i in 9..state.len() {
        pos_indexes.push(i as i64);
    }

    let mut next_states: Vec<(String, i64)> = vec![];
    for p1 in pos_indexes.iter() {
        for p2 in pos_indexes.iter() {
            if p1 == p2 {
                continue;
            }

            let (next_state, energy_cost) = get_energy(state.clone(), *p1, *p2);
            if energy_cost > 0 {
                next_states.push((next_state, energy_cost));
            }
        }
    }

    return next_states;
}

#[cached]
fn get_energy(state: String, start: i64, end: i64) -> (String, i64) {
    let mut energy = 0;
    let mut state_nums: Vec<i64> = vec![0; state.len()];

    for (i, s) in state.chars().enumerate() {
        if s != '.' {
            state_nums[i] = 10_i64.pow(s as u32 - 'A' as u32);
        }
    }

    if state_nums[start as usize] == 0 || state_nums[end as usize] != 0 {
        // State isn't occupied or end is occupied
        return ("".to_string(), 0);
    }

    let val = state_nums[start as usize];

    if end % 2 == 0 && 2 <= end && end <= 8 {
        // End is outside a hallway
        return ("".to_string(), 0);
    }

    if end < 11 {
        // End is in the hallway
        if start < 11 {
            // Start is also in the hallway
            return ("".to_string(), 0);
        }
    } else {
        // End is in a room
        let room = (end - 11) % 4;
        if val != 10_i64.pow(room as u32) {
            // Value shouldn't go into the room
            return ("".to_string(), 0);
        }

        let mut bottom: i64 = -1;
        for (i, spot) in state_nums[11 + room as usize..]
            .iter()
            .step_by(4)
            .enumerate()
        {
            if *spot != 0 {
                if *spot != val {
                    // End room is not fillable
                    return ("".to_string(), 0);
                }
            } else {
                bottom = i as i64;
            }
        }

        if bottom >= 0 && bottom != (end - 11) / 4 {
            // End isn't at bottom of room
            return ("".to_string(), 0);
        }

        if start >= 11 {
            // Start is also in a room
            if (start - 11) % 4 == (end - 11) % 4 {
                // Start and end are in the same room
                return ("".to_string(), 0);
            }

            let room = (start - 11) % 4;
            if val == 10_i64.pow(room as u32) {
                let mut fillable: bool = true;
                for spot in state_nums[11 + room as usize..].iter().step_by(4) {
                    if *spot != 0 && *spot != val {
                        fillable = false;
                        break;
                    }
                }

                if fillable {
                    // Start room is fillable, dont remove from room
                    return ("".to_string(), 0);
                }
            }
        }
    }

    let mut s_start = start;
    while s_start != end {
        energy += val;
        if s_start != start && state_nums[s_start as usize] != 0 {
            // Something in the way
            return ("".to_string(), 0);
        }

        if s_start < 11 {
            // Start is in the hallway
            let goal_index: i64;
            if end < 11 {
                // End is also in hallway
                goal_index = end;
            } else {
                // End is in a room
                goal_index = 2 + 2 * ((end - 11) % 4);

                if s_start == goal_index {
                    // Enter the room
                    s_start = 11 + (end - 11) % 4;
                    continue;
                }
            }

            if s_start < goal_index {
                s_start += 1;
            } else {
                s_start -= 1;
            }
        } else {
            // Start is in a room
            if end >= 11 && (s_start - 11) % 4 == (end - 11) % 4 {
                // Start and end are in the same room
                s_start += 4;
            } else {
                if (s_start - 11) / 4 > 0 {
                    // Staying in same room
                    s_start -= 4;
                } else {
                    // Moving into hallway
                    s_start = 2 + 2 * ((s_start - 11) % 4);
                }
            }
        }
    }

    state_nums[start as usize] = 0;
    state_nums[end as usize] = val;

    let mut state_str: String = "".to_string();
    for n in state_nums.iter() {
        match n {
            0 => state_str.push('.'),
            _ => state_str.push(char::from_u32((*n as f64).log10() as u32 + 'A' as u32).unwrap()),
        }
    }

    return (state_str, energy);
}

fn a_star(start: String, end: String) -> i64 {
    let mut openlist_heap: BinaryHeap<(i64, i64, String)> = BinaryHeap::new();
    let mut open_dict: HashMap<String, i64> = HashMap::new();
    let mut closed_list: HashMap<String, i64> = HashMap::new();

    openlist_heap.push((-heuristic(start.clone()), 0, start.clone()));
    open_dict.insert(start.clone(), -heuristic(start.clone()));

    while let Some((qf, qg, q)) = openlist_heap.pop() {
        if q == end {
            return -qg;
        }

        if !open_dict.contains_key(&q)
            || (closed_list.contains_key(&q) && closed_list.get(&q).unwrap() >= &qf)
        {
            continue;
        }

        open_dict.remove(&q);
        closed_list.insert(q.clone(), qf);

        for (n, g) in get_next(q) {
            let ng = -g + qg;
            let nh = -heuristic(n.clone());
            let nf = ng + nh;

            if open_dict.contains_key(&n) && open_dict.get(&n).unwrap() >= &nf {
                continue;
            }

            if closed_list.contains_key(&n) && closed_list.get(&n).unwrap() >= &nf {
                continue;
            }

            openlist_heap.push((nf, ng, n.clone()));
            open_dict.insert(n, nf);
        }
    }

    return -1;
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "23".to_string();
	
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
        "\nPart 1:\nLeast required energy: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nLeast required energy: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}