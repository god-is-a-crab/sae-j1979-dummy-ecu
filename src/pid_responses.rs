use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;

static FILE: &str = "pid_responses.yaml";

#[derive(Serialize, Deserialize, Debug)]
pub struct PidResponses {
    pub num_data_bytes: usize,
    pub responses: Vec<Vec<u8>>,
}

pub fn parse_pid_responses() -> anyhow::Result<HashMap<u8, PidResponses>> {
    let reader = File::open(FILE)?;
    let yaml: HashMap<u8, PidResponses> = serde_yaml::from_reader(reader)?;
    Ok(yaml)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let responses = parse_pid_responses().unwrap();
        assert_eq!(responses.len(), 10);

        // check periodic parameters contain 245 responses
        for (pid, response_info) in responses.iter() {
            if (0..=1).contains(pid) {
                assert_eq!(response_info.responses.len(), 1);
            } else {
                assert_eq!(response_info.responses.len(), 245);
            }
        }

        // check pid 0 contains correct supported flagss
        let pid0_data = &responses[&0].responses[0];
        let pid0_bits = (pid0_data[0] as u32) << 8 * 3 | (pid0_data[1] as u32) << 8 * 2;
        const BIT_MASK: u32 = 0x80000000;
        for i in 0..32 {
            if ((pid0_bits << i) & BIT_MASK) == BIT_MASK {
                if i + 1 == 2 {
                    // special case
                    continue;
                }
                assert!(responses.get(&(i + 1)).is_some());
            }
        }
    }
}
