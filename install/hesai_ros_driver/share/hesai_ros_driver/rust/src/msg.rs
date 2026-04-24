#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to hesai_ros_driver__msg__UdpPacket
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

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UdpPacket {

    // This member is not documented.
    #[allow(missing_docs)]
    pub stamp: builtin_interfaces::msg::Time,


    // This member is not documented.
    #[allow(missing_docs)]
    pub data: Vec<u8>,


    // This member is not documented.
    #[allow(missing_docs)]
    pub size: u32,

}



impl Default for UdpPacket {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::UdpPacket::default())
  }
}

impl rosidl_runtime_rs::Message for UdpPacket {
  type RmwMsg = super::msg::rmw::UdpPacket;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Owned(msg.stamp)).into_owned(),
        data: msg.data.into(),
        size: msg.size,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        stamp: builtin_interfaces::msg::Time::into_rmw_message(std::borrow::Cow::Borrowed(&msg.stamp)).into_owned(),
        data: msg.data.as_slice().into(),
      size: msg.size,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      stamp: builtin_interfaces::msg::Time::from_rmw_message(msg.stamp),
      data: msg.data
          .into_iter()
          .collect(),
      size: msg.size,
    }
  }
}


// Corresponds to hesai_ros_driver__msg__UdpFrame

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct UdpFrame {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


    // This member is not documented.
    #[allow(missing_docs)]
    pub packets: Vec<super::msg::UdpPacket>,

}



impl Default for UdpFrame {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::UdpFrame::default())
  }
}

impl rosidl_runtime_rs::Message for UdpFrame {
  type RmwMsg = super::msg::rmw::UdpFrame;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        packets: msg.packets
          .into_iter()
          .map(|elem| super::msg::UdpPacket::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
        packets: msg.packets
          .iter()
          .map(|elem| super::msg::UdpPacket::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      packets: msg.packets
          .into_iter()
          .map(super::msg::UdpPacket::from_rmw_message)
          .collect(),
    }
  }
}


// Corresponds to hesai_ros_driver__msg__Firetime

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Firetime {

    // This member is not documented.
    #[allow(missing_docs)]
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    pub data: [f64; 512],

}



impl Default for Firetime {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Firetime::default())
  }
}

impl rosidl_runtime_rs::Message for Firetime {
  type RmwMsg = super::msg::rmw::Firetime;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        data: msg.data,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        data: msg.data,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      data: msg.data,
    }
  }
}


// Corresponds to hesai_ros_driver__msg__Ptp

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Ptp::default())
  }
}

impl rosidl_runtime_rs::Message for Ptp {
  type RmwMsg = super::msg::rmw::Ptp;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        ptp_lock_offset: msg.ptp_lock_offset,
        ptp_status: msg.ptp_status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      ptp_lock_offset: msg.ptp_lock_offset,
        ptp_status: msg.ptp_status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      ptp_lock_offset: msg.ptp_lock_offset,
      ptp_status: msg.ptp_status,
    }
  }
}


// Corresponds to hesai_ros_driver__msg__LossPacket

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::LossPacket::default())
  }
}

impl rosidl_runtime_rs::Message for LossPacket {
  type RmwMsg = super::msg::rmw::LossPacket;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        total_packet_count: msg.total_packet_count,
        total_packet_loss_count: msg.total_packet_loss_count,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      total_packet_count: msg.total_packet_count,
      total_packet_loss_count: msg.total_packet_loss_count,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      total_packet_count: msg.total_packet_count,
      total_packet_loss_count: msg.total_packet_loss_count,
    }
  }
}


