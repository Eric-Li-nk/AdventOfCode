fn get_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        if let [left, right] = line.split_whitespace().collect::<Vec<_>>()[..] {
            left_list.push(left.parse().unwrap());
            right_list.push(right.parse().unwrap());
        }
    }

    (left_list, right_list)
}

fn _p1(_input: &str) -> u32 {

    let (mut left_list, mut right_list) = get_lists(_input);

    left_list.sort_unstable();
    right_list.sort_unstable();

    let mut total_distance = 0;

    for i in 0..left_list.len() {
        total_distance += left_list[i].abs_diff(right_list[i]);
    }
    total_distance
}

fn _p2(_input: &str) -> u32 {

    let (left_list, right_list) = get_lists(_input);

    let mut similarity_score:u32 = 0;

    for i in 0..left_list.len() {
        similarity_score += left_list[i] * right_list.iter().filter(|&&x| x == left_list[i]).count() as u32;
    }
    similarity_score
}

pub fn p1() -> u32 {
    _p1(include_str!("d1.txt"))
}

pub fn p2() -> u32 {
    _p2(include_str!("d1.txt"))
}

#[cfg(test)]
mod test {
    use crate::day1::*;

    #[test]
    fn test_p1() {
        assert_eq!(11, _p1(include_str!("d1_test.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(31, _p2(include_str!("d1_test.txt")));
    }

    #[test]
    fn real_p1() {
        assert_eq!(1197984, p1());
    }

    #[test]
    fn real_p2() {
        assert_eq!(23387399, p2());
    }
}