# rust-dylibbinding

The aim of this repository is to confirm codes from [Linking Rust application with a dynamic library not in the runtime linker search path](https://stackoverflow.com/questions/40602708/linking-rust-application-with-a-dynamic-library-not-in-the-runtime-linker-search).

Rust Version: 1.36.0 

## Build on Linux 

Ubuntu 18.04

### Complie C library to shared object 

```shell script 
$ cd library 
$ gcc -g -shared awesome_math.c -o libawesome_math.so 
``` 

### Build rust code 

```shell script
$ cd dylibbinding
$ cargo rustc -- -C link-args="-Wl,-rpath,../library/"
```

Examine shared object dependencies.
```shell script
$ ldd target/debug/dylib | grep libawesome
libawesome_math.so => ../library/libawesome_math.so
````

Run
```shell script
$ ./target/debug/dylibbinding  # This would be OK. 
Adding: 3
````


## Build on mac OSX

macOS Catalina (10.15.2)

### Complie C library to shared object

```shell script
$ cd library
$ gcc -g -dynamiclib awesome_math.c -o libawesome_math.dylib
```

### Build rust code

```shell script
$ cd dylibbinding
$ cargo rustc -- -C link-args="-Wl,-rpath,../library/"
```

Examine dylib dependencies.
```shell script
$ otool -L target/debug/dylibbinding
target/debug/dylibbinding:
        libawesome_math.dylib (compatibility version 0.0.0, current version 0.0.0)
````

Run
```shell script
$ cp ../library/libawesome_math.dylib .  # Copy dylib here
$ ./target/debug/dylibbinding  # This would be OK. 
Adding: 3
````
