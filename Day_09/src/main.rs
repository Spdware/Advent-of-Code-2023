use std::fs;
use std::vec::Vec;

fn calculate_start_value(values:Vec<i32>)->i32{
    let mut value:i32=0;
    let curr_values:Vec<i32>=values.clone().into_iter().rev().collect();
    for i in curr_values{
        value=i-value;
    }
    return value;
}

fn calulate_report_value(curr_report:Vec<i32>,searched_history:&str)->i32{
    let mut value:Vec<i32>=Vec::new();
    let mut curr_value:Vec<i32>=curr_report;
    loop{
        let mut next_value:Vec<i32>=Vec::new();
        let mut all_zero:bool=true;
        for i in 0..curr_value.len()-1 {
            next_value.push(curr_value.get(i+1).unwrap()-curr_value.get(i).unwrap());
            if *next_value.last().unwrap()!=0{all_zero=false;}
        }
        match searched_history {
            "end"=>{value.push(*curr_value.last().unwrap());},
            "start"=>{value.push(*curr_value.first().unwrap());},
            _=>{},
        }
        if all_zero{
            match searched_history{
                "end"=>return value.iter().sum(),
                "start"=>return calculate_start_value(value),
                _=>{},
            }
        }
        curr_value=next_value;
    }
}

fn part1(strings:Vec<&str>){
    let mut total_value:i32=0;
    for line in strings.iter() {
        let curr_report:Vec <i32>=line.split_whitespace().collect::<Vec<&str>>().iter().map(|&s| s.parse::<i32>().unwrap()).collect();
        total_value+=calulate_report_value(curr_report,"end");
    }
    println!("   Part 1: {total_value}");
}

fn part2(strings:Vec<&str>){
    let mut total_value:i32=0;
    for line in strings.iter() {
        let curr_report:Vec <i32>=line.split_whitespace().collect::<Vec<&str>>().iter().map(|&s| s.parse::<i32>().unwrap()).collect();
        total_value+=calulate_report_value(curr_report,"start");
    }
    println!("   Part 2: {total_value}");
}


fn main() {
    let file =fs::read_to_string("src/data.txt"
    .to_string())
    .expect("File not found");
    let strings:Vec<&str>=file.split("\n").collect();

    println!("--- Day 9: Mirage Maintenance ---\n\n");
    part1(strings.clone());
    part2(strings.clone());
    println!("\n");
}