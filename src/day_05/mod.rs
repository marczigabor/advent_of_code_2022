#[path = "../../src/utils/mod.rs"] mod utils;

use regex::Regex;

fn init () -> Vec<Vec<char>> {
    let mut vecs: Vec<Vec<char>> = Vec::new();

    let mut stack01 = Vec::new();
    stack01.push('F');
    stack01.push('H');
    stack01.push('B');
    stack01.push('V');
    stack01.push('R');
    stack01.push('Q');
    stack01.push('D');
    stack01.push('P');

    let mut stack02 = Vec::new();
    stack02.push('L');
    stack02.push('D');
    stack02.push('Z');
    stack02.push('Q');
    stack02.push('W');
    stack02.push('V');

    let mut stack03 = Vec::new();
    stack03.push('H');
    stack03.push('L');
    stack03.push('Z');
    stack03.push('Q');
    stack03.push('G');
    stack03.push('R');
    stack03.push('P');
    stack03.push('C');

    let mut stack04 = Vec::new();
    stack04.push('R');
    stack04.push('D');
    stack04.push('H');
    stack04.push('F');
    stack04.push('J');
    stack04.push('V');
    stack04.push('B');

    let mut stack05 = Vec::new();
    stack05.push('Z');
    stack05.push('W');
    stack05.push('L');
    stack05.push('C');

    let mut stack06 = Vec::new();
    stack06.push('J');
    stack06.push('R');
    stack06.push('P');
    stack06.push('N');
    stack06.push('T');
    stack06.push('G');
    stack06.push('V');
    stack06.push('M');

    let mut stack07 = Vec::new();
    stack07.push('J');
    stack07.push('R');
    stack07.push('L');
    stack07.push('V');
    stack07.push('M');
    stack07.push('B');
    stack07.push('S');

    let mut stack08 = Vec::new();
    stack08.push('D');
    stack08.push('P');
    stack08.push('J');

    let mut stack09 = Vec::new();
    stack09.push('D');
    stack09.push('C');
    stack09.push('N');
    stack09.push('W');
    stack09.push('V');

    vecs.push(stack01);
    vecs.push(stack02);
    vecs.push(stack03);
    vecs.push(stack04);
    vecs.push(stack05);
    vecs.push(stack06);
    vecs.push(stack07);
    vecs.push(stack08);
    vecs.push(stack09);

    vecs
}

pub fn day_05_01() -> String {
    let file_path = "src/day_05/data.txt";
    let contents;
    contents = utils::read_file(file_path);

    //init 
    let mut vecs = init();

    for line in contents.lines().into_iter() {
        let re = Regex::new(r"\d+").unwrap();
        let moves: Vec<i32> = re.find_iter(line).map(|l| i32::from_str_radix(l.as_str(), 10).unwrap_or(0) ).collect();

        for _ in 0..moves[0] {
            let u = vecs[(moves[1]-1) as usize].pop().unwrap();
            vecs[(moves[2]-1) as usize].push(u);
        }
    }

    let mut result = String::from("");
    for mut v in vecs {
        result.push(v.pop().unwrap());
    }

    result
}

pub fn day_05_02() -> String {
    let file_path = "src/day_05/data.txt";
    let contents;
    contents = utils::read_file(file_path);

    //init 
    let mut vecs = init();

    for line in contents.lines().into_iter() {
        let re = Regex::new(r"\d+").unwrap();
        let moves: Vec<i32> = re.find_iter(line).map(|l| i32::from_str_radix(l.as_str(), 10).unwrap_or(0) ).collect();
        let mut tmp_chars: Vec<char> = Vec::new();

        for _ in 0..moves[0] {
            let u = vecs[(moves[1]-1) as usize].pop().unwrap();
            tmp_chars.push(u);
        }

        loop {
            let p = tmp_chars.pop();
            if p == None {
                break;
            }
            vecs[(moves[2]-1) as usize].push(p.unwrap());
        }
    }

    let mut result = String::from("");
    for mut v in vecs {
        result.push(v.pop().unwrap());
    }
    result

}