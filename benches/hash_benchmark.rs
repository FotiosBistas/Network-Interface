use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;


use network_interface::tcp::{TCB, IdentifyingTCB, TcpState};

fn tcb_hashmap_operations(bencher: &mut Criterion) {
    let mut group = bencher.benchmark_group("TCB-HashMap-Operations");

    group.bench_function("Insert with identifier", |b| {
        let mut hashmap = HashMap::new();
        b.iter(|| {
            for port in 0..1000 {
                let tcb = TCB {
                    identifier: IdentifyingTCB {
                        local_address: [192, 168, 1, 1],
                        remote_address: [10, 0, 0, 1],
                        local_port: port,
                        remote_port: port + 1,
                    },
                    sequence_number: 0,
                    acknowledgment_number: 0,
                    window_size: 1024,
                    state: TcpState::Established,
                };
                hashmap.insert(tcb.identifier, tcb);
            }
        })
    });

    group.bench_function("Insert as u128", |b| {
        let mut hashmap = HashMap::new();
        b.iter(|| {
            for port in 0..1000 {
                let tcb = TCB {
                    identifier: IdentifyingTCB {
                        local_address: [192, 168, 1, 1],
                        remote_address: [10, 0, 0, 1],
                        local_port: port,
                        remote_port: port + 1,
                    },
                    sequence_number: 0,
                    acknowledgment_number: 0,
                    window_size: 1024,
                    state: TcpState::Established,
                };
                //pack the tcb identifier into a u128
                let packed_tcb = tcb.identifier.pack_tcb();
                hashmap.insert(packed_tcb, tcb);
            }
        })
    });

    group.bench_function("Retrieve with identifier", |b| {
        let mut hashmap = HashMap::new();
        for port in 0..1000 {
            let tcb = TCB {
                identifier: IdentifyingTCB {
                    local_address: [192, 168, 1, 1],
                    remote_address: [10, 0, 0, 1],
                    local_port: port,
                    remote_port: port + 1,
                },
                sequence_number: 0,
                acknowledgment_number: 0,
                window_size: 1024,
                state: TcpState::Established,
            };
            hashmap.insert(tcb.identifier, tcb);
        }
        b.iter(|| {
            for port in 0..1000 {
                black_box(hashmap.get(&IdentifyingTCB {
                    local_address: [192, 168, 1, 1],
                    remote_address: [10, 0, 0, 1],
                    local_port: port,
                    remote_port: port + 1,
                }));
            }
        })
    });

    group.bench_function("Retrieve with u128", |b| {
        let mut hashmap = HashMap::new();
        for port in 0..1000 {
            let tcb = TCB {
                identifier: IdentifyingTCB {
                    local_address: [192, 168, 1, 1],
                    remote_address: [10, 0, 0, 1],
                    local_port: port,
                    remote_port: port + 1,
                },
                sequence_number: 0,
                acknowledgment_number: 0,
                window_size: 1024,
                state: TcpState::Established,
            };
            let packed_tcb = tcb.identifier.pack_tcb();  // Declare it inside the loop
            hashmap.insert(packed_tcb, tcb);
        }
        b.iter(|| {
            for port in 0..1000 {
                let lookup_key = IdentifyingTCB {
                    local_address: [192, 168, 1, 1],
                    remote_address: [10, 0, 0, 1],
                    local_port: port,
                    remote_port: port + 1,
                }.pack_tcb();
                black_box(hashmap.get(&lookup_key));
            }
        })
    });
    group.finish();
}

criterion_group!(benches, tcb_hashmap_operations);
criterion_main!(benches);
