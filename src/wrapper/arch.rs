pub use self::Architecture::*;

#[derive(Debug, PartialEq)]
pub enum Architecture 
{
    Unknown,
    Aarch64,
    ARMv7,
    AMD64,
    X86,
    X64,

    /// Others( name )
    Others(String)
}