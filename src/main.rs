// SPDX-License-Identifier: Apache-2.0
// Copyright 2021 IBM Corp.

// TODO: figure out the proper way to do multi-file work. Is this okay?
mod collector;
use collector::{Collector, CollectorValue, CollectorErr};

// TODO: move all the collector function modules into another directory
//  might make importing easier, see mod.rs / module docs
mod cpu;
mod memory;
mod gcc;

// TODO: rename this
struct PlatformData {
    tag: &'static str,
    func: fn(&mut Collector) -> Result<CollectorValue, CollectorErr>,
}

// TODO: This also needed a better name than "platform"
const PLATFORM: &'static [PlatformData] = &[
    PlatformData {tag: "cpu.cores", func: cpu::get_cores},
    PlatformData {tag: "mem.total", func: memory::get_mem_total},
    PlatformData {tag: "mem.used", func: memory::get_mem_used},
    PlatformData {tag: "mem.free", func: memory::get_mem_free},
    PlatformData {tag: "mem.shared", func: memory::get_mem_shared},
    PlatformData {tag: "mem.buff/cache", func: memory::get_mem_buff_and_cache},
    PlatformData {tag: "mem.available", func: memory::get_mem_available},
    PlatformData {tag: "swap.total", func: memory::get_swap_total},
    PlatformData {tag: "swap.used", func: memory::get_swap_used},
    PlatformData {tag: "swap.free", func: memory::get_swap_free},
    PlatformData {tag: "gcc.version", func: gcc::version},
    PlatformData {tag: "gcc.flags", func: gcc::flags},
];

fn main() {
    let mut col = Collector::new();

    // Iterate through each tag:func pair
    //   Call the function, and insert the data from the collecting function
    // TODO: add error checking here?
    for pd in PLATFORM {
        let f = pd.func;
        // YOLO: actually check the error instead of just unwrap >:(
        let data = f(&mut col).unwrap();
        // TODO: Perhaps error check here
        col.add_data(String::from(pd.tag), data)
    }

    col.print();
}


/*
Example of json output, and possible tag list

tags = [
    "cpu.cores",
    "cpu.threads",

    #[cpu == ppc64le]
    "lpar",
]


{
    "data": {
        "cpu.cores": 4,
        "cpu.threads": 7,
    }

    "raw": {
        "lscpu": ".."
    }

}
*/
