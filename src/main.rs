mod pid_responses;

use anyhow::Context;
use embedded_can::{nb::Can, Frame as EmbeddedFrame, StandardId};
use nb::block;
use socketcan::{CanFrame, CanSocket, Frame, Socket};
use std::env;
use pid_responses::parse_pid_responses;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref PID_RESPONSES: HashMap<u8, Vec<u8>> = parse_pid_responses().unwrap();
}

fn response_frame(frame: &CanFrame) -> Option<CanFrame> {
    println!("Processing: {}", frame_to_string(frame));
    if frame.raw_id() != 0x7df {
        return None;
    }
    let service = frame.data()[1];
    if service != 1 {
        return None;
    }
    let pid = frame.data()[2];
    let response = &PID_RESPONSES[&pid];
    let frame = CanFrame::new(StandardId::new(0x7e8).unwrap(), &response).expect("Creating response can frame");
    
    Some(frame)
}

fn main() -> anyhow::Result<()> {
    let iface = env::args().nth(1).unwrap_or_else(|| "can0".into());
    let mut sock = CanSocket::open(&iface)
        .with_context(|| format!("Failed to open socket on interface {}", iface))?;
    sock.set_nonblocking(true)
        .context("Failed to make socket non-blocking")?;

    loop {
        let received_frame = block!(sock.receive()).context("Receiving frame")?;
        if let Some(transmit_frame) = response_frame(&received_frame) {
            block!(sock.transmit(&transmit_frame)).context("Transmitting frame")?;
        } else {
            eprintln!("No response for frame: {}", frame_to_string(&received_frame));
        }
    }
}

fn frame_to_string<F: Frame>(frame: &F) -> String {
    let id = frame.raw_id();
    let data_string = frame
        .data()
        .iter()
        .fold(String::from(""), |a, b| format!("{} {:02x}", a, b));

    format!("{:X}  [{}] {}", id, frame.dlc(), data_string)
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     #[test]
//     fn test_process_can_frame_can_id() {
//         let frame1 = CanFrame::new(StandardId::new(0x7df).unwrap(), &[1, 2, 3, 4]).unwrap();
//         let frame2 = CanFrame::new(StandardId::new(0x1f1).unwrap(), &[1, 2, 3, 4]).unwrap();
//         assert_eq!(respond_to_can_frame(&frame1), None);
//         assert_eq!(!respond_to_can_frame(&frame2), Some());
//     }
// }
