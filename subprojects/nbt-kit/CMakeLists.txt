add_library(rust_sources INTERFACE)

target_sources(rust_sources INTERFACE
        "Cargo.toml"
        "build-rust.sh"
)

add_subdirectory(src)
add_subdirectory(include)

get_target_property(INPUT_FILES rust_sources INTERFACE_SOURCES)
set(GENERATED_DIR "${CMAKE_CURRENT_BINARY_DIR}/generated")
set(GENERATED_LIB "${GENERATED_DIR}/libnbt_kit.a")

add_custom_command(
        OUTPUT "${GENERATED_LIB}"
        COMMAND /bin/zsh -lic "\"${CMAKE_CURRENT_SOURCE_DIR}/build-rust.sh\" \"${GENERATED_DIR}\""
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
        MACOSX_FRAMEWORK_IDENTIFIER fr.julman.NBTKit
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
