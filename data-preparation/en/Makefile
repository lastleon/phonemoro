all: out/model.fst out/us_gold.json out/us_silver.json

# Retrieves data needed for training Phonetisaurus model and lookup dictionary in phonemizer
data:
	bash pull_data_dict.sh

# Processes retrieved data (remove NULL, duplicates, ...)
out/us_gold.json out/us_silver.json: data
	uv run create_cleaned_json.py data -o out/ -v
	mv out/us_gold.processed.json out/us_gold.json
	mv out/us_silver.processed.json out/us_silver.json

# Create training lexicon for Phonetisaurus from retrieved and processed data
out/trainingLexicon.dict: out/us_gold.json out/us_silver.json
	uv run create_fst_training_lexicon.py out/us_gold.json,out/us_silver.json -o out/trainingLexicon.dict -v

# Train Phonetisaurus model
CURRENT_UID_GID= $(shell id -u):$(shell id -g)
out/model.fst: out/trainingLexicon.dict
	docker run --rm -it \
	   -v $$PWD:/work phonetisaurus/phonetisaurus \
		"phonetisaurus-train -v --dir_prefix out/fst --lexicon out/trainingLexicon.dict --seq1_del --seq2_del && chown -R ${CURRENT_UID_GID} out/fst"
	mv out/fst/model.fst out/
	rm out/fst -r
	rm out/trainingLexicon.dict

.PHONY: clean
clean:
	rm -r out/
