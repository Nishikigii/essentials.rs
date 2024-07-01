#[macro_export]
macro_rules! str
{
    ($s:expr) => { $s.to_string() };
    () => { String::new()  };
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