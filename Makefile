RESSOURCES_DIR := ressources

BIOS_URL := "https://gbdev.gg8.se/files/roms/bootroms/"

BIOS_DIR += $(RESSOURCES_DIR)/bios

BIOS += "dmg_boot.bin"

ROMS_URL := "https://projects.intra.42.fr/uploads/document/document/4986/roms.zip"

ROMS_DIR := roms

ROMS := $(ROMS_DIR)/$(_ROMS)

all: requirements

requirements: roms $(BIOS)

$(BIOS): $(BIOS_DIR)
	curl $(BIOS_URL)/$@ > $</$@

$(BIOS_DIR):
	mkdir -p $@

roms: roms.zip
	unzip -o $< 'roms/*' -x '*/.DS_Store'

roms.zip:
	curl -L $(ROMS_URL) > $@

lint:
	cargo clippy --workspace --verbose -- -D warnings

format.check:
	cargo fmt --verbose -- --check

format:
	cargo fmt --all

clean:
	rm -rf roms roms.zip ressources

.PHONY: requirement roms
