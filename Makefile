RESSOURCES_PATH := ressources

BIOS_URL := "https://gbdev.gg8.se/files/roms/bootroms/"

BIOS_PATH += $(RESSOURCES_PATH)/bios

BIOS += "dmg_boot.bin"

ROMS_URL1 := "https://projects.intra.42.fr/uploads/document/document/4986/roms.zip"
ROMS_URL2 := "https://s3roms.download/romfiles/gameboy/a/ayakashi-no-shiro-japan.zip"

ROM2_ZIP := "Ayakashi.zip"
ROM2_NAME := "Ayakashi no Shiro (J) [!].gb"
ROM3_ZIP := "Mary-Kate and Ashley - Pocket Planner (USA, Europe).zip"
ROM3_NAME := "Mary-Kate and Ashley - Pocket Planner (USA, Europe).gbc"

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

requirements: roms fonts $(BIOS)

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
	curl $(BIOS_URL)/$@ > $</$@

$(BIOS_PATH):
	mkdir -p $@

### Roms ###

roms: roms.zip
	unzip -o $< 'roms/*' -x '*/.DS_Store'
	unzip -o $(ROM2_ZIP)
	mv $(ROM2_NAME) roms/Ayakashi.gb

roms.zip:
	curl -L $(ROMS_URL1) > $@
	curl $(ROMS_URL2) > $(ROM2_ZIP)

### utils ###

lint:
	cargo clippy --workspace --verbose -- -D warnings

format.all:
	cargo fmt --all

format:
	cargo fmt --verbose -- --check

clean:
	rm -rf roms.zip  $(ROM2_ZIP) $(ROM3_ZIP) $(HASKLIG_ZIP)

fclean: clean
	rm -rf roms ressources

.PHONY: requirement roms hasklig lint format.all format clean
