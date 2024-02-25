fn get_input() -> &'static str {
    return "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
}

fn main() {
  let count = get_input()
    .lines()
    .enumerate()
    .flat_map(|(idx, line)| {
        return line.chars().nth(idx * 3 % line.len())
    })
    .filter(|&x| x == '#')
    .count();

  println!("{}", count);

  let name = String::from("wzt");
  let ref_name = &name;

  say_hi(ref_name);
  println!("main {}", &ref_name) // borrow of moved value: `name`, value borrowed here after move
}

fn say_hi(name: &String) {
  println!("Hello {}!", name)
}
