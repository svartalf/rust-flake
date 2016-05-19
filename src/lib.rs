extern crate time;

#[derive(Default, Debug)]
pub struct Flake {
    twepoch: u64,

    worker_id_bits: i8,
    datacenter_id_bits: i8,
    sequence_bits: i8,

    worker_id: u64,
    datacenter_id: u64,
    sequence: u64,

    worker_id_shift: i8,
    datacenter_id_shift: i8,
    timestamp_left_shift: i8,
    sequence_mask: u64,

    last_timestamp: u64,
}

impl Flake {
    pub fn new(worker_id: u64, datacenter_id: u64) -> Flake {
        let mut flake: Flake = Default::default();
        flake.twepoch = 1288834974657;

        flake.worker_id_bits = 5;
        flake.datacenter_id_bits = 5;
        flake.sequence_bits = 12;

        flake.worker_id = worker_id;
        flake.datacenter_id = datacenter_id;

        flake.worker_id_shift = flake.sequence_bits;
        flake.datacenter_id_shift = flake.sequence_bits + flake.worker_id_bits;
        flake.timestamp_left_shift = flake.sequence_bits + flake.worker_id_bits +
                                     flake.datacenter_id_bits;
        flake.sequence_mask = !1 ^ (!1 << flake.sequence_bits);

        flake.last_timestamp = 0;

        return flake;
    }

    pub fn next_id(&mut self) -> u64 {
        let mut timestamp = time::precise_time_ns();
        // TODO: handle backwards-ticking clocks

        if timestamp == self.last_timestamp {
            self.sequence = (self.sequence + 1) & self.sequence_mask;
            if self.sequence == 0 {
                timestamp = time::precise_time_ns();
                while timestamp < self.last_timestamp {
                    timestamp = time::precise_time_ns();
                }

            }
        } else {
            self.sequence = 0;
        }

        self.last_timestamp = timestamp;

        return ((timestamp - self.twepoch) << self.timestamp_left_shift) |
               (self.datacenter_id << self.datacenter_id_shift) |
               (self.worker_id << self.worker_id_shift) | self.sequence;
    }
}

#[cfg(test)]
mod tests;
