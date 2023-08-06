use std::fs::read_to_string;

fn extract_names<'a>(s: &'a String) -> Vec<&'a str> {
    s.split(',')
        .map(|name| {
            name.strip_prefix('"')
                .unwrap_or(name)
                .strip_suffix('"')
                .unwrap_or(name)
        })
        .collect()
}

fn calculate_alphabetical_value(name: &str) -> u32 {
    name.as_bytes().iter().map(|&c| (c - b'A' + 1) as u32).sum()
}

fn main() {
    let file_content = read_to_string("input/022_names.txt").expect("failed to load file");
    let mut names = extract_names(&file_content);
    names.sort();

    let result = names
        .iter()
        .enumerate()
        .map(|(i, &name)| (i + 1) as u32 * calculate_alphabetical_value(name))
        .sum::<u32>();

    println!("{result}");
}
