fn print_elements(elements: &[String]) {
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| match el.chars().next() {
        Some(first_char) => *el = first_char.to_string(),
        None => println!("no character found"),
    });
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];

    shorten_strings(&mut colors);
    print_elements(&colors);
}
