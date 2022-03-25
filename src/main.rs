// SPDX-License-Identifier: Apache-2.0
// Copyright 2021 IBM Corp.

// TODO: figure out the proper way to do multi-file work. Is this okay?
mod collector;
use collector::{Collector, CollectorValue, CollectorErr};
use clap::{App, Arg, crate_version};

// TODO: move all the collector function modules into another directory
//  might make importing easier, see mod.rs / module docs
mod cpu;
mod memory;
mod os;
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
    PlatformData {tag: "os.kernel_release", func: os::get_kernel_release},
    PlatformData {tag: "os.os_name", func: os::get_os_name},
    PlatformData {tag: "swap.total", func: memory::get_swap_total},
    PlatformData {tag: "swap.used", func: memory::get_swap_used},
    PlatformData {tag: "swap.free", func: memory::get_swap_free},
    PlatformData {tag: "gcc.version", func: gcc::version},
    PlatformData {tag: "gcc.flags", func: gcc::flags},
];

fn main() {
    let mut col = Collector::new();

	let matches = App::new("conga")
        .version(crate_version!())
        .about("System Configuration Gatherer ")
        .arg(Arg::with_name("verbose")
            .short("v") // TODO: currently unused: set up logging first
            .long("verbose")
            .help("Increase verbosity")
            .multiple(true)
            .takes_value(true))
        .arg(Arg::with_name("allow-failures")
            .short("-f")
            .long("allow-failures")
            .help("Don't abort if a collector fails")
            .takes_value(false))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .help("Output file path to write to")
            .takes_value(true))
        .get_matches();

    // Iterate through each tag:func pair
    //   Call the function, and insert the data from the collecting function
    for pd in PLATFORM {
        let f = pd.func;

        let data = f(&mut col);

        if let Ok(d) = data {
            col.add_data(String::from(pd.tag), d)
        }
        else if matches.is_present("allow-failures") {
            // TODO: Use a CollectorError type here and use that to record the error.
            col.add_data(String::from(pd.tag), CollectorValue::Text(String::from("(null)")));
        }
        else {
            // Propogate the panic upwards and stop the collection.
            // TODO: Maybe be a little nicer about this.
            data.unwrap();
        }
    }

    match matches.value_of("output") {
        Some(o) => col.write_file(o),
        None => col.print(),
    }

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
