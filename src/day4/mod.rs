
fn _p1(_input: &str) -> usize {

    let mut tableau:Vec<Vec<char>> = Vec::new();
    let mut word:Vec<char> = Vec::new();

    word.push('X');
    word.push('M');
    word.push('A');
    word.push('S');

    let mut width = 0;
    let mut height = 0;

    for line in _input.lines() {
        width = line.len();
        height = _input.lines().count();
        tableau.push(line.chars().collect());
    }
    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            total += search(&tableau, &word, width, height, x, y);
        }
    }

    total
}

fn _p2(_input: &str) -> usize {
    let mut tableau:Vec<Vec<char>> = Vec::new();
    let mut word:Vec<char> = Vec::new();

    word.push('M');
    word.push('A');
    word.push('S');

    let mut width = 0;
    let mut height = 0;

    for line in _input.lines() {
        width = line.len();
        height = _input.lines().count();
        tableau.push(line.chars().collect());
    }
    let mut total = 0;
    for y in 0..height {
        for x in 0..width {
            total += search_x(&tableau, &word, width, height, x, y);
        }
    }

    total
}

fn out_of_bounds(width:usize, height:usize, x:usize, y:usize) -> bool {
    x >= width || y >= height
}

fn search(tableau:&Vec<Vec<char>>, word:&Vec<char>, width:usize, height:usize, x:usize, y:usize) -> usize {
    if tableau[y][x] != word[0] { 0; }
    let mut total = 0;
    total += search_left(tableau, word, width, height, x, y);
    total += search_right(tableau, word, width, height, x, y);
    total += search_up(tableau, word, width, height, x, y);
    total += search_down(tableau, word, width, height, x, y);
    total += search_diagonal_up_left(tableau, word, width, height, x, y);
    total += search_diagonal_up_right(tableau, word, width, height, x, y);
    total += search_diagonal_down_left(tableau, word, width, height, x, y);
    total += search_diagonal_down_right(tableau, word, width, height, x, y);
    total
}

fn search_x(tableau:&Vec<Vec<char>>, word:&Vec<char>, width:usize, height:usize, x:usize, y:usize) -> usize {
    if tableau[y][x] != word[1] { 0; }
    search_diagonale_x(tableau, word, width, height, x, y)
}

fn search_diagonale_x(tableau:&Vec<Vec<char>>, word:&Vec<char>, width:usize, height:usize, x:usize, y:usize) -> usize
{
    if x < 1 || y < 1 || x > width-1 || y > height-1 {
        return 0;
    }
    let a = search_diagonal_down_left(tableau, word, width, height, x+1, y-1);
    let b = search_diagonal_up_right(tableau, word, width, height, x-1, y+1);
    let c = search_diagonal_down_right(tableau, word, width, height, x-1, y-1);
    let d = search_diagonal_up_left(tableau, word, width, height, x+1, y+1);

    if (a == 1 || b == 1) && (c == 1 || d == 1) {
        return 1;
    }
    0
}
fn search_diagonal_up_left(tableau:&Vec<Vec<char>>, word:&Vec<char>, width:usize, height:usize, x_tmp:usize, y_tmp:usize) -> usize {
    let mut x = x_tmp;
    let mut y = y_tmp;
    for i in 0..word.len() {
        if out_of_bounds(width, height, x, y) || tableau[y][x] != word[i] {
            return 0;
        }
        if tableau[y][x] == word[word.len()-1] {
            return 1;
        }
        if x == 0 || y == 0 { return 0;}
        x -= 1;
        y -= 1;
    }
    1
}

fn search_diagonal_up_right(tableau:&Vec<Vec<char>>, word:&Vec<char>, width:usize, height:usize, x_tmp:usize, y_tmp:usize) -> usize {
    let mut x = x_tmp;
    let mut y = y_tmp;
    for i in 0..word.len() {
        if out_of_bounds(width, height, x, y) || tableau[y][x] != word[i] {
            return 0;
        }
        if tableau[y][x] == word[word.len()-1] {
            return 1;
        }
        if y == 0 { return 0;}
        x += 1;
        y -= 1;
    }
    1
}

fn search_diagonal_down_right(tableau:&Vec<Vec<char>>, word:&Vec<char>, width:usize, height:usize, x_tmp:usize, y_tmp:usize) -> usize {
    let mut x = x_tmp;
    let mut y = y_tmp;
    for i in 0..word.len() {
        if out_of_bounds(width, height, x, y) || tableau[y][x] != word[i] {
            return 0;
        }
        if tableau[y][x] == word[word.len()-1] {
            return 1;
        }
        x += 1;
        y += 1;
    }
    1
}

fn search_diagonal_down_left(tableau:&Vec<Vec<char>>, word:&Vec<char>, width:usize, height:usize, x_tmp:usize, y_tmp:usize) -> usize {
    let mut x = x_tmp;
    let mut y = y_tmp;
    for i in 0..word.len() {
        if out_of_bounds(width, height, x, y) || tableau[y][x] != word[i] {
            return 0;
        }
        if tableau[y][x] == word[word.len()-1] {
            return 1;
        }
        if x == 0 { return 0;}
        x -= 1;
        y += 1;
    }
    1
}

fn search_right(tableau:&Vec<Vec<char>>, word:&Vec<char>, width:usize, height:usize, x_tmp:usize, y_tmp:usize) -> usize {
    let mut x = x_tmp;
    let y = y_tmp;
    for i in 0..word.len() {
        if out_of_bounds(width, height, x, y) || tableau[y][x] != word[i] {
            return 0;
        }
        if tableau[y][x] == word[word.len()-1] {
            return 1;
        }
        x += 1;
    }
    1
}
fn search_left(tableau:&Vec<Vec<char>>, word:&Vec<char>, width:usize, height:usize, x_tmp:usize, y_tmp:usize) -> usize {
    let mut x = x_tmp;
    let y = y_tmp;
    for i in 0..word.len() {
        if out_of_bounds(width, height, x, y) || tableau[y][x] != word[i] {
            return 0;
        }
        if tableau[y][x] == word[word.len()-1] {
            return 1;
        }
        if x == 0 { return 0; }
        x -= 1;
    }
    1
}
fn search_up(tableau:&Vec<Vec<char>>, word:&Vec<char>, width:usize, height:usize, x_tmp:usize, y_tmp:usize) -> usize {
    let x = x_tmp;
    let mut y = y_tmp;
    for i in 0..word.len() {
        if out_of_bounds(width, height, x, y) || tableau[y][x] != word[i] {
            return 0;
        }
        if tableau[y][x] == word[word.len()-1] {
            return 1;
        }
        if y == 0 { return 0; }
        y -= 1;
    }
    1
}
fn search_down(tableau:&Vec<Vec<char>>, word:&Vec<char>, width:usize, height:usize, x_tmp:usize, y_tmp:usize) -> usize {
    let x = x_tmp;
    let mut y = y_tmp;
    for i in 0..word.len() {
        if out_of_bounds(width, height, x, y) || tableau[y][x] != word[i] {
            return 0;
        }
        if tableau[y][x] == word[word.len()-1] {
            return 1;
        }
        y += 1;
    }
    1
}

pub fn p1() -> usize {
    _p1(include_str!("d4.txt"))
}

pub fn p2() -> usize {
    _p2(include_str!("d4.txt"))
}

#[cfg(test)]
mod test {
    use crate::day4::*;

    #[test]
    fn test_p1() {
        assert_eq!(18, _p1(include_str!("d4_test.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(9, _p2(include_str!("d4_test.txt")));
    }

    #[test]
    fn real_p1() {
        assert_eq!(2545, p1());
    }

    #[test]
    fn real_p2() {
        assert_eq!(1886, p2());
    }
}