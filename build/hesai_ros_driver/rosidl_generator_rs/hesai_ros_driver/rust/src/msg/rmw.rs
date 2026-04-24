#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "hesai_ros_driver__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__hesai_ros_driver__msg__UdpPacket() -> *const std::ffi::c_void;
}

#[link(name = "hesai_ros_driver__rosidl_generator_c")]
extern "C" {
    fn hesai_ros_driver__msg__UdpPacket__init(msg: *mut UdpPacket) -> bool;
    fn hesai_ros_driver__msg__UdpPacket__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UdpPacket>, size: usize) -> bool;
    fn hesai_ros_driver__msg__UdpPacket__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UdpPacket>);
    fn hesai_ros_driver__msg__UdpPacket__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UdpPacket>, out_seq: *mut rosidl_runtime_rs::Sequence<UdpPacket>) -> bool;
}

// Corresponds to hesai_ros_driver__msg__UdpPacket
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// field  size(byte)
/// SOB   2
/// angle  2
/// measure 5
/// block  SOB + angle + measure * 40
/// timestamp 4
/// factory 2
/// reserve 8
/// rpm  2
/// tail  timestamp + factory + reserve + rpm
/// packet block * 6 + tail

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UdpPacket {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::rmw::Time,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: rosidl_runtime_rs::Sequence<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub size: u32,

}



impl Default for UdpPacket {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !hesai_ros_driver__msg__UdpPacket__init(&mut msg as *mut _) {
        panic!("Call to hesai_ros_driver__msg__UdpPacket__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UdpPacket {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hesai_ros_driver__msg__UdpPacket__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hesai_ros_driver__msg__UdpPacket__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hesai_ros_driver__msg__UdpPacket__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UdpPacket {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UdpPacket where Self: Sized {
  const TYPE_NAME: &'static str = "hesai_ros_driver/msg/UdpPacket";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__hesai_ros_driver__msg__UdpPacket() }
  }
}


#[link(name = "hesai_ros_driver__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__hesai_ros_driver__msg__UdpFrame() -> *const std::ffi::c_void;
}

#[link(name = "hesai_ros_driver__rosidl_generator_c")]
extern "C" {
    fn hesai_ros_driver__msg__UdpFrame__init(msg: *mut UdpFrame) -> bool;
    fn hesai_ros_driver__msg__UdpFrame__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<UdpFrame>, size: usize) -> bool;
    fn hesai_ros_driver__msg__UdpFrame__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<UdpFrame>);
    fn hesai_ros_driver__msg__UdpFrame__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<UdpFrame>, out_seq: *mut rosidl_runtime_rs::Sequence<UdpFrame>) -> bool;
}

// Corresponds to hesai_ros_driver__msg__UdpFrame
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UdpFrame {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub packets: rosidl_runtime_rs::Sequence<super::super::msg::rmw::UdpPacket>,

}



impl Default for UdpFrame {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !hesai_ros_driver__msg__UdpFrame__init(&mut msg as *mut _) {
        panic!("Call to hesai_ros_driver__msg__UdpFrame__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for UdpFrame {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hesai_ros_driver__msg__UdpFrame__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hesai_ros_driver__msg__UdpFrame__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hesai_ros_driver__msg__UdpFrame__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for UdpFrame {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for UdpFrame where Self: Sized {
  const TYPE_NAME: &'static str = "hesai_ros_driver/msg/UdpFrame";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__hesai_ros_driver__msg__UdpFrame() }
  }
}


#[link(name = "hesai_ros_driver__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__hesai_ros_driver__msg__Firetime() -> *const std::ffi::c_void;
}

#[link(name = "hesai_ros_driver__rosidl_generator_c")]
extern "C" {
    fn hesai_ros_driver__msg__Firetime__init(msg: *mut Firetime) -> bool;
    fn hesai_ros_driver__msg__Firetime__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Firetime>, size: usize) -> bool;
    fn hesai_ros_driver__msg__Firetime__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Firetime>);
    fn hesai_ros_driver__msg__Firetime__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Firetime>, out_seq: *mut rosidl_runtime_rs::Sequence<Firetime>) -> bool;
}

// Corresponds to hesai_ros_driver__msg__Firetime
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Firetime {

    // This member is not documented.
    #[allow(missing_docs)]
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub data: [f64; 512],

}



impl Default for Firetime {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !hesai_ros_driver__msg__Firetime__init(&mut msg as *mut _) {
        panic!("Call to hesai_ros_driver__msg__Firetime__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Firetime {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hesai_ros_driver__msg__Firetime__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hesai_ros_driver__msg__Firetime__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hesai_ros_driver__msg__Firetime__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Firetime {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Firetime where Self: Sized {
  const TYPE_NAME: &'static str = "hesai_ros_driver/msg/Firetime";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__hesai_ros_driver__msg__Firetime() }
  }
}


#[link(name = "hesai_ros_driver__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__hesai_ros_driver__msg__Ptp() -> *const std::ffi::c_void;
}

#[link(name = "hesai_ros_driver__rosidl_generator_c")]
extern "C" {
    fn hesai_ros_driver__msg__Ptp__init(msg: *mut Ptp) -> bool;
    fn hesai_ros_driver__msg__Ptp__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Ptp>, size: usize) -> bool;
    fn hesai_ros_driver__msg__Ptp__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Ptp>);
    fn hesai_ros_driver__msg__Ptp__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Ptp>, out_seq: *mut rosidl_runtime_rs::Sequence<Ptp>) -> bool;
}

// Corresponds to hesai_ros_driver__msg__Ptp
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Ptp {

    // This member is not documented.
    #[allow(missing_docs)]
    pub ptp_lock_offset: u8,


    // This member is not documented.
    #[allow(missing_docs)]
    pub ptp_status: [u8; 16],

}



impl Default for Ptp {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !hesai_ros_driver__msg__Ptp__init(&mut msg as *mut _) {
        panic!("Call to hesai_ros_driver__msg__Ptp__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Ptp {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hesai_ros_driver__msg__Ptp__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hesai_ros_driver__msg__Ptp__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hesai_ros_driver__msg__Ptp__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Ptp {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Ptp where Self: Sized {
  const TYPE_NAME: &'static str = "hesai_ros_driver/msg/Ptp";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__hesai_ros_driver__msg__Ptp() }
  }
}


#[link(name = "hesai_ros_driver__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__hesai_ros_driver__msg__LossPacket() -> *const std::ffi::c_void;
}

#[link(name = "hesai_ros_driver__rosidl_generator_c")]
extern "C" {
    fn hesai_ros_driver__msg__LossPacket__init(msg: *mut LossPacket) -> bool;
    fn hesai_ros_driver__msg__LossPacket__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<LossPacket>, size: usize) -> bool;
    fn hesai_ros_driver__msg__LossPacket__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<LossPacket>);
    fn hesai_ros_driver__msg__LossPacket__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<LossPacket>, out_seq: *mut rosidl_runtime_rs::Sequence<LossPacket>) -> bool;
}

// Corresponds to hesai_ros_driver__msg__LossPacket
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct LossPacket {

    // This member is not documented.
    #[allow(missing_docs)]
    pub total_packet_count: u32,


    // This member is not documented.
    #[allow(missing_docs)]
    pub total_packet_loss_count: u32,

}



impl Default for LossPacket {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !hesai_ros_driver__msg__LossPacket__init(&mut msg as *mut _) {
        panic!("Call to hesai_ros_driver__msg__LossPacket__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for LossPacket {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hesai_ros_driver__msg__LossPacket__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hesai_ros_driver__msg__LossPacket__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { hesai_ros_driver__msg__LossPacket__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for LossPacket {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for LossPacket where Self: Sized {
  const TYPE_NAME: &'static str = "hesai_ros_driver/msg/LossPacket";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__hesai_ros_driver__msg__LossPacket() }
  }
}


