#!/bin/bash
# Start Isaac Sim, the ROS2 bridge, SLAM, and Nav2 costmap/navigation stack.
# Run this from the repo root after activating env_isaaclab:
#   conda activate env_isaaclab
#   cd ~/isaac-go2-ros2
#   ./start_all.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG_DIR="$SCRIPT_DIR/logs"
mkdir -p "$LOG_DIR"

if [ -z "$CONDA_DEFAULT_ENV" ]; then
  echo "ERROR: conda environment is not active."
  echo "Please run: conda activate env_isaaclab"
  exit 1
fi

echo "Using conda env: $CONDA_DEFAULT_ENV"

# Try to source a standard ROS2 installation if ros2 is not already available.
if ! command -v ros2 >/dev/null 2>&1; then
  if [ -f "/opt/ros/humble/setup.bash" ]; then
    source "/opt/ros/humble/setup.bash"
  elif [ -n "$ROS_DISTRO" ] && [ -f "/opt/ros/$ROS_DISTRO/setup.bash" ]; then
    source "/opt/ros/$ROS_DISTRO/setup.bash"
  else
    echo "ERROR: ros2 command not found and no /opt/ros/<distro>/setup.bash was found."
    echo "Please source your ROS2 installation manually before running this script."
    exit 1
  fi
fi

if ! command -v ros2 >/dev/null 2>&1; then
  echo "ERROR: ros2 still not available after sourcing ROS2 environment."
  exit 1
fi

start_process() {
  local cmd="$1"
  local log="$2"
  echo "Starting: $cmd"
  bash -lc "$cmd" > "$log" 2>&1 &
  echo "  -> log: $log"
}

start_process "cd '$SCRIPT_DIR' && ./start_isaac.sh" "$LOG_DIR/start_isaac.log"
start_process "cd '$SCRIPT_DIR' && python master_bridge.py" "$LOG_DIR/master_bridge.log"
start_process "cd '$SCRIPT_DIR' && ros2 launch slam_toolbox online_async_launch.py slam_params_file:='$SCRIPT_DIR/go2_slam_params.yaml'" "$LOG_DIR/slam_toolbox.log"

echo "Warte 10 Sekunden auf Isaac Sim..."
sleep 10

start_process "cd '$SCRIPT_DIR' && ros2 launch nav2_bringup navigation_launch.py params_file:='$SCRIPT_DIR/nav2_params.yaml'" "$LOG_DIR/nav2.log"

echo
cat <<EOF
All processes have been started in the background.
Check logs in: $LOG_DIR

Next step: open RViz in another terminal:
  rviz2 -d "$SCRIPT_DIR/rviz/go2.rviz" --ros-args -p use_sim_time:=false

If any service fails, inspect the logs above.
