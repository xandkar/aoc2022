macro_rules! path {
    ($day:expr, $type:expr) => {
        std::path::Path::new(concat!(
            "tests/input/day",
            stringify!($day),
            "/",
            $type,
            ".txt"
        ))
    };
}

macro_rules! path_to_example {
    ($day:ident) => {
        path!($day, "example")
    };
}

macro_rules! path_to_input {
    ($day:ident) => {
        path!($day, "input")
    };
}

macro_rules! day {
  ($n:ident, $p1_ex:expr, $p1_in:expr, $p2_ex:expr, $p2_in:expr) => {
       paste::paste! {
            #[test]
            fn [<day $n _part_1 _example>]() {
                let data = aoc2022::[<day $n>]::Data::load(path_to_example!($n)).unwrap();
                assert_eq!($p1_ex, data.part1().unwrap());
            }

            #[test]
            fn [<day $n _part_1 _input>]() {
                let data = aoc2022::[<day $n>]::Data::load(path_to_input!($n)).unwrap();
                assert_eq!($p1_in, data.part1().unwrap());
            }

            #[test]
            fn [<day $n _part_2 _example>]() {
                let data = aoc2022::[<day $n>]::Data::load(path_to_example!($n)).unwrap();
                assert_eq!($p2_ex, data.part2().unwrap());
            }

            #[test]
            fn [<day $n _part_2 _input>]() {
                let data = aoc2022::[<day $n>]::Data::load(path_to_input!($n)).unwrap();
                assert_eq!($p2_in, data.part2().unwrap());
            }
       }
  };
}

day!(_01, 24000, 70374, 45000, 204610);
day!(_02, 15, 8890, 12, 10238);
