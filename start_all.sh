#!/bin/bash
export ROS_DISTRO=humble
export RMW_IMPLEMENTATION=rmw_fastrtps_cpp

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG_DIR="$SCRIPT_DIR/logs"
mkdir -p "$LOG_DIR"

if [ -z "$CONDA_DEFAULT_ENV" ]; then
  echo "ERROR: conda environment is not active."
  exit 1
fi

if ! command -v ros2 >/dev/null 2>&1; then
  if [ -f "/opt/ros/humble/setup.bash" ]; then
    source "/opt/ros/humble/setup.bash"
  elif [ -n "$ROS_DISTRO" ] && [ -f "/opt/ros/$ROS_DISTRO/setup.bash" ]; then
    source "/opt/ros/$ROS_DISTRO/setup.bash"
  fi
fi

start_process() {
  local cmd="$1"
  local log="$2"
  echo "Starting: $cmd"
  bash -c "$cmd" > "$log" 2>&1 &
}

# 1. Isaac Sim starten
start_process "cd '$SCRIPT_DIR' && ./start_isaac.sh" "$LOG_DIR/start_isaac.log"

# 2. Die 3 Original-Bausteine starten (Echtzeit)
start_process "cd '$SCRIPT_DIR' && python odom_tf.py" "$LOG_DIR/odom_tf.log"
start_process "cd '$SCRIPT_DIR' && python lidar_tf.py" "$LOG_DIR/lidar_tf.log"
start_process "cd '$SCRIPT_DIR' && python pc_to_scan.py" "$LOG_DIR/pc_to_scan.log"

# 3. SLAM und Nav2 starten (Ebenfalls Echtzeit!)
start_process "cd '$SCRIPT_DIR' && ros2 launch slam_toolbox online_async_launch.py slam_params_file:='$SCRIPT_DIR/go2_slam_params.yaml' use_sim_time:=false" "$LOG_DIR/slam_toolbox.log"

echo "Warte 10 Sekunden auf Isaac Sim..."
sleep 10

start_process "cd '$SCRIPT_DIR' && ros2 launch nav2_bringup navigation_launch.py params_file:='$SCRIPT_DIR/nav2_params.yaml' use_sim_time:=false" "$LOG_DIR/nav2.log"

cat <<EOF
Basis-System erfolgreich gestartet!
Nutze jetzt deine Original-RViz Datei:
  rviz2 -d "$SCRIPT_DIR/rviz/go2.rviz" --ros-args -p use_sim_time:=false
EOF