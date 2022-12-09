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
                assert_eq!($p1_ex, data.solve1().unwrap());
            }

            #[test]
            fn [<day $n _part_1 _input>]() {
                let data = aoc2022::[<day $n>]::Data::load(path_to_input!($n)).unwrap();
                assert_eq!($p1_in, data.solve1().unwrap());
            }

            #[test]
            fn [<day $n _part_2 _example>]() {
                let data = aoc2022::[<day $n>]::Data::load(path_to_example!($n)).unwrap();
                assert_eq!($p2_ex, data.solve2().unwrap());
            }

            #[test]
            fn [<day $n _part_2 _input>]() {
                let data = aoc2022::[<day $n>]::Data::load(path_to_input!($n)).unwrap();
                assert_eq!($p2_in, data.solve2().unwrap());
            }
       }
  };
}

day!(_01, 24000, 70374, 45000, 204610);
day!(_02, 15, 8890, 12, 10238);
day!(_03, 157, 8105, 70, 2363);
day!(_04, 2, 444, 4, 801);
day!(_05, "CMZ", "GRTSWNJHH", "MCD", "QLFQDBBHM");
day!(_06, 7, 1804, 19, 2508);
day!(_07, 95437, 1391690, Some(24933642), Some(5469168));
day!(_08, 21, 1713, 8, 268464);
