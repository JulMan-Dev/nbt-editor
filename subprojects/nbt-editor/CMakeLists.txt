set_target_properties(NBTEditor PROPERTIES
        MACOSX_BUNDLE TRUE
)
target_include_directories(NBTEditor PRIVATE
        include
)

add_subdirectory(src)
add_subdirectory(include)

target_link_libraries(NBTEditor PRIVATE
        "$<LINK_LIBRARY:FRAMEWORK,Foundation>"
        "$<LINK_LIBRARY:FRAMEWORK,AppKit>"
        NBTKit
)
target_compile_options(NBTEditor PRIVATE
        "$<$<COMPILE_LANGUAGE:OBJC>:-fobjc-arc>"
)
