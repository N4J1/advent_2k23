use std::string::String;
use std::fs;
use std::io::{self, BufRead};

fn get_edges(line: &str) -> u32 { // O(n)
    let string = String::from(line);
    let mut res_arr: Vec<char> = vec![];

    for c in string.chars() {
        if c.is_numeric() {
            res_arr.push(c);
        }
    }

    if res_arr.len() == 1 {
        return (
            res_arr[0].to_digit(10).unwrap() * 10 + res_arr[0].to_digit(10).unwrap()
        ) as u32;
    }

    return (
        res_arr[0].to_digit(10).unwrap() * 10
            + res_arr[res_arr.len() - 1].to_digit(10).unwrap()
    ) as u32;
}

fn main() {
    // test case
    // let res: u32 = get_edges("555diajsdoiada545kjsdaiojdida6");
    // assert_eq!(res, 56);

    // total
    let mut total = 0_u32;
    // opening the file
    let file = fs::File::open("input.txt").unwrap();

    // itterating over lines and adding to the total
    for line in io::BufReader::new(file).lines() {
        let l = line.unwrap();
        let res = get_edges(&l);
        total += res;
    }
    
    println!("{:?}", total); // 54632
    
}