use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    
}

#[tokio::main]
async fn main() {
	let _cli = Args::parse();

}
