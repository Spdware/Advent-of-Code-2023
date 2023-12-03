use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn part1(filepath : String){
    let mut cubes : HashMap<&str, &i32> = HashMap::new();
    cubes.insert("red",&12);
    cubes.insert("green",&13);
    cubes.insert("blue",&14);
    let mut sum_id=0;
    let file = BufReader::new(File::open(filepath).expect("Cannot open file.txt"));
    
    for row in file.lines(){
        let mut valid = true;
        let linetmp = row.unwrap().replace(",","").replace(";","");
        let line : Vec<&str> = linetmp.split_whitespace().collect();
        for i in (2..(line.len()-1)).step_by(2){
            if line[i].parse::<i32>().unwrap() > *cubes[line[i+1]]{
                valid = false;
                break;
            } 
        }
        if valid{sum_id += String::from(line[1].to_string()).replace(":","").parse::<i32>().unwrap();}
    }
    
    println!("Part 1: {}", sum_id);
}

fn part2(filepath : String){
    let mut sum_id=0;
    let file = BufReader::new(File::open(filepath).expect("Cannot open file.txt"));
    
    for row in file.lines(){
        let mut cubes : HashMap<&str, i32> = HashMap::new();
        cubes.insert("red",0);
        cubes.insert("green",0);
        cubes.insert("blue",0);

        let linetmp = row.unwrap().replace(",","").replace(";","");
        let line : Vec<&str> = linetmp.split_whitespace().collect();
        
        for i in (2..(line.len()-1)).step_by(2) {
            let value = line[i].parse::<i32>().unwrap();
            if line[i].parse::<i32>().unwrap() > cubes[line[i+1]]{
                cubes.insert(line[i+1],value);
            } 
        }
        sum_id += cubes["red"]*cubes["blue"]*cubes["green"];
    }
    println!("Part 2: {}", sum_id);
}

fn main(){
    let filepath = String::from("data.txt");
    println!("--- Day 2: Cube Conundrum ---\n\n");
    part1(filepath.clone());
    part2(filepath.clone());
    println!("\n\n-------------------------");
}
