use iced::border::Radius;
use iced::widget::{button, container};
use iced::{Background, Border, Color, Shadow, Vector};

// Onagre-inspired color palette
#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    // Base colors matching onagre's modern theme
    pub background: Color,     // Main app background - dark
    pub surface: Color,        // Container/search background
    pub surface_hover: Color,  // Row hover state
    pub surface_active: Color, // Row selected state

    // Text colors
    pub text: Color,           // Primary text
    pub text_secondary: Color, // Secondary/muted text

    // Accent colors
    pub primary: Color,       // Primary actions
    pub primary_hover: Color, // Primary hover
    pub danger: Color,        // Stop/danger actions

    // Borders
    pub border: Color, // Container borders
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            // Onagre-style dark theme
            background: Color::from_rgb(0.094, 0.094, 0.118), // #181820 - Dark background
            surface: Color::from_rgb(0.141, 0.141, 0.172),    // #242430 - Surface
            surface_hover: Color::from_rgb(0.188, 0.188, 0.235), // #303040 - Hover
            surface_active: Color::from_rgb(0.251, 0.557, 0.969), // #4080F7 - Selected/active

            // Text
            text: Color::from_rgb(0.925, 0.937, 0.957), // #ECF0F5 - Light text
            text_secondary: Color::from_rgb(0.596, 0.608, 0.631), // #989BA1 - Muted text

            // Accent colors
            primary: Color::from_rgb(0.251, 0.557, 0.969), // #4080F7 - Blue
            primary_hover: Color::from_rgb(0.314, 0.620, 1.0), // #509EFF - Lighter blue
            danger: Color::from_rgb(0.957, 0.263, 0.212),  // #F44336 - Red

            // Borders
            border: Color::from_rgba(1.0, 1.0, 1.0, 0.06), // Subtle border
        }
    }
}

// Dynamic, responsive design constants
pub mod design {
    // Base reference resolution (1920x1080)
    pub const REFERENCE_WIDTH: f32 = 1920.0;
    pub const REFERENCE_HEIGHT: f32 = 1080.0;

    // Scaling limits
    pub const MIN_SCALE_FACTOR: f32 = 0.7;
    pub const MAX_SCALE_FACTOR: f32 = 1.5;

    // Window sizing (responsive)
    pub const BASE_WINDOW_WIDTH: f32 = 420.0;
    pub const BASE_WINDOW_HEIGHT: f32 = 520.0;
    pub const MIN_WINDOW_WIDTH: f32 = 320.0;
    pub const MAX_WINDOW_WIDTH: f32 = 600.0;
    pub const MIN_WINDOW_HEIGHT: f32 = 400.0;
    pub const MAX_WINDOW_HEIGHT: f32 = 800.0;

    // Compact window sizing
    pub const COMPACT_WINDOW_WIDTH: f32 = 180.0;
    pub const COMPACT_WINDOW_HEIGHT: f32 = 50.0;
    pub const COMPACT_WINDOW_PADDING: f32 = 20.0;

    // Spacing (responsive)
    pub const BASE_WINDOW_PADDING: u16 = 20;
    pub const BASE_CONTAINER_PADDING: u16 = 12;
    pub const BASE_SECTION_SPACING: u16 = 16;
    #[allow(dead_code)]
    pub const BASE_ELEMENT_SPACING: u16 = 8;

    // Border radius
    pub const BORDER_RADIUS_SMALL: f32 = 8.0;
    pub const BORDER_RADIUS_TINY: f32 = 4.0;
    #[allow(dead_code)]
    pub const BORDER_RADIUS_ROUND: f32 = 25.0;

    // Button sizing (responsive)
    pub const BASE_BUTTON_HEIGHT: u16 = 56;
    pub const BASE_BUTTON_WIDTH: u16 = 110;
    pub const BASE_BUTTON_PADDING_V: u16 = 16;
    pub const BASE_BUTTON_PADDING_H: u16 = 20;
    pub const COMPACT_BUTTON_PADDING: u16 = 6;

    // Text sizes (responsive)
    pub const BASE_TITLE_SIZE: u16 = 24;
    pub const BASE_SUBTITLE_SIZE: u16 = 14;
    pub const BASE_LABEL_SIZE: u16 = 11;
    pub const BASE_BUTTON_TEXT_SIZE: u16 = 14;
    pub const BASE_INPUT_TEXT_SIZE: u16 = 16;
    #[allow(dead_code)]
    pub const BASE_COMPACT_TEXT_SIZE: u16 = 12;
    pub const BASE_TIMER_TEXT_SIZE: u16 = 18;
    pub const BASE_COUNTDOWN_SIZE: u16 = 72;
    pub const BASE_RECORDING_SIZE: u16 = 56;
    pub const COMPACT_COUNTDOWN_SIZE: u16 = 28;
    pub const COMPACT_ICON_SIZE: u16 = 16;

    // Container sizes (responsive)
    pub const BASE_COUNTDOWN_CONTAINER: f32 = 120.0;
    pub const BASE_VERTICAL_SPACE: f32 = 32.0;
    pub const BASE_SMALL_SPACE: f32 = 8.0;
    pub const BASE_TINY_SPACE: f32 = 4.0;

    // Helper functions for responsive sizing
    pub fn scale_factor(screen_width: f32, screen_height: f32) -> f32 {
        let width_scale = screen_width / REFERENCE_WIDTH;
        let height_scale = screen_height / REFERENCE_HEIGHT;
        width_scale
            .min(height_scale)
            .clamp(MIN_SCALE_FACTOR, MAX_SCALE_FACTOR)
    }

    pub fn scaled_size(base_size: u16, scale: f32) -> u16 {
        (base_size as f32 * scale).round() as u16
    }

    pub fn scaled_f32(base_size: f32, scale: f32) -> f32 {
        base_size * scale
    }

    // Dynamic constants based on scale factor
    pub fn window_padding(scale: f32) -> u16 {
        scaled_size(BASE_WINDOW_PADDING, scale)
    }
    pub fn container_padding(scale: f32) -> u16 {
        scaled_size(BASE_CONTAINER_PADDING, scale)
    }
    pub fn section_spacing(scale: f32) -> u16 {
        scaled_size(BASE_SECTION_SPACING, scale)
    }
    pub fn button_height(scale: f32) -> u16 {
        scaled_size(BASE_BUTTON_HEIGHT, scale)
    }
    pub fn button_width(scale: f32) -> u16 {
        scaled_size(BASE_BUTTON_WIDTH, scale)
    }
    pub fn button_padding_v(scale: f32) -> u16 {
        scaled_size(BASE_BUTTON_PADDING_V, scale)
    }
    pub fn button_padding_h(scale: f32) -> u16 {
        scaled_size(BASE_BUTTON_PADDING_H, scale)
    }

    // Text sizes
    pub fn title_size(scale: f32) -> u16 {
        scaled_size(BASE_TITLE_SIZE, scale)
    }
    pub fn subtitle_size(scale: f32) -> u16 {
        scaled_size(BASE_SUBTITLE_SIZE, scale)
    }
    pub fn label_size(scale: f32) -> u16 {
        scaled_size(BASE_LABEL_SIZE, scale)
    }
    pub fn button_text_size(scale: f32) -> u16 {
        scaled_size(BASE_BUTTON_TEXT_SIZE, scale)
    }
    pub fn input_text_size(scale: f32) -> u16 {
        scaled_size(BASE_INPUT_TEXT_SIZE, scale)
    }
    #[allow(dead_code)]
    pub fn compact_text_size(scale: f32) -> u16 {
        scaled_size(BASE_COMPACT_TEXT_SIZE, scale)
    }
    pub fn timer_text_size(scale: f32) -> u16 {
        scaled_size(BASE_TIMER_TEXT_SIZE, scale)
    }
    pub fn countdown_size(scale: f32) -> u16 {
        scaled_size(BASE_COUNTDOWN_SIZE, scale)
    }
    pub fn recording_size(scale: f32) -> u16 {
        scaled_size(BASE_RECORDING_SIZE, scale)
    }
    pub fn compact_countdown_size(scale: f32) -> u16 {
        scaled_size(COMPACT_COUNTDOWN_SIZE, scale)
    }

    // Spacing
    pub fn vertical_space(scale: f32) -> f32 {
        scaled_f32(BASE_VERTICAL_SPACE, scale)
    }
    pub fn small_space(scale: f32) -> f32 {
        scaled_f32(BASE_SMALL_SPACE, scale)
    }
    pub fn tiny_space(scale: f32) -> f32 {
        scaled_f32(BASE_TINY_SPACE, scale)
    }
    pub fn countdown_container(scale: f32) -> f32 {
        scaled_f32(BASE_COUNTDOWN_CONTAINER, scale)
    }
}

#[derive(Default)]
pub struct Theme {
    pub colors: ColorPalette,
}

// Main window container - like onagre's .onagre class
pub struct WindowStyle(pub ColorPalette);

impl container::StyleSheet for WindowStyle {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.background)),
            text_color: Some(self.0.text),
            border: Border {
                color: self.0.background,   // Same as background for seamless look
                width: 4.0,                 // Like onagre's border-width: 4px
                radius: Radius::from(20.0), // Rounded window
            },
            shadow: Shadow::default(),
        }
    }
}

// Container style - like onagre's .container
pub struct ContainerStyle(pub ColorPalette);

impl container::StyleSheet for ContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: None, // Transparent like onagre
            text_color: Some(self.0.text),
            border: Border::default(),
            shadow: Shadow::default(),
        }
    }
}

// Search/input container - like onagre's .search
pub struct SearchStyle(pub ColorPalette);

impl container::StyleSheet for SearchStyle {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.surface)),
            text_color: Some(self.0.text),
            border: Border {
                color: self.0.border,
                width: 0.0,
                radius: Radius::from(design::BORDER_RADIUS_SMALL),
            },
            shadow: Shadow::default(),
        }
    }
}

// Row style - like onagre's .row
pub struct RowStyle(pub ColorPalette, pub bool); // bool for selected state

impl button::StyleSheet for RowStyle {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        if self.1 {
            // Selected state - like .row-selected
            button::Appearance {
                background: Some(Background::Color(self.0.surface_active)),
                text_color: Color::WHITE,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(design::BORDER_RADIUS_SMALL),
                },
                shadow: Shadow::default(),
                shadow_offset: Vector::new(0.0, 0.0),
            }
        } else {
            // Normal row
            button::Appearance {
                background: None, // Transparent background
                text_color: self.0.text,
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::from(design::BORDER_RADIUS_SMALL),
                },
                shadow: Shadow::default(),
                shadow_offset: Vector::new(0.0, 0.0),
            }
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.0.surface_hover)),
            text_color: self.0.text,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(design::BORDER_RADIUS_SMALL),
            },
            shadow: Shadow::default(),
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}

// Primary button - for main actions
pub struct PrimaryButton(pub ColorPalette);

impl button::StyleSheet for PrimaryButton {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.0.primary)),
            text_color: Color::WHITE,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(design::BORDER_RADIUS_SMALL),
            },
            shadow: Shadow::default(),
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.0.primary_hover)),
            text_color: Color::WHITE,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(design::BORDER_RADIUS_SMALL),
            },
            shadow: Shadow::default(),
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}

// Secondary button style
pub struct SecondaryButton(pub ColorPalette);

impl button::StyleSheet for SecondaryButton {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.0.surface)),
            text_color: self.0.text,
            border: Border {
                color: self.0.border,
                width: 1.0,
                radius: Radius::from(design::BORDER_RADIUS_SMALL),
            },
            shadow: Shadow::default(),
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.0.surface_hover)),
            text_color: self.0.text,
            border: Border {
                color: self.0.primary,
                width: 1.0,
                radius: Radius::from(design::BORDER_RADIUS_SMALL),
            },
            shadow: Shadow::default(),
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.hovered(style)
    }
}

// Danger button
pub struct DangerButton(pub ColorPalette);

impl button::StyleSheet for DangerButton {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.0.danger)),
            text_color: Color::WHITE,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(design::BORDER_RADIUS_SMALL),
            },
            shadow: Shadow::default(),
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgb(1.0, 0.32, 0.24))), // Lighter red
            text_color: Color::WHITE,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(design::BORDER_RADIUS_SMALL),
            },
            shadow: Shadow::default(),
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}

// Compact container style - minimal padding and rounded corners for small UI
pub struct CompactStyle(pub ColorPalette);

impl container::StyleSheet for CompactStyle {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgba(
                self.0.surface.r,
                self.0.surface.g,
                self.0.surface.b,
                0.95, // Slightly transparent
            ))),
            text_color: Some(self.0.text),
            border: Border {
                color: self.0.border,
                width: 1.0,
                radius: Radius::from(design::BORDER_RADIUS_TINY),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
        }
    }
}

// Compact button style - tiny buttons for minimal UI
pub struct CompactButton(pub ColorPalette);

impl button::StyleSheet for CompactButton {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgba(
                self.0.danger.r,
                self.0.danger.g,
                self.0.danger.b,
                0.8,
            ))),
            text_color: Color::WHITE,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(design::BORDER_RADIUS_TINY),
            },
            shadow: Shadow::default(),
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.0.danger)),
            text_color: Color::WHITE,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(design::BORDER_RADIUS_TINY),
            },
            shadow: Shadow::default(),
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.hovered(style)
    }
}

// Recording indicator style - pulsing red dot effect
pub struct RecordingIndicator(pub ColorPalette);

impl container::StyleSheet for RecordingIndicator {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgba(
                self.0.danger.r,
                self.0.danger.g,
                self.0.danger.b,
                0.1, // Very subtle background
            ))),
            text_color: Some(self.0.text),
            border: Border {
                color: Color::from_rgba(self.0.danger.r, self.0.danger.g, self.0.danger.b, 0.3),
                width: 1.0,
                radius: Radius::from(25.0), // Rounded like a pill
            },
            shadow: Shadow::default(),
        }
    }
}
