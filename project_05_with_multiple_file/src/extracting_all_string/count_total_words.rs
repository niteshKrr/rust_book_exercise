pub fn count_words(items: &str) -> usize {
    items.split_whitespace().count()
}