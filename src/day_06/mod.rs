#[path = "../../src/utils/mod.rs"] mod utils;
use std::collections::VecDeque;
use std::collections::HashSet;

pub fn day_06_01() -> usize {
    let file_path = "src/day_06/data.txt";
    let contents;
    contents = utils::read_file(file_path);
    let mut q: VecDeque<char> = VecDeque::new();
    let mut h = HashSet::new();
    let mut found; 
    const MARKER: usize = 3;

    for (i, c) in contents.chars().into_iter().enumerate() {
        
        q.push_back(c);

        if i >= MARKER  {
            
            found = true;
            for index in 0..q.len() {
                if h.insert(q[index]) == false {
                    found = false;
                    break;
                }
            }
            if found == true { 
                return i +1; 
            }
            h.clear();
            q.pop_front();
        }
    }
    0
}

pub fn day_06_02() -> usize {
    let file_path = "src/day_06/data.txt";
    let contents;
    contents = utils::read_file(file_path);
    let mut q: VecDeque<char> = VecDeque::new();
    let mut h = HashSet::new();
    let mut found; 
    const MARKER: usize = 13;

    for (i, c) in contents.chars().into_iter().enumerate() {
        
        q.push_back(c);

        if i >= MARKER  {
            
            found = true;
            for index in 0..q.len() {
                if h.insert(q[index]) == false {
                    found = false;
                    break;
                }
            }
            if found == true { 
                return i +1; 
            }
            h.clear();
            q.pop_front();
        }
    }
   0

}