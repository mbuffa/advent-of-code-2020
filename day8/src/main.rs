use std::{collections::HashMap, fs};
use std::iter::FromIterator;

#[derive(Debug)]
struct Instruction {
  operator: String,
  args: Vec<char>
}

fn main() {
  let contents = fs::read_to_string("./example.txt")
    .expect("Cannot load file");

  let instructions = compute_instructions(contents);

  let mut trace: HashMap<usize, u8> = HashMap::new();
  let acc: i64 = evaluate(&instructions, 0, &mut trace, 0);

  // println!("{:?}", instructions);
  println!("{}", acc);
}

fn compute_instructions(buffer: String) -> Vec<Instruction> {
  let raw_instructions: Vec<&str> = buffer.split("\n").collect();

  let mut instructions = Vec::new();

  for raw_instruction in raw_instructions.iter() {
    let operands: Vec<&str> = raw_instruction.split(" ").collect();

    instructions.push(Instruction {
      operator: operands[0].to_owned(),
      args: operands[1].chars().collect()
    });
  }

  instructions
}

fn evaluate(instructions: &Vec<Instruction>, idx: usize, trace: &mut HashMap<usize, u8>, acc: i64) -> i64 {
  let current = &instructions[idx];

  println!("{} {:?}", current.operator, current.args);

  if already_evaluated(idx, trace) {
    println!("Terminating.");
    return acc;
  }

  trace.insert(idx, 1);

  if current.operator == "nop" {
    // println!("nop");
    if already_evaluated(idx + 1, trace) {
      println!("Already evaluated nop next");
      let (sign, offset) = extract(&current.args);

      if sign == '+' {
        return evaluate(instructions, idx + (offset as usize), trace, acc);
      } else if sign == '-' {
        return evaluate(instructions, idx - (offset as usize), trace, acc);
      }
    } else {
      return evaluate(instructions, idx + 1, trace, acc);
    }
  } else if current.operator == "jmp" {
    // println!("jmp {:?}", current.args);
    let (sign, offset) = extract(&current.args);

    if sign == '+' {
      if already_evaluated(idx + (offset as usize), trace) {
        println!("Already evaluated jmp next +");
        return evaluate(instructions, idx + 1, trace, acc);
      } else {
        let range: Vec<usize> = (idx..(offset as usize)).collect();

        // Blacklisting range to replace instruction later.
        for &step in range.iter() {
          trace.insert(step, 1);
        }

        return evaluate(instructions, idx + (offset as usize), trace, acc);
      }
    } else if sign == '-' {
      if already_evaluated(idx - (offset as usize), trace) {
        println!("Already evaluated jmp next -");
        return evaluate(instructions, idx + 1, trace, acc);
      } else {
        let range: Vec<usize> = ((idx - offset as usize)..(idx)).collect();

        // Blacklisting range to replace instruction later.
        for &step in range.iter() {
          trace.insert(step, 1);
        }

        return evaluate(instructions, idx - (offset as usize), trace, acc);
      }
    }
  } else if current.operator == "acc" {
    // println!("acc {}", acc);
    let (sign, offset) = extract(&current.args);

    let mut new_acc: i64 = 0;

    if sign == '+' {
      new_acc = acc + offset;
    } else if sign == '-' {
      new_acc = acc - offset;
    }

    if idx == instructions.len() - 1 {
      return new_acc;
    }

    if sign == '+' {
      return evaluate(instructions, idx + 1, trace, new_acc);
    } else if sign == '-' {
      println!("{} {}", acc, offset);
      return evaluate(instructions, idx + 1, trace, new_acc);
    }
  }

  acc
}

fn extract(args: &Vec<char>) -> (char, i64) {
  let mut args_copy = args.clone();
  let sign: Vec<char> = args_copy.drain(0..1).collect();
  let number_str: String = String::from_iter(args_copy);
  let number: i64 = number_str.parse::<u64>().unwrap() as i64;

  (sign[0], number)
}

fn already_evaluated(idx: usize, trace: &mut HashMap<usize, u8>) -> bool {
  match trace.get(&idx) {
    None => false,
    Some(_) => true
  }
}