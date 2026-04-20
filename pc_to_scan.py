import rclpy
from rclpy.node import Node
from sensor_msgs.msg import PointCloud2, LaserScan
from rclpy.qos import qos_profile_sensor_data
import numpy as np
import math

class PcToScan(Node):
    def __init__(self):
        super().__init__('pc_to_scan_node')
        self.sub = self.create_subscription(PointCloud2, '/unitree_go2/lidar/point_cloud', self.cb, qos_profile_sensor_data)
        self.pub = self.create_publisher(LaserScan, '/scan', 10)
        self.get_logger().info('Turbo-Presse online! (Erzwinge perfekte Systemzeit)')
        self.scan_count = 0

    def cb(self, msg):
        self.scan_count += 1
        if self.scan_count % 50 == 0:
            self.get_logger().info(f'Turbo: {self.scan_count} Scans gesendet.')
        scan = LaserScan()
        
        # DER ZEIT-HACK: Isaac Sim ignorieren, exakte Laptop-Zeit aufzwingen!
        scan.header.stamp = self.get_clock().now().to_msg()
        
        scan.header.frame_id = 'base_link'
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
        mask = (z > -0.2) & (z < 0.2)
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
        self.pub.publish(scan)

def main():
    rclpy.init()
    node = PcToScan()
    try: rclpy.spin(node)
    except KeyboardInterrupt: pass
    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__': main()
