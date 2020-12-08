use std::fs;

extern crate regex;
#[macro_use] extern crate lazy_static;

#[derive(Debug)]
struct RawContent {
  amount: u32,
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
  content: Vec<(u32, Bag)>
}

fn main() {
  let contents = fs::read_to_string("./example.txt")
    .expect("Cannot load file.");

  let rules: Vec<RawRule> = evaluate_rules(&contents);
  // println!("{:?}", rules);

  let bags: Vec<(u32, Bag)> = build_bags(&rules);
  // println!("{:?}", bags);

  let bags_count = browse_bags(&bags, (1, "shiny gold"), 0, 0, "parent");

  println!("{}", bags_count);
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
  let amount: u32 = amount_str.parse::<u32>().unwrap() as u32;
  let identifier: String = captures.get(2).unwrap().as_str().to_string();

  RawContent { amount, identifier }
}

fn build_bag_content(rules: &Vec<RawRule>, contents: &Vec<RawContent>) -> Vec<(u32, Bag)> {
  let mut bags: Vec<(u32, Bag)> = Vec::new();

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

fn build_bags(rules: &Vec<RawRule>) -> Vec<(u32, Bag)> {
  rules
    .iter()
    .map(|rule|
      (0, Bag {
        identifier: rule.identifier.clone(),
        content: build_bag_content(&rules, &rule.content)
      })
    )
    .collect()
}

fn browse_bags(bags: &Vec<(u32, Bag)>, to_find: (u32, &str), idx: usize, valid_bags: u32, mode: &str) -> u32 {
  // println!("received {:?} {} {}", to_find, idx, valid_bags);

  if bags.len() == 0 {
    return 0;
  }

  // println!("Going through: {} {}...", mode, bags[idx].1.identifier,);
  // println!("Going through: {} {}... {:?}", mode, bags[idx].1.identifier, &bags[idx].1.content);

  let valid_in_children = browse_bags(&bags[idx].1.content, to_find, 0, 0, "child");
  // println!("{} in children", valid_in_children);

  if mode == "child" && bags[idx].1.identifier == to_find.1 {
    // println!("{} {}", bags[idx].0, to_find.0);
    if bags[idx].0 >= to_find.0 {
      return 1;
    }

  }

  if idx + 1 < bags.len() {
    return browse_bags(bags, to_find, idx + 1, valid_bags + valid_in_children, "parent");
  } else {
    return valid_bags;
  }
}
