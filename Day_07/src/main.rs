use std::fs;
use std::collections::HashMap;
use std::vec::Vec;
//use ascii_converter::*;

fn calculate_type(hand: &str)-> String{
    let mut char_count:HashMap<char, u32> = HashMap::new();
    let mut type_found:String=String::new();

    for ch in hand.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }
    
    let mut binding:Vec<u32>= char_count.values().cloned().collect::<Vec<u32>>();
    binding.sort_by(|a, b| b.cmp(a));
    match binding.as_slice() {
        [5]=>type_found="five".to_string(),
        [4,1]=>type_found="four".to_string(),
        [3,2]=>type_found="full".to_string(),
        [3,1,1]=>type_found="three".to_string(),
        [2,2,1]=>type_found="two".to_string(),
        [2,1,1,1]=>type_found="high".to_string(),
        [1,1,1,1,1]=>type_found="None".to_string(),
        _ => assert!(false),
    }
    type_found
}

fn parse_to_digit(h:&str)->String{
    let mut hand:String=h.to_string();
    let char:Vec<char>=vec!['T','J','Q','K','A'];
    for (i,value) in char.iter().enumerate() {
        hand=hand.replace(*value,&char::from(('9' as u8)+(i as u8) +1).to_string());
    }
    return hand.to_string();
}

fn calculate_total_winning(collection_hands:HashMap<String,Vec<String>>,hands:HashMap<String,u32>)->u32{
    let mut total_winning:u32=0;
    let type_set1: Vec<String>=vec![String::from("five"),String::from("four"),String::from("full"),String::from("three"),String::from("two"),String::from("high"),String::from("None")];
    let mut hands_order:Vec<String>= Vec::new();
    for label in type_set1 {
        match collection_hands.get(&label) {
            Some(values)=> {
                let mut val: Vec<String> = values.clone();
                val.sort_by(|a, b| b.cmp(a));
                hands_order.append(&mut val);
            },
            None=>{},
        }
    }
    for (i,value) in hands_order.iter().enumerate() {
        //println!("{} {} {}",value,(hands_order.len() as u32)-(i as u32),hands.get(value).expect("Value not found"));
        total_winning+=((hands_order.len() as u32)-(i as u32))*hands.get(value).expect("Value not found");
    }

    return total_winning;
}

fn part1(strings:Vec<&str>){
    let mut hands: HashMap<String,u32>=HashMap::new();
    let mut collection_hands:HashMap<String, Vec<String>>=HashMap::new();
    let type_set: Vec<String>=vec![String::from("five"),String::from("four"),String::from("full"),String::from("three"),String::from("two"),String::from("high"),String::from("None")];
    for t in type_set {
        collection_hands.insert(t.to_string(),Vec::new());
    }

    for row in strings{
        let hand:Vec<&str>=row.split_whitespace().collect();
        //println!("{} {}",hand[0],hand[1]);
        hands.insert(parse_to_digit(hand[0]),hand[1].parse::<u32>().unwrap());
        let tmp_type:String=calculate_type(hand[0]);
        
        collection_hands.entry(tmp_type).and_modify(|tmp_coll:&mut Vec<String>| tmp_coll.push(parse_to_digit(hand[0]))).or_insert(Vec::new());
    }

    println!("   Part 1: {}",calculate_total_winning(collection_hands,hands)); 
}

fn substitute_joker(hand: &str)->String{
    let mut char_count:HashMap<char, u32> = HashMap::new();

    for ch in hand.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }

    char_count.insert('J', 0);

    if let Some((key, _)) = char_count.iter().max_by(|&(_, v1), &(_, v2)| v1.cmp(v2)) {
        return hand.replace('J',&key.to_string()).to_string();
    }
    return hand.to_string();
}

fn parse_to_digit_2(h:&str)->String{
    let mut hand:String=h.to_string();
    let char:Vec<char>=vec!['T','Q','K','A'];
    for (i,value) in char.iter().enumerate() {
        hand=hand.replace(*value,&char::from(('9' as u8)+(i as u8)+1).to_string());
    }
    hand=hand.replace('J',&char::from('0' as u8).to_string());
    return hand.to_string();
}

fn part2(strings:Vec<&str>){
    let mut hands: HashMap<String,u32>=HashMap::new();
    let mut collection_hands:HashMap<String, Vec<String>>=HashMap::new();
    let type_set: Vec<String>=vec![String::from("five"),String::from("four"),
                                String::from("full"),String::from("three"),
                                String::from("two"),String::from("high"),
                                String::from("None")];
    for t in type_set {
        collection_hands.insert(t.to_string(),Vec::new());
    }

    for row in strings{
        let mut tmp_type:String=String::new();
        let hand:Vec<&str>=row.split_whitespace().collect();
        if hand[0].contains('J'){
            let hand1:String=substitute_joker(hand[0]);
            tmp_type.push_str(&calculate_type(&hand1));
        }else{
            tmp_type.push_str(&calculate_type(hand[0]));
        }
        hands.insert(parse_to_digit_2(hand[0]),hand[1].parse::<u32>().unwrap());
        
        collection_hands.entry(tmp_type).and_modify(|tmp_coll:&mut Vec<String>| tmp_coll.push(parse_to_digit_2(hand[0]))).or_insert(Vec::new());
    }

    println!("   Part 2: {}",calculate_total_winning(collection_hands,hands));


}


fn main() {
    let file =fs::read_to_string("/home/davide/Documents/Advent-of-Code-2023/Day_07/src/data.txt"
    .to_string())
    .expect("File not found");
    let strings:Vec<&str>=file.split("\n").collect();

    println!("--- Day 7: Camel Cards ---\n\n");
    part1(strings.clone());
    part2(strings.clone());
    println!("\n");
}
