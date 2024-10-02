pub fn hash(input: &str) -> u32 {
    input.chars().map(|char| char as u32).fold(0u32, |acc, val|{
        ((acc + val)*17) % 256
    })
}