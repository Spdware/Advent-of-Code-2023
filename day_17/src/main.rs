use std::collections::{HashMap, HashSet};
use std::collections::BinaryHeap;
use std::fs;
use std::cmp::Reverse;
fn traversal(blocks:HashMap<(i32,i32),i32>,start:(i32,i32),finish:(i32,i32),max_straight_line:usize,part1:bool)->i32{
    //Position 0: heat loss
    //Position 1: x position
    //Position 2: y position
    //Position 3: x direction
    //Position 4: y direction
    //Position 5: straight line position
    let mut pq:BinaryHeap<Reverse<(i32,i32,i32,i32,i32,usize)>>=BinaryHeap::from([Reverse((0,start.0,start.1,0,0,0))]);
    let mut seen:HashSet<(i32,i32,i32,i32,usize)>=HashSet::new();

    while !pq.is_empty() {
        let Reverse((hl,x,y,nx,ny,msl))=pq.pop().unwrap();
        if (x,y)==finish {
            if part1 { return hl; }
            else{
                if msl >= 4 {
                    return hl;
                }
            }
        }
        if seen.contains(&(x,y,nx,ny,msl)) { continue; }
        seen.insert((x,y,nx,ny,msl));
        if msl < max_straight_line && (nx,ny) != (0, 0){
            let row:i32=x+nx;
            let col:i32=y+ny;
            if blocks.contains_key(&(row,col)) {
                pq.push(Reverse((hl+blocks.get(&(row,col)).unwrap(),row,col,nx,ny,msl+1)));
            }
        }

        if msl >=4 || (x,y)==(0,0) || part1 {
            for el in [(0,1),(0,-1),(1,0),(-1,0)] {
                let next_pos :(i32,i32)= (x + el.0, y + el.1);
                if (nx, ny) == el || (-nx, -ny) == el { continue; }
                if !blocks.contains_key(&next_pos) { continue; }
                pq.push(Reverse((hl+blocks.get(&next_pos).unwrap(), next_pos.0, next_pos.1, el.0, el.1, 1)));
            }
        }
    }
    unreachable!();
}

fn resolve(strings:Vec<&str>) {
    let mut blocks:HashMap<(i32,i32),i32>=HashMap::new();
    let l:&str = strings.get(0).unwrap();
    let max_y=l.len() as i32;
    let max_x=strings.len() as i32;
    for (i,line) in strings.iter().enumerate() {
        for (j, heat) in line.chars().enumerate() {
            blocks.insert((i as i32,j as i32),heat.to_digit(10).unwrap() as i32);
        }
    }
    println!("   Part 1: {}",traversal(blocks.clone(),(0,0),(max_x-1,max_y-1),3,true));
    println!("   Part 2: {}",traversal(blocks.clone(),(0,0),(max_x-1,max_y-1),10,false));
}

fn main() {
    let file =fs::read_to_string("src/data.txt"
        .to_string())
        .expect("File not found");
    let strings:Vec<&str>=file.split("\n").collect();

    println!("--- Day 17: Clumsy Crucible ---\n\n");
    resolve(strings);
    println!("\n");
}