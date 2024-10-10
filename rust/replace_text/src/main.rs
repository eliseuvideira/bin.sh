use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    text: String,
    #[arg(short, long)]
    replacement: String,
    #[arg(index = 1)]
    content: String,
}

fn main() {
    let args = Args::parse();
    let text = args.text;
    let replacement = args.replacement;
    let content = args.content;
    let result = content.replace(&text, &replacement);
    println!("{}", result);
}
