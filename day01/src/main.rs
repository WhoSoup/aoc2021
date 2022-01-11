#[derive(Debug, Clone, Copy)]
pub struct Acc {
    prev: i64,
    count: i64,
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .split("\n")
        .filter(|&x| x.len() > 0)
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let result = input.iter().skip(1).fold(
        Acc {
            prev: input[0],
            count: 0,
        },
        |acc, &x| Acc {
            prev: x,
            count: acc.count + if x > acc.prev { 1 } else { 0 },
        },
    );

    println!("{}", result.count);
    let windows = input
        .windows(3)
        .map(|threeple| threeple[0] + threeple[1] + threeple[2])
        .collect::<Vec<_>>();
    let result2 = windows.iter().skip(1).fold(
        Acc {
            prev: windows[0],
            count: 0,
        },
        |acc, &x| Acc {
            prev: x,
            count: acc.count + if x > acc.prev { 1 } else { 0 },
        },
    );

    println!("{}", result2.count);
}
