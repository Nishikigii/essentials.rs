/// common languages around the world
#[derive(Debug, PartialEq, Eq)]
pub enum Language 
{
    English(English),
    Chinese(Chinese),
    Japanese(Japanese),
    French,
    German,
    Russian,
    Portuguese,
    Arabic,
    Korean
}

/// stlyes of english
#[derive(Debug, PartialEq, Eq)]
pub enum English 
{
    British, American, Both
}

/// stlyes of chinese
#[derive(Debug, PartialEq, Eq)]
pub enum Chinese 
{
    Simplified, Traditional, Both
}

/// stlyes of japanese
#[derive(Debug, PartialEq, Eq)]
pub enum Japanese 
{
    Kanto, Kansai, Both 
}