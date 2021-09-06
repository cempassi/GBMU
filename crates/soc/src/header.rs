mod cartridge;
mod consts;
mod destination;
mod error;
mod flag;
mod newlicense;
mod oldlicense;
mod size;
mod title;

use cartridge::Cartridge;
use consts::{ENTRY_LEN, LOGO_LEN, NEW_LICENSE_LEN, TITLE_LEN};
use destination::Destination;
use error::Error;
use flag::Sgb;
use newlicense::NewLicense;
use oldlicense::OldLicense;
use std::convert::{TryFrom, TryInto};
use title::Title;

/// entry_point: After displaying the Nintendo Logo, the built-in boot procedure jumps to this address
/// logo: bitmap of the Nintendo logo
#[derive(Debug, PartialEq, Eq)]
pub struct Header {
    pub entry_point: [u8; ENTRY_LEN],
    pub logo: [u8; LOGO_LEN],
    pub title: Title,
    pub new_license: NewLicense,
    pub sgb_flag: Sgb,
    pub cartridge: Cartridge,
    pub rom_size: size::Rom,
    pub ram_size: size::Ram,
    pub destination: Destination,
    pub old_license: OldLicense,
    pub rom_version: u8,
    pub header_checksum: u8,
    pub global_checksum: u16,
}

fn extract<T, const N: usize>(v: &mut Vec<T>) -> [T; N] {
    v.drain(..N)
        .collect::<Vec<T>>()
        .try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

impl TryFrom<Vec<u8>> for Header {
    type Error = Error;

    fn try_from(mut raw_header: Vec<u8>) -> Result<Self, Self::Error> {
        let entry_point: [u8; ENTRY_LEN] = extract(&mut raw_header);
        let logo: [u8; LOGO_LEN] = extract(&mut raw_header);
        let title: Title = Title::try_from(extract::<u8, TITLE_LEN>(&mut raw_header))?;
        let new_license: NewLicense =
            NewLicense::try_from(extract::<u8, NEW_LICENSE_LEN>(&mut raw_header))?;
        let sgb_flag: Sgb = Sgb::try_from(raw_header.remove(0)).unwrap();
        let cartridge: Cartridge = Cartridge::try_from(raw_header.remove(0)).unwrap();
        let rom_size: size::Rom = size::Rom::try_from(raw_header.remove(0)).unwrap();
        let ram_size: size::Ram = size::Ram::try_from(raw_header.remove(0)).unwrap();
        let destination: Destination = Destination::try_from(raw_header.remove(0)).unwrap();
        let old_license: OldLicense = OldLicense::try_from(raw_header.remove(0)).unwrap();
        let rom_version = raw_header.remove(0);
        let header_checksum = raw_header.remove(0);
        let global_checksum = (raw_header.remove(0) as u16) << 8 | raw_header.remove(0) as u16;

        Ok(Header {
            entry_point,
            logo,
            title,
            new_license,
            sgb_flag,
            cartridge,
            rom_size,
            ram_size,
            destination,
            old_license,
            rom_version,
            header_checksum,
            global_checksum,
        })
    }
}
