use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{anychar, one_of},
    combinator::rest,
    error::{self, Error},
    multi::many0,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

pub fn run(part: u8) {
    match part {
        1 => part1(),
        2 => part2(),
        _ => println!("Part {} not implemented", part),
    }
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Clone)]
enum Destination {
    Accepted,
    Rejected,
    Workflow(String),
}

impl From<&str> for Destination {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accepted,
            "R" => Self::Rejected,
            _ => Self::Workflow(value.to_string()),
        }
    }
}

type Rule = Box<dyn Fn(&Part) -> Option<Destination>>;

struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

fn part1() {
    let (workflows, part_ratings) = parse_workflows_and_part_ratings();
    let mut workflow_map = HashMap::new();

    for wf in workflows.iter() {
        workflow_map.insert(wf.name.as_str(), wf);
    }

    let res: u64 = part_ratings
        .iter()
        .filter(|p| is_accepted(p, &workflow_map))
        .map(|p| p.x + p.m + p.a + p.s)
        .sum();
    println!("{}", res);
}

fn parse_workflows_and_part_ratings() -> (Vec<Workflow>, Vec<Part>) {
    let input = include_str!("../../puzzle_input/d19").trim();
    let (workflows, part_ratings) = input.split_once("\n\n").unwrap();
    let workflows = workflows
        .lines()
        .map(|l| parse_workflow(l).unwrap().1)
        .collect_vec();

    let part_ratings = part_ratings
        .lines()
        .map(|l| parse_part_rating(l).unwrap().1)
        .collect_vec();

    (workflows, part_ratings)
}

fn is_accepted(part: &Part, workflow_map: &HashMap<&str, &Workflow>) -> bool {
    let mut dest = Destination::Workflow("in".to_string());

    loop {
        match &dest {
            Destination::Accepted => {
                break true;
            }
            Destination::Rejected => {
                break false;
            }
            Destination::Workflow(wf) => {
                let &wf = workflow_map.get(wf.as_str()).unwrap();
                for rule in wf.rules.iter() {
                    if let Some(new_dest) = rule(part) {
                        dest = new_dest;
                        break;
                    }
                }
            }
        }
    }
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (i, name) = is_not("{")(input)?;
    let (_, rules) = delimited(tag("{"), is_not("}"), tag("}"))(i)?;
    let (_, rules) = many0(parse_rule)(rules)?;
    let wf = Workflow {
        name: name.to_string(),
        rules,
    };
    Ok((i, wf))
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    if !input.contains(',') {
        if input.is_empty() {
            return Err(nom::Err::Error(nom::error::Error {
                input,
                code: error::ErrorKind::Eof,
            }));
        }

        let input = input.to_string();
        return Ok((
            "",
            Box::new(move |_: &Part| Some(Destination::from(input.as_str()))),
        ));
    }

    let (remaining_input, rule) = terminated(is_not(","), tag(","))(input)?;
    let (_, (condition, destination)) = separated_pair(is_not(":"), tag(":"), rest)(rule)?;

    let (_, (part_category, cmp, val)) = tuple((anychar, one_of("<>"), rest))(condition)?;
    let val = val.parse().unwrap();

    let destination = Destination::from(destination);
    let rule = move |p: &Part| {
        let var = match part_category {
            'x' => &p.x,
            'm' => &p.m,
            'a' => &p.a,
            's' => &p.s,
            _ => panic!("Unexpected part category!"),
        };

        if cmp == '>' && *var > val || cmp == '<' && *var < val {
            Some(destination.clone())
        } else {
            None
        }
    };
    Ok((remaining_input, Box::new(rule)))
}

fn parse_part_rating(input: &str) -> IResult<&str, Part> {
    let mut part = Part {
        x: 0,
        m: 0,
        a: 0,
        s: 0,
    };

    let (_, ratings) = delimited(tag("{"), is_not("}"), tag("}"))(input)?;
    let ratings = ratings.split(',').collect_vec();
    for rating in ratings {
        let (category, value) = rating.split_once('=').unwrap();
        match category {
            "x" => part.x = value.parse().unwrap(),
            "m" => part.m = value.parse().unwrap(),
            "a" => part.a = value.parse().unwrap(),
            "s" => part.s = value.parse().unwrap(),
            _ => panic!("Cannot parse part rating!"),
        }
    }
    Ok(("", part))
}

// -----------------------------------------------------------------

type WorkflowMap<'a> = HashMap<&'a str, Vec<&'a str>>;

fn part2() {
    let input = include_str!("../../puzzle_input/d19")
        .trim()
        .split_once("\n\n")
        .unwrap()
        .0;

    let workflows = input.lines().map(|l| parse_workflow_2(l));
    let workflow_map: WorkflowMap = HashMap::from_iter(workflows.clone());

    let res = accepted_part_count("in", &workflow_map, &vec![]);
    println!("{}", res);
}

fn parse_workflow_2(input: &str) -> (&str, Vec<&str>) {
    let (i, name) = is_not::<&str, &str, Error<_>>("{")(input).unwrap();
    let (_, rules) = delimited(tag::<&str, &str, Error<_>>("{"), is_not("}"), tag("}"))(i).unwrap();
    let rules = rules.split(',').collect_vec();
    (name, rules)
}

fn accepted_part_count(wf_name: &str, wf_map: &WorkflowMap, requirements: &Vec<String>) -> u64 {
    if wf_name == "R" {
        0
    } else if wf_name == "A" {
        part_count_from_requirements(requirements)
    } else {
        let mut total_count = 0;
        let mut requirements = requirements.clone();
        for &child_wf in wf_map.get(wf_name).unwrap() {
            let (condition, dest) = child_wf.split_once(':').unwrap_or(("", child_wf));
            if condition.is_empty() {
                total_count += accepted_part_count(dest, wf_map, &requirements);
            } else {
                requirements.push(condition.to_string());
                total_count += accepted_part_count(dest, wf_map, &requirements);
                requirements.pop();
                requirements.push(negate_condition(condition));
            }
        }
        total_count
    }
}

fn part_count_from_requirements(requirements: &Vec<String>) -> u64 {
    let mut x_range = (1, 4000);
    let mut m_range = (1, 4000);
    let mut a_range = (1, 4000);
    let mut s_range = (1, 4000);

    for req in requirements {
        let category = req.chars().next().unwrap();
        let cmp = req.chars().nth(1).unwrap();
        let bound: u64 = req.chars().skip(2).collect::<String>().parse().unwrap();

        let req_range = match cmp {
            '>' => (bound + 1, 4000),
            '<' => (0, bound - 1),
            _ => panic!("Cannot parse req!"),
        };

        let range_to_adjust = match category {
            'x' => &mut x_range,
            'm' => &mut m_range,
            'a' => &mut a_range,
            's' => &mut s_range,
            _ => panic!("Cannot parse category!"),
        };

        range_to_adjust.0 = range_to_adjust.0.max(req_range.0);
        range_to_adjust.1 = range_to_adjust.1.min(req_range.1);
    }

    let x_width = (x_range.1 - x_range.0 + 1).max(0);
    let m_width = (m_range.1 - m_range.0 + 1).max(0);
    let a_width = (a_range.1 - a_range.0 + 1).max(0);
    let s_width = (s_range.1 - s_range.0 + 1).max(0);

    x_width * m_width * a_width * s_width
}

fn negate_condition(condition: &str) -> String {
    let category = condition.chars().next().unwrap();
    let cmp = condition.chars().nth(1).unwrap();
    let bound: u64 = condition[2..].parse().unwrap();

    let (cmp, bound) = if cmp == '>' {
        ('<', bound + 1)
    } else {
        ('>', bound - 1)
    };
    format!("{}{}{}", category, cmp, bound)
}
