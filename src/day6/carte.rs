pub struct Carte {
    pub tab: Vec<char>,
    pub width: usize,
    pub height: usize,
}

impl Carte {
    pub fn new(_input:&str) -> Carte {

        let h = _input.lines().count();
        let w = _input.lines().next().unwrap().chars().count();

        let mut t:Vec<char> = vec!['.'; w * h];

        for (y, line) in _input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                t[x + y * w] = c;
            }
        }
        Carte { tab: t, width: w, height: h }
    }

    pub fn find_guard_position(&self) -> (usize, usize) {
        // Find the guards position
        let i = self.tab.iter().position(|&i| i == '^').unwrap();
        (i % self.width, i / self.width)
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        self.tab[x + y * self.width]
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) {
        self.tab[x + y * self.width] = c;
    }

    pub fn copy(&mut self, carte: &Carte) {
        for (i, &c) in carte.tab.iter().enumerate() {
            self.tab[i] = c;
        }
    }

    pub fn clone(&self) -> Carte {
        Carte { tab: self.tab.clone(), width: self.width, height: self.height }
    }

}