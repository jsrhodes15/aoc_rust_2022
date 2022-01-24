fn main() {
    let moves = include_str!("input.txt").lines();

    let moves_parts = moves.map(|l| l.split_once(" ").unwrap());

    let (x, y, _a) = moves_parts.fold((0, 0, 0), |(x, y, aim), (dir, num)| {
        match (dir, num.parse::<i32>().unwrap()) {
            ("forward", num) => (x + num, y + aim * num, aim),
            ("up", num) => (x, y, aim - num),
            ("down", num) => (x, y, aim + num),
            _ => unreachable!()
        }
    });

    println!("{}", x * y);

}