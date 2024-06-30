use std::fmt::{Debug, Display};

pub mod lang;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reason<T>
{
    Success, 
    Failure(T)
}

impl<T> Reason<T> 
{
    pub fn on_success( &self, action: impl Fn() )-> &Self
    {
        if let Self::Success = self
        {
            action();
        }
        return self;
    }

    pub fn on_failure( &self, action: impl Fn(&T) )-> &Self
    {
        if let Self::Failure(reason) = self
        {
            action(reason);
        }
        return self;
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

#[cfg(test)]
mod reason_tests
{

    use super::Reason;

    #[test]
    fn assert_reason()
    {
        let reason = Reason::Failure("some reason");
        reason.on_success(|| println!("success"));
        reason.on_failure(|reason| println!("failure: {reason:?}"));
    }

}


/// path to a data source
pub trait Path 
{

}

/// data source
pub enum Source 
{
    /// Local( path )
    Local(String),

    /// Network( url )
    Network()
}