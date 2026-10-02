fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    match polaris_tui::distance_from_input(&line) {
        Ok(d) => println!("{d}"),
        Err(e) => eprintln!("{e}"),
    }
}
