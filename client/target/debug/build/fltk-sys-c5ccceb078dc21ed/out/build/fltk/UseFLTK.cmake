#
# UseFLTK.CMake - FLTK CMake environment configuration file for external projects.
#
# This file is deprecated and will be removed in FLTK 1.4.0 or later
#
# automatically generated - do not edit
#

include_directories("/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/build/fltk;/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/fltk")

message(AUTHOR_WARNING
" * Warning: UseFLTK.cmake is deprecated and will be removed in FLTK 1.4.0
 * or later. Please use 'include_directories(\${FLTK_INCLUDE_DIRS})' or
 * 'target_include_directories(<target> PUBLIC|PRIVATE \${FLTK_INCLUDE_DIRS})'
 * instead of 'include(\${FLTK_USE_FILE})'.")
