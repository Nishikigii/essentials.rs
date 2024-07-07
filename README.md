# <center>essentials</center>
<center>useful structs functions for rustaceans</center>


## Usage
let our crate be a dependency in your project
> essentials crate currently unstable version, 
> interfaces may be changed in anytime.  
> think twice if you intend it into your important projects!  
```toml
[dependencies]
essentials = "0.2.2"
````
view [github](https://github.com/Nishikigii/essentials.rs) to download the source code and official builds.

## Features
enable all features by specify `features` field as `full`.  
```toml
essentials = { version = "*", features = ["full"] }
```

### wrapper
> main feature for essentials, it provides many useful structs, functions and traits.     
> it enabled by default but could be applied it with following line.
```toml
essentials = { version = "*", features = ["wrapper"] }
```

### marco
### application