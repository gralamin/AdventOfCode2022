extern crate filelib;

pub use filelib::load_no_blanks;
use rustc_hash::FxHashSet;
use std::cmp::max;
use std::collections::VecDeque;
use std::thread;
use std::thread::JoinHandle;

// id, ore_robot_cost, clay_robot_cost, obsidian_ore_cost, obsidian_clay_cost, geode_ore_cost, geode_obsidian_cost
type BlueprintTuple = (usize, usize, usize, usize, usize, usize, usize);
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Blueprint {
    id: usize,
    ore_robot_cost: usize,
    clay_robot_cost: usize,
    obsidian_ore_cost: usize,
    obsidian_clay_cost: usize,
    geode_ore_cost: usize,
    geode_obsidian_cost: usize,
}

impl Blueprint {
    fn new(t: BlueprintTuple) -> Blueprint {
        return Blueprint {
            id: t.0,
            ore_robot_cost: t.1,
            clay_robot_cost: t.2,
            obsidian_ore_cost: t.3,
            obsidian_clay_cost: t.4,
            geode_ore_cost: t.5,
            geode_obsidian_cost: t.6,
        };
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct BuildMaterials {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct RobotInventory {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

//std::mem::size_of is 64
#[derive(Default, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct SearchState {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: usize,
    geode_robot: usize,
}


impl SearchState {
    fn new(r: RobotInventory, m: BuildMaterials, g: usize) -> SearchState {
        return SearchState {
            ore: m.ore,
            clay: m.clay,
            obsidian: m.obsidian,
            geode: g,
            ore_robot: r.ore,
            clay_robot: r.clay,
            obsidian_robot: r.obsidian,
            geode_robot: r.geode,
        };
    }

    fn collect(&self) -> SearchState {
        return SearchState {
            ore: self.ore + self.ore_robot,
            clay: self.clay + self.clay_robot,
            obsidian: self.obsidian + self.obsidian_robot,
            geode: self.geode + self.geode_robot,
            ore_robot: self.ore_robot,
            clay_robot: self.clay_robot,
            obsidian_robot: self.obsidian_robot,
            geode_robot: self.geode_robot,
        };
    }
}

// I cache per blueprint, so don't need blueprint in key.
type Cache = FxHashSet<SearchState>;

fn parse_blueprints(lines: &Vec<String>) -> Vec<Blueprint> {
    return lines.iter().map(|line| Blueprint::new(sscanf::sscanf!(line, "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.", usize, usize, usize, usize, usize, usize, usize).unwrap())).collect();
}

fn calculate_quality(b: &Blueprint, num_geodes: usize) -> usize {
    return b.id * num_geodes;
}

fn solve_single_blueprint(
    blueprint: &Blueprint,
    robot_state: RobotInventory,
    start_materials: BuildMaterials,
    num_turns: usize,
) -> usize {
    let mut cache = Cache::default();
    let mut geodes = robot_state.geode;

    if num_turns == 0 {
        return geodes;
    }

    // never a point in generating more of a resource then we need, with the exception of geodes.
    let max_ore_required: usize = max(
        max(
            max(blueprint.ore_robot_cost, blueprint.clay_robot_cost),
            blueprint.obsidian_ore_cost,
        ),
        blueprint.geode_ore_cost,
    );
    let max_clay_required = blueprint.obsidian_clay_cost;
    let max_obsidian_required = blueprint.geode_obsidian_cost;

    let start_state = SearchState::new(robot_state, start_materials, geodes);
    let mut queue: VecDeque<(SearchState, usize)> = VecDeque::new();
    queue.push_back((start_state, num_turns));

    while let Some((mut state, cur_turns)) = queue.pop_front() {
        if cur_turns == 0 {
            geodes = geodes.max(state.geode);
            continue;
        }

        // Heuristics here to cut out states
        // Once we have the maximum number of robots, we can throw away the waste materials to reduce size of problem
        if state.ore_robot >= max_ore_required {
            state.ore = state.ore.min(max_ore_required);
        }
        if state.clay_robot >= max_clay_required {
            state.clay = state.clay.min(max_clay_required);
        }
        if state.obsidian_robot >= max_obsidian_required {
            state.obsidian = state.obsidian.min(max_obsidian_required);
        }

        if cache.contains(&state) {
            // We've already handled this
            // Since we are BFS, its going to be a worse case of a later turn in the same state.
            continue;
        }
        cache.insert(state);

        if cache.len() % 1000000 == 0 {            
            println!("Cache is at size {}, ", cache.len());
            println!("queue is at size {}, ", queue.len());
            if cache.len() + queue.len() >= 100000000 {
                panic!("Cache and queue combined are taking up almost all memory available");
            }
        }

        // You can think of a turn as made of three phases
        // Construction start (pay costs)
        // collection (Gain resources)
        // construction finish (gain robots)

        let new_state_base = state.collect();

        if state.ore_robot < max_ore_required && blueprint.ore_robot_cost <= state.ore {
            let mut new_state = new_state_base.clone();
            new_state.ore -= blueprint.ore_robot_cost;
            new_state.ore_robot += 1;
            queue.push_back((new_state, cur_turns - 1));
        }
        if state.clay_robot < max_clay_required && blueprint.clay_robot_cost <= state.ore {
            let mut new_state = new_state_base.clone();
            new_state.ore -= blueprint.clay_robot_cost;
            new_state.clay_robot += 1;
            queue.push_back((new_state, cur_turns - 1));
        }
        if blueprint.obsidian_clay_cost <= state.clay
            && blueprint.obsidian_ore_cost <= state.ore
            && state.obsidian_robot < max_obsidian_required
        {
            let mut new_state = new_state_base.clone();
            new_state.ore -= blueprint.obsidian_ore_cost;
            new_state.clay -= blueprint.obsidian_clay_cost;
            new_state.obsidian_robot += 1;
            queue.push_back((new_state, cur_turns - 1));
        }
        if blueprint.geode_obsidian_cost <= state.obsidian && blueprint.geode_ore_cost <= state.ore
        {
            let mut new_state = new_state_base.clone();
            new_state.ore -= blueprint.geode_ore_cost;
            new_state.obsidian -= blueprint.geode_obsidian_cost;
            new_state.geode_robot += 1;
            queue.push_back((new_state, cur_turns - 1));
        } else {
            queue.push_back((new_state_base, cur_turns - 1));
        }
    }
    return geodes;
}

/// Solution to puzzle_a entry point
/// ```
/// let vec1: Vec<String> = vec!["Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.",
/// "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day19::puzzle_a(&vec1), 33);
/// ```
pub fn puzzle_a(input: &Vec<String>) -> usize {
    let blueprints = parse_blueprints(input);
    return solve_puzzle_a_state(blueprints, 24);
}

// seperated out for easy testing
fn solve_puzzle_a_state(blueprints: Vec<Blueprint>, num_rounds: usize) -> usize {
    let num_ore_collecting_robots_start = 1;
    let robot_types = RobotInventory {
        ore: num_ore_collecting_robots_start,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    let start_mats = BuildMaterials {
        ore: 0,
        clay: 0,
        obsidian: 0,
    };

    // Use multi threading :D
    let handles: Vec<JoinHandle<usize>> = blueprints
        .into_iter()
        .map(|b| {
            let r = robot_types.clone();
            let s = start_mats.clone();
            return thread::Builder::new()
                .name(format!("blueprint-thread-{}", b.id).to_string())
                .spawn(move || {
                    return calculate_quality(&b, solve_single_blueprint(&b, r, s, num_rounds));
                })
                .unwrap();
        })
        .collect();
    let joins: Vec<usize> = handles.into_iter().map(|t| t.join().unwrap()).collect();
    return joins.iter().sum();
}

/// Solution to puzzle_b entry point
/// ```
/// let vec1: Vec<String> = vec!["Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.",
/// "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day19::puzzle_b(&vec1), 62 * 56);
/// ```
pub fn puzzle_b(input: &Vec<String>) -> usize {
    let mut blueprints: Vec<Blueprint> = parse_blueprints(input);
    if blueprints.len() > 3 {
        blueprints = blueprints[..3].to_vec();
    }
    return solve_puzzle_b_state(blueprints, 32);
}

fn solve_puzzle_b_state(blueprints: Vec<Blueprint>, num_rounds: usize) -> usize {
    let num_ore_collecting_robots_start = 1;
    let robot_types = RobotInventory {
        ore: num_ore_collecting_robots_start,
        clay: 0,
        obsidian: 0,
        geode: 0,
    };
    let start_mats = BuildMaterials {
        ore: 0,
        clay: 0,
        obsidian: 0,
    };

    let handles: Vec<JoinHandle<usize>> = blueprints
        .into_iter()
        .map(|b| {
            let r = robot_types.clone();
            let s = start_mats.clone();
            return thread::Builder::new()
                .name(format!("blueprint-thread-{}", b.id).to_string())
                .spawn(move || {
                    return solve_single_blueprint(&b, r, s, num_rounds);
                })
                .unwrap();
        })
        .collect();
    let joins: Vec<usize> = handles.into_iter().map(|t| t.join().unwrap()).collect();
    return joins.iter().product();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_blueprint() {
        let expected: Vec<Blueprint> = vec![Blueprint::new((2, 2, 3, 3, 8, 3, 12))];
        assert_eq!(parse_blueprints(&vec!["Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.".to_string()]), 
        expected);
    }

    #[test]
    fn test_calculate_quality() {
        assert_eq!(
            calculate_quality(&Blueprint::new((2, 2, 3, 3, 8, 3, 12)), 12),
            24
        );
    }

    #[test]
    fn test_partial_a() {
        let vec1: Vec<String> = vec!["Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".to_string()];
        let blueprints = parse_blueprints(&vec1);
        assert_eq!(solve_puzzle_a_state(blueprints.clone(), 18), 0);
        assert_eq!(solve_puzzle_a_state(blueprints.clone(), 19), 1);
        assert_eq!(solve_puzzle_a_state(blueprints, 24), 9);
    }

    #[test]
    fn test_partial_b() {
        let vec1: Vec<String> = vec!["Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.".to_string()];
        let blueprints = parse_blueprints(&vec1);
        assert_eq!(solve_puzzle_b_state(blueprints, 32), 56);
    }
}
