add_library(rust_sources INTERFACE)

target_sources(rust_sources INTERFACE
        "Cargo.toml"
)

add_subdirectory(src)
add_subdirectory(include)

get_target_property(INPUT_FILES rust_sources INTERFACE_SOURCES)
set(GENERATED_DIR "${CMAKE_CURRENT_BINARY_DIR}/generated")
set(GENERATED_LIB "${GENERATED_DIR}/libnbt_kit.a")

find_program(RUST_RUSTUP rustup)

if (NOT RUST_RUSTUP)
        message(FATAL_ERROR "rustup is required. Install Rust.")
endif ()

execute_process(
        COMMAND "${RUST_RUSTUP}" which --toolchain stable rustc
        OUTPUT_VARIABLE RUST_RUSTC
        OUTPUT_STRIP_TRAILING_WHITESPACE
)

if ("${CMAKE_BUILD_TYPE}" STREQUAL "Release")
        set(CARGO_RELEASE_FLAG --release)
endif ()

add_custom_command(
        COMMENT "Building Rust crate nbt_kit"
        OUTPUT "${GENERATED_LIB}"
        COMMAND cmake -E env "RUSTC=${RUST_RUSTC}"
                "${RUST_RUSTUP}" run nightly cargo -Z unstable-options build
                        -F objc ${CARGO_RELEASE_FLAG} --artifact-dir "${GENERATED_DIR}"
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
        DEPENDS ${INPUT_FILES}
        VERBATIM
)

add_custom_target(nbt_kit_generated
        DEPENDS "${GENERATED_LIB}"
)

add_library(nbt_kit STATIC IMPORTED GLOBAL)
set_target_properties(nbt_kit PROPERTIES
        IMPORTED_LOCATION "${GENERATED_LIB}"
)

add_dependencies(nbt_kit nbt_kit_generated)

set_target_properties(NBTKit PROPERTIES
        FRAMEWORK TRUE
        MACOSX_FRAMEWORK_BUNDLE_NAME "NBTKit"
        MACOSX_FRAMEWORK_BUNDLE_VERSION "0.0.1"
        MACOSX_FRAMEWORK_IDENTIFIER "fr.julman.NBTKit"
        MACOSX_FRAMEWORK_SHORT_VERSION_STRING "0.0.1"
        PUBLIC_HEADER "${CMAKE_CURRENT_SOURCE_DIR}/include/NBTKit/NBTKit.h"
)
target_include_directories(NBTKit PUBLIC include)
target_link_libraries(NBTKit PRIVATE
        nbt_kit
        "$<LINK_LIBRARY:FRAMEWORK,Foundation>"
)
target_link_options(NBTKit PRIVATE
        "-Wl,-force_load,${GENERATED_LIB}"
)
