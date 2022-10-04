# Install script for directory: /home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "0")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/lib/libcfltk.a")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/lib" TYPE STATIC_LIBRARY FILES "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/build/libcfltk.a")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_box.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_browser.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_button.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_dialog.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_draw.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_enums.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_group.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_image.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_input.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_lock.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_macros.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_menu.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_misc.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_printer.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_surface.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_table.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_text.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_tree.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_utils.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_valuator.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_widget.h;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_widget.hpp;/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk/cfl_window.h")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/include/cfltk" TYPE FILE FILES
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_box.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_browser.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_button.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_dialog.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_draw.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_enums.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_group.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_image.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_input.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_lock.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_macros.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_menu.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_misc.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_printer.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_surface.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_table.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_text.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_tree.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_utils.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_valuator.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_widget.h"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_widget.hpp"
    "/home/morpheus/.cargo/registry/src/github.com-1ecc6299db9ec823/fltk-sys-1.3.14/cfltk/include/cfl_window.h"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/share/cmake/cfltk/cfltkConfig.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/share/cmake/cfltk/cfltkConfig.cmake"
         "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/build/CMakeFiles/Export/79534e5a0e4c1a75968378ddc41d520e/cfltkConfig.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/share/cmake/cfltk/cfltkConfig-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/share/cmake/cfltk/cfltkConfig.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/share/cmake/cfltk/cfltkConfig.cmake")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/share/cmake/cfltk" TYPE FILE FILES "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/build/CMakeFiles/Export/79534e5a0e4c1a75968378ddc41d520e/cfltkConfig.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
     "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/share/cmake/cfltk/cfltkConfig-release.cmake")
    if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
      message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
      message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
    endif()
    file(INSTALL DESTINATION "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/share/cmake/cfltk" TYPE FILE FILES "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/build/CMakeFiles/Export/79534e5a0e4c1a75968378ddc41d520e/cfltkConfig-release.cmake")
  endif()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  list(APPEND CMAKE_ABSOLUTE_DESTINATION_FILES
   "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/share/cmake/cfltk/cfltkConfigVersion.cmake")
  if(CMAKE_WARN_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(WARNING "ABSOLUTE path INSTALL DESTINATION : ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  if(CMAKE_ERROR_ON_ABSOLUTE_INSTALL_DESTINATION)
    message(FATAL_ERROR "ABSOLUTE path INSTALL DESTINATION forbidden (by caller): ${CMAKE_ABSOLUTE_DESTINATION_FILES}")
  endif()
  file(INSTALL DESTINATION "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/share/cmake/cfltk" TYPE FILE FILES "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/build/cfltkConfigVersion.cmake")
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/build/fltk/cmake_install.cmake")

endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/home/morpheus/src/repos/BingoChado/GeorgeLink/client/target/debug/build/fltk-sys-c5ccceb078dc21ed/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
