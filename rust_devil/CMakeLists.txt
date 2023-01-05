if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(CARGO_CMD cargo build --verbose)
    set(TARGET_DIR "debug")
else ()
    set(CARGO_CMD cargo build --release --verbose)
    set(TARGET_DIR "release")
endif ()



set(RUST_DEVIL_LIB "${CMAKE_CURRENT_BINARY_DIR}/${TARGET_DIR}/rust_devil.lib")
set(RUST_DEVIL_CXX "${CMAKE_CURRENT_BINARY_DIR}/rust_devil.cpp")

set(CARGO_TARGET_DIR ${CMAKE_CURRENT_BINARY_DIR})

set(CARGO_ENV_COMMAND ${CMAKE_COMMAND} -E env "CARGO_TARGET_DIR=${CARGO_TARGET_DIR}")

add_library(rust_devil STATIC ${RUST_DEVIL_CXX})
add_custom_command(
    OUTPUT ${RUST_DEVIL_CXX}
    COMMAND ${CARGO_ENV_COMMAND} ${CARGO_CMD} COMMENT "Building rust project"
    COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/rust_devil/src/lib.rs.h ${CMAKE_CURRENT_BINARY_DIR}/rust_devil.h  COMMENT "Copying ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/rust_devil/src/lib.rs.h' library to ${CMAKE_CURRENT_BINARY_DIR}/rust_devil.h"
    COMMAND ${CMAKE_COMMAND} -E copy ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/rust_devil/src/lib.rs.cc ${RUST_DEVIL_CXX} COMMENT "Copying ${CMAKE_CURRENT_BINARY_DIR}/cxxbridge/rust_devil/src/lib.rs.cc library to ${RUST_DEVIL_CXX}"
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
)


if (MSVC)
  target_link_libraries(rust_devil ws2_32 userenv advapi32 shell32)
  if (CMAKE_BUILD_TYPE STREQUAL "Release")
    target_link_libraries(rust_devil msvcrt bcrypt ${RUST_DEVIL_LIB})
  else ()
    target_link_libraries(rust_devil msvcrtd bcrypt ${RUST_DEVIL_LIB})
  endif()
elseif (UNIX)
  target_link_libraries(rust_devil pthread m dl ${RUST_DEVIL_LIB})
endif ()


add_test(NAME rust_devil_test 
    COMMAND cargo test
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
