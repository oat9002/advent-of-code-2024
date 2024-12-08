use std::collections::HashMap;

#[allow(dead_code)]
fn day1_1() {
    let input = include_str!("../input/day1_1.txt");
    let mut sum: i32 = 0;
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let sp: Vec<&str> = line.split(' ').filter(|&s| !s.is_empty()).collect();
        left.push(sp[0].to_string().parse().unwrap());
        right.push(sp[1].to_string().parse().unwrap());
    }

    left.sort();
    right.sort();

    for i in 0..left.len() {
        sum += (right[i] - left[i]).abs();
    }

    println!("sum: {}", sum);
}

fn day1_2() {
    let input = include_str!("../input/day1_2.txt");
    let mut sum: i32 = 0;
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut temp: HashMap<i32, i32> = HashMap::new();

    for line in input.lines() {
        let sp: Vec<&str> = line.split(' ').filter(|&s| !s.is_empty()).collect();
        left.push(sp[0].to_string().parse().unwrap());
        right.push(sp[1].to_string().parse().unwrap());
    }

    for i in 0..left.len() {
        if temp.contains_key(&left[i]) {
            sum += left[i] * temp.get(&left[i]).unwrap();
        } else {
            let appear = right.iter().filter(|&x| *x == left[i]).count() as i32;
            sum += left[i] * appear;
            temp.insert(left[i], appear);
        }
    }

    println!("sum: {}", sum);
}
