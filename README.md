# <center>essentials</center>
<center>useful structs functions for rustaceans</center>


## Usage
let our crate be a dependency in your project
> essentials crate currently unstable version, 
> interfaces may be changed in anytime.  
> think twice if you intend it into your important projects!  
```toml
[dependencies]
essentials = "*"
````
view [github](https://github.com/Nishikigii/essentials.rs) to download the source code and official builds.

## Featurs
*more wrapper types*  
```rust
/// success or reason-known failure  
pub fn bar()-> Reason<String> 
{
    if condition
    {
        return Success;
    }
    return Failure( String::from("some reason") );
}

```

*quick action marcos*  
```rust
/// shorter way to create string isntance from &str
let str: String = str!("content");

/// unwrap results without Ok() match arm 
let result: Result<String,String> = todo!();
let default: String = unwrap!(result; str!("default"));
let action: String = unwrap!(result; msg => { 
    eprintln!("{msg}");
    panic!("test fail");
});

///
````