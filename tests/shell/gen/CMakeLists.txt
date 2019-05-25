file (GLOB_RECURSE shellfiles
      RELATIVE "${CMAKE_CURRENT_SOURCE_DIR}"
	       LIST_DIRECTORIES
	       false
	       *.sh)

foreach (file ${shellfiles})
	if (BUILD_SHARED)
		set (ElektraCodegen_ALL_LIBRARIES "elektra-highlevel;elektra-opts;elektra-invoke;elektra-ease;elektra-kdb;elektra-core")
	elseif (BUILD_FULL)
		set (ElektraCodegen_ALL_LIBRARIES "elektra-full")
	elseif (BUILD_STATIC)
		set (ElektraCodegen_ALL_LIBRARIES "elektra-static")
	endif ()
	configure_file ("${CMAKE_CURRENT_SOURCE_DIR}/${file}" "${CMAKE_CURRENT_BINARY_DIR}/${file}" @ONLY)
endforeach ()

if (INSTALL_TESTING)
	install (DIRECTORY "." DESTINATION "${TARGET_TEST_DATA_FOLDER}/gen")
	install (DIRECTORY "${CMAKE_CURRENT_BINARY_DIR}" DESTINATION "${TARGET_TEST_DATA_FOLDER}/gen" FILES_MATCHING PATTERN "*.sh")
endif (INSTALL_TESTING)