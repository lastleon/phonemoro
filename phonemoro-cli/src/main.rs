use anyhow::Result;
use clap::{Arg, Command};
use phonemoro::en::phonemizer::EnPhonemizer;

fn main() -> Result<()> {
    let matches = Command::new("phonemize")
        .version("0.1.0")
        .arg(
            Arg::new("text_or_file")
                .index(1)
                .help("Pass the path to the file that should be converted to phonemes. If the flag --text is set, this will be interpreted as raw text.")
                .required(true),
        )
        .arg(
            Arg::new("text")
                .short('t')
                .long("text")
                .help("If set, the passed text will be phonemized, instead of interpreted as a file path.")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let text = matches.get_one::<String>("text_or_file").unwrap();
    let is_text = matches.get_flag("text");

    let p = EnPhonemizer::new()?;

    let text = if !is_text {
        &std::fs::read_to_string(text).expect("Should have been able to read the file")
    } else {
        text
    };

    println!("{:?}", p.phonemize(text.as_str()).unwrap());
    Ok(())
}
