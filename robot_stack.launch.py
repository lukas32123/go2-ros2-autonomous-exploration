from launch import LaunchDescription
from launch.actions import IncludeLaunchDescription, ExecuteProcess, SetEnvironmentVariable
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch_ros.substitutions import FindPackageShare
import os

def generate_launch_description():
    ws_dir = os.path.join(os.path.expanduser('~'), 'isaac-go2-ros2')

    return LaunchDescription([
        # Netzwerk-Sicherheit
        SetEnvironmentVariable('ROS_DOMAIN_ID', '42'),
        SetEnvironmentVariable('ROS_LOCALHOST_ONLY', '1'),

        # 1. SLAM Toolbox
        IncludeLaunchDescription(
            PythonLaunchDescriptionSource([FindPackageShare('slam_toolbox'), '/launch/online_async_launch.py']),
            launch_arguments={'slam_params_file': os.path.join(ws_dir, 'slam_sim.yaml'), 'use_sim_time': 'true'}.items()
        ),

        # 2. Nav2 Bringup
        IncludeLaunchDescription(
            PythonLaunchDescriptionSource([FindPackageShare('nav2_bringup'), '/launch/navigation_launch.py']),
            launch_arguments={'params_file': os.path.join(ws_dir, 'nav2_sim.yaml'), 'use_sim_time': 'true'}.items()
        ),

        # 3. RViz
        ExecuteProcess(
            cmd=['rviz2', '-d', os.path.join(ws_dir, 'rviz/go2.rviz'), '--ros-args', '-p', 'use_sim_time:=true'],
            output='screen'
        )
    ])