use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

fn count_present(winners: Vec<&str>, owned : Vec<&str>) -> u32{
    let mut count : u32= 0;
    for num in owned{
        if winners.iter().position(|&x| x==num).is_some(){count+=1;}
    }
    return count;
}

fn check_count(winners : Vec<&str>, owned: Vec<&str>, base : i32) -> i32{
    let mut count : u32= 0;
    count+= count_present(winners,owned);
    if count == 0{return 0;}
    return base.pow(count-1);
}

fn part1(filepath : String){
    let reader = BufReader::new(File::open(filepath).expect("Cannot open file.txt"));
    let mut value_card :i32=0;
    let base :i32 = 2;
    for row in reader.lines(){
        let linetmp = row.unwrap();
        let line : Vec<&str> = linetmp.split(":").collect();
        let numbers:Vec<&str> = line[1].split("|").collect();
        let winner :Vec<&str>= numbers[0].split_whitespace().collect();
        let owned:Vec<&str>=numbers[1].split_whitespace().collect();
        
        value_card+=check_count(winner,owned,base);
        
    }
    println!("Part 1: {}",value_card);
}

fn part2(filepath : String){
    let reader = BufReader::new(File::open(filepath).expect("Cannot open file.txt"));
    let mut cards : HashMap<u32,u32> = HashMap::new();
    let mut value_card :u32=0;
    let lines = reader.lines();
    for row in lines{
        let linetmp=row.unwrap();
        let line : Vec<&str> = linetmp.split(":").collect();
        let i = line[0].replace(":","").split_whitespace().collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let numbers:Vec<&str> = line[1].split("|").collect();
        let winner :Vec<&str>= numbers[0].split_whitespace().collect();
        let owned:Vec<&str>=numbers[1].split_whitespace().collect();
        if !cards.contains_key(&i){cards.insert(i,1);}
        value_card+=cards[&i];
        let copies = count_present(winner,owned);
        for j in 1..=copies{
            if cards.contains_key(&(i+j)){cards.insert(i+j,cards[&(i+j)]+cards[&i]);}
            else{cards.insert(i+j,1+cards[&i]);}
        }
    }

    println!("Part 2: {}",value_card)
}


fn main(){
    println!("\n--- Day 4: Scratchcards ---\n\n");
    let filepath = String::from("data.txt");
    part1(filepath.clone());
    part2(filepath.clone());
    println!("\n\n--------------------------");
}