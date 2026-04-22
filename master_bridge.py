import rclpy
from rclpy.node import Node
from nav_msgs.msg import Odometry
from sensor_msgs.msg import PointCloud2, LaserScan
from tf2_msgs.msg import TFMessage
from geometry_msgs.msg import TransformStamped
from rclpy.qos import qos_profile_sensor_data
import numpy as np
import math

class MasterBridge(Node):
    def __init__(self):
        super().__init__('master_bridge')
        self.tf_pub = self.create_publisher(TFMessage, '/tf', 10)
        self.scan_pub = self.create_publisher(LaserScan, '/scan', 10)
        self.odom_sub = self.create_subscription(Odometry, '/unitree_go2/odom', self.odom_cb, qos_profile_sensor_data)
        self.pc_sub = self.create_subscription(PointCloud2, '/unitree_go2/lidar/point_cloud', self.pc_cb, qos_profile_sensor_data)
        self.get_logger().info('MASTER-BRÜCKE V2 ONLINE! (Zeitreisen deaktiviert)')

    def odom_cb(self, msg):
        t = TransformStamped()
        # KNALLHARTER ECHTZEIT-STEMPEL (Nur vorwärts!)
        t.header.stamp = msg.header.stamp
        t.header.frame_id = 'odom'
        t.child_frame_id = 'base_link'
        t.transform.translation.x = msg.pose.pose.position.x
        t.transform.translation.y = msg.pose.pose.position.y
        t.transform.translation.z = msg.pose.pose.position.z
        t.transform.rotation = msg.pose.pose.orientation
        tf_msg = TFMessage()
        tf_msg.transforms.append(t)
        self.tf_pub.publish(tf_msg)

    def pc_cb(self, msg):
        scan = LaserScan()
        # KNALLHARTER ECHTZEIT-STEMPEL (Nur vorwärts!)
        scan.header.stamp = msg.header.stamp
        scan.header.frame_id = 'lidar_frame'
        scan.angle_min = -math.pi
        scan.angle_max = math.pi
        scan.angle_increment = math.pi / 180.0
        scan.time_increment = 0.0
        scan.scan_time = 0.1
        scan.range_min = 0.2
        scan.range_max = 30.0
        num_rays = int((scan.angle_max - scan.angle_min) / scan.angle_increment)
        try:
            raw_data = np.frombuffer(msg.data, dtype=np.uint8).reshape(-1, msg.point_step)
            xyz = raw_data[:, 0:12].copy().view(dtype=np.float32)
        except Exception: return
        x = xyz[:, 0]; y = xyz[:, 1]; z = xyz[:, 2]
        mask = (z > 0.05) & (z < 0.15)
        x = x[mask]; y = y[mask]
        r = np.hypot(x, y)
        theta = np.arctan2(y, x)
        valid_range = (r > scan.range_min) & (r < scan.range_max)
        r = r[valid_range]; theta = theta[valid_range]
        idx = ((theta - scan.angle_min) / scan.angle_increment).astype(int)
        valid_idx = (idx >= 0) & (idx < num_rays)
        idx = idx[valid_idx]; r = r[valid_idx]
        ranges = np.full(num_rays, np.inf)
        np.minimum.at(ranges, idx, r)
        scan.ranges = ranges.tolist()
        self.scan_pub.publish(scan)

def main():
    rclpy.init()
    node = MasterBridge()
    try: rclpy.spin(node)
    except KeyboardInterrupt: pass
    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__': main()
