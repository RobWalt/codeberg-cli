use std::fmt::Display;

pub fn multi_fuzzy_select_with_key<T>(
    items: Vec<T>,
    prompt: impl AsRef<str>,
    is_selected: impl Fn(&T) -> bool,
) -> anyhow::Result<Vec<T>>
where
    T: Display,
{
    let already_selected = items
        .iter()
        .enumerate()
        .filter(|(_, elem)| is_selected(elem))
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();
    let selected_items = inquire::MultiSelect::new(prompt.as_ref(), items)
        .with_default(&already_selected)
        .prompt()?;
    Ok(selected_items)
}

pub fn fuzzy_select_with_key<T>(items: Vec<T>, prompt: impl AsRef<str>) -> anyhow::Result<Option<T>>
where
    T: Display,
{
    fuzzy_select_with_key_with_default(items, prompt, None)
}

pub fn fuzzy_select_with_key_with_default<T>(
    items: Vec<T>,
    prompt: impl AsRef<str>,
    default_index: Option<usize>,
) -> anyhow::Result<Option<T>>
where
    T: Display,
{
    // return `None` if we have nothing to select from
    if items.is_empty() {
        return Ok(None);
    }

    // build stadard dialogue
    let mut dialogue = inquire::Select::new(prompt.as_ref(), items);

    // optionally add default selection
    if let Some(index) = default_index {
        dialogue = dialogue.with_starting_cursor(index);
    }

    // select an item by key
    let selected_item = dialogue.prompt().map_err(anyhow::Error::from)?;

    Ok(Some(selected_item))
}

pub fn confirm_with_prompt(prompt: &str) -> anyhow::Result<bool> {
    inquire::Confirm::new(prompt)
        .with_help_message("(y/n)?")
        .prompt()
        .map_err(anyhow::Error::from)
}
