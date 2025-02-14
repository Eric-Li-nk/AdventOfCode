use std::collections::HashMap;

// Au début tableau d'indices + tableau des numéros !! Pas possible si un boucle est créé
// Donc dictionnaire

fn _p1(_input: &str) -> usize {

    let mut lines = _input.lines();
    let mut line = lines.next().unwrap();

    let mut regles:HashMap<&str, Vec<&str>> = HashMap::new();

    // Itération sur les règles, on remplie le HashMap de toute les règles
    while line.len() != 0 {

        let numbers:Vec<&str> = line.split("|").collect();
        let gauche = numbers[0];
        let droite = numbers[1];

        match regles.get_mut(droite) {
            Some(number_list) => {
                number_list.push(gauche);
            }
            None => {
                regles.insert(droite,  Vec::new());
                regles.get_mut(droite).unwrap().push(gauche);
            }
        }
        line = lines.next().unwrap();
    }

    let mut total:usize = 0;

    while let Some(line) = lines.next() {
        let mut valide = true;
        let numbers:Vec<&str> = line.split(",").collect();
        'outer: for i in 0..numbers.len() {
            // On ignore le nombre si elle n'est pas dans les règles
            if !regles.contains_key(numbers[i]) {
                continue;
            }
            for j in i+1..numbers.len() {
                // On ignore si on compare les mêmes nombres
                if i == j {
                    continue;
                }
                if regles.get_mut(numbers[i]).unwrap().contains(&numbers[j]) {
                    valide = false;
                    break 'outer;
                }
            }
        }

        if valide {
            total += numbers[numbers.len()/2].parse::<usize>().unwrap();
        }
    }

    total
}

fn _p2(_input: &str) -> usize {

    let mut lines = _input.lines();
    let mut line = lines.next().unwrap();

    let mut regles:HashMap<&str, Vec<&str>> = HashMap::new();

    // Itération sur les règles, on remplie le HashMap de toute les règles
    while line.len() != 0 {

        let numbers:Vec<&str> = line.split("|").collect();
        let gauche = numbers[0];
        let droite = numbers[1];

        match regles.get_mut(droite) {
            Some(number_list) => {
                number_list.push(gauche);
            }
            None => {
                regles.insert(droite,  Vec::new());
                regles.get_mut(droite).unwrap().push(gauche);
            }
        }
        line = lines.next().unwrap();
    }

    let mut total:usize = 0;

    while let Some(line) = lines.next() {
        let mut valide = true;
        let mut numbers:Vec<&str> = line.split(",").collect();
        'outer: for i in 0..numbers.len() {
            // On ignore le nombre si elle n'est pas dans les règles
            if !regles.contains_key(numbers[i]) {
                continue;
            }
            for j in i+1..numbers.len() {
                // On ignore si on compare les mêmes nombres
                if i == j {
                    continue;
                }
                if regles.get(numbers[i]).unwrap().contains(&numbers[j]) {
                    valide = false;
                    break 'outer;
                }
            }
        }

        // Au début essaye de reconstruire la liste en respectant les règles
        // Mais si l'ordre peut ne pas être le même que les nombres d'origine même quand les règles sont respectés

        if !valide {
            for i in 0..numbers.len() {
                match regles.get(numbers[i]) {
                    // On ignore si le nombre n'est pas dans les règles
                    None => { continue; }
                    Some(_) => {
                        for j in i+1..numbers.len() {
                            match regles.get(numbers[j]) {
                                None => { continue; }
                                Some(_) => {
                                    if regles.get(numbers[i]).unwrap().contains(&numbers[j]) {
                                        numbers.swap(i, j);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            total += numbers[numbers.len()/2].parse::<usize>().unwrap();
        }
    }
    total
}

pub fn p1() -> usize {
    _p1(include_str!("d5.txt"))
}

pub fn p2() -> usize {
    _p2(include_str!("d5.txt"))
}

#[cfg(test)]
mod test {
    use crate::day5::*;

    #[test]
    fn test_p1() {
        assert_eq!(143, _p1(include_str!("d5_test.txt")));
    }

    #[test]
    fn test_p2() {
        assert_eq!(123, _p2(include_str!("d5_test.txt")));
    }

    #[test]
    fn real_p1() {
        assert_eq!(4281, p1());
    }

    #[test]
    fn real_p2() {
        assert_eq!(5466, p2());
    }
}