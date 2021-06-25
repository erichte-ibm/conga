use std::collections::HashMap;

struct Collector {
    raw: HashMap<String, String>, // command -> output
    data: HashMap<String, String>, // tag -> value
}

impl Collector {
    fn run_command(&mut self, command: &str) -> Result<String, String> {
        Ok(String::from("test"))
    }
}


fn get_num_cpus(col: Collector) {

    let strig = col.run_command("lscpu");

    // parse(strig)

    // col.add_data(tag, strig)
}


fn main() {


    println!("Hello, world!");
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
