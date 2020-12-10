use std::{collections::HashMap, fs};
use std::iter::FromIterator;

#[derive(Debug, Clone)]
struct Instruction {
  operator: String,
  args: Vec<char>
}

fn main() {
  let contents = fs::read_to_string("./example.txt")
    .expect("Cannot load file");

  let instructions = compute_instructions(contents);

  let acc = debug(instructions);

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

fn swap_instructions(
  instructions: &Vec<Instruction>,
  left: &(usize, String),
  right: &(usize, String)
) -> Vec<Instruction> {
    let mut copy = instructions.clone();

    let left_instruction = instructions.get(left.0).unwrap();
    let right_instruction = instructions.get(right.0).unwrap();

    std::mem::replace(
      &mut copy[left.0],
      Instruction {
        operator: right_instruction.operator.clone(),
        args: left_instruction.args.clone()
      }
    );

    std::mem::replace(
      &mut copy[right.0],
      Instruction {
        operator: left_instruction.operator.clone(),
        args: right_instruction.args.clone()
      }
    );

    copy
  }

fn debug(
  instructions: Vec<Instruction>,
) -> i64 {
  let mut trace: HashMap<usize, u8> = HashMap::new();
  let mut to_swap_ids: Vec<(usize, String)> = Vec::new();
  let mut dummy: Vec<(usize, String)> = Vec::new();
  let acc = 0;

  let (result, mut to_swap_ids, success) =
    evaluate(&instructions, 0, &mut trace, acc, &mut to_swap_ids);

  println!("0: {} {:?} {}", result, to_swap_ids, success);

  if success || to_swap_ids.len() == 0 {
    return result;
  }

  let mut l: usize = 0;
  let mut r = to_swap_ids.len() - 1;

  while l != r {
    let left = to_swap_ids[l].clone();
    let right = to_swap_ids[r].clone();

    if left != right {
      to_swap_ids[l] = right.clone();
      to_swap_ids[r] = left.clone();

      let instructions_copy = swap_instructions(
        &instructions,
        &left,
        &right
      );
      trace.clear();

      let (result, _, success) =
        evaluate(&instructions_copy, 0, &mut trace, acc, &mut dummy);

      if success {
        return result;
      }
    } else {
      l += 1;
      r -= 1;
    }
  }

  result
}

fn evaluate(
  instructions: &Vec<Instruction>,
  idx: usize,
  trace: &mut HashMap<usize, u8>,
  acc: i64,
  to_swap_ids: &mut Vec<(usize, String)>
) -> (i64, Vec<(usize, String)>, bool) {
  let current = &instructions[idx];

  println!("{} {:?}", current.operator, current.args);

  if already_evaluated(idx, trace) {
    println!("Terminating.");
    return (acc, to_swap_ids.clone(), false);
  }

  trace.insert(idx, 1);

  if current.operator == "nop" {
    to_swap_ids.push((idx, current.operator.clone()));
    return evaluate(instructions, idx + 1, trace, acc, to_swap_ids);
  } else if current.operator == "jmp" {
    to_swap_ids.push((idx, current.operator.clone()));
    // println!("jmp {:?}", current.args);
    let (sign, offset) = extract(&current.args);

    if sign == '+' {
        return evaluate(instructions, idx + (offset as usize), trace, acc, to_swap_ids);
    } else if sign == '-' {
      return evaluate(instructions, idx - (offset as usize), trace, acc, to_swap_ids);
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
      println!("Returning {} {:?} {}", new_acc, to_swap_ids, true);
      return (new_acc, to_swap_ids.clone(), true);
    }

    if sign == '+' {
      return evaluate(instructions, idx + 1, trace, new_acc, to_swap_ids);
    } else if sign == '-' {
      // println!("{} {}", acc, offset);
      return evaluate(instructions, idx + 1, trace, new_acc, to_swap_ids);
    }
  }

  (acc, to_swap_ids.clone(), false)
}

fn extract(args: &Vec<char>) -> (char, i64) {
  let mut args_copy = args.clone();
  let sign: Vec<char> = args_copy.drain(0..1).collect();
  let number_str: String = String::from_iter(args_copy);
  let number: i64 = number_str.parse::<i64>().unwrap() as i64;

  (sign[0], number)
}

fn already_evaluated(idx: usize, trace: &mut HashMap<usize, u8>) -> bool {
  match trace.get(&idx) {
    None => false,
    Some(_) => true
  }
}