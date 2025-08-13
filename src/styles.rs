#[allow(dead_code)]
pub mod colors {
    use bevy::color::Srgba;
    use bevy::color::palettes::tailwind;

    pub const BASE_100: Srgba = tailwind::GRAY_100;
    pub const BASE_200: Srgba = tailwind::GRAY_200;
    pub const BASE_300: Srgba = tailwind::GRAY_300;
    pub const NEUTRAL: Srgba = tailwind::GRAY_500;
    pub const NEUTRAL_CONTENT: Srgba = tailwind::GRAY_900;
    pub const PRIMARY: Srgba = tailwind::BLUE_500;
    pub const PRIMARY_CONTENT: Srgba = tailwind::BLUE_100;
    pub const SECONDARY: Srgba = tailwind::PURPLE_500;
    pub const SECONDARY_CONTENT: Srgba = tailwind::PURPLE_100;
    pub const ACCENT: Srgba = tailwind::GREEN_500;
    pub const ACCENT_CONTENT: Srgba = tailwind::GREEN_100;
    pub const INFO: Srgba = tailwind::CYAN_500;
    pub const INFO_CONTENT: Srgba = tailwind::CYAN_100;
    pub const SUCCESS: Srgba = tailwind::GREEN_500;
    pub const SUCCESS_CONTENT: Srgba = tailwind::GREEN_100;
    pub const WARNING: Srgba = tailwind::YELLOW_500;
    pub const WARNING_CONTENT: Srgba = tailwind::YELLOW_100;
    pub const ERROR: Srgba = tailwind::RED_500;
    pub const ERROR_CONTENT: Srgba = tailwind::RED_100;
}
