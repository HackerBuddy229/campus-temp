# Fetch script for temp.campus.ltu.se
> A fetch script made to get the main temprature reading from [temp.campus.ltu.se](https://temp.campus.ltu.se) and accopanying module for polybar

### Build
To build the script run the following commands in the repo root
´´´bash
$ cargo build
´´´

### Install
To install run the following
```bash
$ sudo cp target/debug/temp /usr/bin/.
```
Then copy paste the contents of `temp.module` into your polybar config

