macro_rules! string
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
        assert_eq!(string!("hello"), String::from("hello"));
    }
}