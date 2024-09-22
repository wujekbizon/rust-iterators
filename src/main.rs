// Function to print each element in the `elements` slice twice, separated by a space
fn print_repeated_elements(elements: &[String]) {
    elements
        .iter()
        .map(|el| format!("{} {}", el, el))
        .for_each(|el| println!("{}", el));
}
// Function to truncate each string in `elements` to the first character
fn truncate_strings(elements: &mut [String]) {
    elements
        .iter_mut()
        .for_each(|el: &mut String| el.truncate(1));
}
// Function to convert all strings in `elements` to uppercase and return them as a new vector
fn convert_to_uppercase(elements: &[String]) -> Vec<String> {
    elements
        .iter()
        .map(|el| el.to_uppercase())
        .collect::<Vec<String>>()
}
// Function to move elements from `vec_a` into `vec_b`, consuming `vec_a` in the process
fn transfer_elements(vec_a: Vec<String>, vec_b: &mut Vec<String>) {
    vec_a.into_iter().for_each(|el| vec_b.push(el));
}
// Function to "explode" each string into a vector of characters, where each character is represented as a String
fn split_into_chars(elements: &[String]) -> Vec<Vec<String>> {
    elements
        .iter()
        .map(|el| el.chars().map(|c| c.to_string()).collect())
        .collect::<Vec<Vec<String>>>()
}
// Function to search for a string containing the `search` term, or return the `fallback` if no match is found
fn find_or_default(elements: &[String], search: &str, fallback: &str) -> String {
    elements
        .iter()
        .find(|el| el.contains(search))
        .map_or(String::from(fallback), |el| el.to_string())
}

fn main() {
    let mut colors = vec![
        String::from("red"),
        String::from("green"),
        String::from("blue"),
    ];
    // Try to find a color that contains "red", otherwise fallback to "orange"
    let find_color = find_or_default(&colors, "red", "orange");
    println!("{}", find_color);
    // Convert all colors to uppercase and print them
    let uppercased = convert_to_uppercase(&colors);
    println!("{:#?}", uppercased);
    // Explode each color into a vector of individual characters and print them
    let inside_colors = split_into_chars(&colors);
    println!("{:#?}", inside_colors);
    // Print each color twice
    print_repeated_elements(&colors);
    // Shorten each color to its first character
    truncate_strings(&mut colors);
    // Move the shortened colors to a new vector
    let mut new_colors = vec![];
    transfer_elements(colors, &mut new_colors);
    println!("{:#?}", new_colors);
}
