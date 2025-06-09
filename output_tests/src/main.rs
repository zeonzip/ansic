use ansic_macros::ansi;

const ERROR: &str = ansi!(br.red bold underline italic);
const RESET: &str = ansi!(reset);

fn main() {
    println!("{:?}", ERROR);
    println!("{ERROR}[ERROR]:{RESET} Hello, world!");
}
