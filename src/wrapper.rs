use std::fmt::{Debug, Display};

pub mod lang;

#[cfg(test)]
mod tests
{

    use super::Reason;

    #[test]
    fn create_reason()
    {
        let instance = Reason::Failure("some reason");
        let expect = Reason::Failure("some reason");
        assert_eq!(instance, expect);
    }

    #[test]
    fn compare_reason()
    {
        let instance = Reason::Failure("some reason");
        let expect_fail = Reason::Failure("other reason");
        let expect_sucs: Reason<&str> = Reason::Success;
        assert!( !instance.eq(&expect_fail)  && !instance.eq(&expect_sucs))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reason<T>
{
    Success, 
    Failure(T)
}



impl<T: PartialEq> Reason<T> 
{
    pub fn is_failure( &self )-> bool
    {
        if *self == Self::Success { false } else { true }
    }

    pub fn is_success( &self )-> bool
    {
        if *self == Self::Success { true } else { false }
    }
}


impl<T> Display for Reason<T> where T: Debug
{
    fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result 
    {
        match self 
        {
            Self::Success => write!(f, "Success"),
            Self::Failure(reason) => write!(f, "Failure[{reason:?}]")
        }
    }
}