use anyhow::Context;

pub fn multi_fuzzy_select_with_key<T, K, V>(
    items: Vec<T>,
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
    select_by: impl Fn(&T) -> K,
    return_map: impl Fn(T) -> V,
) -> anyhow::Result<Option<V>>
where
    K: ToString,
{
    if items.is_empty() {
        return Ok(None);
    }

    let items_keys = items.iter().map(select_by).collect::<Vec<_>>();
    let selected_item = dialoguer::FuzzySelect::new()
        .items(&items_keys)
        .interact()
        .map_err(anyhow::Error::from)
        .and_then(|index| {
            items
                .into_iter()
                .map(return_map)
                .nth(index)
                .context("Didn't find selected item")
        })?;
    Ok(Some(selected_item))
}
