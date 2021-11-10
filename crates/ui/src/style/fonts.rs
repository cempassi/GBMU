use iced::Font;

pub const HASKLIG_BOLD: Font = Font::External {
    name: "Hasklig Bold",
    bytes: include_bytes!("../../../../ressources/fonts/hasklig/Hasklig-Bold.ttf"),
};

pub const HASKLIG_LIGHT: Font = Font::External {
    name: "Hasklig light",
    bytes: include_bytes!("../../../../ressources/fonts/hasklig/Hasklig-Light.ttf"),
};

pub const HASKLIG_MEDIUM: Font = Font::External {
    name: "Hasklig light",
    bytes: include_bytes!("../../../../ressources/fonts/hasklig/Hasklig-Medium.ttf"),
};

pub const HASKLIG_MEDIUM_IT: Font = Font::External {
    name: "Hasklig light",
    bytes: include_bytes!("../../../../ressources/fonts/hasklig/Hasklig-MediumIt.ttf"),
};
