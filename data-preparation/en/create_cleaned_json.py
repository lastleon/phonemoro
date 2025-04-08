import json
import argparse
import logging
import coloredlogs
from pathlib import Path

# Create argument parser
parser = argparse.ArgumentParser(description="Script for cleaning the raw datasets and creating json files for use as lookup dicts.")
parser.add_argument("dataset_dir", default="data", type=str, help="Specify the input directory.")
parser.add_argument("-o", "--outdir", default="out", type=str, help="Specify the output directory.")
parser.add_argument("-v", "--verbose", action="store_true", help="Enable verbose logging.")

args = parser.parse_args()

# set up logger
logger = logging.getLogger("Data Preparation")
logger.setLevel(logging.DEBUG if args.verbose else logging.INFO)
coloredlogs.install(level="DEBUG" if args.verbose else "INFO", logger=logger)

# create output directory
INPUT_DIR = Path.resolve(Path(args.dataset_dir))
OUTPUT_DIR = Path.resolve(Path(args.outdir))

OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

# set up everything else
already_used_words = set()

# process us_{gold,silver}.json
for filename in ["us_gold.json", "us_silver.json"]:
    logger.info(f"Processing {filename}")

    # load dataset
    filepath = Path.joinpath(INPUT_DIR, filename)
    with open(filepath, "r") as f:
        word2ipa_src = json.load(f)

    # clean dataset
    word2ipa_cleaned = {}
    for word, val in word2ipa_src.items():
        if word in already_used_words:
            logger.debug(f"Word {word} already in higher ranked dataset, skipping.")
            continue

        if not isinstance(val, dict):
            # TODO: Maybe check for None, "", etc.
            word2ipa_cleaned[word] = val
        else:
            # check dict for invalid entries
            valid_entries = []
            for p_type, phonemes in val.items():
                if p_type == "None" or phonemes is None:  # These are checked because they were found in the dataset
                    continue
                valid_entries.append((p_type, phonemes))

            if len(valid_entries) == 0:
                logger.debug(f"Removed {word}: No valid phonemes.")
                continue
            elif len(valid_entries) == 1:
                word2ipa_cleaned[word] = valid_entries[0][1]
            else:
                word2ipa_cleaned[word] = dict(valid_entries)

        already_used_words.add(word)

    # save cleaned dataset to file, keep already used words to remove them from us_gold, cmudict, and possible other sources
    with open(Path.joinpath(OUTPUT_DIR, filepath.stem + ".processed.json"), "w") as f:
        json.dump(word2ipa_cleaned, f, ensure_ascii=False, indent=4)


logger.info(f"Finished. Total number of words: {len(already_used_words)}")
