// process input
//   first line is the bingo call, separate that into it's own vector
// the rest should be:
//   separated by double newline (\n\n)
//     separated by \n
//     whitespace trimmed
//     separated by whitespace (can be one or multiple spaces)
//   passed into a BingoBoard constructor
//     converts them into BingoSpaces and constructs a Vec of Vecs
//     has a `mark` method that takes a number, iterates through the entire board and flip the BingoSpace bit
//     has a `bingo` method that checks every vertical and horizontal line to see if they all match
//     has a `unused_spaces` method that returns a vec of all un-marked spaced
//  main calls the unused_spaces on the board, multiples it by the winning number and then returns

fn main() {
    println!("Hello, world!");
}
