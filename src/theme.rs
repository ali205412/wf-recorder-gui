use iced::border::Radius;
use iced::widget::{button, container};
use iced::{Background, Border, Color, Shadow, Vector};

// Modern glass-morphism inspired color palette
#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    // Base colors - modern dark theme with glass effects
    pub background: Color,        // Main app background - deep dark
    pub surface: Color,           // Glass card backgrounds
    pub surface_hover: Color,     // Hover states with glow
    pub surface_active: Color,    // Active/selected states
    pub surface_elevated: Color,  // Elevated surfaces (modals, dropdowns)

    // Text colors - high contrast for accessibility
    pub text: Color,              // Primary text - pure white
    pub text_secondary: Color,    // Secondary text - soft gray
    pub text_muted: Color,        // Muted text - darker gray

    // Modern accent colors
    pub primary: Color,           // Electric blue primary
    pub primary_hover: Color,     // Brighter primary hover
    pub primary_light: Color,     // Light primary variant
    pub secondary: Color,         // Purple secondary accent
    pub success: Color,           // Green for positive actions
    pub warning: Color,           // Amber for warnings
    pub danger: Color,            // Red for destructive actions
    pub danger_hover: Color,      // Brighter danger hover

    // Glass effect colors
    pub glass_border: Color,      // Subtle glass borders
    pub glass_shadow: Color,      // Drop shadows
    pub gradient_start: Color,    // Gradient backgrounds start
    pub gradient_end: Color,      // Gradient backgrounds end
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            // Modern dark theme with rich colors
            background: Color::from_rgb(0.05, 0.05, 0.08),      // #0D0D14 - Deep space blue
            surface: Color::from_rgba(0.1, 0.1, 0.15, 0.8),     // Glass card effect
            surface_hover: Color::from_rgba(0.15, 0.15, 0.2, 0.9), // Glowing hover
            surface_active: Color::from_rgba(0.2, 0.25, 0.35, 0.95), // Active state
            surface_elevated: Color::from_rgba(0.12, 0.12, 0.18, 0.95), // Elevated surfaces

            // High contrast text for accessibility
            text: Color::from_rgb(0.98, 0.98, 1.0),            // #FAFAFF - Pure white
            text_secondary: Color::from_rgb(0.7, 0.72, 0.8),   // #B3B8CC - Soft gray
            text_muted: Color::from_rgb(0.5, 0.52, 0.6),       // #808599 - Muted gray

            // Modern vibrant accent colors
            primary: Color::from_rgb(0.0, 0.48, 1.0),          // #007AFF - Electric blue
            primary_hover: Color::from_rgb(0.2, 0.6, 1.0),     // #3399FF - Bright blue
            primary_light: Color::from_rgba(0.0, 0.48, 1.0, 0.15), // Blue tint
            secondary: Color::from_rgb(0.55, 0.27, 0.95),      // #8B44F2 - Purple accent
            success: Color::from_rgb(0.0, 0.78, 0.32),         // #00C851 - Modern green
            warning: Color::from_rgb(1.0, 0.6, 0.0),           // #FF9900 - Amber
            danger: Color::from_rgb(1.0, 0.27, 0.27),          // #FF4444 - Modern red
            danger_hover: Color::from_rgb(1.0, 0.4, 0.4),      // #FF6666 - Bright red

            // Glass morphism effects
            glass_border: Color::from_rgba(1.0, 1.0, 1.0, 0.1), // Subtle glass border
            glass_shadow: Color::from_rgba(0.0, 0.0, 0.0, 0.3), // Deep shadows
            gradient_start: Color::from_rgba(0.0, 0.48, 1.0, 0.1), // Blue gradient start
            gradient_end: Color::from_rgba(0.55, 0.27, 0.95, 0.1), // Purple gradient end
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
    pub const BASE_WINDOW_WIDTH: f32 = 480.0;
    pub const BASE_WINDOW_HEIGHT: f32 = 640.0;
    pub const MIN_WINDOW_WIDTH: f32 = 400.0;
    pub const MAX_WINDOW_WIDTH: f32 = 700.0;
    pub const MIN_WINDOW_HEIGHT: f32 = 520.0;
    pub const MAX_WINDOW_HEIGHT: f32 = 900.0;

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

    // Modern border radius - more rounded for glass effect
    pub const BORDER_RADIUS_SMALL: f32 = 12.0;   // Cards and buttons
    pub const BORDER_RADIUS_MEDIUM: f32 = 16.0;  // Large cards
    pub const BORDER_RADIUS_LARGE: f32 = 20.0;   // Modals and dialogs
    pub const BORDER_RADIUS_TINY: f32 = 6.0;     // Small elements
    #[allow(dead_code)]
    pub const BORDER_RADIUS_ROUND: f32 = 50.0;   // Fully rounded elements

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

        // Use the minimum scale but ensure it's not too small for usability
        let base_scale = width_scale.min(height_scale);

        // For smaller screens, use a slightly higher minimum to ensure UI is usable
        let adjusted_min = if screen_width < 1600.0 || screen_height < 900.0 {
            0.85
        } else {
            MIN_SCALE_FACTOR
        };

        base_scale.clamp(adjusted_min, MAX_SCALE_FACTOR)
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

// Modern glass window container with gradient background
pub struct WindowStyle(pub ColorPalette);

impl container::StyleSheet for WindowStyle {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.background)),
            text_color: Some(self.0.text),
            border: Border {
                color: self.0.glass_border,
                width: 1.0,
                radius: Radius::from(design::BORDER_RADIUS_LARGE),
            },
            shadow: Shadow {
                color: self.0.glass_shadow,
                offset: Vector::new(0.0, 8.0),
                blur_radius: 32.0,
            },
        }
    }
}

// Transparent container for content organization
pub struct ContainerStyle(pub ColorPalette);

impl container::StyleSheet for ContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: None, // Transparent for content flow
            text_color: Some(self.0.text),
            border: Border::default(),
            shadow: Shadow::default(),
        }
    }
}

// Modern glass card style for inputs and content areas
pub struct CardStyle(pub ColorPalette);

impl container::StyleSheet for CardStyle {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(self.0.surface)),
            text_color: Some(self.0.text),
            border: Border {
                color: self.0.glass_border,
                width: 1.0,
                radius: Radius::from(design::BORDER_RADIUS_SMALL),
            },
            shadow: Shadow {
                color: self.0.glass_shadow,
                offset: Vector::new(0.0, 4.0),
                blur_radius: 16.0,
            },
        }
    }
}

// Legacy alias for compatibility
pub type SearchStyle = CardStyle;

// Modern option card button with glass effect
pub struct OptionCardStyle(pub ColorPalette, pub bool); // bool for selected state

impl button::StyleSheet for OptionCardStyle {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        if self.1 {
            // Selected state with vibrant accent
            button::Appearance {
                background: Some(Background::Color(self.0.primary_light)),
                text_color: self.0.text,
                border: Border {
                    color: self.0.primary,
                    width: 2.0,
                    radius: Radius::from(design::BORDER_RADIUS_SMALL),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.48, 1.0, 0.3),
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 12.0,
                },
                shadow_offset: Vector::new(0.0, 0.0),
            }
        } else {
            // Normal card state
            button::Appearance {
                background: Some(Background::Color(self.0.surface)),
                text_color: self.0.text,
                border: Border {
                    color: self.0.glass_border,
                    width: 1.0,
                    radius: Radius::from(design::BORDER_RADIUS_SMALL),
                },
                shadow: Shadow {
                    color: self.0.glass_shadow,
                    offset: Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
                shadow_offset: Vector::new(0.0, 0.0),
            }
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        if self.1 {
            // Selected hover - brighter glow
            button::Appearance {
                background: Some(Background::Color(self.0.primary_light)),
                text_color: self.0.text,
                border: Border {
                    color: self.0.primary_hover,
                    width: 2.0,
                    radius: Radius::from(design::BORDER_RADIUS_SMALL),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.48, 1.0, 0.5),
                    offset: Vector::new(0.0, 6.0),
                    blur_radius: 20.0,
                },
                shadow_offset: Vector::new(0.0, 0.0),
            }
        } else {
            // Normal hover with subtle glow
            button::Appearance {
                background: Some(Background::Color(self.0.surface_hover)),
                text_color: self.0.text,
                border: Border {
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.2),
                    width: 1.0,
                    radius: Radius::from(design::BORDER_RADIUS_SMALL),
                },
                shadow: Shadow {
                    color: self.0.glass_shadow,
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 16.0,
                },
                shadow_offset: Vector::new(0.0, 0.0),
            }
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}

// Legacy alias for compatibility
pub type RowStyle = OptionCardStyle;

// Modern primary button with vibrant gradient and glow
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
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.48, 1.0, 0.4),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 16.0,
            },
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
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.48, 1.0, 0.6),
                offset: Vector::new(0.0, 6.0),
                blur_radius: 24.0,
            },
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}

// Modern secondary button with glass effect
pub struct SecondaryButton(pub ColorPalette);

impl button::StyleSheet for SecondaryButton {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.0.surface)),
            text_color: self.0.text,
            border: Border {
                color: self.0.glass_border,
                width: 1.0,
                radius: Radius::from(design::BORDER_RADIUS_SMALL),
            },
            shadow: Shadow {
                color: self.0.glass_shadow,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
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
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.48, 1.0, 0.2),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.hovered(style)
    }
}

// Modern danger button with vibrant red glow
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
            shadow: Shadow {
                color: Color::from_rgba(1.0, 0.27, 0.27, 0.4),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 16.0,
            },
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn hovered(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(self.0.danger_hover)),
            text_color: Color::WHITE,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(design::BORDER_RADIUS_SMALL),
            },
            shadow: Shadow {
                color: Color::from_rgba(1.0, 0.27, 0.27, 0.6),
                offset: Vector::new(0.0, 6.0),
                blur_radius: 24.0,
            },
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}

// Modern compact floating window with enhanced glass effect
pub struct CompactStyle(pub ColorPalette);

impl container::StyleSheet for CompactStyle {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgba(
                self.0.surface_elevated.r,
                self.0.surface_elevated.g,
                self.0.surface_elevated.b,
                0.95, // High transparency for floating effect
            ))),
            text_color: Some(self.0.text),
            border: Border {
                color: self.0.glass_border,
                width: 1.0,
                radius: Radius::from(design::BORDER_RADIUS_ROUND),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                offset: Vector::new(0.0, 8.0),
                blur_radius: 32.0,
            },
        }
    }
}

// Modern compact button with subtle glass effect
pub struct CompactButton(pub ColorPalette);

impl button::StyleSheet for CompactButton {
    type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Background::Color(Color::from_rgba(
                self.0.danger.r,
                self.0.danger.g,
                self.0.danger.b,
                0.95,
            ))),
            text_color: Color::WHITE,
            border: Border {
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.3),
                width: 1.0,
                radius: Radius::from(design::BORDER_RADIUS_ROUND),
            },
            shadow: Shadow {
                color: Color::from_rgba(1.0, 0.27, 0.27, 0.3),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
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
                radius: Radius::from(design::BORDER_RADIUS_ROUND),
            },
            shadow: Shadow {
                color: Color::from_rgba(1.0, 0.27, 0.27, 0.5),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
            shadow_offset: Vector::new(0.0, 0.0),
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.hovered(style)
    }
}

// Modern recording indicator with vibrant glow effect
pub struct RecordingIndicator(pub ColorPalette);

impl container::StyleSheet for RecordingIndicator {
    type Style = iced::Theme;

    fn appearance(&self, _: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgba(
                self.0.surface.r,
                self.0.surface.g,
                self.0.surface.b,
                0.9, // Glass-morphism background
            ))),
            text_color: Some(self.0.text),
            border: Border {
                color: self.0.glass_border,
                width: 1.0,
                radius: Radius::from(design::BORDER_RADIUS_ROUND),
            },
            shadow: Shadow {
                color: Color::from_rgba(1.0, 0.27, 0.27, 0.4),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 16.0,
            },
        }
    }
}
