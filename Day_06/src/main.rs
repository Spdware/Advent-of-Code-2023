use std::fs;


fn part1(filepath : String){
    let file =fs::read_to_string(filepath).expect("File not found");
    let strings:Vec<&str>=file.split("\n").collect();
    
    let time:Vec<i32>=strings[0].split_whitespace()
    .collect::<Vec<&str>>().iter()
    .map(|x| x.parse::<i32>())
    .filter_map(Result::ok)
    .collect();
    
    let distance:Vec<i32>=strings[1]
    .split_whitespace()
    .collect::<Vec<&str>>()
    .iter().map(|x| x.parse::<i32>())
    .filter_map(Result::ok)
    .collect();

    let mut possible_win:i32=1;
    for i in 0..time.len(){
        let cur_time:i32 = *time.get(i).unwrap();
        let cur_dist=*distance.get(i).unwrap();
        let mut tries:i32=0;
        for j in 1..cur_time{
            if j*(cur_time-j)>=cur_dist{
                tries+=1
            }
        }
        possible_win*=tries;
    }
    println!("  Part 1: {}", possible_win);
}


fn part2(filepath:String){
    let file =fs::read_to_string(filepath).expect("File not found");
    let strings:Vec<&str>=file.split("\n").collect();

    let time :i64 = strings[0].replace(" ","").split(":").collect::<Vec<&str>>()[1].parse::<i64>().unwrap();
    let distance:i64 = strings[1].replace(" ","").split(":").collect::<Vec<&str>>()[1].parse::<i64>().unwrap();

    let mut tries:i64=0;
    for j in 1..time{
        if j*(time-j)>=distance{
            tries+=1
        }
    }
    println!("  Part 2: {tries}");
}

fn main(){
    let filepath = String::from("data.txt");
    println!("--- Day 6: Wait For It ---\n\n");
    part1(filepath.clone());
    part2(filepath.clone());
}