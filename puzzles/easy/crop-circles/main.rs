//Link to puzzle : https://www.codingame.com/ide/puzzle/crop-circles

use core::fmt;
use std::io;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Debug)]
pub enum Action {
    PLANT,
    MOW,
    PLANTMOW
}

impl Action {
    pub fn new(instruction: &str) -> Self {
        if instruction.starts_with("PLANTMOW") {
            Action::PLANTMOW
        } else if instruction.starts_with("PLANT") {
            Action::PLANT
        } else {
            Action::MOW
        }
    }

    pub fn get_shift(&self) -> usize {
        match self {
            Action::PLANT => 5,
            Action::MOW => 0,
            Action::PLANTMOW => 8,
        }
    }

    pub fn do_action_on(&self, crop: &mut bool) {
        match self {
            Action::PLANT => *crop = true,
            Action::MOW => *crop = false,
            Action::PLANTMOW => *crop = !*crop,
        }
    }
}

#[derive(Debug)]
pub struct Circle {
    x : usize,
    y : usize,
    diam : usize,
    action: Action
}

impl Circle {
    pub fn new(instruction: &str) -> Option<Self> {
        let mut instruction = instruction.clone();
        let action = Action::new(instruction);

        instruction = &instruction[action.get_shift()..];

        let x = instruction.chars().nth(0)? as usize - b'a' as usize;
        let y = instruction.chars().nth(1)? as usize - b'a' as usize;
        let diam = instruction[2..].parse::<usize>().ok()?;

        Some(Self{ x, y, diam, action })
    }

    pub fn contains(&self, (x, y) : (usize, usize)) -> bool {
        let r_squared = ((self.diam) as f32 /2.0).powf(2.0) as f32;

        let epsilon: f32 = 1e-5;

        (((x-self.x).pow(2)+(y-self.y).pow(2)) as f32 - r_squared) < epsilon
    }

    pub fn process_on(&self, field: &mut Field ) {
        field.map.iter_mut().enumerate().for_each(|(y, row)| 
            row.iter_mut().enumerate().for_each(|(x, crop)| {
                if self.contains((x, y)) {
                    self.action.do_action_on(crop);
                }

            } )
        );
    }
}

pub struct Field {
    map : Vec<Vec<bool>>
}

impl Field {
    pub fn new() -> Self {
        Self {map : vec![vec![true; 19]; 25]}
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.map.iter() {
            for crop in row.iter() {
                match crop {
                    true => write!(f, "{{}}")?,
                    false => write!(f, "  ")?,
                };
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn main() {
    let mut field = Field::new();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let instructions = input_line.trim_matches('\n').to_string();

    for instr in instructions.split_whitespace() {

        if let Some(circle) = Circle::new(instr) {
            circle.process_on(&mut field);
        }
    }

    print!("{field}");
}

