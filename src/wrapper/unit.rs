pub mod builtin;

pub trait Count
{
    type Unit: Unit;
}


pub trait Unit
{
    type International: Unit;

    const NAME: &'static str;

    fn international( &self ) -> Self::International;
}