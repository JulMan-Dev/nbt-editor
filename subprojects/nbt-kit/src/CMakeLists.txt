add_subdirectory(bin)

target_sources(rust_sources INTERFACE
        lib.rs
        binary.rs
        kind.rs
        libobjc.rs
        traits.rs
        writer.rs
)

target_sources(NBTKit PRIVATE
        NBTKit.m
)
