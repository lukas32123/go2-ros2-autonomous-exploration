#!/bin/bash
# Stop all processes started by start_all.sh
# Run this to cleanly shut down Isaac Sim, ROS2 nodes, etc.

echo "Stopping all processes..."

# Kill Isaac Sim (python isaac_go2_ros2.py)
pkill -f "python.*isaac_go2_ros2" && echo "Isaac Sim stopped." || echo "Isaac Sim not running."

# Kill pc_to_scan.py
pkill -f "python.*pc_to_scan" && echo "pc_to_scan stopped." || echo "pc_to_scan not running."

# Kill SLAM toolbox
pkill -f "slam_toolbox" && echo "SLAM stopped." || echo "SLAM not running."

# Kill Nav2
pkill -f "nav2" && echo "Nav2 stopped." || echo "Nav2 not running."

# Kill RViz
pkill -f "rviz2" && echo "RViz stopped." || echo "RViz not running."

echo "All processes stopped. If Isaac Sim is still running, try: killall -9 python"