pub fn multi_fuzzy_select_with_key<T, K, V>(
    items: Vec<T>,
    select_by: impl Fn(&T) -> K,
    return_map: impl Fn(T) -> V,
) -> anyhow::Result<Vec<V>>
where
    K: ToString,
{
    let items_keys = items.iter().map(select_by).collect::<Vec<_>>();
    let selected_items = dialoguer::MultiFuzzySelect::new()
        .items(&items_keys)
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
