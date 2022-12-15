extern crate filelib;

pub use filelib::load_no_blanks;

fn parse_input(input: &Vec<String>) -> Vec<(i32, i32, i32, i32)> {
    return input.iter().map(|line| parse_line(line)).collect();
}

fn parse_line(line: &str) -> (i32, i32, i32, i32) {
    let (sensor_string, beacon_string) = line.split_once(": ").unwrap();
    let (sensor_x_string, sensor_y_unparsed) = sensor_string.split_once(", y=").unwrap();
    let (_, sensor_x_unparsed) = sensor_x_string.split_once("x=").unwrap();

    let (beacon_x_string, beacon_y_unparsed) = beacon_string.split_once(", y=").unwrap();
    let (_, beacon_x_unparsed) = beacon_x_string.split_once("x=").unwrap();

    let sx = sensor_x_unparsed.parse::<i32>().unwrap();
    let sy = sensor_y_unparsed.parse::<i32>().unwrap();
    let bx = beacon_x_unparsed.parse::<i32>().unwrap();
    let by = beacon_y_unparsed.parse::<i32>().unwrap();

    return (sx, sy, bx, by);
}

pub fn get_sample_input() -> Vec<String> {
    return vec![
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn get_manhatten_distance(px: i32, py: i32, sx: i32, sy: i32) -> usize {
    // Get manhatten distance from sx, sy to px, py
    let v = (px - sx).abs() + (py - sy).abs();
    return v.try_into().unwrap();
}

fn higher(a: i32, b: i32) -> i32 {
    if a > b {
        return a;
    }
    return b;
}

fn lower(a: i32, b: i32) -> i32 {
    if a > b {
        return b;
    }
    return a;
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = day15::get_sample_input();
/// assert_eq!(day15::puzzle_a(&vec1, 10), 26);
/// ```
pub fn puzzle_a(input: &Vec<String>, answer_row: i32) -> usize {
    let sensors_beacons = parse_input(input);
    // first get all the distances
    let distances: Vec<usize> = sensors_beacons
        .iter()
        .map(|(sx, sy, bx, by)| get_manhatten_distance(*bx, *by, *sx, *sy))
        .collect();
    let max_distance_u: usize = *distances.iter().max().unwrap();
    let max_distance: i32 = max_distance_u.try_into().unwrap();
    // get the furthest left x, and furthest right x, to figure out the "width" of the map
    // then adjust by the max distances, in case some are near an edge
    let map_neg_x = sensors_beacons
        .iter()
        .map(|(sx, _, bx, _)| lower(*sx, *bx))
        .min()
        .unwrap()
        - max_distance;
    let map_pos_x = sensors_beacons
        .iter()
        .map(|(sx, _, bx, _)| higher(*sx, *bx))
        .max()
        .unwrap()
        + max_distance;
    let mut impossible_xs = vec![];
    for i in map_neg_x..=map_pos_x {
        //println!("{}", i);
        let matching: Vec<bool> = sensors_beacons
            .iter()
            .map(|(sx, sy, bx, by)| {
                (*sy == answer_row && *sx == i) || (*by == answer_row && *bx == i)
            })
            .filter(|b| *b)
            .collect();
        if matching.len() >= 1 {
            continue;
        }
        // get the distance to this point from every sensor
        let cur_distances: Vec<usize> = sensors_beacons
            .iter()
            .map(|(sx, sy, _, _)| get_manhatten_distance(i, answer_row, *sx, *sy))
            .collect();
        for j in 0..cur_distances.len() {
            if cur_distances[j] <= distances[j] {
                impossible_xs.push(i);
                break;
            }
        }
    }
    return impossible_xs.len();
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = day15::get_sample_input();
/// assert_eq!(day15::puzzle_b(&vec1, 20), 56000011);
/// ```
pub fn puzzle_b(input: &Vec<String>, max_coord: i32) -> usize {
    let sensors_beacons = parse_input(input);
    // first get all the distances
    let distances: Vec<usize> = sensors_beacons
        .iter()
        .map(|(sx, sy, bx, by)| get_manhatten_distance(*bx, *by, *sx, *sy))
        .collect();

    let min_coord = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    // check each sensor for possible spots, it needs to be essentially one outside of an existing beacon and not impossible.
    // so we can search out from each sensor.
    // break label, can break this label specifically.
    'solution: for i in 0..sensors_beacons.len() {
        let (sx, sy, _, _) = sensors_beacons[i];
        let d = distances[i];
        let id: i32 = d.try_into().unwrap();
        for p in 0..=d + 1 {
            let ip: i32 = p.try_into().unwrap();
            // possible diamond spots, roughly, you can figure it out on paper.
            let possible_spots = vec![
                (sx - id - 1 + ip, sy - ip),
                (sx + id + 1 - ip, sy - ip),
                (sx - id - 1 + ip, sy + ip),
                (sx + id + 1 - ip, sy + ip),
            ];
            'next_candidate: for (tx, ty) in possible_spots {
                if tx < min_coord || tx > max_coord || ty < min_coord || ty > max_coord {
                    continue;
                }
                // check if this is possible
                for j in 0..sensors_beacons.len() {
                    let (ox, oy, _, _) = sensors_beacons[j];
                    let od = distances[j];
                    let new_d = get_manhatten_distance(ox, oy, tx, ty);
                    if new_d <= od {
                        continue 'next_candidate;
                    }
                }
                // its possible!
                x = tx;
                y = ty;
                // one solution, just break
                break 'solution;
            }
        }
    }

    let ux: usize = x.try_into().unwrap();
    let uy: usize = y.try_into().unwrap();

    let frequency = ux * 4000000 + uy;

    return frequency;
}
