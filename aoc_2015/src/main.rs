use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::time::Instant;
use once_cell::sync::Lazy;


#[derive(Debug)]
struct Point (u16,u16);
impl Point {
    pub fn new(p1: u16, p2: u16) -> Point {
        Point(p1, p2)
    }
    pub fn from_str(p1: &str, p2: &str) -> Point {
        let p1 = p1.parse::<u16>().unwrap();
        let p2 = p2.parse::<u16>().unwrap();
        Point::new(p1,p2)
    }
}
#[derive(Debug)]
enum Instruction {
    ON(Point, Point),
    OFF(Point, Point),
    TOGGLE(Point, Point),
}

impl Instruction {
    pub fn new(line: &str) -> Instruction {
        let re = Regex::new(r"(?<type>turn off|turn on|toggle) (?<p1>\d+),(?<p2>\d+) through (?<p3>\d+),(?<p4>\d+)").unwrap();
        let caps = re.captures(&line).unwrap();
        let a = Point::from_str(&caps["p1"], &caps["p2"]);
        let b = Point::from_str(&caps["p3"], &caps["p4"]);
        match &caps["type"] {
            "turn on" => Instruction::ON(a,b),
            "turn off" => Instruction::OFF(a,b),
            "toggle" => Instruction::TOGGLE(a,b),
            _ => panic!("Sin match")
        }
    }
}

fn main() {
    let f = File::open("input").unwrap();
    let reader = BufReader::new(f);
    let grid: Vec<Vec<u8>> = vec![vec![0;1000]; 1000];

    let start = Instant::now();
    for (i, line) in reader.lines().enumerate(){
        let line =  match line { Ok(line) => line, Err(e) => panic!("{e}") };
        let inst = Instruction::new(&line);
        println!("{:?}", inst);
    }
    println!("Time: {:?}", start.elapsed());
}
