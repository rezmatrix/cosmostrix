// Copyright (c) 2025 rezk_nightky

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorMode {
    Mono,
    Color16,
    Color256,
    TrueColor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShadingMode {
    Random,
    DistanceFromHead,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoldMode {
    Off,
    Random,
    All,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorScheme {
    User,
    Green,
    Green2,
    Green3,
    Yellow,
    Orange,
    Red,
    Blue,
    Cyan,
    Gold,
    Rainbow,
    Purple,
    Pink,
    Pink2,
    Vaporwave,
    Gray,
}

#[derive(Clone, Debug)]
pub struct UserColor {
    pub index: u8,
    pub rgb_1000: Option<(u16, u16, u16)>,
}

#[derive(Clone, Debug)]
pub struct UserColors {
    pub colors: Vec<UserColor>,
}
