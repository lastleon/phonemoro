import json
import argparse
import logging
import coloredlogs
from pathlib import Path

# Create argument parser
parser = argparse.ArgumentParser(description="Script for creating training lexicon for FST phonemizer.")
parser.add_argument("inputfiles", type=str, help="List input input files, seperate with comma. Should be output of create_cleaned_json.py")
parser.add_argument("-o", "--outfile", default="out/training-lexicon.dict", type=str, help="Specify the output file.")
parser.add_argument("-v", "--verbose", action="store_true", help="Enable verbose logging.")

args = parser.parse_args()

# set up logger
logger = logging.getLogger("Data Preparation")
logger.setLevel(logging.DEBUG if args.verbose else logging.INFO)
coloredlogs.install(level="DEBUG" if args.verbose else "INFO", logger=logger)

# parse data
INPUT_FILES: list[Path] = list(map(lambda f: Path.resolve(Path(f)), args.inputfiles.split(",")))
OUT_FILE_PATH = Path.resolve(Path(args.outfile))

# futher preparation
def split_phonemes(phonemes: str) -> list[str]:
    """Split into list of phonemes, but keep emph (ˈ and ˌ) attached to the following phoneme."""
    split_p = []
    emph = None
    for c in phonemes:
        if c == "ˈ" or c == "ˌ":
            emph = c
        elif emph is not None:
            split_p.append(emph + c)
            emph = None
        else:
            split_p.append(c)
    return split_p

# collect data
merged_data = {}

logger.info("Start collecting data.")
for filepath in INPUT_FILES:
    logger.info(str(filepath) + "...")
    with open(filepath, "r") as f:
        data = json.load(f)

    for word, phonemes in data.items():
        if word in merged_data:
            logger.info(f"Skipped word '{word}', already in dataset.")
            continue

        if isinstance(phonemes, dict):
            merged_data[word] = split_phonemes(phonemes["DEFAULT"])
        else:
            merged_data[word] = split_phonemes(phonemes)

# output data
with open(OUT_FILE_PATH, "w") as f:
    for word, p in merged_data.items():
        f.write(f"{word}\t{" ".join(p)}\n")

logger.info(f"Finished, written to {OUT_FILE_PATH}")
