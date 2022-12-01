macro_rules! path {
    ($day:expr, $type:expr) => {
        std::path::Path::new(concat!(
            "tests/input/day",
            $day,
            "/",
            $type,
            ".txt"
        ))
    };
}

macro_rules! example {
    ($day:expr) => {
        path!($day, "example")
    };
}

macro_rules! input {
    ($day:expr) => {
        path!($day, "input")
    };
}

#[test]
fn day_01() {
    let example = aoc2022::Day01::new(example!("01")).unwrap();
    let input = aoc2022::Day01::new(input!("01")).unwrap();

    assert_eq!(24000, example.part1().unwrap());
    assert_eq!(70374, input.part1().unwrap());
    assert_eq!(45000, example.part2().unwrap());
    assert_eq!(204610, input.part2().unwrap());
}
