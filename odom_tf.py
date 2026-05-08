import rclpy
from rclpy.node import Node
from nav_msgs.msg import Odometry
from geometry_msgs.msg import TransformStamped
from tf2_ros import TransformBroadcaster
from rclpy.qos import qos_profile_sensor_data

class OdomTfBroadcaster(Node):
    def __init__(self):
        super().__init__('odom_tf_broadcaster')
        
        self.set_parameters([rclpy.parameter.Parameter('use_sim_time', rclpy.Parameter.Type.BOOL, True)])
        self.br = TransformBroadcaster(self)
        self.sub = self.create_subscription(Odometry, '/unitree_go2/odom', self.odom_callback, qos_profile_sensor_data)
        self.log_count = 0
        self.get_logger().info('TF-Brücke V4 steht! (QoS-konformer TransformBroadcaster aktiv)')

    def odom_callback(self, msg):
        self.log_count += 1
        if self.log_count % 50 == 0:
            self.get_logger().info('Empfange Odom-Daten von Isaac Sim und sende TF QoS-konform!')

        t = TransformStamped()
        
        # Perfekte Isaac Sim Zeitsynchronisation
        t.header.stamp = self.get_clock().now().to_msg()
        t.header.frame_id = 'odom'
        t.child_frame_id = 'unitree_go2/base_link'
        
        t.transform.translation.x = msg.pose.pose.position.x
        t.transform.translation.y = msg.pose.pose.position.y
        t.transform.translation.z = msg.pose.pose.position.z
        t.transform.rotation = msg.pose.pose.orientation
        
        # Offizieller ROS 2 Weg zum Senden
        self.br.sendTransform(t)

def main():
    rclpy.init()
    node = OdomTfBroadcaster()
    try: 
        rclpy.spin(node)
    except KeyboardInterrupt: 
        pass
    node.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__': 
    main()