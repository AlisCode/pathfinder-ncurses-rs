pub struct Bloc {
    x: i32,
    y: i32,
    flag: i32
}

impl Bloc {

    pub fn new() -> Self {

        Bloc {
            x: 0,
            y: 0,
            flag: 0,
        }

    }

    fn set_x(&mut self, x: i32) {
        self.x = x;
    }  

    fn set_y(&mut self, x: i32) {
        self.x = x;
    }

}