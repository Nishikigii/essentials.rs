pub mod lang;
pub mod arch;
pub mod reason;
pub mod result;
pub mod version;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Wrap<I,U>
{
    pub instance: I,
    pub unit: U
}

impl<I,U> Wrap<I,U>
{
    pub fn new(instance: I, unit: U) -> Self
    {
        Wrap { instance, unit }
    }
}