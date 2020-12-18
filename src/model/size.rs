use enum_iterator::IntoEnumIterator;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, IntoEnumIterator, Debug)]
pub enum Size {
    Small,
    Large,
}
