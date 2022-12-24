#[path = "../../src/utils/mod.rs"] mod utils;

use regex::Regex;

pub fn day_04_01() -> i32 {
    let file_path = "src/day_04/data.txt";
    let contents;
    contents = utils::read_file(file_path);
    let mut count = 0;

    for line in contents.lines().into_iter() {
        let re = Regex::new(r"\d+").unwrap();
        let findings: Vec<i32> = re.find_iter(line).map(|l| i32::from_str_radix(l.as_str(), 10).unwrap_or(0) ).collect();

        if (findings[0]-findings[2] <= 0 && findings[1]-findings[3] >=0) ||
            (findings[0]-findings[2] >= 0 && findings[1]-findings[3] <=0) {
                count += 1;    
        }
    }

    count
}

pub fn day_04_02() -> i32 {
    let file_path = "src/day_04/data.txt";
    let contents;
    contents = utils::read_file(file_path);
    let mut count = 0;

    for line in contents.lines().into_iter() {
        let re = Regex::new(r"\d+").unwrap();
        let findings: Vec<i32> = re.find_iter(line).map(|l| i32::from_str_radix(l.as_str(), 10).unwrap_or(0) ).collect();

        if (findings[0] <= findings[3] && findings[0] >= findings[2]) ||
            (findings[1] >= findings[2] && findings[1] <= findings[3]) ||
            (findings[0] <= findings[3] && findings[1] >= findings[2]) || 
            (findings[2] <= findings[1] && findings[3] >= findings[0]) {
            count += 1;    
        }
    }

    count
}