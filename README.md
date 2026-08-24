# NBT Editor

`NBT Editor` aims to be a NBT editor obviously but familiar for Windows users whose used
[NBTExplorer](https://github.com/jaquadro/NBTExplorer).

It's core, `NBTKit`, is written in Rust and Objective-C. While the GUI interface, the editor, is written in Objective-C
linking with `NBTKit`.

## Build instructions

NBT Editor is meant to be built only on macOS, it may not be compatible with cross-platform compilation setup (Linux
with GNU Step). It requires CMake, GNU make, Rust toolchain (nightly **and** stable) and C compiler (Clang by default on 
macOS).

```bash
$ mkdir build
$ cd build
$ cmake ..
$ make
```

Then you should find `NBTKit.framework` and `NBTEditor.app`.
