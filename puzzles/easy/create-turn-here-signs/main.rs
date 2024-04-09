// Puzzle link : https://www.codingame.com/ide/puzzle/create-turn-here-signs

use core::fmt;
use std::{io, convert::{TryFrom, TryInto}};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right
}

impl TryFrom<&str> for Direction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "left" {
            Ok(Self::Left)
        } else if value == "right" {
            Ok(Self::Right)
        }else {
            Err(format!("Value Error : {}", value))
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Left => write!(f, "<")?,
            Direction::Right => write!(f, ">")?,
        };
        Ok(())
    }
}

#[derive(Debug)]
pub struct Arrow {
    direction : Direction,
    count : usize,
    height: usize,
    thickness: usize,
    spacing: usize,
    indent: usize
}

impl Arrow {
    pub fn new(s: &str) -> Option<Self> {
        let mut split = s.split_whitespace();
        let direction = split.next()?.try_into().ok()?;
        let count = split.next()?.parse::<usize>().ok()?;
        let height = split.next()?.parse::<usize>().ok()?;
        let thickness = split.next()?.parse::<usize>().ok()?;
        let spacing = split.next()?.parse::<usize>().ok()?;
        let indent = split.next()?.parse::<usize>().ok()?;
        Some(Self{ direction, count, height, thickness, spacing, indent })
    }
}

impl fmt::Display for Arrow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arrow = self.direction.to_string().repeat(self.thickness);
        let spacing = ' '.to_string().repeat(self.spacing);
        let line = vec![arrow; self.count].join(&spacing);
        let indents = match self.direction {
            Direction::Left => (0..=self.height/2).rev().chain(1..=self.height/2).collect::<Vec<usize>>(),
            Direction::Right => (0..=self.height/2).chain((0..self.height/2).rev()).collect::<Vec<usize>>(),
        };
        for i in indents {
            write!(f, "{}", vec![" "; self.indent*i].join(""))?;
            writeln!(f, "{line}")?;
        }
        Ok(())
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let input = input_line.trim_matches('\n').to_string();

    if let Some(arrow) = Arrow::new(&input) {
        eprintln!("{arrow:?}");
        print!("{arrow}");
    }
}
