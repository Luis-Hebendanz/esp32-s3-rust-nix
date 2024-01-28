use crate::vcp::*;

// Implementation for Virtual VCP:
pub struct VirtDevice {
    vcp: Vcp,
    position: (i32, i32),
}

impl Communication for VirtDevice {
    fn broadcast(&mut self, p: &Packet) {
        // The virtual sending is handled in VirtManager
        //self.vcp.outgoing_msgs.push(p.clone())
    }
}

impl VirtDevice {
    pub fn new(is_first: bool) -> Self {
        let v = VirtDevice {
            vcp: Vcp::new(is_first),
            position: (0, 0),
        };
        v
    }
}

pub struct VirtManager {
    devices: Vec<VirtDevice>,

    // The max sending distance
    range: i32,
}

/// VirtManager contains all devices and message the "sending" of messages.
/// It checks if the "devices" can hear each other by checking the distance.
impl VirtManager {
    pub fn handle_messages(&mut self) {
        // send all message that are in outgoing_msgs to all devices
        let mut sends: Vec<(usize, Packet, usize)> = Vec::new();
        for (s, ss) in self.devices.iter().enumerate() {
            for m in &ss.vcp.outgoing_msgs {
                for (r, rr) in self.devices.iter().enumerate() {
                    if s == r {
                        continue;
                    }

                    let dist_sqr = ((ss.position.0 - rr.position.0).pow(2)
                        + (ss.position.1 - rr.position.1).pow(2))
                        as f64;

                    if dist_sqr > self.range.pow(2).into() {
                        continue;
                    }
                    sends.push((s, m.clone(), r));
                }
            }
        }
        for (s, m, r) in sends {
            self.devices[r].vcp.receive(&m);
        }

        for d in &mut self.devices {
            d.vcp.outgoing_msgs.clear();
        }
    }

    pub fn add_device(&mut self, pos: (i32, i32)) {
        let mut d = VirtDevice::new(self.devices.len() == 0);
        d.position = pos;
        d.vcp.debug_name = format!("Dev: {}", self.devices.len());
        if self.devices.len() == 0 {
            d.vcp.send_init_message();
        }
        self.devices.push(d);
    }

    pub fn new() -> Self {
        VirtManager {
            devices: Vec::new(),
            range: 10,
        }
    }

    pub fn generate_graph() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works2() {
        let dev1 = VirtDevice::new(true);
        let dev2 = VirtDevice::new(false);
    }
}
