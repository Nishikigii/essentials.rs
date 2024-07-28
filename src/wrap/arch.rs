pub use self::Architecture::*;

#[derive(Debug, Clone, PartialEq)]
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