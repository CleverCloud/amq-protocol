use crate::{frame::parsing::traits::ParsableInput, protocol::*, types::*};
use std::fmt;

/// Enum representing an AMQP channel
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMQPChannel {
    /// The Global (id 0) AMQP channel used for creating other channels and for heartbeat
    Global,
    /// A regular AMQP channel
    Id(ShortUInt),
}

impl AMQPChannel {
    /// Get the channel id
    pub fn get_id(self) -> ShortUInt {
        match self {
            AMQPChannel::Global => 0,
            AMQPChannel::Id(id) => id,
        }
    }
}

impl From<ShortUInt> for AMQPChannel {
    fn from(id: ShortUInt) -> AMQPChannel {
        match id {
            0 => AMQPChannel::Global,
            id => AMQPChannel::Id(id),
        }
    }
}

/// The type of AMQP Frame
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AMQPFrameType {
    /// Call a method
    Method,
    /// Content header
    Header,
    /// Content body
    Body,
    /// Heartbeat frame
    Heartbeat,
}

/// The different possible frames
#[derive(Clone, Debug, PartialEq)]
pub enum AMQPFrame {
    /// Protocol header frame
    ProtocolHeader,
    /// Method call
    Method(ShortUInt, AMQPClass),
    /// Content header
    Header(ShortUInt, ShortUInt, Box<AMQPContentHeader>),
    /// Content body
    Body(ShortUInt, Vec<u8>),
    /// Heartbeat frame
    Heartbeat(ShortUInt),
}

impl AMQPFrame {
    /// Return whether this frame is an AMQPFrame::Header or not
    pub fn is_header(&self) -> bool {
        if let AMQPFrame::Header(..) = self {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for AMQPFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AMQPFrame::ProtocolHeader => f.write_str("AMQPFrame::ProtocolHeader"),
            AMQPFrame::Method(_, klass) => f.write_fmt(format_args!("AMQPFrame::Method({:?})", klass)),
            AMQPFrame::Header(..) => f.write_str("AMQPFrame::Header"),
            AMQPFrame::Body(..) => f.write_str("AMQPFrame::Body"),
            AMQPFrame::Heartbeat(_) => f.write_str("AMQPFrame::Heartbeat"),
        }
    }
}

/// Raw AMQP Frame
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AMQPRawFrame<I: ParsableInput> {
    /// The type of frame
    pub frame_type: AMQPFrameType,
    /// The id this frame was received on
    pub channel_id: ShortUInt,
    /// The payload of the frame
    pub payload: I,
}

/// Contente header
#[derive(Clone, Debug, PartialEq)]
pub struct AMQPContentHeader {
    /// The class of content
    pub class_id: ShortUInt,
    /// The weight of the content
    pub weight: ShortUInt,
    /// The size of the content's body
    pub body_size: LongLongUInt,
    /// The AMQP properties associated with the content
    pub properties: basic::AMQPProperties,
}
