use clap::builder::TypedValueParser;
use clap::Parser;
use test_cli::{process_csv, Opts, SubCommand};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // format!("{}{}","output.",String::from(opts.format))
                format!("{}{}", "output.", Into::<&str>::into(opts.format))
            };
            process_csv(&opts.input, output, opts.format)?;
        }
    }
    Ok(())
}

// 可以接收 String 和 &str 类型的参数
fn process_text<T: AsRef<str>>(text: T) {
    let text_str = text.as_ref();
    println!("Processing text: {}", text_str);
}

fn process_txt(txt: &str) {
    println!("Processing text: {}", txt);
}

// fn main() {
//     let string = String::from("Hello, Rust!");
//     let str_literal = "Hello, World!";
//
//     // process_text(string); // 传递 String
//     // process_text(str_literal); // 传递 &str
//
//     process_txt(&string);
//     process_txt(str_literal);
// }
