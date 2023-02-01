pub fn wrap_text_for_table(text: &str, wrap_width: usize) -> String {
    textwrap::wrap(text, wrap_width)
        .into_iter()
        .map(|line| line.to_string())
        .reduce(|text, line| text + "\n" + line.as_str())
        .unwrap_or_default()
}
