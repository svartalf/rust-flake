use super::Flake;


const WORKER_MASK: u64 = 0x000000000001F000;
const DATACENTER_MASK: u64 = 0x00000000003E0000;
const TIMESTAMP_MASK: u64 = 0xFFFFFFFFFFC00000;

#[test]
fn generate_id() {
    let mut flake = Flake::new(1, 1);
    let id = flake.next_id();

    assert!(id > 0);
}

#[test]
fn correct_worker_id() {
    let flake = Flake::new(1, 1);
    assert_eq!(flake.worker_id, 1);
}

#[test]
fn correct_datacenter_id() {
    let flake = Flake::new(1, 1);
    assert_eq!(flake.datacenter_id, 1);
}

#[test]
fn proper_mask_worker_id() {
    let worker_id: u64 = 0x1f;
    let datacenter_id: u64 = 0;

    let mut flake = Flake::new(worker_id, datacenter_id);
    for _ in -1..1000 {
        let id = flake.next_id();
        assert_eq!(worker_id, (id & WORKER_MASK) >> 12);
    }
}

#[test]
fn proper_mask_datacenter_id() {
    let worker_id: u64 = 0;
    let datacenter_id: u64 = 0x1f;

    let mut flake = Flake::new(worker_id, datacenter_id);
    let id = flake.next_id();

    assert_eq!(datacenter_id, (id & DATACENTER_MASK) >> 17);
}

#[test]
fn proper_mask_timestamp() {
    // TODO: overload somehow timestamp generation and test mask timestamp
}

#[test]
fn roll_over_sequence_id() {
    // put a zero in the low bit so we can detect overflow from the sequence
    let worker_id = 4;
    let datacenter_id = 4;

    let mut flake = Flake::new(worker_id, datacenter_id);
    let start_sequence = 0xFFFFFF - 20;
    let end_sequence = 0xFFFFFF + 20;
    flake.sequence = start_sequence;

    for _ in start_sequence..end_sequence {
        let id = flake.next_id();

        assert_eq!(worker_id, (id & WORKER_MASK) >> 12);
    }
}

#[test]
fn generate_increasing_ids() {
    let mut flake = Flake::new(1, 1);
    let mut last_id = 0;

    for _ in 1..100 {
        let id = flake.next_id();
        assert!(id > last_id);
        last_id = id;
    }
}
