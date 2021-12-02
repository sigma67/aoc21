pub struct Submarine {
    x: i32,
    y: i32,
    aim: i32,
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine { x: 0, y: 0, aim: 0 }
    }

    pub fn move_sub(&mut self, command: &str){
        let comm: Vec<&str> = command.split(' ').collect();
        let distance: i32 = comm[1].parse().expect("not a number");
        match comm[0] {
            "forward" => self.move_x(distance),
            "down" => self.move_y(distance),
            "up" => self.move_y(-distance),
            _ => ()
        }
    }

    pub fn move_sub_aim(&mut self, command: &str){
        let comm: Vec<&str> = command.split(' ').collect();
        let value: i32 = comm[1].parse().expect("not a number");
        match comm[0] {
            "forward" => {
                self.move_x(value);
                self.move_y(value * self.aim);
            },
            "down" => self.change_aim(value),
            "up" => self.change_aim(-value),
            _ => ()
        }
    }

    fn move_x(&mut self, distance: i32){
        self.x += distance;
    }

    fn move_y(&mut self, distance: i32){
        self.y += distance;
    }

    fn change_aim(&mut self, aim: i32){
        self.aim += aim;
    }

    pub fn get_position(&self) -> i32 {
        self.x
    }

    pub fn get_depth(&self) -> i32 {
        self.y
    }
}
