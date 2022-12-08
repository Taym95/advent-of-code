use std::fs;

fn main() {
    let contents = fs::read_to_string("src/input.txt").unwrap();

    let result = contents
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .fold(0, |acc, x| x.parse::<u32>().unwrap() + acc)
        })
        .max()
        .unwrap();

    println!("{:?}", result);
}
