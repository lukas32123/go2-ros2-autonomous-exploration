import rclpy
from rclpy.node import Node
from geometry_msgs.msg import TransformStamped
from tf2_ros.static_transform_broadcaster import StaticTransformBroadcaster

class LidarTf(Node):
    def __init__(self):
        super().__init__('lidar_tf_node')
        
        # Das native ROS 2 Tool: Setzt automatisch die perfekten QoS-Regeln!
        self.broadcaster = StaticTransformBroadcaster(self)
        
        self.get_logger().info('Lidar-Static-TF V2 aktiv! QoS-Firewall umgangen.')
        self.publish_transform()

    def publish_transform(self):
        t = TransformStamped()
        
        # Zeitstempel ist hier die Echtzeit
        t.header.stamp = self.get_clock().now().to_msg()
        t.header.frame_id = 'base_link'
        t.child_frame_id = 'unitree_go2/lidar_frame'
        
        # Der Lidar sitzt 10cm über dem Mittelpunkt
        t.transform.translation.z = 0.1
        t.transform.rotation.w = 1.0
        
        # Sende den Transform genau EINMAL dauerhaft ins Netzwerk
        self.broadcaster.sendTransform(t)

def main():
    rclpy.init()
    node = LidarTf()
    try: 
        rclpy.spin(node)
    except KeyboardInterrupt: 
        pass
    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__': 
    main()