use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
fn calculate_next_position(tile: ((i32,i32),&str),tiles:HashMap<(i32,i32),char>)-> Vec<((i32,i32),&str)> {
    match tile.1 {
        "right"=>{
            if !tiles.contains_key(&(tile.0.0,tile.0.1+1)) { return vec![]; }
            match *tiles.get(&(tile.0.0,tile.0.1+1)).unwrap(){
                '-'|'.'=>{return vec![((tile.0.0,tile.0.1+1),"right")];},
                '|'=>{return vec![((tile.0.0,tile.0.1+1),"up"),
                                  ((tile.0.0,tile.0.1+1),"down")];},
                '/'=>{return vec![((tile.0.0,tile.0.1+1),"up")];},
                '\\'=>{return vec![((tile.0.0,tile.0.1+1),"down")];},
                _=>{return vec![];},
            }
        },
        "left"=>{
            if !tiles.contains_key(&(tile.0.0,tile.0.1-1)) { return vec![]; }
            match *tiles.get(&(tile.0.0,tile.0.1-1)).unwrap(){
                '-'|'.'=>{return vec![((tile.0.0,tile.0.1-1),"left")];},
                '|'=>{return vec![((tile.0.0,tile.0.1-1),"up"),
                                  ((tile.0.0,tile.0.1-1),"down")];},
                '/'=>{return vec![((tile.0.0,tile.0.1-1),"down")];},
                '\\'=>{return vec![((tile.0.0,tile.0.1-1),"up")];},
                _=>{return vec![];},
            }
        },
        "up"=>{
            if !tiles.contains_key(&(tile.0.0-1,tile.0.1)) { return vec![]; }
            match *tiles.get(&(tile.0.0-1,tile.0.1)).unwrap(){
                '|'|'.'=>{return vec![((tile.0.0-1,tile.0.1),"up")];},
                '-'=>{return vec![((tile.0.0-1,tile.0.1),"left"),
                                  ((tile.0.0-1,tile.0.1),"right")];},
                '/'=>{vec![((tile.0.0-1,tile.0.1),"right")]},
                '\\'=>{vec![((tile.0.0-1,tile.0.1),"left")]},
                _=>{return vec![];},
            }
        },
        "down"=>{
            if !tiles.contains_key(&(tile.0.0+1,tile.0.1)) { return vec![]; }
            match *tiles.get(&(tile.0.0+1,tile.0.1)).unwrap(){
                '|'|'.'=>{return vec![((tile.0.0+1,tile.0.1),"down")];},
                '-'=>{ return vec![((tile.0.0+1,tile.0.1),"left"),
                                   ((tile.0.0+1,tile.0.1),"right")];},
                '/'=>{vec![((tile.0.0+1,tile.0.1),"left")]},
                '\\'=>{vec![((tile.0.0+1,tile.0.1),"right")]},
                _=>{return vec![];},
            }
        },
        _=>{
            return vec![];
        },
    }
}
fn calculate_seen_tiles(tiles:HashMap<(i32,i32),char>,start:((i32,i32),&str))->usize {
    let mut seen:HashSet<(i32,i32)>=HashSet::new();
    let mut current:VecDeque<((i32,i32),&str)>=VecDeque::new();
    let mut seen_tiles:HashSet<((i32,i32),&str)>=HashSet::new();

    current.push_back(start);
    loop{
        let pos = current.pop_front().unwrap();
        let pos_next=calculate_next_position(pos,tiles.clone());
        for elem in pos_next {
            //print!("({},{}) {},",elem.0.0,elem.0.1,elem.1);
            if tiles.contains_key(&elem.0) && !seen_tiles.contains(&elem){
                seen.insert(elem.0);
                seen_tiles.insert(elem);
                current.push_back(elem);
            }
        }
        //println!();
        if current.len()==0 { return seen.len();}
    }
}
fn part1(strings: Vec<&str>)->usize {
    let mut tiles:HashMap<(i32,i32),char>=HashMap::new();

    for (i,line) in strings.iter().enumerate() {
        for (j,char) in line.chars().enumerate() {
            tiles.insert((i as i32,j as i32),char);
        }
    }
    return calculate_seen_tiles(tiles,((0,-1),"right"));
}

fn part2(strings:Vec<&str>)->usize {
    let mut max_tiles:usize=0;
    let mut tmp_tile:usize;
    let mut tiles:HashMap<(i32,i32),char>=HashMap::new();

    let l:&str = strings.get(0).unwrap();
    let x:i32=l.len() as i32;
    let y:i32=strings.len() as i32;
    for (i,line) in strings.iter().enumerate() {
        for (j,char) in line.chars().enumerate() {
            tiles.insert((i as i32,j as i32),char);
        }
    }

    for j in -1..x {
        tmp_tile=calculate_seen_tiles(tiles.clone(),((0,j),"down"));
        if tmp_tile > max_tiles{max_tiles=tmp_tile;}
        tmp_tile=calculate_seen_tiles(tiles.clone(),((y-1,j),"up"));
        if tmp_tile > max_tiles{max_tiles=tmp_tile;}

    }
    for i in -1..y {
        tmp_tile=calculate_seen_tiles(tiles.clone(),((i,0),"right"));
        if tmp_tile > max_tiles{max_tiles=tmp_tile;}
        tmp_tile=calculate_seen_tiles(tiles.clone(),((i,x-1),"left"));
        if tmp_tile > max_tiles{max_tiles=tmp_tile;}

    }
    max_tiles
}

fn main() {
    let file =fs::read_to_string("src/data.txt"
        .to_string())
        .expect("File not found");
    let strings:Vec<&str>=file.split("\n").collect();

    println!("--- Day 16: The Floor Will Be Lava ---\n\n");
    println!("   Part 1: {}",part1(strings.clone()));
    println!("   Part 2: {}",part2(strings.clone()));
    println!("\n");
}