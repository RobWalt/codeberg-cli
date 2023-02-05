use anyhow::Context;

pub fn multi_fuzzy_select_with_key<T, K, V>(
    items: Vec<T>,
    prompt: impl Into<String>,
    select_by: impl Fn(&T) -> K,
    return_map: impl Fn(T) -> V,
    is_selected: impl Fn(&T) -> bool,
) -> anyhow::Result<Vec<V>>
where
    K: ToString,
{
    let items_keys = items.iter().map(select_by).collect::<Vec<_>>();
    let already_selected = items.iter().map(is_selected).collect::<Vec<_>>();
    let selected_items = dialoguer::MultiFuzzySelect::new()
        .with_prompt(prompt)
        .items(&items_keys)
        .defaults(&already_selected)
        .interact()
        .map(|indices| {
            items
                .into_iter()
                .enumerate()
                .filter(|(index, _)| indices.contains(index))
                .map(|(_, item)| item)
                .map(return_map)
                .collect::<Vec<_>>()
        })?;
    Ok(selected_items)
}

pub fn fuzzy_select_with_key<T, K, V>(
    items: Vec<T>,
    prompt: impl Into<String>,
    select_by: impl Fn(&T) -> K,
    return_map: impl Fn(T) -> V,
) -> anyhow::Result<Option<V>>
where
    K: ToString,
{
    fuzzy_select_with_key_with_default(items, prompt, select_by, return_map, None)
}

pub fn fuzzy_select_with_key_with_default<T, K, V>(
    items: Vec<T>,
    prompt: impl Into<String>,
    select_by: impl Fn(&T) -> K,
    return_map: impl Fn(T) -> V,
    default_index: Option<usize>,
) -> anyhow::Result<Option<V>>
where
    K: ToString,
{
    // return `None` if we have nothing to select from
    if items.is_empty() {
        return Ok(None);
    }

    // These are shown to the user, most likely some strings
    let items_keys = items.iter().map(select_by).collect::<Vec<_>>();

    // build stadard dialogue
    let mut dialogue = dialoguer::FuzzySelect::new();
    dialogue.with_prompt(prompt).items(&items_keys);

    // optionally add default selection
    if let Some(index) = default_index {
        dialogue.default(index);
    }

    // select an item by index
    let selected_item_index = dialogue.interact().map_err(anyhow::Error::from)?;

    // transform the object with the index and only return the desired data
    let selected_item = items
        .into_iter()
        .map(return_map)
        .nth(selected_item_index)
        .context("Didn't find selected item")?;

    Ok(Some(selected_item))
}
