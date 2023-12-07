use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::convert::TryInto;


fn part1(filepath: String){
    let reader = BufReader::new(File::open(filepath).expect("Cannot open file.txt"));
    let mut sum_part : i32 = 0;
    let mut data : HashMap<(i32,i32),char> = HashMap::new();
    let mut numbers : HashMap<(i32,i32,i32), i32> = HashMap::new();

    let mut i : i32 = 0;

    for line in reader.lines() {
        let row : Vec<char> = line.unwrap().chars().collect();
        let mut j : i32 = 0;
        while j < row.len().try_into().unwrap() {
            let mut numb = String::new();
            let ind_min :i32 = j;
            while j < row.len().try_into().unwrap() && row[j as usize].is_digit(10) { 
                numb.push(row[j as usize]);
                j+=1;
            }
            //Populate symbols HashMap
            if j < row.len().try_into().unwrap() && row[j as usize] != '.'{data.insert((i,j),row[j as usize]);}
            if !numb.trim().is_empty() {
                numbers.insert((i,ind_min,j-1),numb.parse::<i32>().unwrap());
            }
            j+=1;
        }
        i += 1;    
    }

    for (key,value) in numbers.iter(){
        let mut valid = false;
        for i in ((key.0)-1)..=((key.0)+1) {
            for j in ((key.1)-1)..=((key.2)+1){
                if data.contains_key(&(i,j)) {
                    valid = true;
                    continue;
                }
            }
        }
        if valid {sum_part += value;}
    }

    println!("   Part 1: {}",sum_part);
}

fn part2(filepath: String){
    let reader = BufReader::new(File::open(filepath).expect("Cannot open file.txt"));
    let mut sum_part : i32 = 0;
    let mut data : HashMap<(i32,i32),char> = HashMap::new();
    let mut numbers : HashMap<(i32,i32,i32), i32> = HashMap::new();

    let mut i : i32 = 0;

    for line in reader.lines() {
        let row : Vec<char> = line.unwrap().chars().collect();
        let mut j : i32 = 0;
        while j < row.len().try_into().unwrap() {
            let mut numb = String::new();
            let ind_min :i32 = j;
            while j < row.len().try_into().unwrap() && row[j as usize].is_digit(10) { 
                numb.push(row[j as usize]);
                j+=1;
            }
            //Save possible gear position
            if j < row.len().try_into().unwrap() && row[j as usize] == '*'{data.insert((i,j),row[j as usize]);}
            if !numb.trim().is_empty() {
                numbers.insert((i,ind_min,j-1),numb.parse::<i32>().unwrap());
            }
            j+=1;
        }
        i += 1;    
    }

    for (key_data,_) in data.iter(){
        let mut gear_value : i32= 1;
        let mut gear = 0;
        for (key_numbers,value) in numbers.iter(){
            if key_data.0-1 <= key_numbers.0 && key_numbers.0 <= key_data.0+1 && key_numbers.1-1 <= key_data.1 && key_data.1<= key_numbers.2+1{
                gear+=1;
                gear_value*=value;
                if gear == 2{sum_part+=gear_value; continue;}
            } 
        }
    }
    println!("   Part 2: {}",sum_part);
}

fn main(){
    println!("\n--- Day 3: Gear Ratios ---\n\n");
    let filepath = String::from("data.txt");
    part1(filepath.clone());
    part2(filepath.clone());
    println!("\n\n--------------------------");
}