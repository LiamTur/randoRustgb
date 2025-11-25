use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parser_function(&args);


    println!("wow you gave me the string {}, and the path {}",config.query, config.path);
    
    let content = fs::read_to_string(config.path.clone()).expect("this aint a viable file chief");

    println!("It also seems that in the file {} there is the content \n{}", config.path, content);
}

struct CLI {
    query: String,
    path: String,
}

fn parser_function(args: &[String]) -> CLI{
   let query = args[1].clone();
   let path = args[2].clone();

    CLI { query, path }
}
