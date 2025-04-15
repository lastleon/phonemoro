# Phonemoro

_Fast, low-latency and highly portable phonemizer._

Transcribes to [IPA](https://en.wikipedia.org/wiki/International_Phonetic_Alphabet).
Created for the use with [Kokoro](https://huggingface.co/hexgrad/Kokoro-82M), but not limited to that.
Suitable for edge devices. Easy deployment, since all data is statically included in the binary, so no dependencies or other files needed.

Currently only support for US english.

<p align="center">
🚨 <b>WIP, so a lot can still change</b> 🚨
</p>

> ⚠️ This project was renamed from phonemizer-rs to phonemoro. See <https://github.com/lastleon/phonemoro/pull/1#issue-2992845609> for further information.

## Overview

This project started because I needed a phonemizer for use with [Kokoro](https://huggingface.co/hexgrad/Kokoro-82M) on my phone, and because the alternatives were not a good fit in one way or another[^1]. As such, there are four key requirements this needed to fulfill:
[^1]: Also, I wanted to use this as a learning opportunity for Rust :)

- be fast enough
- have low enough latency
- produce IPA phonemes that are compatible with Kokoro, i.e. do not sound weird
- be easy to use and cross compile

With that in mind, this is how the works:

1. **Tokenization**: First, the input text is tokenized using [Logos](https://github.com/maciejhirsz/logos) for easier preprocessing and phonemization logic.
2. **Lookup**: Then, the relevant words are looked up in the grapheme-to-phoneme datasets used by [Misaki](https://github.com/hexgrad/Misaki), the phonemizer behind Kokoro. The datasets are preprocessed and then statically embedded in the binary as a `phf_map` from the [phf](https://github.com/rust-phf/rust-phf) crate.
3. **Fallback**: If the lookup of a word has no result, then the word is phonemized with a finite state transducer (FST) trained with [Phonetisaurus](https://github.com/AdolfVonKleist/Phonetisaurus) on the previously mentioned datasets. The phonemizations produced by the FST are not that great, but it is fast. [phonetisaurus-g2p](https://github.com/lastleon/phonetisaurus-g2p-rs) was created to be an easy to use wrapper for that.

## Usage (lib)

This library requires data that needs to be prepared. You can either do that manually, or you can enable a feature and automatically download the prepared data from the releases page.

By default, automatically downloading the data is disabled.

### Easy Way _(Recommended)_

1. Add this library to your crate, with the `download-data` feature enabled:

```shell
$ cargo add --git https://github.com/lastleon/phonemoro -F download-data
```

> ⚠️ **Warning**:
> This downloads the `release.zip` file from the releases page on GitHub, unzips it, and moves the contents to the appropriate directory.
>
> This only works from **version 0.3.0 onwards**. You should only ever use the latest version of the library anyway, for now.

2. Use the library like so:

```rust
use phonemoro::en::phonemizer::EnPhonemizer;

fn main() {
    let phonemizer = EnPhonemizer::new().unwrap();

    let result = phonemizer.phonemize("hello world").unwrap();
    assert_eq!(result, "həlˈO wˈɜɹld")
}
```

### Harder Way

Use this only if you're uncomfortable downloading from the internet, or you want to use your own data.

1. Clone this repository:

```shell
$ git clone https://github.com/lastleon/phonemoro
```

2. Prepare the data. Currently, only US english is supported, so the instructions focus on that. For that, go to the `data-preparation` directory, and follow the instructions there. Then, copy the artifacts (`model.fst`, `us_gold.json` and `us_silver.json`) to `src/en/data`. Note that this requires additional dependencies, and is currently only supported on Linux and maybe MacOS.

3. Now, go to your own crate, and add `phonemoro` as a dependency:

```shell
$ cargo add --path <path-to-the-cloned-phonemoro-repo>
```

4. Use the library like shown in the previous section.

## Usage (cli)

1. Clone this repository:

```shell
$ git clone https://github.com/lastleon/phonemoro
```

2. Build the cli tool:

- **Easy Way**: Build the cli tool with the `download-data` feature enabled:

  ```shell
  $ cargo build -p phonemoro-cli --release -F download-data
  ```

  > ⚠️ **Warning**:
  > The same warnings as in [Usage (lib) > Easy Way](#easy-way) apply here.

- **Harder Way**: Follow step 2 of [Usage (lib) > Harder Way](#harder-way)

3. Use the tool:

```shell
$ ./target/release/phonemoro-cli --help
Usage: phonemoro-cli [OPTIONS] <text_or_file>

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

- [hexgrad/Misaki](https://github.com/hexgrad/Misaki): Original (and reference) phonemizer for Kokoro, smarter than `phonemoro`.
- [Patchethium/Celosia](https://github.com/Patchethium/Celosia): Another phonemizer in Rust, a good choice. Inspired a lot about how this project works, but does not use the Misaki datasets and its phonetic alphabet is ARPAbet, which makes it incompatible with Kokoro. ARPAbet could theoretically be transcribed to IPA, but it isn't as expressive as IPA (specifically, the stresses are missing), so doesn't work great.

## Attribution

This project utilizes data from [hexgrad/Misaki](https://github.com/hexgrad/Misaki), licensed under the Apache License 2.0.

Only a subset of the files from that project are used. The original LICENSE file is placed next to the original data when downloaded.
The data is cleaned, processed, transformed into a different format, and used for phonemization.

## License

`phonemoro` is licensed under the MIT License.
