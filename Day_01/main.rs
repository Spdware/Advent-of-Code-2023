use std::io::{BufRead, BufReader};
use std::fs::File;

fn part1(filepath : String) {
    let reader = BufReader::new(File::open(filepath).expect("Cannot open file.txt"));
    let mut sum = 0;
    //Part 1
    for row in reader.lines(){
        let line = row.unwrap();
        let mut num = String::new();
        for c in line.chars() {
            if c.is_digit(10){
               num.push(c);
               break;
            }    
        }
        let reverse = line.chars().rev().collect::<String>();
        for c in reverse.chars(){
            if c.is_digit(10){
               num.push(c);
               break;
            }
        }
        sum += num.parse::<i32>().unwrap();
        
    }
    println!("Result part 1: {}", sum);
}

fn part2(filepath : String) {
    let reader = BufReader::new(File::open(filepath).expect("Cannot open file.txt"));
    let mut sum = 0;
    let numstr = ["one","two","three","four","five","six","seven","eight","nine"];

    for row in reader.lines(){
        let mut line = row.unwrap();
        let mut num = String::new();
        
        for (i,numb) in numstr.iter().enumerate(){
            line = line.replace(numb, &String::from(String::from(numb.chars().next().unwrap())+
            &(i+1).to_string()+
            &String::from(numb.chars().last().unwrap())));
        }
        let reverse = line.chars().rev().collect::<String>();
        
        for c in line.chars() {
            if c.is_digit(10){
               num.push(c);
               break;
            }    
        }
        
        for c in reverse.chars(){
            if c.is_digit(10){
               num.push(c);
               break;
            }
        }
        sum += num.parse::<i32>().unwrap();
        
    }
    println!("Result part 2: {}", sum);
}

fn main(){
    let filepath = String::from("data.txt");
    println!("--- Day 1: Trebuchet?! ---\n\n")
    part1(filepath.clone());
    part2(filepath.clone());
    println!("\n\n-------------------------")
}
