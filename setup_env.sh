#!/bin/bash
conda activate env_isaaclab
export RMW_IMPLEMENTATION=rmw_fastrtps_cpp
unset FASTRTPS_DEFAULT_PROFILES_FILE
export ROS_LOCALHOST_ONLY=1
export ROS_DOMAIN_ID=42
echo "Go2 Workspace geladen"
