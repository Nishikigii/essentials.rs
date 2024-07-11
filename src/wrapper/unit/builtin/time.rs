use crate::wrapper::unit::Unit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Second { second: usize }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Minute { minute: usize }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hour { hour: usize }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Day { day: usize }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Week { week: usize }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Year { year: usize }


impl Unit for Second
{
    type International = Self;

    const NAME: &'static str = "second";

    fn international( &self ) -> Self::International { self.clone() }
}

impl Unit for Minute
{
    type International = Second;

    const NAME: &'static str = "minute";

    fn international( &self ) -> Self::International { Second { second: self.minute * 60 } }
}

impl Unit for Hour
{
    type International = Second;

    const NAME: &'static str = "hour";    

    fn international( &self ) -> Self::International { Second { second: self.hour * 3600 } }
}

impl Unit for Day
{
    type International = Second;

    const NAME: &'static str = "day";

    fn international( &self ) -> Self::International { Second { second: self.day * 86400 } }    
}

impl Unit for Week
{
    type International = Second;

    const NAME: &'static str = "week";

    fn international( &self ) -> Self::International { Second { second: self.week * 604800 } }
}

impl Unit for Year
{
    type International = Second;

    const NAME: &'static str = "year";

    fn international( &self ) -> Self::International { Second { second: self.year * 31536000 } }    
}