#[derive(Debug)]
pub struct Manager {
    temp_counter: i32,
    quadruples: Vec<Quadruple>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            temp_counter: 0,
            quadruples: vec![],
        }
    }

    pub fn emit(&mut self, quadruple: Quadruple) {
        self.quadruples.push(quadruple);
        self.temp_counter += 1;
    }
}

#[derive(Debug)]
pub struct Quadruple(String, String, String, String);
