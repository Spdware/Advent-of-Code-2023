use std::collections::HashMap;
use std::fs;

fn calculate_stones(stones:HashMap<(usize,usize),char>,start:(usize,usize))->(usize,i32){
    let mut count:i32=0;
    let mut rolling_stones:i32=0;
    let mut i:usize=start.0;
    while i>0 && *stones.get(&(i,start.1)).unwrap()!='#' {
        if *stones.get(&(i,start.1)).unwrap()=='O' { rolling_stones+=1;}
        i-=1;
    }
    let mut collected:usize=0;
    while (collected as i32)< rolling_stones {
        count+=(start.0-collected) as i32;
        collected+=1;
    }
    if i==0 {i=1;}
    (i,count)
}
fn part1(strings:Vec<&str>)->i32{
    let mut count:i32=0;
    let x_length:usize=strings.get(0).unwrap().len();
    let y_length:usize=strings.len();
    let mut stones:HashMap<(usize,usize),char>=HashMap::new();
    let mut  i:usize=y_length;

    for line in strings {
        for (j,char) in line.chars().enumerate() {
            stones.insert((i,j),char);
        }
        i-=1;
    }

    for j in 0..x_length {
        let mut i:usize=y_length;
        while i>0 {
            if *stones.get(&(i,j)).unwrap()!= '#' {
                let tuple=calculate_stones(stones.clone(),(i,j));
                count+=tuple.1;
                i=tuple.0;
            }
            i-=1;
        }
    }

    count
}
fn pile_up(stone:HashMap<(usize,usize),char>,x:usize,y:usize)->HashMap<(usize,usize),char>{
    let mut new_stone=stone.clone();
    for j in 0..x{
        let mut i:i32=y as i32-1;
        while i>=0{
            if *new_stone.get(&(i as usize,j)).unwrap()=='O'||*new_stone.get(&(i as usize,j)).unwrap()=='#'{
                i-=1;
                continue;
            }
            let mut ind:i32=i-1;
            while ind>=0 && *new_stone.get(&(ind as usize,j)).unwrap()=='.'{ ind-=1; }
            if ind>=0 && *new_stone.get(&(ind as usize,j)).unwrap()=='.'{continue;}
            if ind>=0 {
                new_stone.insert((i as usize,j),'O');
                new_stone.insert((ind as usize,j),'.');
            }
        }
    }
    new_stone
}
fn rotate(stones:HashMap<(usize,usize),char>,num_rows:usize,num_columns:usize )->HashMap<(usize,usize),char> {
    let mut new_stones:HashMap<(usize,usize),char> = HashMap::new();
    for i in 0..num_rows {
        for j in 0..num_columns {
            new_stones.insert((j,num_rows - 1 - i) , *stones.get(&(i,j)).unwrap());
        }
    }
    new_stones
}

fn string_format(stones:HashMap<(usize,usize),char>,max_x:usize,max_y:usize)->String {
    let mut parsed:String=String::new();
    for i in 0..max_x {
        for j in 0..max_y {
            parsed.push(*stones.get(&(i,j)).unwrap());
        }
        parsed.push('\n');
    }
    parsed
}

fn cycle(stones:HashMap<(usize,usize),char>,max_x:usize,max_y:usize,num_cycle:i32)->HashMap<(usize,usize),char> {
    let mut new_stones:HashMap<(usize,usize),char>=stones.clone();
    let mut seen:HashMap<String,i32>=HashMap::new();
    let mut current:i32=0;
    while current < num_cycle {
        let curr_hash=string_format(new_stones.clone(),max_x,max_y);
        if seen.contains_key(&curr_hash){
            current=num_cycle-((num_cycle-seen.get(&curr_hash).unwrap()) % (current-seen.get(&curr_hash).unwrap()));
        }
        seen.insert(curr_hash,current);
        for i in 0..4 {
            match i % 2 {
                0=>{
                    new_stones=pile_up(new_stones.clone(),max_x,max_y);
                    new_stones=rotate(new_stones.clone(),max_x,max_y);
                },
                1=>{
                    new_stones=pile_up(new_stones.clone(),max_x,max_y);
                    new_stones=rotate(new_stones.clone(),max_x,max_y);
                },
                _=>{},
            }
        }
    }
    new_stones
}

fn part2(strings:Vec<&str>)->i32{
    let mut count:i32=0;
    let x_length:usize=strings.get(0).unwrap().len();
    let y_length:usize=strings.len();
    let mut stones:HashMap<(usize,usize),char>=HashMap::new();

    for (i,line) in strings.iter().enumerate() {
        for (j,char) in line.chars().enumerate() {
            stones.insert((i,j),char);
        }
    }

    stones=cycle(stones.clone(),x_length,y_length,1000);

    for j in 0..x_length {
        for i in 0..y_length {
            if *stones.get(&(i,j)).unwrap()=='O'{count+=(y_length-i) as i32;}
        }
    }
    count
}

fn main() {
    let file =fs::read_to_string("src/test.txt"
        .to_string())
        .expect("File not found");
    let strings:Vec<&str>=file.split("\n").collect();

    println!("--- Day 14: Parabolic Reflector Dish ---\n\n");
    println!("   Part 1: {}",part1(strings.clone()));
    println!("   Part 2: {}",part2(strings.clone()));
    println!("\n");
}