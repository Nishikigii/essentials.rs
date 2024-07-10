use crate::unwrap;

use super::Wrap;

pub trait Ver
{
    fn version( &self )-> &Version;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Version
{
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
}

impl<'a> Version
{
    /// wrap instance with a version number.
    pub fn wrap<I>( instance: &'a I, version: &str )-> Wrap<I,Version> where I: Clone
    {
        Wrap::new( instance.clone(), Self::parse(version) )
    }

    /// parse version from string
    /// # Example
    ///     let instnce = Version::parse("1.1.1")
    ///     let another = Version::parse("2.1.1")
    ///     assert_eq!(instnce < another, true);
    /// # Note
    ///     sections more than 3 will be ignored for version which be passed in.
    ///     for example it will eventually be parsed into "1.2.3" if "1.2.3.4.5" 
    ///     is passed in as version
    pub fn parse( version: &str )-> Self
    {
        let ver: Vec<&str> = version.split('.').collect();
        match ver.len()
        {
            0 => Version::new(0,0,0),
            1 => Version::new( unwrap!(ver[0].parse(); 0), 0, 0),
            2 => Version::new( unwrap!(ver[0].parse(); 0), unwrap!(ver[1].parse(); 0), 0),
            _ => Version::new( unwrap!(ver[0].parse(); 0), unwrap!(ver[1].parse(); 0), unwrap!(ver[2].parse(); 0)),
        }
    }

    pub fn new( major: usize, minor: usize, patch: usize )-> Self
    {
        Version { major, minor, patch }
    }
}
