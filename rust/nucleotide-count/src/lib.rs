use std::collections::HashMap;

fn is_valid(nucleotide: &char) -> bool {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => true,
        _ => false,
    }
}
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    match nucleotide {
        'A' | 'C' | 'G' | 'T' => dna.chars().try_fold(0, |acc, c| {
            if is_valid(&c) {
                Ok(acc + (c == nucleotide) as usize)
            } else {
                Err(c)
            }
        }),
        _ => Err(nucleotide),
    }
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();
    for c in dna.chars() {
        if is_valid(&c) {
            *counts.entry(c).or_insert(0) += 1
        } else {
            return Err(c);
        }
    }
    for &c in &['A', 'C', 'G', 'T'] {
        counts.entry(c).or_insert(0);
    }
    Ok(counts)
}
