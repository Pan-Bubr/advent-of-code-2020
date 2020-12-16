use std::ops::RangeInclusive;

fn main() {
    let mut input = include_str!("../../../inputs/16.txt").split("\n\n");

    let requirements: Vec<Requirement> = input
        .next()
        .unwrap()
        .lines()
        .map(parse_requirement)
        .map(Result::unwrap)
        .collect();

    let _your_ticket = parse_ticket(input.next().unwrap().lines().skip(1).next().unwrap());

    let nearby_tickets: Vec<Ticket> = input
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect();

    println!(
        "[Part One] Solution: {}",
        part1(nearby_tickets.clone(), requirements.clone())
    );
}

fn part1(nearby_tickets: Vec<Ticket>, requirements: Vec<Requirement>) -> u32 {
    nearby_tickets
        .iter()
        .map(|ticket: &Ticket| {
            ticket
                .0
                .iter()
                .map(|v: &u32| {
                    if requirements.iter().any(|r: &Requirement| r.validate(v)) {
                        &0
                    } else {
                        v
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>()
}

#[derive(Debug, Clone)]
struct Requirement(RangeInclusive<u32>, RangeInclusive<u32>);

impl Requirement {
    fn validate(&self, value: &u32) -> bool {
        self.0.contains(value) || self.1.contains(value)
    }
}

#[derive(Debug, Clone)]
struct Ticket(Vec<u32>);

fn parse_requirement(s: &str) -> anyhow::Result<Requirement> {
    peg::parser! {
      grammar parser() for str {
        rule number() -> u32
          = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule range() -> RangeInclusive<u32>
          = min:number() "-" max:number() { min..=max }

        rule byte() -> u8
          = letter:$(['a'..='z']) { letter.as_bytes()[0] }

        rule entry() -> ()
          = string:$(['a'..='z']+) {}

        pub(crate) rule line() -> Requirement
          = _:entry() " "? _:entry()? ": " range1:range() " or " range2:range() {
              Requirement(range1, range2)
          }
      }
    }

    Ok(parser::line(s)?)
}

fn parse_ticket(s: &str) -> Ticket {
    Ticket(
        s.split(",")
            .map(str::parse::<u32>)
            .map(Result::unwrap)
            .collect(),
    )
}
