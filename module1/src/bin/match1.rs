
// Fill the blanks
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::North;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South  => println!("South or North"),
        _ => println!("West"),
    };
}

