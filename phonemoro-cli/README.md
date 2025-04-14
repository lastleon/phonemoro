# phonemoro-cli
## Build
See project [README](../README.md) for build instructions.

## Usage
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
