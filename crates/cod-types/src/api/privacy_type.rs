use clap::ValueEnum;
use strum::{Display, EnumString, EnumVariantNames};

#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Display, EnumVariantNames, EnumString, ValueEnum,
)]
pub enum Privacy {
    #[default]
    Private,
    Public,
}
