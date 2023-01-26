use strum::{Display, EnumString, EnumVariantNames};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Display, EnumVariantNames, EnumString)]
pub enum Privacy {
    #[default]
    Private,
    Public,
}
