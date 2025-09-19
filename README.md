# gvrtex-cli

`gvrtex-cli` is a command-line tool that interfaces with the [GVR texture format](https://code.google.com/archive/p/puyotools/wikis/GVRTexture.wiki). It allows for encoding standardized image files (like .png or .jpg files) into GVR texture files, and decoding GVR texture files into standardized image files as well. The internal encoding/decoding is handled by the library it interfaces with, [gvrtex](https://github.com/Exortile/gvrtex).

## Building

You can compile this like any other Rust project:
```
cargo build
```

Or run:
```
cargo run
```

## Usage

To decode a GVR texture file:
```
gvrtex decode input_texture.gvr output.png
```

To encode a GVR texture file with the default settings:
```
gvrtex encode input_image.png output.gvr
```

For more information, check the help information on each command:
```
gvrtex encode --help
gvrtex decode --help
```
