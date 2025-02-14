use std::collections::HashSet;

fn _p1(_input: &str) -> usize {

    let mut tableau:Vec<Vec<char>> = Vec::new();
    let mut visited:Vec<Vec<bool>> = Vec::new();

    for line in _input.lines() {
        tableau.push(line.chars().collect());
        visited.push(vec![false; line.len()]);
    }

    let height = tableau.len();
    let width = tableau[0].len();

    let mut pos_x = 0;
    let mut pos_y = 0;

    // Find the guards position
    for y in 0..height {
        for x in 0..width {
            if tableau[y][x] == '^' {
                pos_x = x;
                pos_y = y;
                visited[pos_y][pos_x] = true;
            }
        }
    }

    while go_forward(&mut tableau, &mut visited, width, height, &mut pos_x, &mut pos_y) {

    }

    let mut total = 0;

    for y in 0..height {
        for x in 0..width {
            if visited[y][x] {
                total += 1;
            }
        }
    }

    total
}

// Pour p2 essayer de dessiner à la lettre comme dans l'exemple dans 1 tableau, trop compliqué

fn _p2(_input: &str) -> usize {

    let mut tableau:Vec<Vec<char>> = Vec::new();
    let mut visited:Vec<Vec<bool>> = Vec::new();

    for line in _input.lines() {
        tableau.push(line.chars().collect());
        visited.push(vec![false; line.len()]);
    }

    let height = tableau.len();
    let width = tableau[0].len();

    let mut pos_x = 0;
    let mut pos_y = 0;

    // Find the guards position
    for y in 0..height {
        for x in 0..width {
            if tableau[y][x] == '^' {
                pos_x = x;
                pos_y = y;
                visited[pos_y][pos_x] = true;
            }
        }
    }
    let mut temp_x = pos_x.clone();
    let mut temp_y = pos_y.clone();
    let mut tableau_visit = tableau.clone();

    // Permet de faire un passage sans obstruction pour avoir la liste des cases visité
    while go_forward(&mut tableau_visit, &mut visited, width, height, &mut temp_x, &mut temp_y) {}

    // Trop long de faire avec toute les cases
    let mut total:usize = 0;
    for y in 0..height {
        for x in 0..width {

            // On ignore si la case n'est jamais visité
            if !visited[y][x] { continue; }

            if tableau[y][x] == '.' {
                let mut tableau_with_obstacle = tableau.clone();
                tableau_with_obstacle[y][x] = '#';

                let mut new_x = pos_x.clone();
                let mut new_y = pos_y.clone();

                let mut tableau_direction:HashSet<((usize, usize), char)> = HashSet::default();

                while go_forward_draw_path(&mut tableau_with_obstacle, &mut tableau_direction, width, height, &mut new_x, &mut new_y) {
                    if tableau_direction.contains(&((new_x, new_y), tableau_with_obstacle[new_y][new_x])) {
                        total += 1;
                        break;
                    }
                }
            }
        }
    }
    total
}

fn debug_print_map(tableau: &Vec<Vec<char>>) {
    let mut map = "".to_string();
    let width = tableau[0].len();
    let height = tableau.len();

    for y in 0..height {
        for x in 0..width {
            map.push(tableau[y][x]);
        }
        map.push('\n');
    }
    print!("{map}");
    println!();
}

/*
fn debug_print_double_map(tableau: &Vec<Vec<char>>, tableau_direction: &Vec<Vec<char>>) {
    let mut map = "".to_string();
    let width = tableau[0].len();
    let height = tableau.len();

    for y in 0..height {
        for x in 0..width {
            map.push(tableau[y][x]);
        }
        map.push('\t');
        for x in 0..width {
            map.push(tableau_direction[y][x]);
        }
        map.push('\n');
    }
    print!("{map}");
    println!();
}
*/

fn go_forward_draw_path(tableau:&mut Vec<Vec<char>>, tableau_direction:&mut HashSet<((usize, usize), char)>, width:usize, height:usize, x:&mut usize, y:&mut usize) -> bool {
    let x_origin = *x;
    let y_origin = *y;

    let guard = tableau[y_origin][x_origin];

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
            debug_print_map(&tableau);
        }
    }
    if tableau[*y][*x] == '#' {
        *x = x_origin;
        *y = y_origin;
        let turning_pos:Vec<char> = vec!['^', '>', 'v', '<'];
        tableau[*y][*x] = turning_pos[(turning_pos.iter().position(|c| c == &guard).unwrap() + 1) % 4];
    }
    else {
        tableau[y_origin][x_origin] = '.';
        tableau[*y][*x] = guard;
    }

    true
}

/*
fn go_forward_draw_path(tableau:&mut Vec<Vec<char>>, tableau_direction:&mut Vec<Vec<char>>, total:&mut usize, width:usize, height:usize, x:&mut usize, y:&mut usize) -> bool {
    let x_origin = *x;
    let y_origin = *y;

    let guard = tableau[y_origin][x_origin];

    // A chaque tour on vérifie si lorsqu'on tourne et avance tout droit on tombe sur un mur et on a déjà tourné à ce mur
    if guard == '<' {
        for search_y in (0..*y).rev() {
            if tableau[search_y][*x] == '#' && tableau_direction[search_y+1][*x] == '+' { *total+= 1; }
        }
    }
    else if guard == '>' {
        for search_y in *y+1..height {
            if tableau[search_y][*x] == '#' && tableau_direction[search_y-1][*x] == '+' { *total+= 1; }
        }
    }
    else if guard == '^' {
        for search_x in *x+1..width {
            if tableau[*y][search_x] == '#' && tableau_direction[*y][search_x-1] == '+' { *total += 1; }
        }
    }
    else if guard == 'v' {
        for search_x in (0..*x).rev() {
            if tableau[*y][search_x] == '#' && tableau_direction[*y][search_x+1] == '+' { *total += 1; }
        }
    }

    // On regarde ce qu'il y a en face du garde si la position est valable sinon return false
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
            debug_print_map(&tableau);
        }
    }

    // Si on voit un mur en face, le garde tourne à droite
    if tableau[*y][*x] == '#' {

        *x = x_origin;
        *y = y_origin;

        // A chaque tour on vérifie si lorsqu'on se retourne (mur en face et mur posé à droite) et avance tout droit on tombe sur un mur et on a déjà tourné à ce mur
        if guard == '<' {
            for search_x in *x+1..width {
                if tableau[*y][search_x] == '#' && tableau_direction[*y][search_x-1] == '+' { *total+= 1; }
            }
        }
        else if guard == '>' {
            for search_x in (0..*x).rev() {
                if tableau[*y][search_x] == '#' && tableau_direction[*y][search_x+1] == '+' { *total += 1; }
            }
        }
        else if guard == '^' {
            for search_y in *y+1..height {
                if tableau[search_y][*x] == '#' && tableau_direction[search_y-1][*x] == '+' { *total+= 1; }
            }
        }
        else if guard == 'v' {
            for search_y in (0..*y).rev() {
                if tableau[search_y][*x] == '#' && tableau_direction[search_y+1][*x] == '+' { *total+= 1; }
            }
        }
        let turning_pos:Vec<char> = vec!['^', '>', 'v', '<'];
        let new_guard_rotation = turning_pos[(turning_pos.iter().position(|c| c == &guard).unwrap() + 1) % 4];

        tableau_direction[*y][*x] = '+';
        tableau[*y][*x] = new_guard_rotation;
    }
    else {
        // Détermine la direction du garde
        let mut direction = 'O';
        if guard == '^' || guard == 'v' {
            direction = '|';
        }
        else if guard == '<' || guard == '>' {
            direction = '-';
        }

        let old_direction = tableau_direction[*y][*x];

        // Les chemin traversé en opposé à la direction où le garde va devient un croisement
        if old_direction == '|' {
            if direction == '-' { tableau_direction[*y][*x] = '+'; }
        }
        else if old_direction == '-' {
            if direction == '|' { tableau_direction[*y][*x] = '+'; }
        }
        else if old_direction == '.' {
            tableau_direction[*y][*x] = direction;
        }

        tableau[y_origin][x_origin] = '.';
        tableau[*y][*x] = guard;
    }
    true
}
*/

fn go_forward(tableau:&mut Vec<Vec<char>>, visited:&mut Vec<Vec<bool>>, width:usize, height:usize, x:&mut usize, y:&mut usize) -> bool {
    let x_origin = *x;
    let y_origin = *y;

    let guard = tableau[y_origin][x_origin];

    match guard
    {
        '^' => {
            if *y == 0 {
                tableau[y_origin][x_origin] = 'X';
                visited[y_origin][x_origin] = true;
                return false;
            }
            *y -= 1;
        }
        '>' => {
            if *x == width - 1 {
                tableau[y_origin][x_origin] = 'X';
                visited[y_origin][x_origin] = true;
                return false;
            }
            *x += 1;
        }
        'v' => {
            if *y == height - 1 {
                tableau[y_origin][x_origin] = 'X';
                visited[y_origin][x_origin] = true;
                return false;
            }
            *y += 1;
        }
        '<' => {
            if *x == 0 {
                tableau[y_origin][x_origin] = 'X';
                visited[y_origin][x_origin] = true;
                return false;
            }
            *x -= 1;
        }
        _ => {
            debug_print_map(&tableau);
        }
    }
    if tableau[*y][*x] == '#' {
        *x = x_origin;
        *y = y_origin;
        let turning_pos:Vec<char> = vec!['^', '>', 'v', '<'];
        tableau[*y][*x] = turning_pos[(turning_pos.iter().position(|c| c == &guard).unwrap() + 1) % 4];
    }
    else {
        tableau[y_origin][x_origin] = 'X';
        visited[y_origin][x_origin] = true;
        tableau[*y][*x] = guard;
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