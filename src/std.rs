pub trait Search
{
    type Result;
    type Decoy;

    fn search( &self, item: &Self::Decoy )-> Self::Result;
}


pub trait Summarize<H>
{
    type Protected;

    fn summary( self, handle: H )-> Self::Protected;
}