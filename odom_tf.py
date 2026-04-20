import rclpy
from rclpy.node import Node
from nav_msgs.msg import Odometry
from tf2_msgs.msg import TFMessage
from geometry_msgs.msg import TransformStamped
from rclpy.qos import qos_profile_sensor_data

class OdomTfBroadcaster(Node):
    def __init__(self):
        super().__init__('odom_tf_broadcaster')
        self.sub = self.create_subscription(Odometry, '/unitree_go2/odom', self.odom_callback, qos_profile_sensor_data)
        self.tf_pub = self.create_publisher(TFMessage, '/tf', 10)
        self.get_logger().info('TF-Brücke steht! (Erzwinge perfekte Systemzeit)')
        self.odom_count = 0

    def odom_callback(self, msg):
        self.odom_count += 1
        if self.odom_count % 50 == 0:
            self.get_logger().info(f'TF-Brücke aktiv: {self.odom_count}')
        t = TransformStamped()
        
        # DER ZEIT-HACK: Isaac Sim ignorieren, exakte Laptop-Zeit aufzwingen!
        t.header.stamp = self.get_clock().now().to_msg()
        
        t.header.frame_id = 'odom'
        t.child_frame_id = 'base_link'
        t.transform.translation.x = msg.pose.pose.position.x
        t.transform.translation.y = msg.pose.pose.position.y
        t.transform.translation.z = msg.pose.pose.position.z
        t.transform.rotation = msg.pose.pose.orientation
        tf_msg = TFMessage()
        tf_msg.transforms.append(t)
        self.tf_pub.publish(tf_msg)

def main():
    rclpy.init()
    node = OdomTfBroadcaster()
    try: rclpy.spin(node)
    except KeyboardInterrupt: pass
    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__': main()
