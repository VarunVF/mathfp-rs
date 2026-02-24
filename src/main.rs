mod token;

fn main() {
    let mut scanner = token::Scanner::new(concat!(
        "x := 2 * 5. / 1.0; y := 7\n",
        "f := n |-> if n > -.5 then (x + y) else y\n",
    ));
    let tokens = scanner.scan();
    println!("tokens = {:?}", tokens);
}
