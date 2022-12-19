extern crate filelib;

pub use filelib::load_no_blanks;
use std::cmp::max;
use std::collections::VecDeque;
use std::thread;
use std::thread::JoinHandle;
use rustc_hash::FxHashSet;

// id, ore_robot_cost, clay_robot_cost, obsidian_ore_cost, obsidian_clay_cost, geode_ore_cost, geode_obsidian_cost
type BlueprintTuple = (usize, usize, usize, usize, usize, usize, usize);
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct BuildMaterials {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct RobotInventory {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

// I cache per blueprint, so don't need blueprint in key.
type Cache = FxHashSet<(RobotInventory, BuildMaterials, usize)>;

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
    cache: &mut Cache,
) -> usize {
    let mut geodes = robot_state.geode;

    if num_turns == 0 {
        return geodes;
    }

    let mut queue: VecDeque<(RobotInventory, BuildMaterials, usize, usize)> = VecDeque::new();
    queue.push_back((robot_state, start_materials, geodes, num_turns));

    while let Some((cur_robot_state, cur_materials, cur_geodes, cur_turns)) = queue.pop_front() {
        if cur_turns == 0 {
            geodes = geodes.max(cur_geodes);
            continue;
        }
        let cache_key = (cur_robot_state, cur_materials, cur_geodes);
        if cache.contains(&cache_key) {
            // We've already handled this
            // Since we are BFS, its going to be a worse case of a later turn in the same state.
            continue;
        }
        cache.insert(cache_key);

        // You can think of a turn as made of three phases
        // Construction start (pay costs)
        // collection (Gain resources)
        // construction finish (gain robots)
        let build_queue =
            get_possible_single_robots_to_build(blueprint, &cur_materials, &cur_robot_state);

        for (possible_build, possible_material_state) in build_queue {
            let (next_mats, add_geos) = collect_phase(cur_robot_state, possible_material_state);
            let possible_robot = RobotInventory {
                ore: possible_build.ore + cur_robot_state.ore,
                clay: possible_build.clay + cur_robot_state.clay,
                obsidian: possible_build.obsidian + cur_robot_state.obsidian,
                geode: possible_build.geode + cur_robot_state.geode,
            };
            queue.push_back((
                possible_robot,
                next_mats,
                cur_geodes + add_geos,
                cur_turns - 1,
            ));
        }
    }
    return geodes;
}

fn collect_phase(
    robot_state: RobotInventory,
    start_materials: BuildMaterials,
) -> (BuildMaterials, usize) {
    let mut next_material_state = start_materials.clone();
    next_material_state.ore += robot_state.ore;
    next_material_state.clay += robot_state.clay;
    next_material_state.obsidian += robot_state.obsidian;
    return (next_material_state, robot_state.geode);
}

fn get_possible_single_robots_to_build(
    blueprint: &Blueprint,
    materials: &BuildMaterials,
    robot_filter: &RobotInventory,
) -> Vec<(RobotInventory, BuildMaterials)> {
    let mut possiblities: Vec<(RobotInventory, BuildMaterials)> = vec![];

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

    if blueprint.ore_robot_cost <= materials.ore && robot_filter.ore < max_ore_required {
        possiblities.push((
            RobotInventory {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            BuildMaterials {
                ore: materials.ore - blueprint.ore_robot_cost,
                clay: materials.clay,
                obsidian: materials.obsidian,
            },
        ));
    }
    if blueprint.clay_robot_cost <= materials.ore && robot_filter.clay < max_clay_required {
        possiblities.push((
            RobotInventory {
                ore: 0,
                clay: 1,
                obsidian: 0,
                geode: 0,
            },
            BuildMaterials {
                ore: materials.ore - blueprint.clay_robot_cost,
                clay: materials.clay,
                obsidian: materials.obsidian,
            },
        ));
    }
    if blueprint.obsidian_clay_cost <= materials.clay
        && blueprint.obsidian_ore_cost <= materials.ore
        && robot_filter.obsidian < max_obsidian_required
    {
        possiblities.push((
            RobotInventory {
                ore: 0,
                clay: 0,
                obsidian: 1,
                geode: 0,
            },
            BuildMaterials {
                ore: materials.ore - blueprint.obsidian_ore_cost,
                clay: materials.clay - blueprint.obsidian_clay_cost,
                obsidian: materials.obsidian,
            },
        ));
    }
    if blueprint.geode_obsidian_cost <= materials.obsidian
        && blueprint.geode_ore_cost <= materials.ore
    {
        possiblities.push((
            RobotInventory {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 1,
            },
            BuildMaterials {
                ore: materials.ore - blueprint.geode_ore_cost,
                clay: materials.clay,
                obsidian: materials.obsidian - blueprint.geode_obsidian_cost,
            },
        ));
    } else {
        // Optimization, handle the no build case here
        // Never want to do it if we could build a geode, as building a geode will always be better... I think.
        possiblities.push((
            RobotInventory {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            BuildMaterials {
                ore: materials.ore,
                clay: materials.clay,
                obsidian: materials.obsidian,
            },
        ));
    }
    return possiblities;
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
            return thread::Builder::new()
                .name(format!("blueprint-thread-{}", b.id).to_string())
                .spawn(move || {
                    let mut cache = Cache::default();
                    return calculate_quality(
                        &b,
                        solve_single_blueprint(&b, robot_types, start_mats, num_rounds, &mut cache),
                    );
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
            return thread::Builder::new()
                .name(format!("blueprint-thread-{}", b.id).to_string())
                .spawn(move || {
                    let mut cache = Cache::default();
                    return solve_single_blueprint(
                        &b,
                        robot_types,
                        start_mats,
                        num_rounds,
                        &mut cache,
                    );
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
