use std::{collections::HashMap, fs};

extern crate regex;
#[macro_use] extern crate lazy_static;

struct Passport {
  fields: HashMap<String, String>
}

impl Passport {
  fn load_batch(filename: &str) -> Vec<Passport> {
    let data = fs::read_to_string(filename)
      .expect("Can't load file.");

    let passport_data: Vec<&str> = data.split("\n\n").collect();

    passport_data
      .iter()
      .map(|data| Passport::load(data))
      .collect()
  }

  fn has_valid_value(key: &String, value: &str) -> bool {
    lazy_static! {
      static ref BYR_RE: regex::Regex =
        regex::Regex::new(r"^19[2-9][0-9]|200[0-2]$").unwrap();

      static ref IYR_RE: regex::Regex =
        regex::Regex::new(r"^201[0-9]|2020$").unwrap();

      static ref EYR_RE: regex::Regex =
        regex::Regex::new(r"^202[0-9]|2030$").unwrap();

      static ref HGT_RE: regex::Regex =
        regex::Regex::new(r"(?x)
        ^
        ^1(?:[5-8][0-9]|[5-9][0-3])cm$
        |
        ^(?:59|6[0-9]|7[0-6])in$
        $
        ").unwrap();

      static ref HCL_RE: regex::Regex =
        regex::Regex::new(r"^#[0-9a-f]{6}$").unwrap();

      static ref ECL_RE: regex::Regex =
        regex::Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth){1}$").unwrap();

      static ref PID_RE: regex::Regex =
        regex::Regex::new(r"^\d{9}$").unwrap();
    }

    match key.as_str() {
      "byr" => {
        BYR_RE.is_match(value)
      },
      "iyr" => {
        IYR_RE.is_match(value)
      },
      "eyr" => {
        EYR_RE.is_match(value)
      },
      "hgt" => {
        HGT_RE.is_match(value)
      },
      "hcl" => {
        HCL_RE.is_match(value)
      },
      "ecl" => {
        ECL_RE.is_match(value)
      },
      "pid" => {
        PID_RE.is_match(value)
      },
      "cid" => true,
      _ => false
    }
  }

  fn load(data: &str) -> Passport {
    let re = regex::Regex::new(r" |\n").unwrap();
    let fields_data: Vec<&str> = re.split(data).collect();
    let mut fields: HashMap<String, String> = HashMap::new();

    for field_pair in fields_data {
      let pair: Vec<&str> = field_pair.split(":").collect();

      fields.insert(pair[0].to_owned(), pair[1].to_owned());
    }

    Passport {
      fields: fields
    }
  }

  fn is_valid(&self) -> Option<bool> {
    let required_fields: Vec<String> = vec![
      "byr".to_owned(),
      "iyr".to_owned(),
      "eyr".to_owned(),
      "hgt".to_owned(),
      "hcl".to_owned(),
      "ecl".to_owned(),
      "pid".to_owned(),
    ];

    let mut valid = required_fields
      .iter()
      .map(|field| self.fields.contains_key(field).clone())
      .all(|present| present);

    if valid == false {
      return None;
    }

    valid = self.fields
      .keys().into_iter()
      .all(|key| Passport::has_valid_value(key, &self.fields[key]));

    if valid {
      Some(true)
    } else {
      None
    }
  }
}

fn main() {
  let passports = Passport::load_batch("./input.txt");

  let valid_count = passports
    .iter()
    .filter_map(|passport| passport.is_valid())
    .count();

  println!("{} valid passports.", valid_count);
}
