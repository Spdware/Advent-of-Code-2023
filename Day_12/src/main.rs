use std::collections::HashMap;
use std::fs;
use indicatif::ProgressBar;
fn calculate_positions(string: String)->Vec<i32>{
    let positions:Vec<&str>=string.split('.').collect();
    let mut counts:Vec<i32>=Vec::new();
    for elem in positions{
        if elem.len()>0{counts.insert(counts.len(),elem.len() as i32);}
    }
    counts
}
fn hotspring_combination(hotspring:String,searched_comb:Vec<i32>,cache:&mut HashMap<(String,Vec<i32>),i128>)->i128{
    let key=(hotspring.clone(),searched_comb.clone());
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }
    if !hotspring.contains('?') {
        return if calculate_positions(hotspring)==searched_comb{ 1 } else { 0 }
    }
    let mut tmp_count:i128=0;
    let index:usize=hotspring.char_indices().find(|(_, c)| *c == '?').unwrap().0;
    let mut string:String = hotspring.clone();
    string.replace_range(index..index+1,".");
    tmp_count+=hotspring_combination(string.clone(),searched_comb.clone(),cache);
    string.replace_range(index..index+1,"#");
    tmp_count+=hotspring_combination(string.clone(),searched_comb.clone(),cache);
    cache.insert(key,tmp_count);
    return tmp_count;
}
fn part1(strings: Vec<&str>)->i128{
    let mut count:i128=0;

    for line in strings.iter(){
        let row: Vec<&str>=line.split_whitespace().collect();
        let hotspring:String=String::from(row[0]);
        let destroyed:Vec<i32>=row[1].split(',').collect::<Vec<&str>>().iter()
            .map(|&s| s.parse::<i32>().unwrap()).collect();
        count+=hotspring_combination(hotspring.clone(), destroyed.clone(),&mut HashMap::new());
    }
    count
}

fn part2(strings: Vec<&str>)->i128{
    let mut count:i128=0;
    let pb = ProgressBar::new(1000);

    for line in strings.iter() {
        pb.inc(1);
        let row: Vec<&str> = line.split_whitespace().collect();
        let mut hotspring: String = String::from(row[0]);
        let d: Vec<i32> = row[1].split(',').collect::<Vec<&str>>().iter()
            .map(|&s| s.parse::<i32>().unwrap()).collect();
        for _ in 0..4 {
            hotspring.push_str(&("?".to_owned()+row[0]));
        }
        let destroyed:Vec<i32>=[d.clone(),d.clone(),d.clone(),d.clone(),d.clone()].concat();
        count+=hotspring_combination(hotspring.clone(), destroyed.clone(),&mut HashMap::new());
    }
    pb.finish_with_message("Completed");
    count
}

fn main() {
    let file =fs::read_to_string("src/data.txt"
        .to_string())
        .expect("File not found");
    let strings:Vec<&str>=file.split("\n").collect();

    println!("--- Day 12: Hot Springs ---\n\n");
    println!("   Part 1: {}",part1(strings.clone()));
    println!("   Part 2: {}",part2(strings.clone()));
    println!("\n");
}
