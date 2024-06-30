macro_rules! str
{
    ($s:expr)=> 
    {
        $s.to_string()
    };
}

#[cfg(test)]
mod tests
{

    #[test]
    fn create_string()
    {
        assert_eq!(str!("hello"), String::from("hello"));
    }
}