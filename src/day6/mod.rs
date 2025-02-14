mod carte;

use std::collections::HashSet;
use crate::day6::carte::Carte;
// &mut Vec<Vec<bool>> => &mut [Vec<bool>]

fn _p1(_input: &str) -> usize {

    let mut carte:Carte = carte::Carte::new(_input);
    let mut visited:Vec<Vec<bool>> = Vec::new();

    for line in _input.lines() {
        visited.push(vec![false; line.len()]);
    }

    let height = carte.height;
    let width = carte.width;

    let (mut pos_x, mut pos_y) = carte.find_guard_position();
    visited[pos_y][pos_x] = true;

    while go_forward(&mut carte, &mut visited, width, height, &mut pos_x, &mut pos_y) {

    }

    let mut total = 0;

    for ligne in visited {
        for visit in ligne {
            if visit { total += 1; }
        }
    }

    total
}

// Pour p2 essayer de dessiner à la lettre comme dans l'exemple dans 1 tableau, trop compliqué

fn _p2(_input: &str) -> usize {

    let mut visited:Vec<Vec<bool>> = Vec::new();

    for line in _input.lines() {
        visited.push(vec![false; line.len()]);
    }

    let carte:Carte = carte::Carte::new(_input);

    let height = carte.height;
    let width = carte.width;

    let (pos_x, pos_y) = carte.find_guard_position();
    visited[pos_y][pos_x] = true;

    let mut temp_x = pos_x;
    let mut temp_y = pos_y;
    let mut carte_visit = carte.clone();

    // Permet de faire un passage sans obstruction pour avoir la liste des cases visité
    while go_forward(&mut carte_visit, &mut visited, width, height, &mut temp_x, &mut temp_y) {}

    let mut carte_with_obstacle = carte.clone();

    // Trop long de faire avec toute les cases
    let mut total:usize = 0;
    for y in 0..height {
        for x in 0..width {

            // On ignore si la case n'est jamais visité
            if !visited[y][x] { continue; }

            if carte.get(x, y) == '.' {
                // Peut-être le clone le souci
                carte_with_obstacle.copy(&carte);
                carte_with_obstacle.set(x, y, '#');

                let mut new_x = pos_x;
                let mut new_y = pos_y;

                let mut tableau_direction:HashSet<((usize, usize), char)> = HashSet::default();

                while go_forward_draw_path(&mut carte_with_obstacle, &mut tableau_direction, width, height, &mut new_x, &mut new_y) {
                    if tableau_direction.contains(&((new_x, new_y), carte_with_obstacle.get(new_x, new_y))) {
                        total += 1;
                        break;
                    }
                }
            }
        }
    }
    total
}
/*
fn debug_print_map(tableau: &[Vec<char>]) {
    let mut map = "".to_string();

    for ligne in tableau {
        for &c in ligne {
            map.push(c);
        }
        map.push('\n');
    }
    print!("{map}");
    println!();
}
*/
fn go_forward_draw_path(carte:&mut Carte, tableau_direction:&mut HashSet<((usize, usize), char)>, width:usize, height:usize, x:&mut usize, y:&mut usize) -> bool {
    let x_origin = *x;
    let y_origin = *y;

    let guard = carte.get(x_origin, y_origin);

    tableau_direction.insert(((*x, *y), guard));

    match guard
    {
        '^' => {
            if *y == 0 { return false; }
            *y -= 1;
        }
        '>' => {
            if *x == width - 1 { return false; }
            *x += 1;
        }
        'v' => {
            if *y == height - 1 { return false; }
            *y += 1;
        }
        '<' => {
            if *x == 0 { return false; }
            *x -= 1;
        }
        _ => {
            // debug_print_map(tableau);
        }
    }
    if carte.get(*x, *y) == '#' {
        *x = x_origin;
        *y = y_origin;
        let turning_pos:Vec<char> = vec!['^', '>', 'v', '<'];
        carte.set(*x, *y, turning_pos[(turning_pos.iter().position(|c| c == &guard).unwrap() + 1) % 4]);
    }
    else {
        carte.set(x_origin, y_origin, '.');
        carte.set(*x, *y, guard);
    }

    true
}

fn go_forward(carte:&mut Carte, visited:&mut [Vec<bool>], width:usize, height:usize, x:&mut usize, y:&mut usize) -> bool {
    let x_origin = *x;
    let y_origin = *y;

    let guard = carte.get(x_origin, y_origin);

    match guard
    {
        '^' => {
            if *y == 0 {
                carte.set(x_origin, y_origin, 'X');
                visited[y_origin][x_origin] = true;
                return false;
            }
            *y -= 1;
        }
        '>' => {
            if *x == width - 1 {
                carte.set(x_origin, y_origin, 'X');
                visited[y_origin][x_origin] = true;
                return false;
            }
            *x += 1;
        }
        'v' => {
            if *y == height - 1 {
                carte.set(x_origin, y_origin, 'X');
                visited[y_origin][x_origin] = true;
                return false;
            }
            *y += 1;
        }
        '<' => {
            if *x == 0 {
                carte.set(x_origin, y_origin, 'X');
                visited[y_origin][x_origin] = true;
                return false;
            }
            *x -= 1;
        }
        _ => {
            //debug_print_map(carte.tab);
        }
    }
    if carte.get(*x, *y) == '#' {
        *x = x_origin;
        *y = y_origin;
        let turning_pos:Vec<char> = vec!['^', '>', 'v', '<'];
        carte.set(*x, *y, turning_pos[(turning_pos.iter().position(|c| c == &guard).unwrap() + 1) % 4]);
    }
    else {
        carte.set(x_origin, y_origin, 'X');
        visited[y_origin][x_origin] = true;
        carte.set(*x, *y, guard);
    }
    true
}

pub fn p1() -> usize {
    _p1(include_str!("d6.txt"))
}

pub fn p2() -> usize {
    _p2(include_str!("d6.txt"))
}

#[cfg(test)]
mod test {
    use crate::day6::*;

    #[test]
    fn test_p1() {
        assert_eq!(41, _p1(include_str!("d6_test.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(6, _p2(include_str!("d6_test.txt")));
    }

    #[test]
    fn real_p1() {
        assert_eq!(4711, p1());
    }

    #[test]
    fn real_p2() {
        assert_eq!(1562, p2());
    }
}