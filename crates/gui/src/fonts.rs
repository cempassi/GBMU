use iced_winit::Font;

pub const HASKLIG_BOLD : Font = Font::External{
    name: "Hasklig Bold",
    bytes: include_bytes!("../../../ressources/fonts/Hasklig/Hasklig-Bold.ttf"),
};

pub const HASKLIG_LIGHT : Font = Font::External{
    name: "Hasklig light",
    bytes: include_bytes!("../../../ressources/fonts/Hasklig/Hasklig-Light.ttf"),
};
