pub fn substring(s: &str, begin: usize, end: usize) -> &str {
    let l = s.chars().count();
    if end < begin {
        return "";
    }

    s.char_indices()
        .nth(begin)
        .and_then(|(start_pos, _)| {
            if end >= l {
                return Some(&s[start_pos..]);
            }

            s[start_pos..]
                .char_indices()
                .nth(end - begin)
                .map(|(end_pos, _)| &s[start_pos..start_pos + end_pos])
        })
        .unwrap_or("")
    // self.source.chars().skip(start).take(end - start).collect()
}
