
fn _p1(_input: &str) -> usize {
    let mut total = 0;
    let mut index:Vec<_>;
    for line in _input.lines() {
        index = line.match_indices("mul(").map(|x| {x.0}).collect();
        for i in index {
            total += get_mult(i, line);
        }
    }
    total
}

fn _p2(_input: &str) -> usize {
    let mut total = 0;
    let mut mult_activated = true;
    for line in _input.lines() {
        for (i, _) in line.chars().enumerate() {
            if i + 3 < line.len() {
                if &line[i..i+4] == "do()" {
                    mult_activated = true;
                }
            }
            if i + 6 < line.len() {
                if &line[i..i+7] == "don't()" {
                    mult_activated = false;
                }
            }
            if mult_activated {
                total += get_mult(i, line);
            }
        }
    }
    total
}

fn get_mult(index: usize, line:&str) -> usize {
    let mut closing_parentesis_index = 0;
    let mut comma_index = 0;
    if index + 7 < line.len() {
        if &line[index..index+4] != "mul(" {
            return 0;
        }
    }
    else { return 0; }
    for i in 0..8 {
        let c = line.chars().nth(index + 4 + i).unwrap();
        if c == '\n' {
            return 0
        }
        else if c == ')' {
            closing_parentesis_index = i;
            break;
        }
        else if c == ',' {
            comma_index = i;
        }
        else if !c.is_digit(10) {
            return 0
        }
    }
    if comma_index == 0 || closing_parentesis_index == 0
    {
        return 0
    }
    let digits = &line[index+4..index+4+closing_parentesis_index].split(",").collect::<Vec<&str>>();
    digits[0].parse::<usize>().unwrap() * digits[1].parse::<usize>().unwrap()
}

pub fn p1() -> usize {
    _p1(include_str!("d3.txt"))
}

pub fn p2() -> usize {
    _p2(include_str!("d3.txt"))
}

#[cfg(test)]
mod test {
    use crate::day3::*;

    #[test]
    fn test_p1() {
        assert_eq!(161, _p1(include_str!("d3_test.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(48, _p2(include_str!("d3_test.txt")));
    }

    #[test]
    fn real_p1() {
        assert_eq!(170068701, p1());
    }

    #[test]
    fn real_p2() {
        assert_eq!(78683433, p2());
    }
}