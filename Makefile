RESSOURCES_DIR := ressources

BIOS_URL := "https://gbdev.gg8.se/files/roms/bootroms/"

BIOS_DIR += $(RESSOURCES_DIR)/bios

BIOS += "dmg_boot.bin"

ROMS_URL1 := "https://projects.intra.42.fr/uploads/document/document/4986/roms.zip"
ROMS_URL2 := "https://s3roms.download/romfiles/gameboy/a/ayakashi-no-shiro-japan.zip"

ROM2_ZIP := "Ayakashi.zip"
ROM2_NAME := "Ayakashi no Shiro (J) [!].gb"

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
	unzip -o $(ROM2_ZIP)
	mv $(ROM2_NAME) roms/Ayakashi.gb

roms.zip:
	curl -L $(ROMS_URL1) > $@
	curl $(ROMS_URL2) > $(ROM2_ZIP)

lint:
	cargo clippy --workspace --verbose -- -D warnings

format.all:
	cargo fmt --all

format:
	cargo fmt --verbose -- --check

clean:
	rm -rf roms roms.zip ressources $(ROM2_ZIP)

.PHONY: requirement roms
