/// application
pub trait Application<'a>
{
    /// application identifier
    fn identifier( &self )-> String;
}

/// executable application
pub trait Binary<'a>: Application<'a> 
{
    type Settings;
    type Error: Sized;

    /// start the application with arguments
    fn start( settings: Self::Settings )-> Result<Self,String> where Self: Sized;
}