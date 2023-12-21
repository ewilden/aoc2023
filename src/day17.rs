use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use petgraph::{algo::dijkstra, Graph};

#[aoc_generator(day17)]
fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| format!("{c}").parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn all() -> [Dir; 4] {
        use Dir::*;
        [Up, Right, Down, Left]
    }

    fn apply(&self, (row, col): (i64, i64)) -> (i64, i64) {
        match self {
            Dir::Up => (row - 1, col),
            Dir::Right => (row, col + 1),
            Dir::Down => (row + 1, col),
            Dir::Left => (row, col - 1),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Dir::Up => Dir::Down,
            Dir::Right => Dir::Left,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    facing: Dir,
    times_traveled: u8,
    loc: (i64, i64),
}

impl State {
    fn neighbors_and_costs<'a>(
        &'a self,
        grid: &'a HashMap<(i64, i64), i64>,
    ) -> impl IntoIterator<Item = (State, i64)> + 'a {
        let Self {
            facing,
            times_traveled,
            loc,
        } = *self;
        std::iter::empty()
            .chain((times_traveled < 3).then_some((facing, times_traveled + 1)))
            .chain(Dir::all().into_iter().filter_map(move |dir| {
                (dir != facing && dir != facing.opposite()).then_some((dir, 1))
            }))
            .filter_map(move |(facing, times_traveled)| {
                let next_loc = facing.apply(loc);
                grid.get(&next_loc).map(|cost| {
                    (
                        Self {
                            facing,
                            times_traveled,
                            loc: next_loc,
                        },
                        *cost,
                    )
                })
            })
    }

    fn neighbors_and_costs_part2<'a>(
        &'a self,
        grid: &'a HashMap<(i64, i64), i64>,
    ) -> impl IntoIterator<Item = (State, i64)> + 'a {
        let Self {
            facing,
            times_traveled,
            loc,
        } = *self;
        std::iter::empty()
            .chain((times_traveled < 10).then_some((facing, times_traveled + 1)))
            .chain(Dir::all().into_iter().filter_map(move |dir| {
                (dir != facing && dir != facing.opposite() && times_traveled >= 4)
                    .then_some((dir, 1))
            }))
            .filter_map(move |(facing, times_traveled)| {
                let next_loc = facing.apply(loc);
                grid.get(&next_loc).map(|cost| {
                    (
                        Self {
                            facing,
                            times_traveled,
                            loc: next_loc,
                        },
                        *cost,
                    )
                })
            })
    }
}

#[aoc(day17, part1)]
fn part1(input: &[Vec<i64>]) -> i64 {
    let grid = input
        .into_iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.into_iter().enumerate().map(move |(c, cost)| {
                (
                    (i64::try_from(r).unwrap(), i64::try_from(c).unwrap()),
                    *cost,
                )
            })
        })
        .collect::<HashMap<_, _>>();
    let num_rows = i64::try_from(input.len()).unwrap();
    let num_cols = i64::try_from(input[0].len()).unwrap();

    let mut node_weight_to_node = HashMap::new();

    let mut graph = Graph::<State, i64>::new();
    for r in 0i64..num_rows {
        for c in 0i64..num_cols {
            for times_traveled in 0u8..=3u8 {
                for facing in Dir::all() {
                    let state: State = State {
                        facing,
                        times_traveled,
                        loc: (r, c),
                    };
                    let node = graph.add_node(state);
                    assert!(node_weight_to_node.insert(state, node).is_none());
                }
            }
        }
    }

    for r in 0i64..num_rows {
        for c in 0i64..num_cols {
            for times_traveled in 0u8..=3u8 {
                for facing in Dir::all() {
                    let state: State = State {
                        facing,
                        times_traveled,
                        loc: (r, c),
                    };
                    for (neighbor, cost) in state.neighbors_and_costs(&grid) {
                        graph.add_edge(
                            *node_weight_to_node.get(&state).unwrap(),
                            *node_weight_to_node.get(&neighbor).unwrap(),
                            cost,
                        );
                    }
                }
            }
        }
    }

    let costs = dijkstra(
        &graph,
        *node_weight_to_node
            .get(&State {
                facing: Dir::Right,
                times_traveled: 0,
                loc: (0, 0),
            })
            .unwrap(),
        None,
        |e| *e.weight(),
    );

    let end_nodes = {
        let mut end_nodes = Vec::new();
        for times_traveled in 0u8..=3u8 {
            for facing in Dir::all() {
                let state: State = State {
                    facing,
                    times_traveled,
                    loc: (num_rows - 1, num_cols - 1),
                };
                end_nodes.push(*node_weight_to_node.get(&state).unwrap());
            }
        }
        end_nodes
    };

    *end_nodes
        .into_iter()
        .flat_map(|node| costs.get(&node))
        .min()
        .unwrap()
}

#[aoc(day17, part2)]
fn part2(input: &[Vec<i64>]) -> i64 {
    let grid = input
        .into_iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.into_iter().enumerate().map(move |(c, cost)| {
                (
                    (i64::try_from(r).unwrap(), i64::try_from(c).unwrap()),
                    *cost,
                )
            })
        })
        .collect::<HashMap<_, _>>();
    let num_rows = i64::try_from(input.len()).unwrap();
    let num_cols = i64::try_from(input[0].len()).unwrap();

    let mut node_weight_to_node = HashMap::new();

    let mut graph = Graph::<State, i64>::new();
    for r in 0i64..num_rows {
        for c in 0i64..num_cols {
            for times_traveled in 0u8..=10 {
                for facing in Dir::all() {
                    let state: State = State {
                        facing,
                        times_traveled,
                        loc: (r, c),
                    };
                    let node = graph.add_node(state);
                    assert!(node_weight_to_node.insert(state, node).is_none());
                }
            }
        }
    }

    for r in 0i64..num_rows {
        for c in 0i64..num_cols {
            for times_traveled in 0u8..=10 {
                for facing in Dir::all() {
                    let state: State = State {
                        facing,
                        times_traveled,
                        loc: (r, c),
                    };
                    for (neighbor, cost) in state.neighbors_and_costs_part2(&grid) {
                        graph.add_edge(
                            *node_weight_to_node.get(&state).unwrap(),
                            *node_weight_to_node.get(&neighbor).unwrap(),
                            cost,
                        );
                    }
                }
            }
        }
    }

    let costs = dijkstra(
        &graph,
        *node_weight_to_node
            .get(&State {
                facing: Dir::Right,
                times_traveled: 0,
                loc: (0, 0),
            })
            .unwrap(),
        None,
        |e| *e.weight(),
    );

    let end_nodes = {
        let mut end_nodes = Vec::new();
        for times_traveled in 4u8..=10u8 {
            for facing in Dir::all() {
                let state: State = State {
                    facing,
                    times_traveled,
                    loc: (num_rows - 1, num_cols - 1),
                };
                end_nodes.push(*node_weight_to_node.get(&state).unwrap());
            }
        }
        end_nodes
    };

    *end_nodes
        .into_iter()
        .flat_map(|node| costs.get(&node))
        .min()
        .unwrap()
}
