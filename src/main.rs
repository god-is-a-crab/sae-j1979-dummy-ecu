mod pid_responses;

use anyhow::Context;
use embedded_can::{nb::Can, Frame as EmbeddedFrame, StandardId};
use nb::block;
use pid_responses::{parse_pid_responses, PidResponses};
use socketcan::{CanFrame, CanSocket, Frame, Socket};
use std::collections::HashMap;
use std::env;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref PID_RESPONSES: HashMap<u8, PidResponses> = parse_pid_responses().unwrap();
}

fn response_frame(
    frame: &CanFrame,
    pid_responses_index: &mut HashMap<u8, u32>,
) -> Option<CanFrame> {
    // Check broadcast id
    if frame.raw_id() != 0x7df {
        return None;
    }
    // Check service is 01
    let service = frame.data()[1];
    if service != 1 {
        return None;
    }
    let pid: u8 = frame.data()[2];
    if let Some(responses_info) = PID_RESPONSES.get(&pid) {
        // ECU Id
        let id = StandardId::new(0x7e8).unwrap();

        // Count service and pid with num_data_bytes
        // Add 0x40 to service: https://en.wikipedia.org/wiki/OBD-II_PIDs#Response
        let mut data: Vec<u8> = vec![
            responses_info.num_data_bytes as u8 + 2u8,
            service + 0x40,
            pid,
        ];
        if responses_info.responses.len() == 1 {
            data.extend_from_slice(&responses_info.responses[0]);
            data.push(0); // Last data byte is part of the standard can frame but unused.
            CanFrame::new(id, &data)
        } else {
            let i = pid_responses_index[&pid];
            data.extend_from_slice(&responses_info.responses[i as usize]);
            data.push(0); // Last data byte is part of the standard can frame but unused.
            *pid_responses_index
                .get_mut(&pid)
                .expect("Couldn't retrieve pid responses index") = i + 1;
            CanFrame::new(id, &data)
        }
    } else {
        None
    }
}

fn main() -> anyhow::Result<()> {
    let iface = env::args().nth(1).unwrap_or_else(|| "can0".into());
    let mut sock = CanSocket::open(&iface)
        .with_context(|| format!("Failed to open socket on interface {}", iface))?;
    sock.set_nonblocking(true)
        .context("Failed to make socket non-blocking")?;

    let mut pid_responses_index: HashMap<u8, u32> = HashMap::new();
    for pid in PID_RESPONSES.keys() {
        pid_responses_index.insert(*pid, 0u32);
    }

    loop {
        let received_frame = block!(sock.receive()).context("Receiving frame")?;
        if let Some(transmit_frame) = response_frame(&received_frame, &mut pid_responses_index) {
            block!(sock.transmit(&transmit_frame)).context("Transmitting frame")?;
        } else {
            eprintln!("No response for pid: {}", received_frame.data()[2]);
        }
    }
}
