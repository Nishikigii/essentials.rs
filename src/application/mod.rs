#[cfg(test)]
mod tests
{
    use super::*;

    #[derive(Debug)]
    struct App;

    impl<'a> Application<'a> for App
    {
        const IDENTIFIER: &'a str = "com.example.application";
    }

    impl<'a> Binary<'a> for App 
    {
        type Settings = &'a str;
        type Error = &'a str;
    
        fn start( settings: Self::Settings )-> Result<Box<Self>, Self::Error> 
        {
            println!("app start with settings: {settings}");
            Ok( Box::new( App {} ) )
        }
    }

    #[test]
    fn implement()
    {
        println!("{:?}", App::start("-test").unwrap())
    }

}

/// application
pub trait Application<'a>
{
    /// application identifier
    const IDENTIFIER: &'a str;
}

/// executable application
pub trait Binary<'a>: Application<'a> 
{
    type Settings;
    type Error: Sized;

    /// start the application with arguments
    fn start( settings: Self::Settings )-> Result<Box<Self>, Self::Error>;

}