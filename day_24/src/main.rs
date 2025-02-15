use std::fs;
use rusymbols::Expression;
fn calculate_collision(vector:Vec<Vec<i128>>,min_coord:i128,max_coord:i128)->u32 {
    let mut collision:u32=0;
    //Equation is ax+by=c <==> vy*x-vx*y=vy*x0-vx*y0
    for i  in 0..vector.len()-1 {
        //par will contain (vy,-vx,vy*x0-vx*y0) => (a,b,c)
        let par1:(i128,i128,i128) = (vector[i][4],
                                        -vector[i][3],
                                        vector[i][4]*vector[i][0] - vector[i][3]*vector[i][1]);
         for j in i+1..vector.len() {
             //par will contain (vy,-vx,vy*x0-vx*y0) => (a,b,c)
             let par2:(i128,i128,i128) = (vector[j][4],
                                             -vector[j][3],
                                             vector[j][4]*vector[j][0] - vector[j][3]*vector[j][1]);
             if par1.0*par2.1 == par2.0*par1.1 { continue; }
             let x:i128=(par1.2*par2.1 - par2.2*par1.1 )/(par1.0*par2.1 - par2.0*par1.1);
             let y:i128=(par1.2*par2.0 - par2.2*par1.0 )/(par1.1*par2.0  - par2.1*par1.0);

             if (x - vector[i][0])*vector[i][3] >= 0 && (y - vector[i][1])*vector[i][4] >= 0 &&
                 (x - vector[j][0])*vector[j][3] >= 0 && (y - vector[j][1])*vector[j][4] >= 0 &&
                 x >= min_coord && y >= min_coord &&
                 x <= max_coord && y <= max_coord {
                 collision+=1;
             }
         }
    }
    collision
}
fn part1(strings:Vec<&str>)->u32 {
    let mut vector:Vec<Vec<i128>>=Vec::new();
    for string in strings {
        let r = string.replace(" ","");
        let row = r.replace("@",",");
        let line:Vec<&str> =row.split(",").collect();
        vector.insert(vector.len(),line.into_iter().map(|x| x.parse::<i128>().unwrap()).collect());
    }
    return calculate_collision(vector,200000000000000,400000000000000);
}
fn main() {
    let file =fs::read_to_string("src/data.txt"
        .to_string())
        .expect("File not found");
    let strings:Vec<&str>=file.split("\n").collect();

    println!("--- Day 23: A Long Walk ---\n\n");
    println!("   Part 1: {}",part1(strings.clone()));
    //println!("   Part 2: {}",part2(strings.clone()));
    println!("\n");
}