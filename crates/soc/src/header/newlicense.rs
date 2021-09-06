use num_enum::TryFromPrimitive;
use super::consts::NEW_LICENSE_LEN;
use super::error::Error;
use std::convert::TryFrom;
use std::u8;

#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum NewLicense {
    None = 0x00,
    NintendoRnD1 = 0x01,
    Capcom = 0x08,
    #[num_enum(alternatives = [0x69])]
    ElectronicArts = 0x13,
    HudsonSoft = 0x18,
    BAi = 0x19,
    Kss = 0x20,
    Pow = 0x22,
    PCMComplete = 0x24,
    SanX = 0x25,
    KemcoJapan = 0x28,
    Seta = 0x29,
    Viacom = 0x30,
    Nintendo = 0x31,
    Bandai = 0x32,
    #[num_enum(alternatives = [0x93])]
    OceanAcclaim = 0x33,
    #[num_enum(alternatives = [0x54, 0xA4])]
    Konami = 0x34,
    Hector = 0x35,
    Taito = 0x37,
    Hudson = 0x38,
    Banpresto = 0x39,
    UbiSoft = 0x41,
    Atlus = 0x42,
    Malibu = 0x44,
    Angel = 0x46,
    BulletRoof = 0x47,
    Irem = 0x49,
    Absolute = 0x50,
    Acclaim = 0x51,
    Activision = 0x52,
    AmericanSammy = 0x53,
    HiTechEntertainment = 0x55,
    LJN = 0x56,
    Matchbox = 0x57,
    Mattel = 0x58,
    MiltonBradley = 0x59,
    Titus = 0x60,
    Virgin = 0x61,
    LucasArts = 0x64,
    Ocean = 0x67,
    Infogrames = 0x70,
    Interplay = 0x71,
    Broderbund = 0x72,
    Sculptured = 0x73,
    Sci = 0x75,
    THQ = 0x78,
    Accolade = 0x79,
    Misawa = 0x80,
    Lozc = 0x83,
    TokumaShotenIntermedia = 0x86,
    TsukudaOriginal = 0x87,
    Chunsoft = 0x91,
    VideoSystem = 0x92,
    Varie = 0x95,
    YonezawaSpal = 0x96,
    Kaneko = 0x97,
    PackInSoft = 0x99,
}

impl TryFrom<[u8; NEW_LICENSE_LEN]> for NewLicense {
    type Error = Error;

    fn try_from(raw_license: [u8; NEW_LICENSE_LEN]) -> Result<Self, Self::Error> {
        let s = String::from_utf8(raw_license.into())?;
        if s == "\0\0" {
            return Ok(NewLicense::None);
        };
        let license = u8::from_str_radix(&s, 16)?;
        if let Ok(code) = NewLicense::try_from(license) {
            Ok(code)
        } else {
           Err(Error::InvalidNewLicense(s))
        }
    }
}
