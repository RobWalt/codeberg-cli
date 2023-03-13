use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SearchResults<T> {
    pub data: T,
    pub ok: bool,
}
