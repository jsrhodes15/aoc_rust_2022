
struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

impl Position {
    fn new() -> Self {
        Position {
            x: 0,
            y: 0,
            aim: 0,
        }
    }

    fn up(&mut self, num: i32) {
        self.aim -= num;
    }

    fn down(&mut self, num: i32) {
        self.aim += num
    }

    fn forward(&mut self, num: i32) {
        self.x += num;
        self.y += self.aim * num
    }
}

fn main() {
    let moves: Vec<(&str, i32)> = include_str!("input.txt")
        .split('\n')
        .map(|l| {
           let mut stuff = l.split_whitespace();
            return (stuff.next().unwrap(), stuff.next().unwrap().parse::<i32>().unwrap());
        })
        .collect();

    let mut position = Position::new();

    for (direction, num) in moves {
        match direction {
            "forward" => position.forward(num),
            "up" => position.up(num),
            "down" => position.down(num),
            _ => (),
        }
    }


    println!("{}", position.x * position.y)
}
