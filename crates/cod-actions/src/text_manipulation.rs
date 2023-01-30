pub(crate) fn select_prompt_for(object: impl ToString) -> String {
    let object_name = object.to_string();
    format!("Select the desired {object_name}")
}
