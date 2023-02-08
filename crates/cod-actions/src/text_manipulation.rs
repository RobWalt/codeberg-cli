use std::fmt::Display;

pub(crate) fn select_prompt_for(object: impl Display) -> String {
    format!("Select the desired {object}")
}

pub(crate) fn edit_prompt_for(object: impl Display) -> String {
    format!("Open editor to write {object}")
}
