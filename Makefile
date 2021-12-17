RESSOURCES_PATH := ressources

BIOS_URL := "https://gbdev.gg8.se/files/roms/bootroms/"

BIOS_PATH += $(RESSOURCES_PATH)/bios

BIOS += "dmg_boot.bin"

ROMS_URL := "https://projects.intra.42.fr/uploads/document/document/4986/roms.zip"

# Fonts
FONT_PATH := $(RESSOURCES_PATH)/fonts

## Hasklig
HASKLIG_URL := "https://github.com/i-tu/Hasklig/releases/download/v1.2/Hasklig-1.2.zip"
HASKLIG_PATH := $(FONT_PATH)/hasklig
HASKLIG_ZIP := "Hasklig-1.2.zip"

ROMS_PATH := roms

ROMS := $(ROMS_PATH)/$(_ROMS)

### Base ###

all: requirements clean

requirements: roms blarg fonts $(BIOS)

### Fonts ###

fonts: hasklig

### Hasklig ###

hasklig: $(HASKLIG_ZIP)
	unzip -o $<
	rm -rf ./__MACOSX OTF
	mv TTF/* $(HASKLIG_PATH)
	rm -rf TTF

$(HASKLIG_ZIP): $(HASKLIG_PATH)
	curl -L $(HASKLIG_URL) > $@

$(HASKLIG_PATH):
	mkdir -p $@

### Bios ###

$(BIOS): $(BIOS_PATH)
	curl -k $(BIOS_URL)/$@ > $</$@

$(BIOS_PATH):
	mkdir -p $@

### Roms ###

roms: roms.zip
	unzip -o $< 'roms/*' -x '*/.DS_Store'

roms.zip:
	curl -L $(ROMS_URL) > $@

blarg:
	git clone git@github.com:retrio/gb-test-roms.git ./ressources/test_roms

### utils ###

check: format.all format lint test

lint:
	cargo clippy --workspace --verbose -- -D warnings

format.all:
	cargo fmt --all

format:
	cargo fmt --verbose -- --check

test:
	cargo test

clean:
	rm -rf roms.zip  $(HASKLIG_ZIP)

fclean: clean
	cargo clean
	cargo cache -a
	rm -rf roms ressources

.PHONY: requirement roms hasklig check lint format.all format clean fclean
