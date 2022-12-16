extern crate filelib;

pub use filelib::load_no_blanks;
use rustc_hash::FxHashMap;

pub fn sample_input() -> Vec<String> {
    return vec![
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
        "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
        "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
        "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
        "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
        "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
        "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
        "Valve HH has flow rate=22; tunnel leads to valve GG",
        "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
        "Valve JJ has flow rate=21; tunnel leads to valve II",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
}

fn parse_valve(line: &str) -> (&str, usize, Vec<&str>) {
    let (valve_str, tunnel_str) = match line.split_once("; tunnels lead to valves ") {
        Some((v, t)) => (v, t),
        None => line.split_once("; tunnel leads to valve ").unwrap(),
    };
    let (valve_name_str, flow_rate_unparsed) = valve_str.split_once(" has flow rate=").unwrap();
    let (_, valve_name_unparsed) = valve_name_str.split_once("Valve ").unwrap();

    let tunnels: Vec<&str> = tunnel_str.split(", ").collect();
    let flow_rate = flow_rate_unparsed.parse::<usize>().unwrap();
    return (valve_name_unparsed.trim(), flow_rate, tunnels);
}

fn parse_input(lines: &Vec<String>) -> Vec<(&str, usize, Vec<&str>)> {
    return lines.iter().map(|s| parse_valve(s)).collect();
}

fn solve(valves: &mut Vec<(&str, usize, Vec<&str>)>, max_time: usize, start: &str) -> usize {
    valves.sort_by(|a, b| b.1.cmp(&a.1));
    let label_indexes = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0, i))
        .collect::<FxHashMap<_, _>>();
    let num_positive_flow_rate = valves.iter().filter(|v| v.1 > 0).count();
    let num_valves = valves.len();
    
    // adjacency map
    let mut adj = vec![vec![0usize; 0]; num_valves];
    let mut flow = vec![0usize; num_valves];
    for valve in valves.iter() {
        let i = label_indexes[valve.0];
        flow[i] = valve.1;
        for w in valve.2.iter() {
            adj[i].push(label_indexes[w]);
        }
    }
    let start_index = label_indexes[start];
    let positive_bitset = 1 << num_positive_flow_rate;

    // dynamic programming, via 3 dimensional array [time left, current node, bitset of available valves]
    let mut opt = vec![vec![vec![0; max_time]; num_valves]; positive_bitset];
    for time in 1..max_time {
        for valve_index in 0..num_valves {
            let cur_bitmask = 1 << valve_index;
            for x in 0..positive_bitset {
                let mut o = opt[x][valve_index][time];
                if cur_bitmask & x != 0 && time >= 2 {
                    o = o.max(opt[x - cur_bitmask][valve_index][time - 1] + flow[valve_index] * time as usize);
                }
                for &j in adj[valve_index].iter() {
                    o = o.max(opt[x][j][time - 1]);
                }
                opt[x][valve_index][time] = o;
            }
        }
    }

    return opt[positive_bitset - 1][start_index][max_time - 1];
}


/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = day16::sample_input();
/// assert_eq!(day16::puzzle_a(&vec1), 1651);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> usize {
    let max_time = 30;
    let start = "AA";
    let mut valves = parse_input(input);
    return solve(&mut valves, max_time, start);
}

fn solve_elephant(valves: &mut Vec<(&str, usize, Vec<&str>)>, max_time: usize, start: &str) -> usize {
    let elephant_time = 4;
    valves.sort_by(|a, b| b.1.cmp(&a.1));
    let label_indexes = valves
        .iter()
        .enumerate()
        .map(|(i, v)| (v.0, i))
        .collect::<FxHashMap<_, _>>();
    let num_positive_flow_rate = valves.iter().filter(|v| v.1 > 0).count();
    let num_valves = valves.len();
    
    // adjacency map
    let mut adj = vec![vec![0usize; 0]; num_valves];
    let mut flow = vec![0usize; num_valves];
    for valve in valves.iter() {
        let i = label_indexes[valve.0];
        flow[i] = valve.1;
        for w in valve.2.iter() {
            adj[i].push(label_indexes[w]);
        }
    }
    let start_index = label_indexes[start];
    let positive_bitset = 1 << num_positive_flow_rate;

    // dynamic programming, via 3 dimensional array [time left, current node, bitset of available valves]
    let mut opt = vec![vec![vec![0; max_time]; num_valves]; positive_bitset];
    for time in 1..max_time {
        for valve_index in 0..num_valves {
            let cur_bitmask = 1 << valve_index;
            for x in 0..positive_bitset {
                let mut o = opt[x][valve_index][time];
                if cur_bitmask & x != 0 && time >= 2 {
                    o = o.max(opt[x - cur_bitmask][valve_index][time - 1] + flow[valve_index] * time as usize);
                }
                for &j in adj[valve_index].iter() {
                    o = o.max(opt[x][j][time - 1]);
                }
                opt[x][valve_index][time] = o;
            }
        }
    }

    let mut best = 0;
    let ret_time = max_time - elephant_time - 1;
    for x in 0..positive_bitset / 2 {
        let y = positive_bitset - 1 - x;
        // split in half, y is elephant, x is you.
        best = best.max(opt[x][start_index][ret_time] + opt[y][start_index][ret_time]);
    }
    return best;
}


/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = day16::sample_input();
/// assert_eq!(day16::puzzle_b(&vec1), 1707);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> usize {
    let max_time = 30;
    let start = "AA";
    let mut valves = parse_input(input);
    println!("This answer is actually off by 8, but not sure whats wrong.");
    println!("Need to figure out why");
    return solve_elephant(&mut valves, max_time, start);
}
