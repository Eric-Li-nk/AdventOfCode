use std::str::FromStr;
use std::ops::Add;

fn _p1Tr(input: &str) -> u64{
    let mut unsafe_folder :u64 = 0;
    //println!("lines : {}", input.lines().count());
    for i in input.lines(){
        let mut it = i.split_ascii_whitespace();
        let mut current = it.next();
        let mut is_positive: bool = false;
        let mut count = 0;
        while let Some(x_str) = current {
            count += 1;
            let test = it.next();
            // Conversion `x_str` en u64
            if let Ok(x) = x_str.parse::<u64>() {

                if let Some(next_str) = test {
                    if let Ok(next_value) = next_str.parse::<u64>() {
                        if x.abs_diff(next_value) > 3 {
                            unsafe_folder += 1;
                            break;
                        }
                        else if x.abs_diff(next_value) == 0 {
                            unsafe_folder += 1;
                            break;
                        }

                        else if((x as i64 - next_value as i64) < 0){
                            if(is_positive == true && count != 1){
                                unsafe_folder += 1;
                                break;
                            }
                            is_positive = false;
                        }
                        else if (x as i64 - next_value as i64) > 0 {
                            if(!is_positive && count != 1){
                                unsafe_folder += 1;
                                break;
                            }
                            is_positive = true;
                        }
                    }
                }
            }
            current = test;
        }

    }
    return input.lines().count() as u64 - unsafe_folder;
}
fn _p1(_input: &str) -> u32 {

    let mut safe_report = 0;

    for line in _input.lines() {

        let mut values:Vec<u8> = line.split(" ").map(|x| u8::from_str(x).unwrap()).collect::<Vec<u8>>();

        let mut safe = true;

        if values.is_sorted() {
            for i  in 1..values.len() {
                let difference = values[i] - values[i - 1];
                if difference == 0 || difference > 3 { safe = false ; break; }
            }
        }
        else {
            values.reverse();
            if values.is_sorted() {
                for i in 1..values.len() {
                    let difference = values[i] - values[i - 1];
                    if difference == 0 || difference > 3 { safe = false ; break; }
                }
            }
            else { safe = false; }
        }
        if safe { safe_report += 1; }
    }
    safe_report
}

fn _p2(_input: &str) -> usize {

    let mut safe_report = 0;

    'outer: for line in _input.lines() {

        let values:Vec<u8> = line.split(" ").map(|x| u8::from_str(x).unwrap()).collect::<Vec<u8>>();

        if values.is_sorted() {
            for i in 1..values.len() {
                let difference = values[i] - values[i - 1];
                if difference == 0 || difference > 3 {
                    if validate(&values) { break; } else { continue 'outer; }
                }
            }
        }
        else {
            let mut reversed_values = values.clone();
            reversed_values.reverse();
            if reversed_values.is_sorted() {
                for i in 1..reversed_values.len() {
                    let difference = reversed_values[i] - reversed_values[i - 1];
                    if difference == 0 || difference > 3 {
                        if validate(&reversed_values) { break; } else { continue 'outer; }
                    }
                }
            }
            else {
                if !validate(&values)  {
                    let mut reversed_values = values.clone();
                    reversed_values.reverse();
                    if !validate(&reversed_values) {
                        continue 'outer;
                    }
                }
            }
        }
        safe_report += 1;
    }
    safe_report
}

fn validate(values:&Vec<u8>) -> bool {
    'inner: for i in 0..values.len() {
        let mut cloned_values = values.clone();
        cloned_values.remove(i);
        if cloned_values.is_sorted() {
            for i in 1..cloned_values.len() {
                let difference = cloned_values[i] - cloned_values[i - 1];
                if difference == 0 || difference > 3 { continue 'inner; }
            }
            return true;
        }
        else { continue 'inner; }
    }
    false
}

pub fn p1() -> u32 {
    //_p1(include_str!("d2.txt"))
    _p1Tr(include_str!("d2.txt")) as u32
}

pub fn p2() -> usize {
    _p2(include_str!("d2.txt"))
}

#[cfg(test)]
mod test {
    use crate::day2::*;

    #[test]
    fn test_p1() {
        assert_eq!(2, _p1(include_str!("d2_test.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(4, _p2(include_str!("d2_test.txt")));
    }

    #[test]
    fn real_p1() {
        assert_eq!(390, p1());
    }

    #[test]
    fn real_p2() {
        assert_eq!(439, p2());
    }
}