use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

fn calculate_path(tiles:HashMap<(i32,i32),char>,start:(i32,i32),finish:(i32,i32),already_seen:HashSet<(i32,i32)>,path_len:u32)->u32 {
    let mut seen:HashSet<(i32,i32)>=already_seen.clone();
    let mut pq:BinaryHeap<(u32,i32,i32)>=BinaryHeap::from([(path_len,start.0,start.1)]);
    let mut longest_path:u32 = u32::MIN;

    while !pq.is_empty() {
        let curr :(u32,i32,i32)= pq.pop().unwrap();

        if (curr.1,curr.2)==finish { return curr.0; }
        if seen.contains(&(curr.1,curr.2)) { continue; }
        seen.insert((curr.1,curr.2));
        match tiles.get(&(curr.1,curr.2)).unwrap() {
            '.'=> {
                let mut neighbour: Vec<(i32, i32)> = Vec::new();
                for elem in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let next_tile = (curr.1 + elem.0, curr.2 + elem.1);
                    if !tiles.contains_key(&next_tile) { continue; }
                    neighbour.push(elem);
                }
                if neighbour.len() < 2 {
                    for elem in neighbour {
                        let next_tile = (curr.1 + elem.0, curr.2 + elem.1);
                        pq.push((curr.0 + 1, next_tile.0, next_tile.1));
                    }
                } else {
                    for elem in neighbour {
                        let next_tile = (curr.1 + elem.0, curr.2 + elem.1);
                        if seen.contains(&next_tile) { continue; }
                        let path_len=calculate_path(tiles.clone(),next_tile,finish,seen.clone(),curr.0+1);
                        if path_len > longest_path { longest_path=path_len;}
                    }
                    return longest_path;
                }
            },
            '<'=>{
                let next_tile = (curr.1,curr.2-1);
                if !tiles.contains_key(&next_tile) { continue; }
                pq.push((curr.0+1,next_tile.0,next_tile.1));},
            '>'=>{
                let next_tile = (curr.1,curr.2+1);
                if !tiles.contains_key(&next_tile) { continue; }
                pq.push((curr.0+1,next_tile.0,next_tile.1));},
            'v'=>{
                let next_tile = (curr.1+1,curr.2);
                if !tiles.contains_key(&next_tile) { continue; }
                pq.push((curr.0+1,next_tile.0,next_tile.1));},
            '^'=>{
                let next_tile = (curr.1-1,curr.2);
                if !tiles.contains_key(&next_tile) { continue; }
                pq.push((curr.0+1,next_tile.0,next_tile.1));}
            _ => unreachable!(),
        }
    }
    return longest_path;
}
/*
fn part2(strings:Vec<&str>)->u32 {
    let mut tiles:HashMap<(i32,i32),char>=HashMap::new();

    for (i,line) in strings.iter().enumerate() {
        for (j,char) in line.chars().enumerate() {
            if char != '#' { tiles.insert((i as i32,j as i32),'.'); }
        }
    }
    let row_len= strings.len() as i32;
    let column_len=strings[0].len() as i32;
    return calculate_path_no_slippery(tiles,(0,1),(row_len-1,column_len-2),HashSet::new(),0);
}
*/
fn part1(strings:Vec<&str>)->u32 {
    let mut tiles:HashMap<(i32,i32),char>=HashMap::new();

    for (i,line) in strings.iter().enumerate() {
        for (j,char) in line.chars().enumerate() {
            if char != '#' { tiles.insert((i as i32,j as i32),char); }
        }
    }
    let row_len= strings.len() as i32;
    let column_len=strings[0].len() as i32;
    return calculate_path(tiles,(0,1),(row_len-1,column_len-2),HashSet::new(),0);
}

fn main() {
    let file =fs::read_to_string("src/test.txt"
        .to_string())
        .expect("File not found");
    let strings:Vec<&str>=file.split("\n").collect();

    println!("--- Day 23: A Long Walk ---\n\n");
    println!("   Part 1: {}",part1(strings.clone()));
    //println!("   Part 2: {}",part2(strings.clone()));
    println!("\n");
}