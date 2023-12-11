use std::collections::{HashMap, VecDeque};
use std::fs;

fn part1(strings: Vec<&str>){
    let mut universe:Vec<&str>=Vec::new();
    let mut galaxies:VecDeque<(i32,i32)>=VecDeque::new();
    for line in strings.iter(){
        if !line.contains('#'){
            universe.push(line);
        }
        universe.push(line);
    }


    let mut empty_plus:i32=0;
    let len_column:usize = universe.clone().get(0).unwrap().len();
    let len_row:usize=universe.clone().len();
    for j in 0..len_column{
        let mut empty:bool=true;
        for i in 0..len_row{
            if universe.get(i).unwrap().chars().nth(j).unwrap()=='#'{
                empty=false;
                galaxies.push_back((i as i32,j as i32+empty_plus));
            }
        }
        if empty{ empty_plus+=1; }
    }
    let mut min_distance:i32=0;
    while !galaxies.clone().is_empty(){
        let cur_galaxy=galaxies.pop_front().unwrap();
        for galaxy in &galaxies{
            min_distance+=(cur_galaxy.0-galaxy.0).abs()+(cur_galaxy.1-galaxy.1).abs();
        }
    }
    println!("   Part 1: {}",min_distance);
}

fn part2(strings: Vec<&str>){
    let age_offset:i32=1000000;
    let mut galaxies:VecDeque<(i32,i32)>=VecDeque::new();
    let mut rows_offset:HashMap<usize,i32>=HashMap::new();
    let mut row_empty_plus:i32=0;
    for (i,line) in strings.iter().enumerate(){
        rows_offset.insert(i,row_empty_plus);
        if !line.contains('#'){
            row_empty_plus+=age_offset-1;
        }
    }

    let mut empty_plus:i32=0;
    let len_column:usize = strings.clone().get(0).unwrap().len();
    let len_row:usize=strings.clone().len();
    for j in 0..len_column{
        let mut empty:bool=true;
        for i in 0..len_row{
            if strings.get(i).unwrap().chars().nth(j).unwrap()=='#'{
                empty=false;
                galaxies.push_back((i as i32+rows_offset.get(&i).unwrap(),j as i32+empty_plus));
            }
        }
        if empty{ empty_plus+=age_offset-1; }
    }
    let mut min_distance:i64=0;
    while !galaxies.clone().is_empty(){
        let cur_galaxy=galaxies.pop_front().unwrap();
        for galaxy in &galaxies{
            min_distance+=(cur_galaxy.0-galaxy.0).abs() as i64+(cur_galaxy.1-galaxy.1).abs() as i64;
        }
    }
    println!("   Part 1: {}",min_distance);

}

fn main() {
    let file =fs::read_to_string("src/data.txt"
        .to_string())
        .expect("File not found");
    let strings:Vec<&str>=file.split("\n").collect();

    println!("--- Day 11: Cosmic Expansion ---\n\n");
    part1(strings.clone());
    part2(strings.clone());
    println!("\n");
}
