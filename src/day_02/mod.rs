#[path = "../../src/utils/mod.rs"] mod utils;
use std::collections::HashMap;

pub fn day_02_01() -> i32 {
    let file_path = "src/day_02/data.txt";
    let contents;
    let mut point = 0;
    contents = utils::read_file(file_path);

    let mut dict = HashMap::new();
    dict.insert( "X", 1 );
    dict.insert( "Y", 2 );
    dict.insert( "Z", 3 );

    for line in contents.lines().into_iter() {
        let v: Vec<&str> = line.split(' ').collect();

        point += *dict.get(v[1]).unwrap();
        
        // A - X = rock
        // B - Y = paper
        // C - Z = scissors

        if (v[1].eq("X") && v[0].eq("C")) ||
           (v[1].eq("Y") && v[0].eq("A")) ||
            (v[1].eq("Z") && v[0].eq("B")) {
            
                point += 6;
        
        } else 
            if (v[1].eq("X") && v[0].eq("A")) ||
                (v[1].eq("Y") && v[0].eq("B")) ||
                (v[1].eq("Z") && v[0].eq("C")) {
            
                    point += 3;
        }
    }

    point
}

pub fn day_02_02() -> i32 {
    let file_path = "src/day_02/data.txt";
    let contents;
    let mut point = 0;
    let mut selected;
    contents = utils::read_file(file_path);

    let mut dict = HashMap::new();
    dict.insert( "A", 1 );
    dict.insert( "B", 2 );
    dict.insert( "C", 3 );

    for line in contents.lines().into_iter() {
        let v: Vec<&str> = line.split(' ').collect();
        
        // X = lose
        // Y = draw
        // Z = win
        selected = "A";
        if v[1].eq("X") {
            match v[0] {
                "A" => selected = "C",
                "B" => selected = "A",
                "C" => selected = "B",
                &_ => todo!()
            }
        } else if v[1].eq("Y") {
            selected = v[0];
            point += 3;
        } else if v[1].eq("Z"){
            match v[0] {
                "A" => selected = "B",
                "B" => selected = "C",
                "C" => selected = "A",
                &_ => todo!()
            }
            point += 6;
        }

        point += *dict.get(selected).unwrap();
    }

    point
}