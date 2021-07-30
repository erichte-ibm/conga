

mod collector;
use collector::Collector;



fn get_num_cpus(col: &mut Collector) {

    let strig = col.run_command("lscpu");

    // parse(strig)

    // col.add_data(tag, strig)
}


fn main() {
    let mut col = Collector::new();

    println!("{:?}", col.run_command("ls"));
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
