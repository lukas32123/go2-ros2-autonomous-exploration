import rclpy
from rclpy.node import Node
from nav_msgs.msg import Odometry
from tf2_ros import TransformBroadcaster
from geometry_msgs.msg import TransformStamped

class TFBridge(Node):
    def __init__(self):
        super().__init__('tf_bridge')
        self.br = TransformBroadcaster(self)
        # Wir hören auf die echte Go2 Odometrie
        self.sub = self.create_subscription(Odometry, '/utlidar/robot_odom', self.odom_cb, 10)
        self.get_logger().info("TF Bridge läuft! Übersetze Odom -> TF")

    def odom_cb(self, msg):
        t = TransformStamped()
        # Wir nehmen exakt den Zeitstempel des Roboters (extrem wichtig für SLAM!)
        t.header.stamp = msg.header.stamp 
        t.header.frame_id = 'odom'
        t.child_frame_id = 'base_link'
        
        # Position und Drehung in den TF-Baum kopieren
        t.transform.translation.x = msg.pose.pose.position.x
        t.transform.translation.y = msg.pose.pose.position.y
        t.transform.translation.z = msg.pose.pose.position.z
        t.transform.rotation = msg.pose.pose.orientation
        
        # TF Senden!
        self.br.sendTransform(t)

def main():
    rclpy.init()
    node = TFBridge()
    rclpy.spin(node)
    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()