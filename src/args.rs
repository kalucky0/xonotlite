#[derive(Debug)]
pub struct Args {
    pub input_file: String,
    pub output_file: String,
    pub arch: String,
}

impl Args {
    pub fn read() -> Option<Args> {
        let args: Vec<String> = std::env::args().collect();
        if args.len() == 1 {
            println!("Usage: xonotlite <input file> [options]");
            println!("Options:");
            println!("  -o <output file>  Specify output file");
            println!("  -arch <arch>      Specify architecture");
            return None;
        }
        let mut res = Args {
            input_file: String::new(),
            output_file: String::new(),
            arch: String::from("x86_64"),
        };
        for i in (1..args.len()).step_by(2) {
            if args[i - 1] == "-o" {
                res.output_file = args[i].clone();
            } else if args[i - 1] == "-arch" {
                res.arch = args[i].clone();
            } else {
                res.input_file = args[i].clone();
                res.output_file = args[i].replace(".xon", "");
            }
        }
        Some(res)
    }
}
