fn print_elements(elements: &[String]) {
    // for element in elements {
    //     println!("{}", element);
    // }
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}

fn shorten_strings(elements: &mut [String]) {
    elements.iter_mut().for_each(|el| {
        if let Some(first_char) = el.chars().next() {
            *el = first_char.to_string();
        }
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
