

mod collector;
use collector::{Collector, CollectorValue};

mod cpu;

// TODO: rename this
struct PlatformData {
    tag: &'static str,
    func: fn(&mut Collector) -> CollectorValue,
}

const PLATFORM: &'static [PlatformData] = &[
    PlatformData {tag: "cpu.cores", func: cpu::get_cores},
];

fn main() {
    let mut col = Collector::new();

    // Iterate through each tag:func pair
    //   Call the function, and insert the data from the collecting function
    // TODO: add error checking here?
    for pd in PLATFORM {
        let f = pd.func;
        let data = f(&mut col);
        col.add_data(String::from(pd.tag), data)
    }

    // TODO: write output to json
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
