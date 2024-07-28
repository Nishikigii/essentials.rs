use crate::std::Summarize;

#[macro_export]
macro_rules! str
{
    ($s:expr) => { $s.to_string() };
    () => { String::new()  };
}

impl<T,E,H> Summarize<H> for Result<T,E> where H: FnOnce(E)-> T
{
    type Protected = T;

    fn summary( self, handle: H )-> Self::Protected
    {
        return match self 
        {
            Ok(v) => v,
            Err(e) => handle(e)
        }
    }
}

impl<T,H> Summarize<H> for Option<T> where H: FnOnce()-> T
{
    type Protected = T;

    fn summary( self, handle: H )-> Self::Protected 
    {
        return match self
        {
            Some(v) => v,
            None => handle()
        }
    }
}