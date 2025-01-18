# Installation

##  Rust backend

### install diesel cli

see diesel documentation https://diesel.rs/guides/getting-started for different ways to install the cli.

adapt the path in the ```diesel.toml``` file


copy the ```env.template``` file to ```.env```  and fill in the credentials / links. 

run the migrations
```
diesel migration run 
```

```
cargo build  --release
```

run the backend
```
RUST_LOG=debug RUST_BACKTRACE=1   target/release/ocwd
```


## SvelteKit FrontEnd

```
npm run build
```

To use locally building a preview seems sufficient.
```
npm run preview
```

