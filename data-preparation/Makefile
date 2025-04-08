.PHONY: en clean
all: en

release: en
	mkdir -p release/workdir/en
	cp en/out/* release/workdir/en  # copy artifacts
	cp en/data/LICENSE release/workdir/en  # copy LICENSE
	echo "# Attribution\nThese files are based on data from [hexgrad/Misaki](https://github.com/hexgrad/Misaki), licensed under the Apache License 2.0. The original Apache License 2.0 is provided within this directory." > release/workdir/en/README.md

	cd release/workdir && zip -r release.zip en/* && mv release.zip ..
	rm release/workdir -r

# English
en/out:
	cd en && make all
en: en/out

clean:
	cd en && make clean
