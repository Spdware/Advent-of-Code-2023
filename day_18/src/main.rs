//This solution utilize shoelace formula and Pick's theorem

use std::collections::HashMap;
use std::fs;

fn create_border(trenches:Vec<(&str,i64)>)-> Vec<(i64,i64)>{
    let directions:HashMap<&str,(i64,i64)>=HashMap::from([("U",(-1,0)),("D",(1,0)),("L",(0,-1)),("R",(0,1))]);
    let mut border:Vec<(i64,i64)>=Vec::from([(0,0)]);
    for elem in trenches {
        let current:(i64,i64)=*border.iter().last().unwrap();
        let direction:(i64,i64)=*directions.get(elem.0).unwrap();
        border.push((current.0+direction.0*elem.1,current.1+direction.1*elem.1));
    }
    border
}


fn part1(strings:Vec<&str>)->i64 {
    let mut trenches:Vec<(&str,i64)>=Vec::new();
    let mut boundaries:i64=0;
    for line in strings {
        let row:Vec<&str>=line.split_whitespace().collect();
        trenches.push((row[0],row[1].to_string().parse::<i64>().unwrap()));
        boundaries+=row[1].to_string().parse::<i64>().unwrap();
    }
    let terrain:Vec<(i64,i64)>=create_border(trenches);
    let mut area:i64=0;
    for i in 1..terrain.len() {
        let x:i64=terrain.get(i).unwrap().0;
        let y1:i64=terrain.get(i-1).unwrap().1;
        let y2:i64=terrain.get((i+1)%terrain.len()).unwrap().1;
        area+=x*(y1-y2);
    }
    let x:i64=terrain.get(0).unwrap().0;
    let y1:i64=terrain.iter().last().unwrap().1;
    let y2:i64=terrain.get(1).unwrap().1;
    area+=x*(y1-y2);
    area= area.abs()/2;
    return (area-boundaries/2+1)+boundaries;
}

fn part2(strings:Vec<&str>)->i64 {
    let mut trenches:Vec<(&str,i64)>=Vec::new();
    let directions:&str="RDLU";
    let mut boundaries:i64=0;
    for line in strings {
        let row:Vec<&str>=line.split_whitespace().collect();
        let code=&row[2][2..8];
        let ind=code[5..6].to_string().parse::<usize>().unwrap();
        trenches.push((directions.get(ind..ind+1).unwrap(),i64::from_str_radix(&code[0..5],16).unwrap()));
        boundaries+=i64::from_str_radix(&code[0..5],16).unwrap();
    }
    let terrain:Vec<(i64,i64)>=create_border(trenches);
    let mut area:i64=0;
    for i in 1..terrain.len() {
        let x:i64=terrain.get(i).unwrap().0;
        let y1:i64=terrain.get(i-1).unwrap().1;
        let y2:i64=terrain.get((i+1)%terrain.len()).unwrap().1;
        area+=x*(y1-y2);
    }
    let x:i64=terrain.get(0).unwrap().0;
    let y1:i64=terrain.iter().last().unwrap().1;
    let y2:i64=terrain.get(1).unwrap().1;
    area+=x*(y1-y2);
    area= area.abs()/2;
    return (area-boundaries/2+1)+boundaries;
}

fn main() {
    let file =fs::read_to_string("src/data.txt"
        .to_string())
        .expect("File not found");
    let strings:Vec<&str>=file.split("\n").collect();

    println!("--- Day 18: Lavaduct Lagoon ---\n\n");
    println!("   Part 1: {}",part1(strings.clone()));
    println!("   Part 2: {}",part2(strings.clone()));
    println!("\n");
}