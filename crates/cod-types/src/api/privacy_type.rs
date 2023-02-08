use clap::ValueEnum;
use strum::{Display, EnumIter, EnumString};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Display, EnumString, ValueEnum, EnumIter)]
pub enum Privacy {
    #[default]
    Private,
    Public,
}
