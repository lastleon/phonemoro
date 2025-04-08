# Phonemizer-rs

_Fast, low-latency and highly portable phonemizer._

Transcribes to [IPA](https://en.wikipedia.org/wiki/International_Phonetic_Alphabet).
Created for the use with [Kokoro](https://huggingface.co/hexgrad/Kokoro-82M), but not limited to that.
Suitable for edge devices. Easy deployment, since all data is statically included in the binary, so no dependencies or other files needed.

Currently only support for US english.

⚠️ **WIP, so a lot can still change** ⚠️

## Overview

This project started because I needed a phonemizer for use with [Kokoro](https://huggingface.co/hexgrad/Kokoro-82M) on my phone, and because the alternatives were not a good fit in one way or another[^1]. As such, there are four key requirements this needed to fulfill:
[^1]: Also, I wanted to use this as a learning opportunity for Rust :)

- be fast enough
- have low enough latency
- produce IPA phonemes that are compatible with Kokoro, i.e. do not sound weird
- be easy to use and cross compile

With that in mind, this is how the phonemizer works:

1. **Tokenization**: First, the input text is tokenized using [Logos](https://github.com/maciejhirsz/logos) for easier preprocessing and phonemization logic.
2. **Lookup**: Then, the relevant words are looked up in the grapheme-to-phoneme datasets used by [Misaki](https://github.com/hexgrad/Misaki), the phonemizer behind Kokoro. The datasets are preprocessed and then statically embedded in the binary as a `phf_map` from the [phf](https://github.com/rust-phf/rust-phf) crate.
3. **Fallback**: If the lookup of a word has no result, then the word is phonemized with a finite state transducer (FST) trained with [Phonetisaurus](https://github.com/AdolfVonKleist/Phonetisaurus) on the previously mentioned datasets. The phonemizations produced by the FST are not that great, but it is fast. [phonetisaurus-g2p](https://github.com/lastleon/phonetisaurus-g2p-rs) was created to be an easy to use wrapper for that.

## Usage (lib)

1. Add the repository as a submodule to your crate:

```shell
$ git submodule add https://github.com/lastleon/phonemizer-rs
```

2. Prepare the data. Currently, only US english is supported, so the instructions focus on that. You have two options:

   - **Build the data yourself**. For that, go to the `data-preparation` directory, and follow the instructions there. Then, copy the artifacts (`model.fst`, `us_gold.json` and `us_silver.json`) to `src/en/data`. Note that this requires additional dependencies, and is currently only supported on Linux and maybe MacOS.
   - **Download the data from the _Releases_ page** (_Recommended_). Copy everything within the `en/out` folder from the release into `src/en/data`.

3. Now, back in your crate, add `phonemizer-rs` as a dependency:

```shell
$ cargo add --path ./phonemizer-rs
```

4. Use the library like so:

```rust
use phonemizer_rs::en::phonemizer::EnPhonemizer;

fn main() {
    let phonemizer = EnPhonemizer::new().unwrap();

    let result = phonemizer.phonemize("hello world").unwrap();
    assert_eq!(result, "həlˈO wˈɜɹld")
}
```

## Usage (cli)

1. Clone the repository:

```shell
$ git clone https://github.com/lastleon/phonemizer-rs
```

2. Prepare data the same way as for library usage, so step 2 of the previous section.

3. Build the cli:

```shell
$ cargo build -p phonemizer-cli --release
```

4. Use the binary, no other files needed:

```shell
$ ./target/release/phonemizer-cli --help
Usage: phonemizer-cli [OPTIONS] <text_or_file>

Arguments:
  <text_or_file>  Pass the path to the file that should be converted to phonemes. If the flag --text is set, this will be interpreted as raw text.

Options:
  -t, --text     If set, the passed text will be phonemized, instead of interpreted as a file path.
  -h, --help     Print help
  -V, --version  Print version
```

## TODO

- [ ] Add better preprocessing, e.g. "$" => "dollar", "25" => "twenty five"
- [ ] Add functions to get phonemes grouped by sentences
- [ ] Add homograph disambiguation (`read` (present) <-> `read` past)
- [ ] Add traced phonemization: Show from which dictionary the phonemes come from and whether the fallback was used
- [ ] Explore using [fst](https://docs.rs/fst/latest/fst/) crate instead of phf
- [ ] Add smarter dictionary lookup
- [ ] Add benchmark
- [ ] Fuzz test tokenizer && phonemizer
- [ ] Improve documentation
- [ ] Clean up crates

## Acknowledgements

- [hexgrad/Misaki](https://github.com/hexgrad/Misaki): Original (and reference) phonemizer for Kokoro, smarter than `phonemizer-rs`.
- [Patchethium/Celosia](https://github.com/Patchethium/Celosia): Another phonemizer in Rust, a good choice. Inspired a lot about how this project works, but does not use the Misaki datasets and its phonetic alphabet is ARPAbet, which makes it incompatible with Kokoro. ARPAbet could theoretically be transcribed to IPA, but it isn't as expressive as IPA (specifically, the stresses are missing), so doesn't work great.

## Attribution

This project utilizes data from [hexgrad/Misaki](https://github.com/hexgrad/Misaki), licensed under the Apache License 2.0.

Only a subset of the files from that project are used. The original LICENSE file is placed next to the original data when downloaded.
The data is cleaned, processed, transformed into a different format, and used for phonemization.

## License

`phonemizer-rs` is licensed under the MIT License.
