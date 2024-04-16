//Puzzle link : https://www.codingame.com/training/expert/unfolding-paper
use core::fmt::{self, Debug};
use std::{io, collections::HashMap};

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

const LEFT   : u8 = 0b0001;
const RIGHT  : u8 = 0b0010;
const TOP    : u8 = 0b0100;
const BOTTOM : u8 = 0b1000;

#[derive(Clone, Default)]
struct Shape(u8,);

impl Debug for Shape {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Shape : {} ", self.0)?;

        let mut connections = Vec::new();

        if self.0 & LEFT != 0 {
            connections.push("Left");
        }
        if self.0 & RIGHT != 0 {
            connections.push("Right");
        }
        if self.0 & BOTTOM != 0 {
            connections.push("Bottom");
        }
        if self.0 & TOP != 0 {
            connections.push("Top");
        }
        write!(f, "[{}]", connections.join(", "))?;
        Ok(())
    }
}

impl Shape {
    fn connect_to(&mut self, sides: u8){
        self.0 |= sides;
    }
}

#[derive(Default)]
pub struct Sheet {
    shape_counts : HashMap<u8, usize>,
}

impl Sheet {
    fn new(paper : &mut Vec<String>) -> Self {
        let shapes = find_shapes(paper);


        let mut map = HashMap::new();

        for shape in shapes.iter() {
            map.entry(shape.0)
            .and_modify(|count| *count += 1)
            .or_insert_with(|| 1);
        }
        Self{ shape_counts: map }
    }
}

fn flood_fill(map: &mut Vec<String>, shapes: &mut Vec<Shape>, row: usize, col: usize, shape_id: usize) {
    let rows = map.len();
    let cols = map[0].len();

    // Check if current position is within bounds and is a '#'
    if row >= rows || col >= cols || map[row].chars().nth(col).unwrap() != '#' {
        return;
    }

    // Mark current position as handled (# becomes .)
    map[row] = format!("{}.{}", map[row][0..col].to_string(), map[row][col+1..].to_string());

    if col == 0 {
        shapes[shape_id].connect_to(LEFT);
    }
    if col == cols-1 {
        shapes[shape_id].connect_to(RIGHT);
    }
    if row == 0 {
        shapes[shape_id].connect_to(TOP);
    }
    if row == rows-1 {
        shapes[shape_id].connect_to(BOTTOM);
    }

    // Recursively flood-fill adjacent positions
    flood_fill(map, shapes, row + 1, col, shape_id);
    flood_fill(map, shapes, row - 1, col, shape_id);
    flood_fill(map, shapes, row, col + 1, shape_id);
    flood_fill(map, shapes, row, col - 1, shape_id);
}

fn find_shapes(paper: &mut Vec<String>) -> Vec<Shape> {
    let rows = paper.len();
    let cols = paper[0].len();
    let mut shape_id = 0;

    let mut shapes = Vec::new();

    for row in 0..rows {
        for col in 0..cols {
            if paper[row].chars().nth(col).unwrap() == '#' {
                // Found a new shape, perform flood-fill to label it
                shapes.push(Shape::default());
                flood_fill(paper, &mut shapes, row, col, shape_id);
                // Increment shape ID for the next shape
                shape_id += 1;
            }
        }
    }
    shapes
}


fn main() {

    let mut paper = Vec::new();

    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, u32);
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let inputs = input_line.split(" ").collect::<Vec<_>>();
    let _ = parse_input!(inputs[0], i32);
    let h = parse_input!(inputs[1], i32);
    for _ in 0..h as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let row = input_line.trim_matches('\n').to_string();
        paper.push(row);
    }

    let sheet = Sheet::new(&mut paper);
    let mut count = 0;

    for (&shape, &shape_count) in sheet.shape_counts.iter() {
        count += shape_count as u32 * if shape == 0 {
            //Shape is not connected to any side
            2_u32.pow(2*n)
        } else if shape == LEFT || shape == TOP {
            //Shape is only connected to one folding side
            (2_u32.pow(n-1))*(2_u32.pow(n))
        } else if shape == RIGHT || shape == BOTTOM {
            //Shape is only connected to one NON folding side
            (1+2_u32.pow(n-1))*(2_u32.pow(n))
        } else if shape == (LEFT | TOP) {
            //Shape is connected to both folding sides
            2_u32.pow(2*(n-1))   
        } else if shape == RIGHT | BOTTOM {
            //Shape is connected to both NON folding sides
            (1 + 2_u32.pow(n-1)).pow(2)
        } else if shape == TOP | RIGHT  || 
        shape == LEFT | BOTTOM  {
            (1+2_u32.pow(n-1))*2_u32.pow(n-1)
        } else if shape == TOP | BOTTOM  ||
        shape == LEFT | RIGHT  {
            //Shape is connected to both NON folding sides
            2_u32.pow(n)
        } else if shape == (LEFT | TOP | RIGHT) || 
        shape == (LEFT | TOP | BOTTOM) {
            2_u32.pow(n-1)
        } else if shape == (TOP | RIGHT | BOTTOM) || 
        shape == ( LEFT | RIGHT | BOTTOM) {
            (1+2_u32.pow(n-1))
        } else if shape == (LEFT | RIGHT | TOP | BOTTOM) {
            //Shape is connected to all sides
            1   
        }else { 
            0 
        } ;
    }

    println!("{count}");

}

