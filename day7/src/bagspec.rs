use std::fmt;

use multimap::MultiMap;

use nom::{
    Finish,
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, space1},
    combinator::{map, opt},
    error::VerboseError,
    multi::many0,
    sequence::{separated_pair, tuple},
};

pub type Res<T, U> = IResult<T, U, VerboseError<T>>;

pub type BagType<'a> = (&'a str, &'a str);
pub type BagRules<'a> = MultiMap<BagType<'a>, (BagType<'a>, usize)>;

pub struct FormattedRules<'a>(pub BagRules<'a>);

impl fmt::Display for FormattedRules<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (k, vv) in &self.0 {
            write!(f, "{} {} bag contain ", k.0, k.1)?;

            if vv.is_empty() {
                write!(f, "no other bags")?;
            } else {
                for (i, v) in vv.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }

                    write!(f, "{} {} {} {}", 
                        v.1, v.0.0, v.0.1,
                        if v.1 == 1 {"bag"} else {"bags"}
                    )?;
                }
            }

            write!(f, ".\n")?;
        }

        Ok(())
    }
}

pub struct BagSearch<'a> {
    rules: BagRules<'a>,
    search_bag: BagType<'a>
}

impl<'a> BagSearch<'a> {
    pub fn new(rules: BagRules<'a>, search_bag: BagType<'a>) -> Self {
        BagSearch {
            rules,
            search_bag
        }
    }

    pub fn results(&'a self) -> Vec<&'a BagType<'a>> {
        self.rules.keys()
            .filter(|&k| self.contains_search(k))
            .collect()
    }

    fn contains_search(&self, entry: &BagType<'a>) -> bool {
        match self.rules.get_vec(entry) {
            Some(bts) => {
                if bts.iter().any(|&(bt, _)| bt == self.search_bag){
                    return true;
                } else {
                    return bts.iter().any(|&(bt, _)| self.contains_search(&bt));
                }
            },
            None => { return false; }
        }
    }

    pub fn count_containing(&'a self, bag_type: &BagType<'a>) -> usize {
        match self.rules.get_vec(bag_type) {
            None => 0,
            Some(bts) => {
                let this_content: usize = bts.iter().map(|(_, n)| n).sum();
                let sub_contents: usize = bts.iter().map(|(bt, n)| n * self.count_containing(bt)).sum();
                this_content + sub_contents
            }
        }
    }
}

pub fn parse_rules<'a>(input: &'a str) -> Result<BagRules<'a>, VerboseError<&'a str>> {
    let mut rules: BagRules<'a> = Default::default();

    let parse_results = input.lines().map(parse_line);

    for res in parse_results {
        match res.finish() {
            Ok((_, (btype, contents))) => {
                match contents {
                    Some(cons) => {
                        for c in cons {
                            rules.insert(btype, c);
                    }},
                    None => (),
                }
            }
            Err(e) => {return Err(e);}
        }
    }

    Ok(rules)
}

fn parse_line<'a>(input: &'a str) -> Res<&'a str, (BagType<'a>, Option<Vec<(BagType<'a>, usize)>>)> {
    map(
        tuple((parse_bagtype, tag(" bags contain "), parse_bagvalues, tag("."))),
        |(bt, _, bvs, _)| (bt, bvs)
    )(input)
}

fn parse_bagtype<'a>(input: &'a str) -> Res<&'a str, BagType<'a>> {
    map(
        separated_pair(alpha1, space1, alpha1), 
        |(s1, s2): (&'a str, &'a str)| (s1, s2)
    )(input)
}

fn parse_bagvalues<'a>(input: &'a str) -> Res<&'a str, Option<Vec<(BagType<'a>, usize)>>> {
    let no_bags = map(tag("no other bags"), |_| None);
    let vitem = tuple((digit1, space1, parse_bagtype, tag(" bag"), opt(tag("s")), opt(tag(", "))));
    let mapper = map(vitem, |(d, _, bt, _, _, _)| (bt, d.parse::<usize>().unwrap()));

    alt((
        no_bags, 
        map(many0(mapper), |r| Some(r))
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::iter::zip;

    #[test]
    fn test_parse_bagtype() {
        let texts = vec!["dull blue", "dotted green"];
        let results = vec![("dull", "blue"), ("dotted", "green")];

        for (t, r) in zip(texts, results) {
            assert_eq!(parse_bagtype(t), Ok(("", r)));
        }
    }

    #[test]
    fn test_parse_bagvalues() {
        let text = "2 dotted green bags, 1 dull brown bag, 3 striped tomato bags, 5 muted blue bags";
        let expected = vec![
            (("dotted", "green"), 2),
            (("dull", "brown"), 1),
            (("striped", "tomato"), 3),
            (("muted", "blue"), 5)
        ];

        assert_eq!(parse_bagvalues(text), Ok(("", Some(expected))));
    }

    #[test]
    fn test_parse_line() {
        let text = "dull blue bags contain 2 dotted green bags, 1 dull brown bag, 3 striped tomato bags, 5 muted blue bags.";

        let result = parse_line(text);
        let expected_output = (("dull", "blue"), Some(vec![
            (("dotted", "green"), 2),
            (("dull", "brown"), 1),
            (("striped", "tomato"), 3),
            (("muted", "blue"), 5)
        ]));

        assert_eq!(result, Ok(("", expected_output)));
    }

    fn example_ruleset<'a>() -> BagRules<'a> {
        let kv1 = (("big", "red"), vec![(("tiny", "blue"), 2), (("shiny", "gold"), 3)]);
        let kv2 = (("tiny", "blue"), vec![(("small", "green"), 1)]);

        let rsets = vec![kv1, kv2];
        let mut rules: BagRules<'_> = Default::default();
        for (key, values) in rsets {
            for v in values {
                rules.insert(key, v);
            }
        }
        rules
    }

    #[test]
    fn test_contains_bag() {
        let search = BagSearch::new(example_ruleset(), ("small", "green"));

        assert!(search.contains_search(&("big", "red")));
        assert!(search.contains_search(&("tiny", "blue")));

        let bad_search = BagSearch::new(example_ruleset(), ("shiny", "gold"));
        assert!(!bad_search.contains_search(&("tiny", "blue")));
    }

    #[test]
    fn test_count_contents() {
        let search = BagSearch::new(example_ruleset(), ("big", "red"));
        assert_eq!(search.count_containing(&search.search_bag), 7);
        assert_eq!(search.count_containing(&("tiny", "blue")), 1);
    }
}