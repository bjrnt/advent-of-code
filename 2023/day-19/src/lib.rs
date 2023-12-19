use std::{collections::HashMap, ops::RangeInclusive};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Clone, Copy, Debug)]
enum Op {
    Less,
    Greater,
}

#[derive(Clone, Debug)]
struct Rule(usize, Op, u32, String);

impl Rule {
    pub fn matches(&self, p: [u32; 4]) -> bool {
        match self.1 {
            Op::Less => p[self.0] < self.2,
            Op::Greater => p[self.0] > self.2,
        }
    }

    pub fn constrain(
        &self,
        mut p: [RangeInclusive<u32>; 4],
        want_match: bool,
    ) -> [RangeInclusive<u32>; 4] {
        match (self.1, want_match) {
            (Op::Less, true) => p[self.0] = *p[self.0].start()..=(self.2 - 1).min(*p[self.0].end()),
            (Op::Greater, true) => {
                p[self.0] = (self.2 + 1).max(*p[self.0].start())..=*p[self.0].end()
            }
            (Op::Less, false) => p[self.0] = (self.2).max(*p[self.0].start())..=*p[self.0].end(),
            (Op::Greater, false) => p[self.0] = *p[self.0].start()..=self.2.min(*p[self.0].end()),
        };
        p
    }
}

#[derive(Clone, Debug)]
struct Workflow(String, Vec<Rule>, String);

impl Workflow {
    pub fn run(&self, part: [u32; 4]) -> String {
        if let Some(matching_rule) = self.1.iter().find(|rule| rule.matches(part)) {
            matching_rule.3.clone()
        } else {
            self.2.clone()
        }
    }

    pub fn potential_outcomes(
        &self,
        part: [RangeInclusive<u32>; 4],
    ) -> Vec<([RangeInclusive<u32>; 4], String)> {
        let mut outcomes = vec![];
        let fallback = self.1.iter().fold(part, |part, rule| {
            let matching = rule.constrain(part.clone(), true);
            if matching.iter().all(|r| !r.is_empty()) {
                outcomes.push((matching, rule.3.clone()));
            };
            // not matching, so it goes to the next rule
            rule.constrain(part, false)
        });
        if fallback.iter().all(|r| !r.is_empty()) {
            outcomes.push((fallback, self.2.clone()));
        }
        outcomes
    }
}

fn parse_rule(input: &str) -> IResult<&str, Rule> {
    // indices for our 4-length arrays
    let (input, idx) = alt((
        complete::char('x').map(|_| 0),
        complete::char('m').map(|_| 1),
        complete::char('a').map(|_| 2),
        complete::char('s').map(|_| 3),
    ))(input)?;
    let (input, op) = alt((
        complete::char('>').map(|_| Op::Greater),
        complete::char('<').map(|_| Op::Less),
    ))(input)?;
    let (input, v) = complete::u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, dest_label) = alpha1(input)?;
    Ok((input, Rule(idx, op, v, dest_label.to_owned())))
}

fn parse_part(input: &str) -> IResult<&str, [u32; 4]> {
    let (input, _) = tag("{x=")(input)?;
    let (input, x) = complete::u32(input)?;
    let (input, _) = tag(",m=")(input)?;
    let (input, m) = complete::u32(input)?;
    let (input, _) = tag(",a=")(input)?;
    let (input, a) = complete::u32(input)?;
    let (input, _) = tag(",s=")(input)?;
    let (input, s) = complete::u32(input)?;
    let (input, _) = tag("}")(input)?;
    Ok((input, [x, m, a, s]))
}

fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    let (input, label) = alpha1(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, rules) = separated_list1(tag(","), parse_rule)(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, dest) = alpha1(input)?;
    let (input, _) = tag("}")(input)?;
    Ok((input, Workflow(label.to_owned(), rules, dest.to_owned())))
}

fn parse_input(input: &str) -> IResult<&str, (HashMap<String, Workflow>, Vec<[u32; 4]>)> {
    let (input, (workflows, parts)) = separated_pair(
        separated_list1(newline, parse_workflow),
        tag("\n\n"),
        separated_list1(newline, parse_part),
    )(input)?;
    Ok((
        input,
        (
            workflows.into_iter().map(|wf| (wf.0.clone(), wf)).collect(),
            parts,
        ),
    ))
}

pub fn process_part1(input: &str) -> String {
    let (input, (workflows, parts)) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");

    parts
        .into_iter()
        .filter_map(|part| {
            let mut curr_wf_label: String = "in".into();
            while curr_wf_label != "R" && curr_wf_label != "A" {
                curr_wf_label = workflows.get(&curr_wf_label).unwrap().run(part);
            }
            (curr_wf_label == "A").then(|| part.iter().sum::<u32>())
        })
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (input, (workflows, _)) = parse_input(input).unwrap();
    debug_assert_eq!(input, "");

    let initial = vec![([1..=4000, 1..=4000, 1..=4000, 1..=4000], "in".to_string())];
    let mut accepted = vec![];

    aoc_utils::complete_bfs(initial.into_iter(), |(part, workflow)| {
        if workflow == "A" {
            accepted.push(part);
            return None;
        };
        (workflow != "R").then(|| workflows.get(&workflow).unwrap().potential_outcomes(part))
    });

    accepted
        .into_iter()
        .map(|accepted| accepted.into_iter().map(|r| r.count()).product::<usize>())
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}",
        "19114"
    )]
    #[trace]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(&process_part1(input), expected);
    }

    #[rstest]
    #[case(
        "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=2127,m=1623,a=2188,s=1013}",
        "167409079868000"
    )]
    #[trace]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(&process_part2(input), expected);
    }
}
