#[path = "../../src/utils/mod.rs"] mod utils;
use std::collections::{HashSet};

pub fn day_03_01() -> i32 {
    let file_path = "src/day_03/data.txt";
    let contents;
    contents = utils::read_file(file_path);
    let mut sum = 0;
    let mut hash1 = HashSet::new();
    let mut hash2 = HashSet::new();

    for line in contents.lines().into_iter() {
        hash1.clear();
        hash2.clear();
        for i in 0..line.len() {
            if check(&mut hash1, &mut hash2, &line.as_bytes()[i], &mut sum) { break; }
            if check(&mut hash2, &mut hash1, &line.as_bytes()[&line.len()/2+i], &mut sum) { break; }
        }
    }

    sum
}

fn check<'a>(hash1: &mut HashSet<u8>, hash2: &mut HashSet<u8>, c: &u8, sum: &mut i32) -> bool {
    if hash1.contains(c) { 
        if *c >= 97 { //a
            *sum += *c as i32 - 96;
        } else {
            *sum += *c as i32 - 38;
        }

        // println!("{:?}",'a' as u32);
        // println!("{:?}",'z' as u32);
        // println!("{:?}",'A' as u32);
        // println!("{:?}",'Z' as u32);

        return true;
    } else { 
        hash2.insert(*c); 
        return false;
    }
}

pub fn day_03_02() -> i32 {
    let file_path = "src/day_03/data.txt";
    let contents;
    contents = utils::read_file(file_path);
    //let mut dict = HashMap::new();
    const CAPACITY: usize = 3;
    let mut sum: i32 = 0;
    let mut lines: Vec<&str> = Vec::new();

    for line in contents.lines().into_iter() {
        lines.push(line);

        if lines.len() == CAPACITY {
            let line1 = lines.iter().min_by_key(|p| p.len()).unwrap();

            for ch in line1.chars() {
                if lines[0].contains(ch) && 
                   lines[1].contains(ch) &&
                   lines[2].contains(ch) {

                    if ch as i8 >= 97 { //a
                        sum += ch as i32 - 96;
                    } else {
                        sum += ch as i32 - 38;
                    }                            
                    break;

                }
            }
            lines.clear();
        }
    }






    // for line in contents.lines().into_iter() {

    //     if lines.len() == CAPACITY {

    //         //println!("{:?}", lines.iter().max_by_key(|p| p.len()).unwrap().len());

    //         let length = lines.iter().max_by_key(|p| p.len()).unwrap().len() -1;

    //         for i in 0..length {
    //             //for line in &lines {
    //             for (j, line) in lines.iter().enumerate() {
    //                 if i < line.len() {
    //                     let c = line.as_bytes()[i];
    //                     //hashes[i].insert(c);
    //                     if !dict.contains_key(&c) {
    //                         dict.insert(c,  Vec::with_capacity(CAPACITY));
    //                         if let Some(x) = dict.get_mut(&c) {
    //                             for _ in 0..CAPACITY {
    //                                 x.push(false);
    //                             }
    //                         };
    //                     } 
    //                     if let Some(x) = dict.get_mut(&c) {
    //                         x[j] = true;
    //                     };
    //                     //dict.get(&c).all
    //                     if Some(dict.get(&c)).iter().all(|x| x.unwrap().iter().all(|y| *y == true)) {
    //                         if c >= 97 { //a
    //                             sum += c as i32 - 96;
    //                         } else {
    //                             sum += c as i32 - 38;
    //                         }                            
    //                         break;
    //                     }
    //                 }
    //             }
    //         }

    //         lines.clear();
    //         dict.clear();
    //     }

    //     lines.push(line);
    // }
    sum
}