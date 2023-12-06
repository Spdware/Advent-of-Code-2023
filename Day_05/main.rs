use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::vec::Vec;

fn change_state(state : &str) -> &'static str{
    let mut next_state : &str = "seed";
    match state{
        "seed"=> next_state = "soil",
        "soil"=> next_state = "fertilizer",
        "fertilizer"=> next_state ="water",
        "water"=> next_state ="light",
        "light"=> next_state ="temperature",
        "temperature"=> next_state ="humidity",
        "humidity"=> next_state ="location",
        _ => {},
    }
    return next_state;
}

fn add_element(map:&mut HashMap<(u128,u128),u128>, line: Vec<&str>){
    let _=&map.insert((line[1].parse::<u128>().unwrap(),line[0].parse::<u128>().unwrap()),line[2].parse::<u128>().unwrap());
}

fn add_element_tree(map:&mut BTreeMap<(u128,u128),u128>, line: Vec<&str>){
    let _=&map.insert((line[1].parse::<u128>().unwrap(),line[0].parse::<u128>().unwrap()),line[2].parse::<u128>().unwrap());
}

fn search_key(map:HashMap<(u128,u128),u128>,cur_key:u128)->u128{
    for (key,value) in map.iter(){
        if key.0 <= cur_key && cur_key < (key.0 + (value)){
            return key.1+(cur_key-key.0);
        } 
    }
    return cur_key;
}

fn search_keys(map:BTreeMap<(u128,u128),u128>,cur_keys: Vec<(u128,u128)>)->Vec<(u128,u128)>{
    let mut values:BTreeMap<(u128,u128),u128>=BTreeMap::new();
    for cur_key in &cur_keys{
        let mut min_not_check=cur_key.0;
        for (key,value) in map.iter(){
            if (key.0 > cur_key.1)||(cur_key.0 > (key.0+value-1)){continue;}
            if min_not_check < key.0{values.insert((min_not_check,(key.0)-1),1);min_not_check=key.0;}
            values.insert((key.1+min_not_check-key.0 ,key.1+(value-1).min(cur_key.1-key.0)),1);
            min_not_check=(key.0+value-1).min(cur_key.1)+1;
        }
    }
    if values.keys().len()==0{return cur_keys;}
    return values.keys().cloned().collect::<Vec<(u128,u128)>>();
}


fn part1(filepath : String){
    let reader = BufReader::new(File::open(filepath).expect("Cannot open file.txt"));
    let mut seed : Vec<String> = Vec::new();
    let mut seed_to_soil : HashMap<(u128,u128),u128> = HashMap::new();
    let mut soil_to_fertilizer:HashMap<(u128,u128),u128>=HashMap::new();
    let mut fertilizer_to_water : HashMap<(u128,u128),u128> = HashMap::new();
    let mut water_to_light : HashMap<(u128,u128),u128> = HashMap::new();
    let mut light_to_temperature : HashMap<(u128,u128),u128> = HashMap::new();
    let mut temperature_to_humidity:HashMap<(u128,u128),u128>=HashMap::new();
    let mut humidity_to_location:HashMap<(u128,u128),u128>=HashMap::new();
    let mut state : &str = "seed";

    for row in reader.lines(){
        let linetmp = row.unwrap();
        let line: Vec<&str> = linetmp.split_whitespace().collect();
        if line.len() >0{
            let tmp = line[0];
            if tmp == "seeds:"{
                for sd in line[1..].iter(){seed.push(sd.to_string());}
            }
            match tmp.parse::<u128>(){
                Ok(_) => {
                    match state{
                        "soil"=>add_element(&mut seed_to_soil,line.clone()),
                        "fertilizer"=>add_element(&mut soil_to_fertilizer,line.clone()),
                        "water"=>add_element(&mut fertilizer_to_water,line.clone()),
                        "light"=>add_element(&mut water_to_light,line.clone()),
                        "temperature"=>add_element(&mut light_to_temperature,line.clone()),
                        "humidity"=>add_element(&mut temperature_to_humidity,line.clone()),
                        "location"=>add_element(&mut humidity_to_location,line.clone()),
                        _=>{},
                    }
                },
                Err(_) =>{},
            }
        } else{state = change_state(state);}
    }

    let mut min:u128 = u128::MAX;
    let container_list : Vec<HashMap<(u128,u128),u128>>=vec![seed_to_soil,soil_to_fertilizer,fertilizer_to_water,
                                                              water_to_light,light_to_temperature,
                                                              temperature_to_humidity,humidity_to_location];
    for i in seed{
        let mut cur_key:u128=i.parse::<u128>().unwrap();
        for list in &container_list{
            cur_key=search_key(list.clone(),cur_key);
        }
        if cur_key<min{min = cur_key;}
    } 
    println!("  Part 1: {}", min);
}

fn part2(filepath : String){
    let reader = BufReader::new(File::open(filepath).expect("Cannot open file.txt"));
    let mut seed : Vec<String> = Vec::new();
    let mut seed_to_soil : BTreeMap<(u128,u128),u128> = BTreeMap::new();
    let mut soil_to_fertilizer:BTreeMap<(u128,u128),u128>=BTreeMap::new();
    let mut fertilizer_to_water : BTreeMap<(u128,u128),u128> = BTreeMap::new();
    let mut water_to_light : BTreeMap<(u128,u128),u128> = BTreeMap::new();
    let mut light_to_temperature : BTreeMap<(u128,u128),u128> = BTreeMap::new();
    let mut temperature_to_humidity:BTreeMap<(u128,u128),u128>=BTreeMap::new();
    let mut humidity_to_location:BTreeMap<(u128,u128),u128>=BTreeMap::new();
    let mut state : &str = "seed";

    for row in reader.lines(){
        let linetmp = row.unwrap();
        let line: Vec<&str> = linetmp.split_whitespace().collect();
        if line.len() >0{
            let tmp = line[0];
            if tmp == "seeds:"{
                for sd in line[1..].iter(){seed.push(sd.to_string());}
            }
            match tmp.parse::<u128>(){
                Ok(_) => {
                    match state{
                        "soil"=>add_element_tree(&mut seed_to_soil,line.clone()),
                        "fertilizer"=>add_element_tree(&mut soil_to_fertilizer,line.clone()),
                        "water"=>add_element_tree(&mut fertilizer_to_water,line.clone()),
                        "light"=>add_element_tree(&mut water_to_light,line.clone()),
                        "temperature"=>add_element_tree(&mut light_to_temperature,line.clone()),
                        "humidity"=>add_element_tree(&mut temperature_to_humidity,line.clone()),
                        "location"=>add_element_tree(&mut humidity_to_location,line.clone()),
                        _=>{},
                    }
                },
                Err(_) =>{},
            }
        } else{state = change_state(state);}
    }

    let mut min:u128=u128::MAX;
    let container_list : Vec<BTreeMap<(u128,u128),u128>>=vec![seed_to_soil,soil_to_fertilizer,fertilizer_to_water,
                                                              water_to_light,light_to_temperature,
                                                              temperature_to_humidity,humidity_to_location];
    for i in (0..seed.len()).step_by(2){
        let min_seed=seed[i].parse::<u128>().unwrap();
        let max_seed=min_seed+seed[i+1].parse::<u128>().unwrap()-1;
        let mut cur_key_range:Vec<(u128,u128)>=vec![(min_seed,max_seed)];
        for list in &container_list{
            cur_key_range=search_keys(list.clone(),cur_key_range);
        }
        let val=cur_key_range.get(0).unwrap().0;
        if val<min{min = val;} 
    } 
    println!("  Part 2: {}", min);
}

fn main(){
    println!("\n--- Day 5: If You Give A Seed A Fertilizer ---\n\n");
    let filepath = String::from("data.txt");
    part1(filepath.clone());
    part2(filepath.clone());
    println!("\n");
}