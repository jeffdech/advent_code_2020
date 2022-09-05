mod bagspec;
use crate::bagspec::*;

fn main() {
    let text = include_str!("input.txt");
    let rules = parse_rules(text);

    if let Err(e) = rules {
        println!("Encountered a parsing error: {:?}", e);
        return ();
    }

    let search_bag = ("shiny", "gold");
    let search = BagSearch::new(rules.unwrap(), search_bag);
    let containing = search.results();

    println!("There are {} bags which contain a {} {} bag!", containing.len(), search_bag.0, search_bag.1);

    println!("A {} {} bag contains {} other bags!", search_bag.0, search_bag.1, search.count_containing(&search_bag));
}
