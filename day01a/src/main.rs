fn main() {
    let moves = include_str!("input.txt").lines();

    let moves_parts = moves.map(|l| l.split_once(" ").unwrap());

    let (x, y) = moves_parts.fold((0, 0), |(x, y), (dir, num)| {
        match (dir, num.parse::<i32>().unwrap()) {
            ("forward", num) => (x + num, y),
            ("up", num) => (x, y - num),
            ("down", num) => (x, y + num),
            _ => unreachable!()
        }
    });

    println!("{}", x * y);

}
