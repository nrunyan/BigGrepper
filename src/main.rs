use clap::Parser;
#[derive(Parser)]
struct ci{

    pattern:String,
    path:std::path::PathBuf,
}


fn main() {
    println!("Hello, world!");
    let args=ci::parse();
    println!("pattern: {:?}, path: {:?}",args.pattern,args.path);
    let content = std::fs::read_to_string(&args.path).expect("Couldn't read file");
    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{:?} in {:?}", &args.pattern, line);
        }
    }
}
