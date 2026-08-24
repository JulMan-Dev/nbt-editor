cmake_minimum_required(VERSION 4.0.0)

project(
        NBTEditor
        VERSION 0.1.0
        DESCRIPTION "NBT Editor for macOS"
        LANGUAGES OBJC
)

add_library(NBTKit SHARED)
add_executable(NBTEditor)

add_subdirectory(subprojects)

add_custom_command(TARGET NBTEditor POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_directory
        "$<TARGET_BUNDLE_DIR:NBTKit>"
        "$<TARGET_BUNDLE_DIR:NBTEditor>/Contents/Frameworks/NBTKit.framework"
)
