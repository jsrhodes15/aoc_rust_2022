fn main() {

    let mut count = 0;
    let input: Vec<i32> = include_str!("input.txt").split('\n').map(|ln| str::parse::<i32>(ln).unwrap()).collect();

    let stop = input.len() - 3;
    for (i, _) in input.iter().enumerate() {
        if i <= stop {
            if i > 0 {
                let a = &input[i-1] + &input[i] + &input[i+1];
                let b = &input[i] + &input[i + 1] + &input[i + 2];
                if b > a {
                    count += 1;
                }
            }
        }
    }

    println!("Count is: {}", count);
}
