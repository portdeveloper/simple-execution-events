use std::time::Duration;

use monad_event_ring::{
    DecodedEventRing,
    EventNextResult,
    EventPayloadResult,
    EventRingPath,
};

use monad_exec_events::{
    ExecEvent,
    ExecEventReaderExt,
    ExecEventRing,
    ExecEventType,
};

const EVENT_RING_PATH: &str =
    "/var/lib/hugetlbfs/user/monad/pagesize-2MB/event-rings/monad-exec-events";

fn main() {
    let event_ring_path =
        EventRingPath::resolve(EVENT_RING_PATH).expect("Failed to resolve path");

    let ring =
        ExecEventRing::new(&event_ring_path).expect("Failed to open event ring");

    println!("Connected to event ring");
    println!("Waiting for events...\n");

    let mut reader = ring.create_reader();
    reader.consensus_prev(Some(ExecEventType::BlockStart));

    loop {
        match reader.next_descriptor() {
            EventNextResult::Ready(event) => {
                let seqno = event.info().seqno;

                match event.try_read() {
                    EventPayloadResult::Ready(exec_event) => {
                        print_event(&exec_event, seqno);
                    }
                    EventPayloadResult::Expired => {
                        eprintln!("Payload expired!");
                        reader.reset();
                    }
                }
            }

            EventNextResult::Gap => {
                eprintln!("Gap occurred");
                reader.reset();
            }

            EventNextResult::NotReady => {
                std::thread::sleep(Duration::from_millis(10));
            }
        }
    }
}

