use bevy::prelude::*;
use bevy_egui::{
    egui::{self, epaint::Shadow, style::Margin, Color32, Stroke, Visuals},
    EguiContext,
};

#[derive(Debug, Clone, Copy)]
pub struct ColorGroup {
    pub main: Color,
    pub light: Color,
    pub dark: Color,
    pub text: Color,
}

impl Default for ColorGroup {
    fn default() -> Self {
        Self {
            main: Color::hex("424242").unwrap_or_default(),
            light: Color::hex("6d6d6d").unwrap_or_default(),
            dark: Color::hex("1b1b1b").unwrap_or_default(),
            text: Color::WHITE,
        }
    }
}

fn color_float_to_u8(f: f32) -> u8 {
    (f * 255.) as u8
}

impl ColorGroup {
    pub fn new(main: &str, light: &str, dark: &str, text: &str) -> Self {
        Self {
            main: Color::hex(main).unwrap_or_default(),
            light: Color::hex(light).unwrap_or_default(),
            dark: Color::hex(dark).unwrap_or_default(),
            text: Color::hex(text).unwrap_or_default(),
        }
    }

    pub fn color32(&self) -> (Color32, Color32, Color32, Color32) {
        let main = Color32::from_rgb(
            color_float_to_u8(self.main.r()),
            color_float_to_u8(self.main.g()),
            color_float_to_u8(self.main.b()),
        );
        let light = Color32::from_rgb(
            color_float_to_u8(self.light.r()),
            color_float_to_u8(self.light.g()),
            color_float_to_u8(self.light.b()),
        );
        let dark = Color32::from_rgb(
            color_float_to_u8(self.dark.r()),
            color_float_to_u8(self.dark.g()),
            color_float_to_u8(self.dark.b()),
        );
        let text = Color32::from_rgb(
            color_float_to_u8(self.text.r()),
            color_float_to_u8(self.text.g()),
            color_float_to_u8(self.text.b()),
        );
        (main, light, dark, text)
    }
}

#[derive(Debug, Clone, Copy, Resource)]
pub struct Colors {
    primary: ColorGroup,
    secondary: ColorGroup,
    info: ColorGroup,
    danger: ColorGroup,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            primary: ColorGroup::new("004D40", "39796b", "00251a", "ffffff"),
            secondary: ColorGroup::new("fbc02d", "fff263", "c49000", "000000"),
            info: ColorGroup::new("64B5F6", "9be7ff", "2286c3", "000000"),
            danger: ColorGroup::new("E64A19", "ff7d47", "ac0800", "ffffff"),
        }
    }
}

pub struct StylePlugin;

impl Plugin for StylePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<Colors>()
            .add_startup_system(set_egui_color);
    }
}

fn set_egui_color(mut ctx: ResMut<EguiContext>, colors: Res<Colors>) {
    let mut visuals = Visuals::dark();

    visuals.widgets.noninteractive.bg_fill = colors.primary.color32().0;
    visuals.widgets.noninteractive.fg_stroke.color = colors.primary.color32().3;
    visuals.widgets.noninteractive.rounding = 0.0.into();
    visuals.widgets.noninteractive.bg_stroke = Stroke {
        width: 1.0,
        color: colors.primary.color32().2,
    };

    visuals.widgets.inactive.bg_fill = colors.secondary.color32().0;
    visuals.widgets.inactive.fg_stroke.color = colors.secondary.color32().3;
    visuals.widgets.inactive.bg_stroke.color = colors.secondary.color32().2;
    visuals.widgets.inactive.rounding = 0.0.into();

    visuals.widgets.hovered.bg_fill = colors.secondary.color32().1;
    visuals.widgets.hovered.fg_stroke.color = colors.secondary.color32().3;
    visuals.widgets.hovered.bg_stroke.color = colors.secondary.color32().2;
    visuals.widgets.hovered.rounding = 0.0.into();

    visuals.widgets.open.bg_fill = colors.secondary.color32().1;
    visuals.widgets.open.fg_stroke.color = colors.secondary.color32().3;
    visuals.widgets.open.bg_stroke.color = colors.secondary.color32().2;
    visuals.widgets.open.rounding = 0.0.into();

    visuals.widgets.active.bg_fill = colors.secondary.color32().2;
    visuals.widgets.active.fg_stroke.color = colors.secondary.color32().3;
    visuals.widgets.active.bg_stroke.color = colors.secondary.color32().2;
    visuals.widgets.active.rounding = 0.0.into();

    visuals.selection.bg_fill = colors.primary.color32().2;
    visuals.faint_bg_color = colors.primary.color32().0;
    visuals.extreme_bg_color = colors.primary.color32().1;

    visuals.hyperlink_color = colors.info.color32().1;

    visuals.warn_fg_color = colors.danger.color32().1;
    visuals.error_fg_color = colors.danger.color32().2;

    visuals.window_rounding = 0.0.into();
    visuals.window_shadow = Shadow::small_light();

    let mut style = egui::style::Style {
        visuals,
        ..default()
    };

    style.spacing.item_spacing = egui::Vec2::new(3., 3.);
    style.spacing.window_margin = Margin::same(5.);
    style.spacing.button_padding = egui::Vec2::new(3., 3.);
    style.spacing.interact_size = egui::Vec2::new(10., 10.);

    ctx.ctx_mut().set_style(style);
}
