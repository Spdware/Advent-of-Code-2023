use std::collections::HashMap;
use std::fs;

fn calculate_ratings(option:HashMap<&str,u32>,workflows:HashMap<&str,Vec<(&str,&str,u32,&str)>>)->u32 {
    let mut key:&str="in";
    loop{
        for elem in workflows.get(&key).unwrap() {
            match elem.1 {
                ">"=>{
                    let part_value:u32=*option.get(&elem.0).unwrap();
                    if part_value > elem.2{ key=elem.3;break; }
                },
                "<"=>{
                    let part_value:u32=*option.get(&elem.0).unwrap();
                    if part_value < elem.2{ key=elem.3;break; }
                },
                "return"=>{ key=elem.0;break; },
                _=>unreachable!(),
            }
        }
        if key=="A" {return option.values().sum()}
        if key=="R" {return 0;}
    }
}
fn part1(strings:Vec<&str>)->u32 {
    let mut total:u32=0;
    let mut workflows:HashMap<&str,Vec<(&str,&str,u32,&str)>>=HashMap::new();
    let block1:Vec<&str>=strings[0].split("\n").collect();
    for line in  block1{
        let row:Vec<&str> = line[0..line.len()-1].split("{").collect();
        let mut entries:Vec<(&str,&str,u32,&str)>=Vec::new();
        let r:Vec<&str>=row[1].split(",").collect();
        for l in  r {
            let tmp: Vec<&str> = l.split(":").collect();
            if tmp.len() == 1 {
                entries.push((&tmp[0], "return", 0, ""));
            } else {
                entries.push((&tmp[0][0..1], &tmp[0][1..2], tmp[0][2..].to_string().parse::<u32>().unwrap(), tmp[1]));
            }
        }
        workflows.insert(row[0],entries);
    }
    let block2:Vec<&str>=strings[1].split("\n").collect();
    for line in  block2{
        let tmp_option:Vec<&str>=line[1..line.len()-1].split(",").collect();
        let mut option:HashMap<&str,u32>=HashMap::new();
        for o in tmp_option{
            option.insert(&o[0..1],o[2..].to_string().parse::<u32>().unwrap());
        }
        total+=calculate_ratings(option,workflows.clone());
    }
    total
}

fn sum_up(option:HashMap<&str,(u64,u64)>)->u64{
    let mut prod:u64=1;
    for value in option.values() {
        let result:u64= match value.1.checked_sub(value.0){
            Some(val)=> val+1,
            None=>4000-value.1+value.0+2,
        };
        prod*=result;
    }
    prod
}
fn calculate_possibilities<'a>(workflows:HashMap<&str,Vec<(&'a str,&str,u64,&str)>>,mut option:HashMap<&'a str,(u64,u64)>,key:&str)->u64 {
    if key=="A"{return sum_up(option); }
    if key=="R" { return 0; }
    let mut tmp:u64=0;
    for elem in workflows.get(&key).unwrap() {
        match elem.1 {
            ">"=>{
                let mut tmp_option:HashMap<&'a str,(u64,u64)>=option.clone();
                let opt:(u64,u64)= *tmp_option.get(&elem.0).unwrap();
                tmp_option.insert(elem.0,(elem.2+1,opt.1));
                tmp+=calculate_possibilities(workflows.clone(),tmp_option.clone(),elem.3);
                option.insert(elem.0,(opt.0,elem.2));
            },
            "<"=>{
                let mut tmp_option:HashMap<&'a str,(u64,u64)>=option.clone();
                let opt:(u64,u64)= *tmp_option.get(&elem.0).unwrap();
                tmp_option.insert(elem.0,(opt.0,elem.2-1));
                tmp+=calculate_possibilities(workflows.clone(),tmp_option.clone(),elem.3);
                option.insert(elem.0,(elem.2,opt.1));
            },
            "return"=>{ tmp+=calculate_possibilities(workflows.clone(),option.clone(),elem.0); },
            _=>unreachable!(),
        }
    }
    tmp
}

fn part2(strings:&str)->u64{
    let mut workflows:HashMap<&str,Vec<(&str,&str,u64,&str)>>=HashMap::new();
    let block1:Vec<&str>=strings.split("\n").collect();
    for line in  block1{
        let row:Vec<&str> = line[0..line.len()-1].split("{").collect();
        let mut entries:Vec<(&str,&str,u64,&str)>=Vec::new();
        let r:Vec<&str>=row[1].split(",").collect();
        for l in  r {
            let tmp: Vec<&str> = l.split(":").collect();
            if tmp.len() == 1 {
                entries.push((&tmp[0], "return", 0, ""));
            } else {
                entries.push((&tmp[0][0..1], &tmp[0][1..2], tmp[0][2..].to_string().parse::<u64>().unwrap(), tmp[1]));
            }
        }
        workflows.insert(row[0],entries);
    }
    let option:HashMap<&str,(u64,u64)>=HashMap::from([("x",(1,4000)),("m",(1,4000)),("a",(1,4000)),("s",(1,4000))]);
    return calculate_possibilities(workflows,option,"in");
}

fn main() {
    let file =fs::read_to_string("src/data.txt"
        .to_string())
        .expect("File not found");
    let strings:Vec<&str>=file.split("\n\n").collect();

    println!("--- Day 19: Aplenty ---\n\n");
    println!("   Part 1: {}",part1(strings.clone()));
    println!("   Part 2: {}",part2(strings[0]));
    println!("\n");
}