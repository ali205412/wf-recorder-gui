use iced::{Background, Border, Color, Shadow, Vector};
use iced::widget::{button, container, text};
use iced::border::Radius;

// Onagre-inspired color palette
#[derive(Debug, Clone, Copy)]
pub struct ColorPalette {
    // Base colors matching onagre's modern theme
    pub background: Color,         // Main app background - dark
    pub surface: Color,           // Container/search background
    pub surface_hover: Color,     // Row hover state
    pub surface_active: Color,    // Row selected state
    
    // Text colors
    pub text: Color,              // Primary text
    pub text_secondary: Color,    // Secondary/muted text
    
    // Accent colors
    pub primary: Color,           // Primary actions
    pub primary_hover: Color,     // Primary hover
    pub danger: Color,            // Stop/danger actions
    
    // Borders
    pub border: Color,            // Container borders
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            // Onagre-style dark theme
            background: Color::from_rgb(0.094, 0.094, 0.118),      // #181820 - Dark background
            surface: Color::from_rgb(0.141, 0.141, 0.172),         // #242430 - Surface
            surface_hover: Color::from_rgb(0.188, 0.188, 0.235),   // #303040 - Hover
            surface_active: Color::from_rgb(0.251, 0.557, 0.969),  // #4080F7 - Selected/active
            
            // Text
            text: Color::from_rgb(0.925, 0.937, 0.957),            // #ECF0F5 - Light text
            text_secondary: Color::from_rgb(0.596, 0.608, 0.631),  // #989BA1 - Muted text
            
            // Accent colors
            primary: Color::from_rgb(0.251, 0.557, 0.969),         // #4080F7 - Blue
            primary_hover: Color::from_rgb(0.314, 0.620, 1.0),     // #509EFF - Lighter blue
            danger: Color::from_rgb(0.957, 0.263, 0.212),          // #F44336 - Red
            
            // Borders
            border: Color::from_rgba(1.0, 1.0, 1.0, 0.06),         // Subtle border
        }
    }
}

// Onagre-style design constants
pub mod design {
    // Exact values from onagre themes
    pub const WINDOW_PADDING: u16 = 20;
    pub const CONTAINER_PADDING: u16 = 12;
    pub const SECTION_SPACING: u16 = 16;
    pub const BORDER_RADIUS: f32 = 12.0;  // For containers
    pub const BORDER_RADIUS_SMALL: f32 = 8.0;  // For buttons
    pub const BORDER_WIDTH: f32 = 0.0;  // No borders by default
    
    // Row/button sizing
    pub const BUTTON_HEIGHT: u16 = 56;
    pub const BUTTON_WIDTH: u16 = 110;
    pub const BUTTON_PADDING_V: u16 = 16;
    pub const BUTTON_PADDING_H: u16 = 20;
    
    // Text sizes
    pub const TITLE_SIZE: u16 = 24;
    pub const SUBTITLE_SIZE: u16 = 14;
    pub const LABEL_SIZE: u16 = 11;
    pub const BUTTON_TEXT_SIZE: u16 = 14;
    pub const INPUT_TEXT_SIZE: u16 = 16;
}

pub struct Theme {
    pub colors: ColorPalette,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            colors: ColorPalette::default(),
        }
    }
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
                color: self.0.background,  // Same as background for seamless look
                width: 4.0,  // Like onagre's border-width: 4px
                radius: Radius::from(20.0),  // Rounded window
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
            background: None,  // Transparent like onagre
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
pub struct RowStyle(pub ColorPalette, pub bool);  // bool for selected state

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
                background: None,  // Transparent background
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
    
    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
    
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.active(style)
    }
}