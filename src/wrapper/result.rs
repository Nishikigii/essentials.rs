#[macro_export]
macro_rules! unwrap
{
    ($result:expr; $err:ident => $handle:expr )=> 
    {
        match $result
        {
            Ok(value) => value,
            Err($err) => $handle
        }
    };
    ($result:expr; $default:expr)=>
    {
        match $result
        {
            Ok(value) => {value},
            Err(_) => $default
        }
    };
}
