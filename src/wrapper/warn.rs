use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unsafe<T,W,E> where W: Debug, E: Debug
{
    Safe(T),
    Warn(T, W),
    Error(E)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Err<T,W,E> where W: Debug, E: Debug
{
    Warn(T, W),
    Error(E),
}

impl<T,W,E> Unsafe<T,W,E> where W: Debug, E: Debug
{
    /// unwrap
    pub fn unwrap( self )-> T
    {
        match self 
        {
            Unsafe::Safe(value) => value,
            Unsafe::Warn(value, warn) => 
            { 
                println!("[WARN] {warn:?}"); 
                value
            },
            Unsafe::Error(error) => panic!("[ERROR] {error:?}"),
        }
    }

    pub fn expect( self, msg: &str )-> T
    {
        match self 
        {
            Unsafe::Safe(value) => value,
            Unsafe::Warn(value, warn) => 
            { 
                println!("[WARN] {warn:?}"); 
                value
            },
            Unsafe::Error(_) => panic!("{msg}"),
        }
    }


    /// convert self to `Result`
    pub fn as_result( self )-> Result<T,E>
    {
        match self 
        {
            Self::Safe(value) => Ok(value),
            Self::Error(error) => Err(error),
            Self::Warn(value, _) => Ok(value)
        }
    }
}

impl<T,W,E> Unsafe<T,W,E> where T: Clone, W: Debug, E: Clone + Debug
{

    /// convert to `Result` without self droping
    pub fn result( &self )-> Result<T,E>
    {
        match self 
        {
            Self::Safe(value) => Ok(value.clone()),
            Self::Error(error) => Err(error.clone()),
            Self::Warn(value, _) => Ok(value.clone())
        }
    }
}