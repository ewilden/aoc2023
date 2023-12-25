use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Part {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum CmpDir {
    Less,
    Greater,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Matcher {
    cmp_dir: CmpDir,
    category: Category,
    threshold: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Action {
    Workflow(String),
    Accept,
    Reject,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Rule {
    matcher: Option<Matcher>,
    action: Action,
}

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

#[aoc_generator(day19)]
fn parse(input: &str) -> (Vec<Workflow>, Vec<Part>) {
    let (workflows, parts) = input.trim().split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|line| {
            let (name, rest) = line.split_once("{").unwrap();
            let rest = rest.strip_suffix("}").unwrap();
            let rules = rest
                .split(",")
                .map(|rule| {
                    let (matcher, action) = match rule.split_once(":") {
                        Some((matcher, action)) => (Some(matcher), action),
                        None => (None, rule),
                    };
                    let matcher = matcher.map(|matcher| {
                        let cmp_dir = if matcher.contains('<') {
                            CmpDir::Less
                        } else if matcher.contains('>') {
                            CmpDir::Greater
                        } else {
                            unreachable!()
                        };
                        let (category, threshold) =
                            matcher.split_once(|c| ['<', '>'].contains(&c)).unwrap();
                        let category = match category {
                            "x" => Category::X,
                            "m" => Category::M,
                            "a" => Category::A,
                            "s" => Category::S,
                            _ => unreachable!(),
                        };
                        let threshold = threshold.parse::<i64>().unwrap();
                        Matcher {
                            cmp_dir,
                            category,
                            threshold,
                        }
                    });
                    let action = match action {
                        "A" => Action::Accept,
                        "R" => Action::Reject,
                        s => Action::Workflow(s.to_owned()),
                    };
                    Rule { matcher, action }
                })
                .collect_vec();
            Workflow {
                name: name.to_owned(),
                rules,
            }
        })
        .collect_vec();
    let parts = parts
        .lines()
        .map(|line| {
            let line = line.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
            let mut properties = line.split(',').map(|n| n[2..].parse::<i64>().unwrap());
            let x = properties.next().unwrap();
            let m = properties.next().unwrap();
            let a = properties.next().unwrap();
            let s = properties.next().unwrap();
            assert!(properties.next().is_none());
            Part { x, m, a, s }
        })
        .collect_vec();
    (workflows, parts)
}

#[aoc(day19, part1)]
fn part1(input: &(Vec<Workflow>, Vec<Part>)) -> i64 {
    let (workflows, parts) = input;
    let workflows = workflows
        .iter()
        .map(|workflow| (workflow.name.clone(), workflow.rules.clone()))
        .collect::<HashMap<_, _>>();

    parts
        .iter()
        .map(|part| {
            let Part { x, m, a, s } = *part;
            let mut curr_workflow = "in".to_owned();

            'outer: loop {
                let workflow = workflows.get(&curr_workflow).unwrap();
                for rule in workflow {
                    let Rule { matcher, action } = rule;
                    let matcher_result = match matcher {
                        Some(matcher) => {
                            let Matcher {
                                cmp_dir,
                                category,
                                threshold,
                            } = *matcher;
                            let comparand = match category {
                                Category::X => x,
                                Category::M => m,
                                Category::A => a,
                                Category::S => s,
                            };
                            match cmp_dir {
                                CmpDir::Less => comparand < threshold,
                                CmpDir::Greater => comparand > threshold,
                            }
                        }
                        None => true,
                    };
                    if matcher_result {
                        match action {
                            Action::Workflow(name) => {
                                curr_workflow = name.clone();
                                continue 'outer;
                            }
                            Action::Accept => {
                                let Part { x, m, a, s } = *part;
                                return x + m + a + s;
                            }
                            Action::Reject => {
                                return 0;
                            }
                        }
                    }
                }
                unreachable!()
            }
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct PartBounds {
    x: (i64, i64),
    m: (i64, i64),
    a: (i64, i64),
    s: (i64, i64),
}

impl PartBounds {
    fn count(&self) -> u64 {
        let Self { x, m, a, s } = self;
        [x, m, a, s]
            .into_iter()
            .map(|(lo, hi)| {
                let lo = *lo;
                let hi = *hi;
                u64::try_from(hi - lo).unwrap()
            })
            .product()
    }

    fn split(
        &self,
        Matcher {
            cmp_dir,
            category,
            threshold,
        }: Matcher,
    ) -> (Option<Self>, Option<Self>) {
        let Self { x, m, a, s } = *self;

        match category {
            Category::X => {
                let (matched, nomatched) = split_bound(x, cmp_dir, threshold);
                (
                    matched.map(|x| Self { x, ..*self }),
                    nomatched.map(|x| Self { x, ..*self }),
                )
            }
            Category::M => {
                let (matched, nomatched) = split_bound(m, cmp_dir, threshold);
                (
                    matched.map(|m| Self { m, ..*self }),
                    nomatched.map(|m| Self { m, ..*self }),
                )
            }
            Category::A => {
                let (matched, nomatched) = split_bound(a, cmp_dir, threshold);
                (
                    matched.map(|a| Self { a, ..*self }),
                    nomatched.map(|a| Self { a, ..*self }),
                )
            }
            Category::S => {
                let (matched, nomatched) = split_bound(s, cmp_dir, threshold);
                (
                    matched.map(|s| Self { s, ..*self }),
                    nomatched.map(|s| Self { s, ..*self }),
                )
            }
        }
    }
}

fn is_nonempty((lo, hi): (i64, i64)) -> bool {
    lo < hi
}

fn split_bound(
    (lo, hi): (i64, i64),
    cmp_dir: CmpDir,
    threshold: i64,
) -> (Option<(i64, i64)>, Option<(i64, i64)>) {
    match cmp_dir {
        CmpDir::Less => {
            let lomatch = lo;
            let himatch = hi.min(threshold);
            // to not match, lo must be >=.
            let lonomatch = lo.max(threshold);
            let hinomatch = hi;
            (
                Some((lomatch, himatch)).filter(|x| is_nonempty(*x)),
                Some((lonomatch, hinomatch)).filter(|x| is_nonempty(*x)),
            )
        }
        CmpDir::Greater => {
            let lomatch = lo.max(threshold + 1);
            let himatch = hi;

            let lonomatch = lo;
            let hinomatch = hi.min(threshold + 1);

            (
                Some((lomatch, himatch)).filter(|x| is_nonempty(*x)),
                Some((lonomatch, hinomatch)).filter(|x| is_nonempty(*x)),
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    bounds: PartBounds,
    curr_workflow: (String, usize),
}

fn count_solns(
    table: &mut HashMap<State, u64>,
    workflows: &HashMap<String, Vec<Rule>>,
    state: &State,
) -> u64 {
    if let Some(solns) = table.get(state) {
        return *solns;
    }

    let solns = count_solns_impl(table, workflows, &state);
    table.insert(state.clone(), solns);
    solns
}

fn count_solns_impl(
    table: &mut HashMap<State, u64>,
    workflows: &HashMap<String, Vec<Rule>>,
    state: &State,
) -> u64 {
    let State {
        bounds,
        curr_workflow,
    } = state;
    let (curr_workflow, step) = curr_workflow;

    let curr_rules = workflows.get(curr_workflow).unwrap();
    let Rule { matcher, action } = &curr_rules[*step];
    match matcher {
        Some(matcher) => {
            let (matched, nomatched) = bounds.split(*matcher);

            let mut sum = 0u64;
            if let Some(matched) = matched {
                match action {
                    Action::Accept => {
                        sum += matched.count();
                    }
                    Action::Reject => {
                        // 0.
                    }
                    Action::Workflow(next_workflow) => {
                        sum += count_solns(
                            table,
                            workflows,
                            &State {
                                bounds: matched,
                                curr_workflow: (next_workflow.to_owned(), 0),
                            },
                        );
                    }
                }
            }

            if let Some(nomatched) = nomatched {
                let step = *step + 1;
                if step < curr_rules.len() {
                    sum += count_solns(
                        table,
                        workflows,
                        &State {
                            bounds: nomatched,
                            curr_workflow: (curr_workflow.to_owned(), step),
                        },
                    );
                }
            }
            sum
        }
        None => match action {
            Action::Workflow(workflow) => count_solns(
                table,
                workflows,
                &State {
                    bounds: *bounds,
                    curr_workflow: (workflow.to_owned(), 0),
                },
            ),
            Action::Accept => bounds.count(),
            Action::Reject => 0u64,
        },
    }
}

#[aoc(day19, part2)]
fn part2(input: &(Vec<Workflow>, Vec<Part>)) -> u64 {
    let (workflows, _) = input;
    let workflows = workflows
        .iter()
        .map(|workflow| (workflow.name.clone(), workflow.rules.clone()))
        .collect::<HashMap<_, _>>();

    let bounds = PartBounds {
        x: (1, 4001),
        m: (1, 4001),
        a: (1, 4001),
        s: (1, 4001),
    };

    let mut table = HashMap::new();
    count_solns(
        &mut table,
        &workflows,
        &State {
            bounds,
            curr_workflow: ("in".to_owned(), 0),
        },
    )
}
