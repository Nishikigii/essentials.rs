# <center>essentials</center>
<center>useful structs functions for rustaceans</center>


## Usage
let our crate make a important role in your project!  
> essentials currently unstable version,  
> everything may refactoring in anytime.  
> think twice if you intend it into your important projects!  
```toml
[dependencies]
essentials = "0.3.0"
````
view [github](https://github.com/Nishikigii/essentials.rs) to download the source code and official builds.

## Featurs
- shotcuts for common functions
```rust
let result: Result<&str,&str> = Err("error");
let option: Option<&str> = None;

let result = result.summary( |_| "default" );
let option = option.summary( | | "default" );

println!("result: {result}");
println!("option: {option}");
```
- useful structs which missing in std lib
```rust
/// actions may be failure
pub fn execute()-> Reason<String>
{
    return Success;
}

let result = execute();
result.on_success(| | println!("success"));
result.on_failure(|e| println!("fail for {e}"));
```

## Document
*none document published yet but comming soon*