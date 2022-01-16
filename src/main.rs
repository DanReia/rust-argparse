use argparse::ArgParser;

fn main(){
    // let arg_parser = ArgParser::new().collect_args().parse();
    let mut arg_parser = ArgParser::new();
    let arg = String::from("-a");
    let res = arg_parser.process_short_flag(arg);
    println!("{:?}",res);
}
