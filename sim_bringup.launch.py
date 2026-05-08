from launch import LaunchDescription
from launch.actions import ExecuteProcess, TimerAction, SetEnvironmentVariable
import os

def generate_launch_description():
    ws_dir = os.path.join(os.path.expanduser('~'), 'isaac-go2-ros2')

    return LaunchDescription([
        # Netzwerk-Sicherheit (wie bisher)
        SetEnvironmentVariable('ROS_DOMAIN_ID', '42'),
        SetEnvironmentVariable('ROS_LOCALHOST_ONLY', '1'),

        # 1. Starte Isaac Sim (Das Äquivalent zum Einschalten des echten Go2)
        ExecuteProcess(
            cmd=['./start_isaac.sh'],
            cwd=ws_dir,
            output='screen'
        ),

        # 2. Warte 15 Sekunden auf Isaac Sim, starte dann das KOMPLETTE Fundament
        TimerAction(
            period=15.0,
            actions=[
                ExecuteProcess(cmd=['python3', 'odom_tf.py'], cwd=ws_dir, output='screen'),
                ExecuteProcess(cmd=['python3', 'pc_to_scan.py'], cwd=ws_dir, output='screen'),
                # NEU: Der Lidar-TF gehört zur Hardware-Basis!
                ExecuteProcess(cmd=['python3', 'lidar_tf.py'], cwd=ws_dir, output='screen')
            ]
        )
    ])