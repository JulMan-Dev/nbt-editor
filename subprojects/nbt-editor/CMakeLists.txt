set_target_properties(NBTEditor PROPERTIES
        MACOSX_BUNDLE TRUE
        MACOSX_BUNDLE_INFO_STRING "NBT Editor for macOS"
        MACOSX_BUNDLE_GUI_IDENTIFIER "fr.julman.NBTEditor"
        MACOSX_BUNDLE_BUNDLE_NAME "NBT Editor"
        MACOSX_BUNDLE_COPYRIGHT "2026 © JulMan"
)
target_include_directories(NBTEditor PRIVATE
        include
)

add_subdirectory(src)
add_subdirectory(include)
add_subdirectory(resources)

target_link_libraries(NBTEditor PRIVATE
        "$<LINK_LIBRARY:FRAMEWORK,Foundation>"
        "$<LINK_LIBRARY:FRAMEWORK,AppKit>"
        NBTKit
)
target_compile_options(NBTEditor PRIVATE
        "$<$<COMPILE_LANGUAGE:OBJC>:-fobjc-arc>"
)
