use std::fs;
#[path = "../../src/utils/mod.rs"] mod utils;
use utils::read_file;

pub fn day_01_01() -> i32 {
    let file_path = "src/day_01/data.txt";
    let contents;
    let mut max = 0;
    let mut local_max = 0;

    contents = read_file(file_path);

    for line in contents.lines().into_iter() {
        if line.is_empty() {
            if local_max > max {
                max = local_max;
            }
            local_max = 0;
        } else {
            local_max += line.parse::<i32>().unwrap();
        }
    }

    if local_max > max {
        max = local_max;
    }

    max
}

pub fn day_01_02() -> i32 {
    let file_path = "src/day_01/data.txt";
    let contents;
    let mut sum = 0;
    let mut vec = Vec::new();

    contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    for line in contents.lines().into_iter() {
        if line.is_empty() {
            vec.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<i32>().unwrap();
        }
    }

    if sum > 0 {
        vec.push(sum);
    }

    vec.sort_by(|a, b| b.cmp(a));
    vec.iter().take(3).sum()

}
