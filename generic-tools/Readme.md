# Tools for use as functions in the Ollama Chat backend

This is a collection of tools which will be made available as tools in the 
chat API.

Run the examples:


## txt2img examples 

### On Mac OSX:

```
cargo run  --release --example wuerstchen --features=metal
```

```
cargo run  --release --example stablediffusion --features=metal
```



### On linux if CUDA is available:

```
cargo run  --release --example wuerstchen --features=cuda
```

```
cargo run  --release --example stablediffusion --features=cuda
```


## PostGres example

you probably have to replace the values in the config  
```
cargo run  --release --example postgres  
```


