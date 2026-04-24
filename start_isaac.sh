#!/bin/bash
export ROS_DISTRO=humble
export RMW_IMPLEMENTATION=rmw_fastrtps_cpp

if [ -z "$LD_LIBRARY_PATH" ]; then
    export LD_LIBRARY_PATH="/home/kilabor/isaacsim/exts/isaacsim.ros2.bridge/humble/lib"
else
    export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:/home/kilabor/isaacsim/exts/isaacsim.ros2.bridge/humble/lib"
fi

# ROS2 is already in the env_isaaclab conda env, no need to source system ROS2

python isaac_go2_ros2.py