#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unsafe<T,W,E>
{
    Safe(T),
    Warn(T, W),
    Error(E)
}

impl<T,W,E> Unsafe<T,W,E>
{
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

impl<T,W,E> Unsafe<T,W,E> where T: Clone, E: Clone
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