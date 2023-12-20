use std::collections::{HashMap, VecDeque};
use std::fs;
use num::integer::lcm;

fn flipflop_pulses<'a>(state:bool,next_component:Vec<&'a str>,sender:&'a str)->VecDeque<(bool,&'a str,&'a str)> {
    let mut tmp:VecDeque<(bool,&str,&str)>=VecDeque::new();
    for comp in next_component {
        tmp.push_back((state,comp,sender));
    }
    tmp
}
fn calculte_inputs(states:HashMap<&str,bool>)->bool {
    for val in states.values(){
         if !val {return true;}
    }
    return false;
}
fn conjuction_pulses<'a>(pulse:bool,next_component:Vec<&'a str>,sender:&'a str)->VecDeque<(bool,&'a str,&'a str)> {
    let mut tmp:VecDeque<(bool,&str,&str)>=VecDeque::new();
    for comp in next_component {
        tmp.push_back((pulse,comp,sender));
    }
    tmp
}

fn broadcaster_pulses<'a>(pulse:bool,next_component:Vec<&'a str>,sender:&'a str)->VecDeque<(bool,&'a str,&'a str)> {
    let mut tmp:VecDeque<(bool,&str,&str)>=VecDeque::new();
    for comp in next_component {
        tmp.push_back((pulse,comp,sender));
    }
    tmp
}

fn calculate_pulses<'a>(broadcaster:Vec<&'a str>,
                        mut flipflop:HashMap<&'a str,(bool,Vec<&'a str>)>,
                        mut conjuction:HashMap<&'a str,(HashMap<&'a str,bool>,Vec<&'a str>)>)->u32 {
    let mut low:u32=0;
    let mut high:u32=0;
    //Pulse level, Receiver, Sender
    let mut pulses:VecDeque<(bool,&str,&str)>=VecDeque::new();
    for _i in 0..1000 {
        low+=1;
        let mut new_pulses:VecDeque<(bool,&str,&str)>=broadcaster_pulses(false,broadcaster.clone(),"broadcaster");
        low+=new_pulses.len() as u32;
        pulses.append(&mut new_pulses);
        while !pulses.is_empty(){
            let op: (bool, &str, &str);
            if let Some(tmp) = pulses.pop_front() {
                op = tmp;
            } else {
                unreachable!();
            }
            if flipflop.contains_key(op.1) {
                if op.0 { continue; }
                let tmp_ff: (bool, Vec<&str>) = flipflop.get(op.1).unwrap().clone();
                flipflop.insert(op.1, (!tmp_ff.0, tmp_ff.1.clone()));
                let mut new_pulses: VecDeque<(bool, &str, &str)> = flipflop_pulses(!tmp_ff.0, tmp_ff.1.clone(), op.1);
                if !tmp_ff.0 {
                    high += new_pulses.len() as u32;
                } else {
                    low += new_pulses.len() as u32;
                }
                pulses.append(&mut new_pulses);
            } else if conjuction.contains_key(op.1) {
                let mut tmp_con: (HashMap<&str, bool>, Vec<&str>) = conjuction.get(&op.1).unwrap().clone();
                tmp_con.0.insert(op.2, op.0);
                conjuction.insert(op.1, tmp_con.clone());
                let output: bool = calculte_inputs(tmp_con.clone().0.clone());
                let mut new_pulses: VecDeque<(bool, &str, &str)> = conjuction_pulses(output, tmp_con.clone().1.clone(), op.1);
                if output {
                    high += new_pulses.len() as u32;
                } else {
                    low += new_pulses.len() as u32;
                }
                pulses.append(&mut new_pulses);
            }
        }
    }
    return low*high;
}


fn calculate_minimum<'a>(broadcaster:Vec<&'a str>,
                         mut flipflop:HashMap<&'a str,(bool,Vec<&'a str>)>,
                         mut conjuction:HashMap<&'a str,(HashMap<&'a str,bool>,Vec<&'a str>)>,abilitator:Vec<&str>,searched_abilitator:&str)->u64{
    //Pulse level, Receiver, Sender
    let mut pulses:VecDeque<(bool,&str,&str)>=VecDeque::new();
    let mut pressed:u64=0;
    let mut abilitator_seen:HashMap<&str,u64>=HashMap::new();
    let mut first_seen:HashMap<&str,u64>=HashMap::new();
    for key in &abilitator{abilitator_seen.insert(key,0);}
    loop{
        pressed += 1;
        let mut new_pulses:VecDeque<(bool,&str,&str)>=broadcaster_pulses(false,broadcaster.clone(),"broadcaster");
        pulses.append(&mut new_pulses);
        while !pulses.is_empty() {
            let op: (bool, &str, &str);
            if let Some(tmp) = pulses.pop_front() {
                op = tmp;
            } else {
                unreachable!();
            }
            if abilitator.contains(&op.2) && searched_abilitator==op.1 && op.0 {
                //abilitator_seen.insert(op.2,abilitator_seen.get(&op.2).unwrap()+1);
                first_seen.insert(op.2,pressed);
                if first_seen.len()==abilitator.len(){
                    let mut x:u64=1;
                    for elm in &first_seen {
                        x=lcm(x,*elm.1);
                    }
                    return x;

                }
            }
            if flipflop.contains_key(op.1) {
                if op.0 { continue; }
                let tmp_ff: (bool, Vec<&str>) = flipflop.get(op.1).unwrap().clone();
                flipflop.insert(op.1, (!tmp_ff.0, tmp_ff.1.clone()));
                let mut new_pulses: VecDeque<(bool, &str, &str)> = flipflop_pulses(!tmp_ff.0, tmp_ff.1.clone(), op.1);
                pulses.append(&mut new_pulses);
            } else if conjuction.contains_key(op.1) {
                let mut tmp_con: (HashMap<&str, bool>, Vec<&str>) = conjuction.get(&op.1).unwrap().clone();
                tmp_con.0.insert(op.2, op.0);
                conjuction.insert(op.1, tmp_con.clone());
                let output: bool = calculte_inputs(tmp_con.clone().0.clone());
                let mut new_pulses: VecDeque<(bool, &str, &str)> = conjuction_pulses(output, tmp_con.clone().1.clone(), op.1);
                pulses.append(&mut new_pulses);
            }
        }
    }
}

fn parser(strings:Vec<&str>,part1:bool)->u64 {
    let mut flipflop:HashMap<&str,(bool,Vec<&str>)>=HashMap::new();
    let mut conjuction:HashMap<&str,(HashMap<&str,bool>,Vec<&str>)>=HashMap::new();
    let mut broadcaster:Vec<&str>=Vec::new();
    let searched:&str="rx";
    let mut searched_abilitator:&str="";
    for line in strings {
        let row:Vec<&str>=line.split(" -> ").collect();
        if row[0][0..1]==*"%" {
            let modules:Vec<&str>=row[1].split(", ").collect();
            flipflop.insert(&row[0][1..],(false,modules.clone()));
            if modules.contains(&searched){ searched_abilitator=&row[0][1..]; }
        }else if row[0][0..1]==*"&" {
            let modules:Vec<&str>=row[1].split(", ").collect();
            conjuction.insert(&row[0][1..],(HashMap::new(),modules.clone()));
            if modules.contains(&searched){ searched_abilitator=&row[0][1..]; }
        }else if row[0]=="broadcaster"{
            broadcaster=row[1].split(", ").collect();
            if broadcaster.contains(&searched){ searched_abilitator="broadcaster"; }
        }
    }
    let mut abilitator:Vec<&str>=Vec::new();
    if broadcaster.contains(&searched_abilitator) {
        abilitator.push("broadcaster")
    }
    for (key_ff,elem) in &flipflop {
        if elem.1.contains(&searched_abilitator) {
            abilitator.push(key_ff);
        }
    }
    let ks:Vec<&str>=conjuction.keys().cloned().collect();
    for key_con in ks {
        let tmp1:(HashMap<&str,bool>,Vec<&str>)=conjuction.get(&key_con).unwrap().clone();
        if tmp1.1.contains(&searched_abilitator) {
            abilitator.push(key_con);
        }
    }

    let keyset:Vec<&str>=conjuction.keys().cloned().collect();
    for keys in &keyset {
        if broadcaster.contains(&keys) {
            let mut tmp:(HashMap<&str,bool>,Vec<&str>)=conjuction.get(keys).unwrap().clone();
            tmp.0.insert("broadcaster",false);
            conjuction.insert(keys,tmp);
        }
        for (key_ff,elem) in &flipflop {
            if elem.1.contains(&keys) {
                let mut tmp:(HashMap<&str,bool>,Vec<&str>)=conjuction.get(keys).unwrap().clone();
                tmp.0.insert(key_ff,false);
                conjuction.insert(keys,tmp);
            }
        }
        let ks:Vec<&str>=keyset.clone();
        for key_con in ks {
            let tmp1:(HashMap<&str,bool>,Vec<&str>)=conjuction.get(&key_con).unwrap().clone();
            if tmp1.1.contains(&keys) {
                let mut tmp:(HashMap<&str,bool>,Vec<&str>)=conjuction.get(keys).unwrap().clone();
                tmp.0.insert(key_con,false);
                conjuction.insert(keys,tmp);
            }
        }
    }
    return if part1 {
        calculate_pulses(broadcaster.clone(), flipflop.clone(), conjuction.clone()) as u64
    } else {
        calculate_minimum(broadcaster.clone(), flipflop.clone(), conjuction.clone(), abilitator.clone(),searched_abilitator)
    }

}

fn main() {
    let file =fs::read_to_string("src/data.txt"
        .to_string())
        .expect("File not found");
    let strings:Vec<&str>=file.split("\n").collect();

    println!("--- Day 20: Pulse Propagation ---\n\n");
    println!("   Part 1: {}",parser(strings.clone(),true));
    println!("   Part 2: {}",parser(strings.clone(),false));
    println!("\n");
}