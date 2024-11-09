trait LettersCount {
    fn letters_count(&self, ch: char) -> usize;
}

impl LettersCount for str {
    fn letters_count(&self, ch: char) -> usize {
        self.chars().filter(|c| *c == ch).count()
    }
}

fn main() {
    println!("{} counts: {} from {}", 'd', "__dead_beef__".letters_count('d'),
             "__dead_beef__");
}