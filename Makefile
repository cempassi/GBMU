RESSOURCES_DIR := ressources

BIOS_URL := "https://gbdev.gg8.se/files/roms/bootroms/"

BIOS_DIR += $(RESSOURCES_DIR)/bios

BIOS += "dmg_boot.bin"

ROMS_URL := "https://projects.intra.42.fr/uploads/document/document/4986/roms.zip"

ROMS_URL_3 := "https://romskingdom.com/en/download-roms/gbc-nintendo-gameboy-color/mary-kate-and-ashley-pocket-planner-usa/start?ajax=1&redirect=1"

ROM3_ZIP := "Mary-Kate and Ashley - Pocket Planner (USA, Europe).zip"
ROM3_NAME := "Mary-Kate and Ashley - Pocket Planner (USA, Europe).gbc"

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
	unzip -o $(ROM3_ZIP)
	mv $(ROM3_NAME) roms/$(ROM3_NAME)

roms.zip:
	curl -L $(ROMS_URL) > $@
	curl -L $(ROMS_URL_3) > $(ROM3_ZIP)

lint:
	cargo clippy --workspace --verbose -- -D warnings

format.check:
	cargo fmt --verbose -- --check

format:
	cargo fmt --all

clean:
	rm -rf roms roms.zip ressources "Ayakashi.zip" $(ROM3_ZIP)

.PHONY: requirement roms
