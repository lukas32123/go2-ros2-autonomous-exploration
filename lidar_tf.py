import rclpy
from rclpy.node import Node
from tf2_msgs.msg import TFMessage
from geometry_msgs.msg import TransformStamped

class LidarTf(Node):
    def __init__(self):
        super().__init__('lidar_tf_node')
        # Standard-Kanal, kein Static mehr
        self.pub = self.create_publisher(TFMessage, '/tf', 10)
        # Sende die Info 20x pro Sekunde!
        self.timer = self.create_timer(0.05, self.timer_callback)
        self.get_logger().info('Dauerhaftes Klebeband aktiv! (20x pro Sekunde)')

    def timer_callback(self):
        t = TransformStamped()
        t.header.stamp = self.get_clock().now().to_msg()
        t.header.frame_id = 'base_link'
        t.child_frame_id = 'lidar_frame'
        
        t.transform.translation.z = 0.1
        t.transform.rotation.w = 1.0
        
        msg = TFMessage()
        msg.transforms.append(t)
        self.pub.publish(msg)

def main():
    rclpy.init()
    node = LidarTf()
    try: rclpy.spin(node)
    except KeyboardInterrupt: pass
    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__': main()
