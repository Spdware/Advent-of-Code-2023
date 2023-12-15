use std::collections::HashMap;
use std::fs;

fn hash_algorithm(line:&str)->u32 {
    let mut curr_value:u32=0;
    for char in line.chars(){
        curr_value+=(char as u8) as u32;
        curr_value=curr_value*17;
        curr_value=curr_value % 256;
    }
    curr_value
}
fn part1(strings:Vec<&str>)->u32{
    let mut hash_result:u32=0;
    for line in strings{
        hash_result+=hash_algorithm(line);
    }
    hash_result
}

fn calculate_lens_result(lens_slot:HashMap<u32,Vec<(&str,u32)>>)->u32{
    let mut lens_result:u32=0;
    for (boxnum,lens) in lens_slot.iter(){
        for (slot,len) in lens.iter().enumerate() {
            lens_result+=(1+boxnum)*((slot+1) as u32)*len.1;
        }
    }
    lens_result
}

fn adding<'a>(mut tmp_hash:Vec<(&'a str,u32)>, entry:Vec<&'a str>)->Vec<(&'a str,u32)> {
    if let Some(tuple) = tmp_hash.iter().find(|&&(v, _)| v == entry[0]) {
        let index=tmp_hash.iter().position(|&value| value == *tuple).unwrap();
        tmp_hash.remove(index);
        tmp_hash.insert(index,(entry[0],entry[1].to_string().parse::<u32>().unwrap()));
    }else{
        tmp_hash.push((entry[0],entry[1].to_string().parse::<u32>().unwrap()));
    }
    return tmp_hash;
}

fn removing<'a>(mut tmp_hash:Vec<(&'a str,u32)>, entry:Vec<&'a str>)->Vec<(&'a str,u32)> {
    if let Some(tuple) = tmp_hash.iter().find(|&&(v, _)| v == entry[0]) {
        tmp_hash.remove(tmp_hash.iter().position(|&value| value == *tuple).unwrap());
    }
    return tmp_hash;
}

fn part2(strings:Vec<&str>)->u32 {
    let mut lens_slots:HashMap<u32,Vec<(&str,u32)>>=HashMap::new();
    for i in 0..256{lens_slots.insert(i as u32,Vec::new());}

    for line in strings{
        let entry:Vec<&str>;
        let operation:char;
        if line.contains('='){
            operation='=';
            entry=line.split('=').collect();
        }else{
            operation='-';
            entry=line.split('-').collect();
        }
        let hash=hash_algorithm(entry[0]);
        match operation{
            '='=>{
                lens_slots.insert(hash,adding(lens_slots.get(&hash).unwrap().clone(),entry));
            },
            '-'=>{
                lens_slots.insert(hash,removing(lens_slots.get(&hash).unwrap().clone(),entry));
            },
            _=>{},
        }
    }
    return calculate_lens_result(lens_slots.clone());
}

fn main() {
    let reader =fs::read_to_string("src/data.txt"
        .to_string())
        .expect("File not found");
    let file = reader.replace("\n","");
    let strings:Vec<&str>=file.split(",").collect();

    println!("--- Day 15: Lens Library ---\n\n");
    println!("   Part 1: {}",part1(strings.clone()));
    println!("   Part 2: {}",part2(strings.clone()));
    println!("\n");
}