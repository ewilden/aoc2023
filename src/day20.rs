use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use num::integer::lcm;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct FlipFlop {
    on: bool,
}

impl Default for FlipFlop {
    fn default() -> Self {
        FlipFlop { on: false }
    }
}

impl FlipFlop {
    fn pulse(&mut self, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                self.on = !self.on;

                Some(if self.on { Pulse::High } else { Pulse::Low })
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Conjunction {
    memory: HashMap<String, Pulse>,
}

impl Conjunction {
    fn new(inputs: impl IntoIterator<Item = String>) -> Self {
        Self {
            memory: inputs.into_iter().map(|name| (name, Pulse::Low)).collect(),
        }
    }

    fn pulse(&mut self, source: String, pulse: Pulse) -> Pulse {
        self.memory.insert(source, pulse);
        if self.memory.values().all(|pulse| *pulse == Pulse::High) {
            Pulse::Low
        } else {
            Pulse::High
        }
    }
}

const BROADCASTER: &str = "broadcaster";
const BUTTON: &str = "button";

#[derive(Debug, Clone, PartialEq, Eq)]
enum ModuleType {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    ty: ModuleType,
    dests: Vec<String>,
}

#[aoc_generator(day20)]
fn parse(input: &str) -> BTreeMap<String, Module> {
    let mut modules = BTreeMap::new();
    let mut module_name_to_inputs = HashMap::<String, Vec<String>>::new();

    for line in input.lines() {
        let (module, dests) = line.split_once(" -> ").unwrap();
        let dests = dests.split(", ").map(|s| s.to_owned()).collect_vec();
        let module = if module == BROADCASTER {
            BROADCASTER.to_owned()
        } else {
            module[1..].to_owned()
        };
        for dest in &dests {
            module_name_to_inputs
                .entry(dest.clone())
                .or_default()
                .push(module.clone());
        }
    }

    for line in input.lines() {
        let (module, dests) = line.split_once(" -> ").unwrap();
        let dests = dests.split(", ").map(|s| s.to_owned()).collect_vec();
        if module == BROADCASTER {
            assert!(modules
                .insert(
                    BROADCASTER.to_owned(),
                    Module {
                        ty: ModuleType::Broadcast,
                        dests
                    }
                )
                .is_none());
        } else if &module[..1] == "%" {
            let module = module[1..].to_owned();
            assert!(modules
                .insert(
                    module,
                    Module {
                        ty: ModuleType::FlipFlop(FlipFlop::default()),
                        dests
                    }
                )
                .is_none());
        } else if &module[..1] == "&" {
            let module = module[1..].to_owned();
            assert!(modules
                .insert(
                    module.clone(),
                    Module {
                        ty: ModuleType::Conjunction(Conjunction::new(
                            module_name_to_inputs
                                .get(&module)
                                .unwrap()
                                .iter()
                                .map(|x| x.clone())
                        )),
                        dests
                    }
                )
                .is_none());
        } else {
            unreachable!();
        }
    }

    modules
}

#[aoc(day20, part1)]
fn part1(input: &BTreeMap<String, Module>) -> usize {
    let mut input = input.clone();
    let mut low_pulses = 0usize;
    let mut high_pulses = 0usize;

    let mut pulse_queue = VecDeque::<(String, String, Pulse)>::new();

    for _ in 1..=1000 {
        pulse_queue.push_back((BUTTON.to_owned(), BROADCASTER.to_owned(), Pulse::Low));

        while let Some((src, dest, pulse)) = pulse_queue.pop_front() {
            match pulse {
                Pulse::High => high_pulses += 1,
                Pulse::Low => low_pulses += 1,
            }
            let Some(module) = input.get_mut(&dest) else {
                continue
            };
            let me = dest;
            match &mut module.ty {
                ModuleType::FlipFlop(flipflop) => {
                    let pulse = flipflop.pulse(pulse);
                    if let Some(pulse) = pulse {
                        for dest in &module.dests {
                            pulse_queue.push_back((me.clone(), dest.clone(), pulse));
                        }
                    }
                }
                ModuleType::Conjunction(conj) => {
                    let pulse = conj.pulse(src, pulse);
                    for dest in &module.dests {
                        pulse_queue.push_back((me.clone(), dest.clone(), pulse));
                    }
                }
                ModuleType::Broadcast => {
                    for dest in &module.dests {
                        pulse_queue.push_back((me.clone(), dest.clone(), pulse));
                    }
                }
            }
        }
    }

    low_pulses * high_pulses
}

#[aoc(day20, part2)]
fn part2(input: &BTreeMap<String, Module>) -> usize {
    let mut input = input.clone();

    let reversegraph = {
        input
            .iter()
            .flat_map(|(modname, module)| {
                module
                    .dests
                    .iter()
                    .map(|dest| (dest.to_owned(), modname.to_owned()))
            })
            .into_grouping_map()
            .collect::<HashSet<_>>()
    };

    // "kc" is the conjunction component right before "rx" in my input.
    let senders_to_kc = reversegraph.get("kc").unwrap();
    let mut sent_hi_to_kc = HashMap::new();

    let mut pulse_queue = VecDeque::<(String, String, Pulse)>::new();

    for num_pulses in 1usize.. {
        if sent_hi_to_kc.len() == senders_to_kc.len() {
            break;
        }

        pulse_queue.push_back((BUTTON.to_owned(), BROADCASTER.to_owned(), Pulse::Low));

        while let Some((src, dest, pulse)) = pulse_queue.pop_front() {
            match pulse {
                Pulse::High => {
                    if &dest == "kc" {
                        sent_hi_to_kc.entry(src.clone()).or_insert(num_pulses);
                    }
                }
                Pulse::Low => {}
            }

            let Some(module) = input.get_mut(&dest) else {
                continue
            };
            let me = dest;
            match &mut module.ty {
                ModuleType::FlipFlop(flipflop) => {
                    let pulse = flipflop.pulse(pulse);
                    if let Some(pulse) = pulse {
                        for dest in &module.dests {
                            pulse_queue.push_back((me.clone(), dest.clone(), pulse));
                        }
                    }
                }
                ModuleType::Conjunction(conj) => {
                    let pulse = conj.pulse(src, pulse);
                    for dest in &module.dests {
                        pulse_queue.push_back((me.clone(), dest.clone(), pulse));
                    }
                }
                ModuleType::Broadcast => {
                    for dest in &module.dests {
                        pulse_queue.push_back((me.clone(), dest.clone(), pulse));
                    }
                }
            }
        }
    }

    // This, again, doesn't seem like something that should be strictly right unless
    // we always loop back to the starting condition, but it worked for the ghosts
    // question before.
    sent_hi_to_kc.values().copied().fold(1usize, lcm)
}
