target_sources(rust_sources INTERFACE
        lib.rs
        binary.rs
        kind.rs
        libc.rs
        libobjc.rs
        parser.rs
)

target_sources(NBTKit PRIVATE
        NBTKit.m
)
