use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let result = nucleotide_counts(dna);

    match result {
        Ok(map) => match map.get(&nucleotide) {
            Some(entry) => Ok(*entry),
            None => Err(nucleotide),
        },
        Err(_err) => Err(_err),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut map = HashMap::new();
    map.entry('A').or_insert(0);
    map.entry('T').or_insert(0);
    map.entry('C').or_insert(0);
    map.entry('G').or_insert(0);
    for letter in dna.chars() {
        match letter {
            'A' | 'T' | 'C' | 'G' => {
                let count = map.entry(letter).or_insert(0);
                *count += 1;
            }
            _ => return Err(letter),
        }
    }

    Ok(map)
}
