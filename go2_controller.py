import rclpy
from rclpy.node import Node
from geometry_msgs.msg import Twist
from nav_msgs.msg import Odometry
import math

class Go2Controller(Node):
    def __init__(self):
        super().__init__('go2_coordinate_controller')
        
        # Publisher für Geschwindigkeitsbefehle
        self.publisher_ = self.create_publisher(Twist, '/unitree_go2/cmd_vel', 10)
        
        # Subscriber für die aktuelle Position (Odometrie)
        self.subscription = self.create_subscription(Odometry, '/unitree_go2/odom', self.odom_callback, 10)
        
        # HIER DEIN ZIEL EINTRAGEN (in Metern)
        self.target_x = 10.0  # 2 Meter nach vorne
        self.target_y = 0.0  # 1 Meter nach links
        
        # Aktueller Zustand
        self.current_x = 0.0
        self.current_y = 0.0
        self.current_yaw = 0.0
        
        # Kontrollschleife (läuft 10 mal pro Sekunde)
        self.timer = self.create_timer(0.1, self.control_loop)
        self.get_logger().info(f"Hund laeuft zu Koordinaten: X={self.target_x}, Y={self.target_y}")

    def euler_from_quaternion(self, x, y, z, w):
        # Wandelt das unleserliche Quaternion-Format in Gierwinkel (Yaw/Drehung) um
        t3 = +2.0 * (w * z + x * y)
        t4 = +1.0 - 2.0 * (y * y + z * z)
        yaw_z = math.atan2(t3, t4)
        return yaw_z

    def odom_callback(self, msg):
        # Lese aktuelle Position
        self.current_x = msg.pose.pose.position.x
        self.current_y = msg.pose.pose.position.y
        
        # Lese aktuelle Drehung
        q = msg.pose.pose.orientation
        self.current_yaw = self.euler_from_quaternion(q.x, q.y, q.z, q.w)

    def control_loop(self):
        msg = Twist()
        
        # Berechne die Entfernung zum Ziel
        distance = math.sqrt((self.target_x - self.current_x)**2 + (self.target_y - self.current_y)**2)
        
        if distance < 0.1: # Ziel erreicht
            msg.linear.x = 0.0
            msg.angular.z = 0.0
            self.publisher_.publish(msg)
            self.get_logger().info("Ziel erreicht! Hund wartet.")
            return

        # Berechne den Winkel zum Ziel und den Fehler
        target_angle = math.atan2(self.target_y - self.current_y, self.target_x - self.current_x)
        angle_error = target_angle - self.current_yaw
        angle_error = math.atan2(math.sin(angle_error), math.cos(angle_error)) # Normalisieren auf [-pi, pi]
        
        # --- DER NEUE, FLÜSSIGE REGLER ---
        
        # 1. Drehung: Er darf sich immer eindrehen, weich und proportional zum Fehler
        msg.angular.z = 1.0 * angle_error
        
        # 2. Vorwärts: Er läuft vorwärts, aber je stärker er in die falsche Richtung guckt, 
        # desto langsamer läuft er, damit er Kurven fahren kann.
        speed_factor = max(0.0, 1.0 - abs(angle_error) / 1.0) 
        
        base_speed = min(0.5 * distance, 0.5) # Max 0.5 m/s
        msg.linear.x = base_speed * speed_factor
            
        self.publisher_.publish(msg)

def main(args=None):
    rclpy.init(args=args)
    controller = Go2Controller()
    rclpy.spin(controller)
    controller.destroy_node()
    rclpy.shutdown()

if __name__ == '__main__':
    main()