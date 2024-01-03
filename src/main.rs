
fn rule(x: &bool, y: &bool,z: &bool) -> bool {
    return (x | y) ^ (z | y);
}

struct Game {
    v: Vec<bool>,
    rule: fn(&bool, &bool, &bool) -> bool,
}

impl Game {
    fn new(n: u32, rule: fn(&bool, &bool, &bool) -> bool) -> Game {
        let mut v: Vec<bool> = Vec::with_capacity(n as usize);
        for _ in 0..n {
            v.push(rand::random());
        }
        Game { v, rule }
    }

    fn next(&mut self) {
        let mut v_temp: Vec<bool> = Vec::with_capacity(self.v.len());
        for i in 0..self.v.len() {
            if i == 0 {
                // first element
                let new_value = (self.rule)(&self.v[self.v.len()-1], &self.v[i], &self.v[i+1]);
                v_temp.push(new_value);
            } else if i == self.v.len() - 1 {
                // last element
                v_temp.push((self.rule)(&self.v[i-1], &self.v[i], &self.v[0]));
            } else {
                v_temp.push((self.rule)(&self.v[i-1], &self.v[i], &self.v[i+1]));
            }
        }
        // println!("v_temp: {:?}", v_temp);
        self.v = v_temp;
    }


    fn biggest_triangle(&self) -> u32{
        let mut count: u32 = 0;
        let mut max_count: u32 = 0;
        for i in 0..self.v.len() {
            
            if self.v[i]==true {
                if max_count <= count {
                    max_count = count;
                }
                count = 0;
                continue;
            } else {
                count += 1;
            }
        }
        return max_count;
    }

    fn display(&self) {
        let display_string: String = self.v
            .iter()
            .map(|&cell| if cell { '\u{2588}' } else { ' ' })
            .collect();
        println!("{}", display_string);
    }
}

fn main() {
    
    // create a vector of i32 and a given length of 5
    let mut g: Game = Game::new(100, rule);

    let n:u32 = 250; 

    let mut largest_triangle: u32 = 0;

    for _ in 0..n {
        g.next();
        // print the vector
        g.display();
        let current = g.biggest_triangle();
        if largest_triangle <= current {
            largest_triangle = current;
        }
    }

    // print the largest triangle
    println!("Largest triangle: {}", largest_triangle);

}
