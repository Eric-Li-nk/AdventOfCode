use std::str::FromStr;

fn _p1(_input: &str) -> usize {

    let mut safe_report = 0;

    for line in _input.lines() {

        let mut values:Vec<u8> = line.split(" ").map(|x| u8::from_str(x).unwrap()).collect::<Vec<u8>>();

        let mut safe = true;

        if values.is_sorted() {
            for i in 1..values.len() {
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

pub fn p1() -> usize {
    _p1(include_str!("d2.txt"))
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