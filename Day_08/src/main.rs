use std::fs;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use num_integer::lcm;


fn part1(strings:Vec<&str>){
    let mut positions:HashMap<String, Vec<String>>=HashMap::new();
    let sequence:String = String::from(strings[0]);
    for line in strings[2..].iter() {
        let member: Vec<&str>=line.split_whitespace().collect();
        let mut tmp_vec:Vec<String>=Vec::new();
        tmp_vec.insert(0,member[2].replace("(","").replace(",","").to_string());
        tmp_vec.insert(1,member[3].replace(")","").to_string());
        positions.insert(String::from(member[0].to_string()),tmp_vec.clone());
    }

    let mut steps:i32=0;
    let mut curr_location:String=String::from("AAA");

    while curr_location!= "ZZZ" {
        for direction in sequence.chars().into_iter(){
            match direction{
                'R'=>{curr_location=positions.get(&curr_location).unwrap().get(1).unwrap().to_string();},
                'L'=>{curr_location=positions.get(&curr_location).unwrap().get(0).unwrap().to_string();},
                _=>{},
            }
            steps+=1;
            if curr_location=="ZZZ"{break;}
        }
    }
    println!("   Part 1: {}",steps);
}


fn calculate_steps(sequence:String,positions:HashMap<String,Vec<String>>,elem:String)->(String,i64){
    let mut steps:i64=0;
    let mut curr_location:String=String::from(elem);

    loop {
        for direction in sequence.chars().into_iter(){
            match direction{
                'R'=>{curr_location=positions.get(&curr_location).unwrap().get(1).unwrap().to_string();},
                'L'=>{curr_location=positions.get(&curr_location).unwrap().get(0).unwrap().to_string();},
                _=>{},
            }
            steps+=1;
            if curr_location.chars().last().unwrap() == 'Z'{return (curr_location,steps);};
        }
    }
}

fn part2(strings:Vec<&str>){
    let mut positions:HashMap<String, Vec<String>>=HashMap::new();
    let sequence:String = String::from(strings[0]);
    let mut curr_points:HashSet<String>=HashSet::new();
    for line in strings[2..].iter() {
        let member: Vec<&str>=line.split_whitespace().collect();
        let mut tmp_vec:Vec<String>=Vec::new();
        tmp_vec.insert(0,member[2].replace("(","").replace(",","").to_string());
        tmp_vec.insert(1,member[3].replace(")","").to_string());
        positions.insert(String::from(member[0].to_string()),tmp_vec.clone());
        if member[0].chars().last().unwrap()=='A'{curr_points.insert(member[0].to_string());}
    }

    let mut steps:Vec<i64>=Vec::new();
    for elem in curr_points{
        let result_tuple= calculate_steps(sequence.clone(), positions.clone(),elem);
        steps.push(calculate_steps(sequence.clone(), positions.clone(), result_tuple.0).1);
    }

    println!("   Part 2: {}", steps.iter().fold(1, |acc, &x| lcm(acc, x)));   
}

fn main() {
    let file =fs::read_to_string("src/data.txt"
    .to_string())
    .expect("File not found");
    let strings:Vec<&str>=file.split("\n").collect();

    println!("--- Day 8: Haunted Wasteland ---\n\n");
    part1(strings.clone());
    part2(strings.clone());
    println!("\n");
}


