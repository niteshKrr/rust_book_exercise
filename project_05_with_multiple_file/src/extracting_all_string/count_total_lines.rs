pub fn count_lines(items: &str) -> usize {
    let mut line_count = 0;
    for _ in items.lines() {
        line_count += 1;
    }

    line_count
}