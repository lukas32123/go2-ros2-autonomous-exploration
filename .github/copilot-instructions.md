<!--
Brief, actionable instructions for AI coding agents working on this repo.
Keep content limited to discoverable, project-specific patterns and workflows.
-->
# Copilot / AI Agent Guidelines — Isaac Sim Unitree Go2 ROS2

Purpose: quickly orient an AI coding assistant so it can make correct, minimal edits and suggest changes confidently.

1) Big picture
- This repo runs Unitree Go2 inside NVIDIA Isaac Sim / Isaac Lab and exposes sensors & state via a ROS2 bridge.
- High-level flow: `isaac_go2_ros2.py` launches the Isaac app -> builds Go2 RL environment (`go2/`), sensors (`go2/go2_sensors.py`), and ROS2 bridge (`ros2/go2_ros2_bridge.py`) -> simulation loop steps the environment and calls `RobotDataManager.pub_ros2_data()` to publish topics.

2) Entry points & run commands
- Primary runner: `python isaac_go2_ros2.py` (expects Isaac Lab environment / conda env `isaaclab`).
- Helper script: `start_isaac.sh` (sets `LD_LIBRARY_PATH` for the ros2 bridge then runs the python launcher).
- Use `rviz2 -d rviz/go2.rviz` to visualize topics after starting the sim.

3) Key files and what to edit
- `isaac_go2_ros2.py` — main launcher, hydra config loader (`cfg/sim.yaml`) and simulation loop.
- `cfg/sim.yaml` — primary runtime settings (num_envs, freq, sensors, camera_follow, resolution).
- `go2/`:
  - `go2_env.py` — Isaac/MDP environment config (observation/action groups, camera follow helper).
  - `go2_ctrl.py` — policy loading, keyboard teleop hooks; `base_vel_cmd_input` is the global tensor used by both teleop and ROS cmd_vel callbacks.
  - `go2_sensors.py` — camera + RTX LIDAR creation; uses Isaac sensor creators and replicator writers.
- `ros2/go2_ros2_bridge.py` — the ROS2 bridge: publishers, subscribers, tf broadcasters, and Replicator writers for images/pointclouds. Many project-specific topic names and frame conventions live here.
- `pc_to_scan.py` — example node converting `PointCloud2` (/unitree_go2/lidar/point_cloud) to `LaserScan` (/scan); good reference for message parsing and frame ids.

4) Important conventions & patterns
- Multi-robot support: controlled by `cfg.sim.yaml` -> `num_envs`; topics and frames use `unitree_go2` or `unitree_go2_{i}` naming consistently in the bridge.
- Teleop & commands: keyboard events write into `go2.go2_ctrl.base_vel_cmd_input` (tensor). ROS cmd_vel subscriber in `RobotDataManager` updates the same tensor for remote control.
- Sensors: camera objects expose a `_render_product_path` used by replicator writers (see `pub_color_image`, `pub_depth_image`, `pub_semantic_image`).
- Frequencies: odom/pose published at ~50Hz, lidar ~15Hz (hard-coded in bridge). Camera frequency comes from `cfg.freq` and `go2_sensors.Camera(...)`.
- Frames: TF frame naming uses `base_link`, `unitree_go2/base_link`, `unitree_go2/lidar_frame`, and camera frames like `unitree_go2/front_cam` — maintain these when changing publishers/subscribers.

5) Checkpoints, training, and assets
- Trained policies are loaded via `go2/go2_ctrl.py` using `isaaclab_tasks.utils.get_checkpoint_path` and `ckpts/` is the default search directory.
- USD/asset environment paths are referenced via Isaac Nucleus root (used in `env/sim_env.py`).

6) Editing guidance (do this first when modifying code)
- Small runtime changes: update `cfg/sim.yaml` (num_envs, sensor flags, freq) and rerun via `start_isaac.sh`.
- To add a new camera topic or change image encoding, edit `ros2/go2_ros2_bridge.py` writer initialization (look for `rep.writers.get("...ROS2PublishImage")`).
- When changing topics or TF frame names, update both the bridge and any nodes that subscribe (e.g., `pc_to_scan.py`) to keep names consistent.
- When adding new sensor attachments, prefer editing `go2/go2_sensors.py` to keep creation and replicator setup centralized.

7) Debugging & common pitfalls
- Ensure the Isaac Lab conda env is active (`conda activate isaaclab`) and that `LD_LIBRARY_PATH` includes the isaacsim.ros2.bridge libs (see `start_isaac.sh`).
- The bridge enables its extension programmatically — edits to bridge behavior may require enabling the extension in the running Isaac app (see top of `ros2/go2_ros2_bridge.py`).
- Use `rclpy.spin_once(node, timeout_sec=0.0)` in small loops (already used in the main loop) — avoid blocking spins inside the sim loop.
- For performance tuning, check `cfg.freq`, `Go2RSLEnvCfg.decimation`, and `sim.dt` interplay in `go2/go2_env.py`.

8) Quick examples (copy/paste)
- Run sim (local):
```
conda activate isaaclab
./start_isaac.sh
```
- Visualize in RViz:
```
rviz2 -d rviz/go2.rviz
```
- Example ROS topic names (single-robot): `/unitree_go2/odom`, `/unitree_go2/pose`, `/unitree_go2/lidar/point_cloud`, `/unitree_go2/front_cam/color_image`, `/unitree_go2/cmd_vel`.

9) When to ask the maintainer
- If you need changes that require a different Isaac Sim extension, or changes to the IsaacLab conda environment, ask — these are environment-dependent and not discoverable purely from code.

---
If any sections are unclear or you want more concrete examples (unit tests, a small runnable smoke-test, or common refactors), say which part to expand and I will iterate.
