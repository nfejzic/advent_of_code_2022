use std::collections::BTreeMap;

use anyhow::Result;

fn main() -> Result<()> {
    let input = utils::read_file()?;

    if let Some(index) = find_message_with_len(&input, 4) {
        println!("{index}");
    }

    if let Some(index) = find_message_with_len(&input, 14) {
        println!("{index}");
    }

    Ok(())
}

fn find_message_with_len(input: &str, size: usize) -> Option<usize> {
    let input: Vec<char> = input.chars().collect();

    let mut map: BTreeMap<char, usize> = BTreeMap::new();

    for (index, chars) in input.windows(size).enumerate() {
        for letter in chars {
            *map.entry(*letter).or_default() += 1;
        }

        if map.values().all(|count| *count == 1) {
            return Some(index + size);
        } else {
            map.clear();
        }
    }

    None
}
