extern crate iced;

use iced::widget::{ button };
use iced::{ Background, Color, Vector };
use iced::Theme;

#[derive(Default)]
pub struct Btn {
    border_radius: f32,
    active_bg_color: Color,
    active_fg_color: Color,
    hover_bg_color: Color,
    hover_fg_color: Color,
    press_bg_color: Color,
    press_fg_color: Color,
}

enum StyleTypes {
    Active,
    Pressed,
    Hovered,
    Disabled,
}

fn create_style(bg_color: &Color, fg_color: &Color, border_radius: f32) -> button::Appearance {
    button::Appearance {
        shadow_offset: Vector::new(0.0, 0.0),
        background: Some(iced::Background::Color(*bg_color)),
        border_radius,
        border_width: 0.0,
        border_color: *bg_color,
        text_color: *fg_color,
    }
}

impl Btn {
    pub fn new(border_radius: f32, color: (i32, i32, i32)) -> Self {
        let color: Vec<f32> = vec![
            color.0 as f32 / 255.0,
            color.1 as f32 / 255.0,
            color.2 as f32 / 255.0
        ];
        let active_bg_color = Color::new(color[0], color[1], color[2], 1.0);
        let active_fg_color = Color::WHITE;

        let hov_color = color.iter().map(|&col| {
            if col + 0.2 > 1.0 { 1.0 } else { col + 0.2 }
        }).collect::<Vec<f32>>();

        let hover_bg_color = Color::new(hov_color[0], hov_color[1], hov_color[2], 1.0);
        let hover_fg_color = Color::WHITE;

        let press_color = color.iter().map(|&col| {
            if col - 0.2 < 0.0 { 0.0 } else { col - 0.2 }
        }).collect::<Vec<f32>>();

        let press_bg_color = Color::new(press_color[0], press_color[1], press_color[2], 1.0);
        let press_fg_color = Color::WHITE;
        Btn {
            border_radius,
            active_bg_color,
            active_fg_color,
            hover_bg_color,
            hover_fg_color,
            press_bg_color,
            press_fg_color
        }
    }
    fn get_style(&self, btn_type: StyleTypes) -> button::Appearance {
        let bg_color = match btn_type {
            StyleTypes::Active => self.active_bg_color,
            StyleTypes::Hovered => self.hover_bg_color,
            StyleTypes::Pressed => self.press_bg_color,
            StyleTypes::Disabled => Color::new(0.2, 0.2, 0.2, 1.0)
        };
        let fg_color = match btn_type {
            StyleTypes::Active => self.active_fg_color,
            StyleTypes::Hovered => self.hover_fg_color,
            StyleTypes::Pressed => self.press_fg_color,
            StyleTypes::Disabled => Color::BLACK
        };
        button::Appearance {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(iced::Background::Color(bg_color)),
            border_radius: self.border_radius,
            border_width: 0.0,
            border_color: bg_color,
            text_color: fg_color,
        }
    }
}



impl button::StyleSheet for Btn {
    type Style = Theme;
    fn active(&self, style: &Self::Style) -> button::Appearance {
        self.get_style(StyleTypes::Active)
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        self.get_style(StyleTypes::Hovered)
    }
    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.get_style(StyleTypes::Pressed)
    }
    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        self.get_style(StyleTypes::Disabled)
    }
}