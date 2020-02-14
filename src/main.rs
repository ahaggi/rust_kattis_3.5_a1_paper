use std::io ;
// use std::io::{self ,BufReader , BufRead , Lines , Error};

fn main() {
    let stdin = io::stdin();
    stdin.read_line(&mut " ".to_string()).expect("Failed to read line");// ignoring the first input "the A-size of the smallest papers Björn has" 

    let mut val = String::new();
    stdin.read_line(&mut val).expect("Failed to read line");// reading the number of sheets he has of each paper size starting with A2 and ending with An.
    
    let input: Vec<u32> = val.trim().split_whitespace().map(|num| num.parse().unwrap()).collect();

    if check_if_enough_paper(&input[..]) {
        match rec(0, 2, 0.0, &input[..]) {
            Some(n) => println!("{}", n),
            _ => println!("Not enough paper"),
        };
    }else{
        println!("Not enough paper")
    }
}



const A2_LENGTH: f64 = 0.5946035575013605;
const A2_WIDTH: f64 = 0.42044820762685725;

fn rec(indx: usize, need_to_use: u32, tape_length: f64, data: &[u32]) -> Option<f64> {
    //ind    p_size                  need_to_use                                                    tape_length
    // 0       A2                         2                                                             2/2 = 1
    // 1       A3        (prev_need_to_use - prev_available) * 2 >= x <= 4                    prev_tape_length + (need_to_use /2)  * leng
    // 2       A4        (prev_need_to_use - prev_available) * 2 >= x <= 8                    prev_tape_length + (need_to_use /2)  * leng

    if indx >= data.len() {
        return None;
    }

    let leng = if indx % 2 == 0 {
        A2_LENGTH / (2 as f64).powf(indx as f64 / 2.0)
    } else {
        A2_WIDTH / (2 as f64).powf((indx - 1) as f64 / 2.0)
    };

    let available: u32 = data[indx];

    let tape_length = tape_length + ((need_to_use as f64) / 2.0) * leng;

    if available >= need_to_use {
        // To tape 2 paper, we just need a tape the long side of both  ==> ((need_to_use as f64) / 2.0) * leng
        println!(" leng: {} x {}", need_to_use / 2, leng);
        return Some(tape_length);
    } else {
        // we don't have enough paper of the current size
        // available < need_to_use
        println!(" leng: ' {} x {}", need_to_use / 2, leng);
        let next_level_need_to_use = (need_to_use - available) * 2;
        return rec(indx + 1, next_level_need_to_use, tape_length, data);
    }
}


fn check_if_enough_paper(data: &[u32] ) ->bool {
    //ind    p_size    max_need_to_use
    // 0       a2        2
    // 1       a3        4
    // 2       a4        8

    let mut indx: i32 = 0;
    let target_area:f32 = 2.0;
    let mut available_area:f32 = 0.0;
    let mut enough:bool = available_area > target_area;

    while !enough && indx < data.len() as i32{
        available_area = available_area + (data[indx as usize] as f32) * (2 as f32).powf((0.0 - indx as f32) as f32);
        enough = available_area >= target_area;
        indx += 1;
    }
    enough 
    
}




