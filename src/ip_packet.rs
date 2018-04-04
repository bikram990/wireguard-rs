use pnet_packet::ipv4::Ipv4Packet;
use pnet_packet::ipv6::Ipv6Packet;
use std::net::IpAddr;

pub enum IpPacket<'a> {
    V4(Ipv4Packet<'a>),
    V6(Ipv6Packet<'a>),
}

impl<'a> IpPacket<'a> {
    pub fn new(packet: &'a [u8]) -> Option<Self> {
        if packet.is_empty() {
            return None;
        }

        match packet[0] >> 4 {
            4 => Ipv4Packet::new(packet).map(IpPacket::V4),
            6 => Ipv6Packet::new(packet).map(IpPacket::V6),
            _ => None
        }
    }

    pub fn source(&self) -> IpAddr {
        match *self {
            IpPacket::V4(ref packet) => packet.get_source().into(),
            IpPacket::V6(ref packet) => packet.get_source().into(),
        }
    }

    pub fn destination(&self) -> IpAddr {
        match *self {
            IpPacket::V4(ref packet) => packet.get_destination().into(),
            IpPacket::V6(ref packet) => packet.get_destination().into(),
        }
    }

    pub fn length(&self) -> u16 {
        match *self {
            IpPacket::V4(ref packet) => packet.get_total_length(),
            IpPacket::V6(ref packet) => 40 + packet.get_payload_length(),
        }

    }
}
