pub mod rat;

use clap::Parser;


#[derive(Parser)]
struct Args {
    #[arg(short, action)]
    numbered: bool,

    #[arg(short, action)]
    blank_nums: bool,

    sources: Vec<String>
}


fn main() {
    let args = Args::parse();
    let mut result: Vec<String> = vec![];
    if args.sources.len() == 1 {
        println!("No input provided");
        std::process::exit(1);
    }

    let args_iter = args.sources.iter();
    for a in args_iter {
        result.append(&mut rat::get_content(a.to_string()).unwrap());
    }

    if args.numbered {
        for (i, l) in result.iter().enumerate() {
            println!("{} {}", i+1, l);
        }
        if args.sources.len() == 0 {
            let result = rat::get_content("-".to_string()).unwrap();
            for (i, l) in result.iter().enumerate() {
                println!("{} {}", i+1, l);
            }
        }

    } else if args.blank_nums {
        let mut i = 0usize;
        for l in result.iter() {
            if l.len() > 0 {
                print!("{} ", i+1);
                i += 1;
            }
            println!("{}", l);
        }
        if args.sources.len() == 0 {
            let result = rat::get_content("-".to_string()).unwrap();
            for l in result.iter() {
                if l.len() > 0 {
                    print!("{} ", i+1);
                    i += 1;
                }
                println!("{}", l);
            }
        }

    } else {
        for l in result.iter() {
            println!("{}", l);
        }
    }
}