pub struct Submarine {
    x: i32,
    y: i32,
    aim: i32,
}

struct Command<'a> {
    command: &'a str,
    value: i32
}

impl Submarine {
    pub fn new() -> Submarine {
        Submarine { x: 0, y: 0, aim: 0 }
    }

    pub fn move_sub(&mut self, input: &str){
        let c: Command = parse_command(input);
        match c.command {
            "forward" => self.move_x(c.value),
            "down" => self.move_y(c.value),
            "up" => self.move_y(-c.value),
            _ => ()
        }
    }

    pub fn move_sub_aim(&mut self, input: &str){
        let c: Command = parse_command(input);
        match c.command {
            "forward" => {
                self.move_x(c.value);
                self.move_y(c.value * self.aim);
            },
            "down" => self.change_aim(c.value),
            "up" => self.change_aim(-c.value),
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

fn parse_command(command: &str) -> Command {
    let comm: Vec<&str> = command.split(' ').collect();
    Command {
        command: comm[0],
        value: comm[1].parse().expect("Not a number")
    }
}
