use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

fn pathfinding(blocks:HashMap<(i32,i32),char>,start:(i32,i32),steps:u32)->u32 {
    let mut seen:HashSet<(i32,i32)>=HashSet::from([start]);
    let mut stops:HashSet<(i32,i32)>=HashSet::new();
    let mut pq:BinaryHeap<(u32,i32,i32)>=BinaryHeap::from([(steps,start.0,start.1)]);

    while !pq.is_empty() {
        let curr:(u32,i32,i32) = pq.pop().unwrap();
        if curr.0 % 2 == 0 { stops.insert((curr.1,curr.2)); }

        if curr.0 == 0 { continue; }

        for next in [(curr.1-1,curr.2),(curr.1+1,curr.2),(curr.1,curr.2-1),(curr.1,curr.2+1)] {
            if !blocks.contains_key(&next) || seen.contains(&next){ continue; }
            seen.insert(next);
            pq.push((curr.0-1,next.0,next.1));
        }
    }
    return stops.len() as u32;
}

fn pathfinding_2(blocks:HashMap<(i32,i32),char>,start:(i32,i32),steps:u64)->u64 {
    let mut seen:HashSet<(u64,(i32,i32))>=HashSet::from([(0,start)]);
    let mut stops:HashSet<(i32,i32)>=HashSet::new();
    let mut pq:BinaryHeap<(u64,i32,i32)>=BinaryHeap::from([(steps,start.0,start.1)]);
    while !pq.is_empty() {
        let curr:(u64,i32,i32) = pq.pop().unwrap();
        if curr.0 % 2 == 1 || curr.0 == 0{ stops.insert((curr.1,curr.2)); }

        if curr.0 == 0 {
            continue;
        }

        for next in adjacent(next,start.0*2) {
            if !blocks.contains_key(&next){
                if next.0==-1{

                }
            }
            if seen.contains(&(curr.0-1,next)){ continue; }
            seen.insert((curr.0-1,next));
            pq.push((curr.0-1,next.0,next.1));
        }
    }
    return stops.len() as u64;
}
fn create_blocks(strings:Vec<&str>)->HashMap<(i32,i32),char> {
    let mut blocks:HashMap<(i32,i32),char>=HashMap::new();
    for (i,line) in strings.iter().enumerate() {
       for (j,char) in line.chars().enumerate() {
           if char != '#' {blocks.insert((i as i32, j as i32), char);}
       }
    }
    blocks
}
/*
fn create_blocks_2(strings:Vec<&str>)->HashMap<(i32,i32),char> {
    let mut blocks:HashMap<(i32,i32),char>=HashMap::new();
    let lenght:i32= strings.len() as i32;
    for (i,line) in strings.iter().enumerate() {
        for (j,char) in line.chars().enumerate() {
            if char != '#' {
                for elm in [(0,0),(0,1),(0,2),(1,0),(1,1),(1,2),(2,0),(2,1),(2,2)] {
                    blocks.insert((i as i32+ lenght*elm.0 ,j as i32 + lenght*elm.1),char);
                }
            }
        }
    }
    blocks
}
*/
fn main() {
    let file =fs::read_to_string("src/data.txt"
        .to_string())
        .expect("File not found");
    let strings:Vec<&str>=file.split("\n").collect();
    let len_line=strings.len() as i32;
    let start:(i32,i32)=(len_line/2,len_line/2);
    //let start_2:(i32,i32)=(len_line+len_line/2,len_line+len_line/2);
    println!("--- Day 20: Pulse Propagation ---\n\n");
    println!("   Part 1: {}",pathfinding(create_blocks(strings.clone()),start,64));
    println!("   Part 2: {}",pathfinding_2(create_blocks(strings.clone()),start,26501365));
    println!("\n");
}