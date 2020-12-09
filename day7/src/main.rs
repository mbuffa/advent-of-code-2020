use std::fs;

extern crate regex;
#[macro_use] extern crate lazy_static;

#[derive(Debug)]
struct RawContent {
  amount: u64,
  identifier: String
}

#[derive(Debug)]
struct RawRule {
  identifier: String,
  content: Vec<RawContent>
}

#[derive(Debug, Clone)]
struct Bag {
  identifier: String,
  content: Vec<(u64, Bag)>
}

fn main() {
  let contents = fs::read_to_string("./input.txt")
    .expect("Cannot load file.");

  let rules: Vec<RawRule> = evaluate_rules(&contents);
  // println!("{:?}", rules);

  let bags: Vec<(u64, Bag)> = build_bags(&rules);
  // display_bags(&bags, "\t".to_owned());

  let bags_count = compute_bag_capacity(&bags, (1, "shiny gold"));

  println!("{}", bags_count);
}

fn display_bags(bags: &Vec<(u64, Bag)>, prefix: String) {
  let mut new_prefix: String = prefix.clone();
  new_prefix.push_str("\t");

  for (amount, bag) in bags.iter() {
    println!("{} {} {} can contain:", prefix, amount, bag.identifier);
    display_bags(&bag.content, new_prefix.clone());
  }
}

fn evaluate_rules(rules: &String) -> Vec<RawRule> {
  lazy_static! {
    static ref RE: regex::Regex =
      regex::Regex::new(r"([\w ]+) bags contain (?:no other bags|(.*))\.\n")
        .unwrap();
  }

  let mut raw_rules: Vec<RawRule> = Vec::new();

  for captures in RE.captures_iter(rules) {
    let identifier: String = captures[1].to_string();

    match captures.get(2) {
      None => raw_rules.push(RawRule { identifier, content: Vec::new() }),
      Some(_) => {
        let children_str: String = captures[2].to_string();

        let children: Vec<&str> = children_str.split(", ").collect();

        let content: Vec<RawContent> = children
            .iter()
            .map(|child| build_content(*child))
            .collect();

        raw_rules.push(RawRule {
          identifier,
          content
        });
      }
    }
  }

  raw_rules
}

fn build_content(child: &str) -> RawContent {
  lazy_static! {
    static ref CHILD_RE: regex::Regex =
      regex::Regex::new(r"^(\d+) ([\w ]+) (?:bag|bags)$")
        .unwrap();
  }

  let captures = CHILD_RE.captures(child).unwrap();
  let amount_str: &str = captures.get(1).unwrap().as_str();
  let amount: u64 = amount_str.parse::<u32>().unwrap() as u64;
  let identifier: String = captures.get(2).unwrap().as_str().to_string();

  RawContent { amount, identifier }
}

fn build_bag_content(rules: &Vec<RawRule>, contents: &Vec<RawContent>) -> Vec<(u64, Bag)> {
  let mut bags: Vec<(u64, Bag)> = Vec::new();

  for content in contents {
    let sub_content = match rules
      .iter()
      .find(|r| *r.identifier == content.identifier) {
        None => Vec::new(),
        Some(rule) => {
          build_bag_content(&rules, &rule.content)
        }
      };

    bags.push((
      content.amount,
      Bag {
        identifier: content.identifier.clone(),
        content: sub_content
      }));
  }

  bags
}

fn build_bags(rules: &Vec<RawRule>) -> Vec<(u64, Bag)> {
  rules
    .iter()
    .map(|rule|
      (1, Bag {
        identifier: rule.identifier.clone(),
        content: build_bag_content(&rules, &rule.content)
      })
    )
    .collect()
}

fn contains(bag: &Bag, to_find: (u64, &str)) -> bool {
  if bag.identifier == to_find.1 {
    return true;
  }

  for (_, child) in bag.content.iter() {
    if contains(child, to_find) {
      return true;
    }
  }

  false
}

fn count_in_children(bags: &Vec<(u64, Bag)>, to_find: (u64, &str)) -> u32 {
  let mut total = 0;

  for (_, bag) in bags.iter() {
    if contains(&bag, to_find) && bag.identifier != to_find.1 {
      total += 1;
    }
  }

  total
}

fn compute_child_capacity(bags: &Vec<(u64, Bag)>) -> u64 {
  bags
    .iter()
    .map(|(amount, bag)| amount * compute_child_capacity(&bag.content))
    .fold(1, |acc, x| acc + x)
}

fn compute_bag_capacity(bags: &Vec<(u64, Bag)>, to_find: (u64, &str)) -> u64 {
  let parent: Option<&(u64, Bag)> =
    bags
      .iter()
      .find(|(amount, bag)| *amount == to_find.0 && bag.identifier == to_find.1);

  let (parent_amount, bag) = parent.unwrap();

  bag.content
    .iter()
    .map(|(amount, bag)| parent_amount * amount * compute_child_capacity(&bag.content))
    .fold(0, |acc, size| acc + size)
}
