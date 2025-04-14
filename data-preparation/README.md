# Data Preparation
In this module, the data for the phonemizer is prepared.

## English (en)
English phonemization is based on the `us_gold.json` and `us_silver.json` datasets from [hexgrad/misaki](https://github.com/hexgrad/misaki/tree/main/misaki/data).


First, the datasets are cleaned and possible duplicates are removed. Then, a [FST](https://en.wikipedia.org/wiki/Finite-state_transducer) is trained using [Phonetisaurus](https://github.com/AdolfVonKleist/Phonetisaurus) for phonemization of out-of-dictionary words.
The processed datasets are used to look up words, the FST is used as a fallback in case the words are unknown. The phonemization of the FST is pretty bad, but fast, so it is only used as a last resort.

### Requirements
Only works on Linux currently, since utilities like `make` and `curl` are used. Nothing inherently prevents this from running on Windows, this is just how the project evolved. MacOS probably works, but wasn't tested.

So, assuming some Linux distribution with the usual utils, this is additionally needed:
- `Python` (any version should work fine)
- [uv](https://github.com/astral-sh/uv) [^1]
- `Docker` (for running Phonetisaurus)

[^1]: You can use something else, but then you need to modify the Makefile

### Usage
Create a uv project, and add the requirements:
```shell
phonemoro/data-preparation $ uv init && uv venv
phonemoro/data-preparation $ uv add -r requirements.txt
```

Then, just build the artifacts:
```shell
phonemoro/data-preparation $ make en
```

### Artifacts
The two processed lookup dictionaries, and the Phonetisaurus model in OpenFST format.
```shell
data-preparation/en/out/us_gold.json
data-preparation/en/out/us_silver.json
data-preparation/en/out/model.fst
```

## Attribution
This project utilizes data from [hexgrad/Misaki](https://github.com/hexgrad/Misaki), licensed under the Apache License 2.0.

Only a subset of the files from that project are used. The original LICENSE file is placed next to the original data when downloaded.
The data is cleaned, processed, transformed into a different format, and used for phonemization.
