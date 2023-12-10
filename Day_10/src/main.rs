use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

fn bfs(pipes:HashMap<(i32,i32),Vec<(i32,i32)>>,start:(i32,i32))->HashSet<(i32,i32)>{
    let mut visited:HashSet<(i32,i32)>=HashSet::new();
    visited.insert(start);
    let mut search:VecDeque<(i32,i32)>= VecDeque::new();
    for elem in pipes.get(&start).unwrap().iter() {
        search.push_back(*elem);
    }
    while let Some(node) = search.pop_front() {
        let neighbors = pipes.get(&node).unwrap();
        for neighbor in neighbors {
            if visited.insert(*neighbor) {
                search.push_back(*neighbor);
            }
        }
    }
    visited
}

fn create_matrix(strings:Vec<&str>)->HashMap<(i32,i32),Vec<(i32,i32)>>{
    let mut pipes: HashMap<(i32,i32),Vec<(i32,i32)>>=HashMap::new();
    let mut curr_position:(i32,i32);
    for (i,line) in strings.iter().enumerate() {
        for (j,char) in line.chars().enumerate(){
            match char {
                'S'|'|' | '-' | 'L' | 'J' | '7' | 'F' => curr_position=(i as i32,j as i32),
                _ =>curr_position=(i32::MAX,i32::MAX),
            }
            if char == 'S'{pipes.insert(curr_position,vec![(curr_position.0-1,curr_position.1),
                                                           (curr_position.0+1,curr_position.1),
                                                           (curr_position.0,curr_position.1-1),
                                                           (curr_position.0,curr_position.1+1)]);}
            if char != 'S' && char!='.' && curr_position.0 != i32::MAX{
                match char{
                    '|'=>pipes.insert(curr_position,vec![(curr_position.0-1,curr_position.1),(curr_position.0 +1,curr_position.1)]),
                    '-'=>pipes.insert(curr_position,vec![(curr_position.0,curr_position.1-1),(curr_position.0,curr_position.1+1)]),
                    'L'=>pipes.insert(curr_position,vec![(curr_position.0-1,curr_position.1),(curr_position.0,curr_position.1+1)]),
                    'J'=>pipes.insert(curr_position,vec![(curr_position.0-1,curr_position.1),(curr_position.0,curr_position.1-1)]),
                    '7'=>pipes.insert(curr_position,vec![(curr_position.0+1,curr_position.1),(curr_position.0,curr_position.1-1)]),
                    'F'=>pipes.insert(curr_position,vec![(curr_position.0+1,curr_position.1),(curr_position.0,curr_position.1+1)]),
                    _=>pipes.insert(curr_position,vec![curr_position,curr_position]),
                };
            };
        }
    }
    pipes
}

fn search_start(strings:Vec<&str>)->(i32,i32) {
    for (i,line) in strings.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char=='S'{return (i as i32, j as i32);}
        }
    }
    return (i32::MAX,i32::MAX);
}

fn part1(strings:Vec<&str>){
    let start:(i32,i32)=search_start(strings.clone());
    let pipes: HashMap<(i32,i32),Vec<(i32,i32)>>=create_matrix(strings);
    println!("   Part 1: {}",bfs(pipes,start).len()/2);
}
fn straight_line_evaluator(pipe: char)->i32 {
    return if "JL|S".to_string().contains(pipe) { 1 } else { 0 }
}
fn is_tile(pipes:HashMap<(i32,i32),char>,bfs:HashSet<(i32,i32)>,searching:(i32,i32))->i32{
    let mut count:i32=0;
    let mut curr:(i32,i32)=(searching.0,0);
    while curr != searching{
        if bfs.contains(&curr) {
            count+=straight_line_evaluator(*pipes.get(&curr).unwrap());
        }
        curr=(curr.0,curr.1+1);
    }
    return count%2
}
fn part2(strings:Vec<&str>){
    let start:(i32,i32)=search_start(strings.clone());
    let pipes: HashMap<(i32,i32),Vec<(i32,i32)>>=create_matrix(strings.clone());

    let mut count_tiles:i32=0;
    let bfs:HashSet<(i32,i32)> = bfs(pipes.clone(),start);
    let mut char_pipes:HashMap<(i32,i32),char>=HashMap::new();
    for (i,line) in strings.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            char_pipes.insert((i as i32,j as i32),char);
        }
    }
    for (key,_value) in char_pipes.iter() {
        if !bfs.contains(&key) {
            count_tiles+=is_tile(char_pipes.clone(),bfs.clone(),*key);
        }
    }
    println!("   Part 2: {}",count_tiles);
}

fn main() {
    let file =fs::read_to_string("src/data.txt"
        .to_string())
        .expect("File not found");
    let strings:Vec<&str>=file.split("\n").collect();

    println!("--- Day 10: Pipe Maze ---\n\n");
    part1(strings.clone());
    part2(strings.clone());
    println!("\n");
}
