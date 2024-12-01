use std::{collections::HashMap, io};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut left_val = vec![];
    let mut right_val = vec![];

    take_input(&mut left_val, &mut right_val)?;

    // solve_part_1(left_val, right_val);
    solve_part_2(&left_val, &right_val);
    Ok(())
}

fn solve_part_1(left_val: Vec<i32>, right_val: Vec<i32>) {
    let mut left_val = left_val.clone();
    let mut right_val = right_val.clone();

    left_val.sort();
    right_val.sort();

    let sum = left_val
        .iter()
        .zip(right_val.iter())
        .map(|(x, y)| (x - y).abs())
        .sum::<i32>();

    println!("{}", sum);
}

fn solve_part_2(left_val: &[i32], right_val: &[i32]) {
    let mut right_freq = HashMap::new();

    for x in right_val.iter() {
        match right_freq.get_mut(x) {
            Some(val) => {
                *val += 1;
            }
            None => {
                right_freq.insert(x, 1);
            }
        }
    }

    let mut sum = 0;
    for k in left_val.iter() {
        if let Some(val) = right_freq.get(k) {
            sum += k * val;
        }
    }

    println!("{}", sum);
}
fn take_input(
    left_val: &mut Vec<i32>,
    right_val: &mut Vec<i32>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input)?;
        let mut values = input.split_whitespace();

        let left_value = match values.next() {
            Some(val) => match val.parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    break;
                }
            },
            None => {
                break;
            }
        };

        let right_value = match values.next() {
            Some(val) => match val.parse::<i32>() {
                Ok(val) => val,
                Err(_) => {
                    break;
                }
            },
            None => {
                break;
            }
        };
        left_val.push(left_value);
        right_val.push(right_value);
        input.clear();
    }
    Ok(())
}
