use nalgebra::{dmatrix, dvector, DMatrix, DVector};
use relative_path::RelativePath;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::time::Instant;

fn part1(contents: String) -> i64 {
    let min_common: usize = 2;
    let mut scanners: Vec<Vec<DVector<i64>>> = Vec::new();

    for line in contents.lines() {
        if line.len() == 0 {
            continue;
        }

        if !line.contains(",") {
            scanners.push(Vec::new());
            continue;
        }

        let beacon: DVector<i64> =
            DVector::from_iterator(3, line.split(',').map(|x| x.parse::<i64>().unwrap()));
        scanners.last_mut().unwrap().push(beacon);
    }

    let (scanner_abs_pos, finished_scanners) = abs_positions(scanners.clone());

    let mut beacons: HashSet<DVector<i64>> = HashSet::new();
    for beacs in finished_scanners {
        let this_beacons: HashSet<DVector<i64>> =
            HashSet::from_iter(beacs.iter().map(|x| x.clone()));
        beacons = beacons.union(&this_beacons).map(|x| x.clone()).collect();
    }


    return beacons.len() as i64;
}

fn part2(contents: String) -> i64 {
    let min_common: usize = 2;
    let mut scanners: Vec<Vec<DVector<i64>>> = Vec::new();

    for line in contents.lines() {
        if line.len() == 0 {
            continue;
        }

        if !line.contains(",") {
            scanners.push(Vec::new());
            continue;
        }

        let beacon: DVector<i64> =
            DVector::from_iterator(3, line.split(',').map(|x| x.parse::<i64>().unwrap()));
        scanners.last_mut().unwrap().push(beacon);
    }

    let (scanner_abs_pos, _) = abs_positions(scanners.clone());

    return scanner_abs_pos
        .iter()
        .map(|x| {
            scanner_abs_pos
                .iter()
                .map(|y| manhat_dist(&x.clone().unwrap(), &y.clone().unwrap()))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
}

fn abs_positions(
    scanners: Vec<Vec<DVector<i64>>>,
) -> (Vec<Option<DVector<i64>>>, Vec<Vec<DVector<i64>>>) {
    let min_common: usize = 2;

    let mut scanner_dists: Vec<HashSet<i64>> = Vec::new();
    let mut between_beacons: Vec<HashMap<DVector<i64>, HashSet<i64>>> = Vec::new();

    for scanner in &scanners {
        let mut between_beacon: HashMap<DVector<i64>, HashSet<i64>> = HashMap::new();
        let mut dists: HashSet<i64> = HashSet::new();

        for i in 0..scanner.len() {
            for j in i + 1..scanner.len() {
                let d = pythag_dist(&scanner[i], &scanner[j]);
                dists.insert(d);
                between_beacon
                    .entry(scanner[i].clone())
                    .or_insert(HashSet::new())
                    .insert(d);
                between_beacon
                    .entry(scanner[j].clone())
                    .or_insert(HashSet::new())
                    .insert(d);
            }
        }

        scanner_dists.push(dists);
        between_beacons.push(between_beacon);
    }

    let mut new_scanners = scanners.clone();
    let mut scanner_abs_pos: Vec<Option<DVector<i64>>> = vec![None; scanners.len()];
    scanner_abs_pos[0] = Some(dvector![0, 0, 0]);

    while scanner_abs_pos.iter().any(|x| x.is_none()) {
        for f_ix in 0..scanner_abs_pos.len() {
            if scanner_abs_pos[f_ix].is_none() {
                continue;
            }

            for nf_ix in 0..scanner_abs_pos.len() {
                if scanner_abs_pos[nf_ix].is_some() {
                    continue;
                }

                let nf = &new_scanners[nf_ix];
                let nf_dists = &scanner_dists[nf_ix];
                let f_dists = &scanner_dists[f_ix];
                let nf_btw = &between_beacons[nf_ix];
                let f_btw = &between_beacons[f_ix];

                let mut abs_rel: HashMap<DVector<i64>, DVector<i64>> = HashMap::new();
                let same_dists: HashSet<i64> = nf_dists.intersection(f_dists).map(|d| *d).collect();
                for (abs_b, abs_set) in f_btw.iter() {
                    let abs_same: HashSet<i64> =
                        abs_set.intersection(&same_dists).map(|d| *d).collect();
                    for (rel_b, rel_set) in nf_btw.iter() {
                        if abs_same.intersection(rel_set).count() > 1 {
                            abs_rel.insert(abs_b.clone(), rel_b.clone());
                        }
                    }
                }

                if abs_rel.len() >= min_common {
                    let (abs_beac1, rel_beac1) = abs_rel.iter().nth(0).unwrap();
                    let (abs_beac2, rel_beac2) = abs_rel.iter().nth(1).unwrap();

                    let rots: Vec<DMatrix<i64>> = vec![
                        dmatrix![1, 0, 0; 0, 1, 0; 0, 0, 1],
                        dmatrix![0, 0, 1; 0, 1, 0; -1, 0, 0],
                        dmatrix![-1, 0, 0; 0, 1, 0; 0, 0, -1],
                        dmatrix![0, 0, -1; 0, 1, 0; 1, 0, 0],
                        dmatrix![0, -1, 0; 1, 0, 0; 0, 0, 1],
                        dmatrix![0, 0, 1; 1, 0, 0; 0, 1, 0],
                        dmatrix![0, 1, 0; 1, 0, 0; 0, 0, -1],
                        dmatrix![0, 0, -1; 1, 0, 0; 0, -1, 0],
                        dmatrix![0, 1, 0; -1, 0, 0; 0, 0, 1],
                        dmatrix![0, 0, 1; -1, 0, 0; 0, -1, 0],
                        dmatrix![0, -1, 0; -1, 0, 0; 0, 0, -1],
                        dmatrix![0, 0, -1; -1, 0, 0; 0, 1, 0],
                        dmatrix![1, 0, 0; 0, 0, -1; 0, 1, 0],
                        dmatrix![0, 1, 0; 0, 0, -1; -1, 0, 0],
                        dmatrix![-1, 0, 0; 0, 0, -1; 0, -1, 0],
                        dmatrix![0, -1, 0; 0, 0, -1; 1, 0, 0],
                        dmatrix![1, 0, 0; 0, -1, 0; 0, 0, -1],
                        dmatrix![0, 0, -1; 0, -1, 0; -1, 0, 0],
                        dmatrix![-1, 0, 0; 0, -1, 0; 0, 0, 1],
                        dmatrix![0, 0, 1; 0, -1, 0; 1, 0, 0],
                        dmatrix![1, 0, 0; 0, 0, 1; 0, -1, 0],
                        dmatrix![0, -1, 0; 0, 0, 1; -1, 0, 0],
                        dmatrix![-1, 0, 0; 0, 0, 1; 0, 1, 0],
                        dmatrix![0, 1, 0; 0, 0, 1; 1, 0, 0],
                    ];

                    for r in rots {
                        let r1: DVector<i64> = r.clone() * rel_beac1.clone();
                        let r2: DVector<i64> = r.clone() * rel_beac2.clone();

                        let a1: DVector<i64> = abs_beac1.clone() - r1;
                        let a2: DVector<i64> = abs_beac2.clone() - r2;

                        if a1 == a2 {
                            let new_btw: HashMap<DVector<i64>, HashSet<i64>> = HashMap::from_iter(
                                nf_btw
                                    .iter()
                                    .map(|(k, v)| (r.clone() * k.clone() + a1.clone(), v.clone())),
                            );

                            let new_beacs: Vec<DVector<i64>> = nf
                                .iter()
                                .map(|x| r.clone() * x.clone() + a1.clone())
                                .collect();

                            between_beacons[nf_ix] = new_btw;
                            new_scanners[nf_ix] = new_beacs;
                            scanner_abs_pos[nf_ix] = Some(a1);
                            break;
                        }
                    }
                }
            }
        }
    }

    return (scanner_abs_pos, new_scanners);
}

fn pythag_dist(a: &DVector<i64>, b: &DVector<i64>) -> i64 {
    return (a[0] - b[0]).pow(2) + (a[1] - b[1]).pow(2) + (a[2] - b[2]).pow(2);
}

fn manhat_dist(a: &DVector<i64>, b: &DVector<i64>) -> i64 {
    return (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part1(contents), 79);
    }

    #[test]
    fn p2_test() {
        let contents =
            fs::read_to_string("example.txt").expect("Should have been able to read the file");

        assert_eq!(part2(contents), 3621);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
	let year = "2021".to_string();
	let day = "19".to_string();
	
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
        "\nPart 1:\nNumber of beacons: {}\nRan in {:.5?}",
        part1(contents.clone()),
        part1_timer.elapsed()
    );

    let part2_timer = Instant::now();
    println!(
        "\nPart 2:\nDistance between farthest apart scanners: {}\nRan in {:.5?}",
        part2(contents.clone()),
        part2_timer.elapsed()
    );
}