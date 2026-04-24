#----------------------------------------------------------------
# Generated CMake target import file for configuration "Release".
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "hesai_ros_driver::hesai_ros_driver__rosidl_generator_py" for configuration "Release"
set_property(TARGET hesai_ros_driver::hesai_ros_driver__rosidl_generator_py APPEND PROPERTY IMPORTED_CONFIGURATIONS RELEASE)
set_target_properties(hesai_ros_driver::hesai_ros_driver__rosidl_generator_py PROPERTIES
  IMPORTED_LINK_DEPENDENT_LIBRARIES_RELEASE "hesai_ros_driver::hesai_ros_driver__rosidl_generator_c;Python::Python;hesai_ros_driver::hesai_ros_driver__rosidl_typesupport_c;builtin_interfaces::builtin_interfaces__rosidl_generator_py;std_msgs::std_msgs__rosidl_generator_py"
  IMPORTED_LOCATION_RELEASE "${_IMPORT_PREFIX}/lib/libhesai_ros_driver__rosidl_generator_py.so"
  IMPORTED_SONAME_RELEASE "libhesai_ros_driver__rosidl_generator_py.so"
  )

list(APPEND _cmake_import_check_targets hesai_ros_driver::hesai_ros_driver__rosidl_generator_py )
list(APPEND _cmake_import_check_files_for_hesai_ros_driver::hesai_ros_driver__rosidl_generator_py "${_IMPORT_PREFIX}/lib/libhesai_ros_driver__rosidl_generator_py.so" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
