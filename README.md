# Example Devzat plugin made with Rust

To run the example you must get a plugin token which can be obtained with the following command:

`grant [user] [description]`

Afterwards you need export the following enviornment variables:

```
PLUGIN_HOST=https://devzat.hackclub.com:5556 # gRCP server
PLUGIN_TOKEN=dvz.token@hello.world1234 # The plugin token
```

## Running

Release build:

```
cargo run --release
```

Development build:

```
cargo run
```