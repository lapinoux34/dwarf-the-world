use bevy::prelude::*;
use crate::game::zone::{WorldZone, ZoneType, ZoneEffect};

#[derive(Component)]
pub struct ZoneNode {
    pub zone_id: u32,
}

#[derive(Component)]
pub struct ZoneName {
    pub zone_id: u32,
}

#[derive(Component)]
pub struct ZoneCardCount {
    pub zone_id: u32,
}

/// Create the world map background
pub fn create_world_map_background(commands: &mut Commands) -> Entity {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.02, 0.05, 0.08)),
            ..default()
        },
    )).id()
}

/// Get the color scheme for a zone type
pub fn zone_type_color(zone_type: ZoneType) -> Color {
    match zone_type {
        ZoneType::Trade => Color::rgb(0.45, 0.35, 0.25),
        ZoneType::Wealth => Color::rgb(0.72, 0.53, 0.15),
        ZoneType::Resource => Color::rgb(0.35, 0.28, 0.22),
        ZoneType::Supply => Color::rgb(0.5, 0.4, 0.25),
        ZoneType::Military => Color::rgb(0.35, 0.32, 0.38),
        ZoneType::Production => Color::rgb(0.55, 0.22, 0.08),
        ZoneType::Recruitment => Color::rgb(0.42, 0.3, 0.18),
        ZoneType::Danger => Color::rgb(0.25, 0.1, 0.08),
        ZoneType::Any => Color::rgb(0.35, 0.35, 0.3),
    }
}

pub fn zone_border_color(zone_type: ZoneType) -> Color {
    match zone_type {
        ZoneType::Trade => Color::rgb(0.65, 0.5, 0.25),
        ZoneType::Wealth => Color::rgb(0.95, 0.8, 0.3),
        ZoneType::Resource => Color::rgb(0.45, 0.38, 0.28),
        ZoneType::Supply => Color::rgb(0.55, 0.42, 0.28),
        ZoneType::Military => Color::rgb(0.55, 0.55, 0.6),
        ZoneType::Production => Color::rgb(0.75, 0.35, 0.1),
        ZoneType::Recruitment => Color::rgb(0.55, 0.38, 0.2),
        ZoneType::Danger => Color::rgb(0.5, 0.15, 0.1),
        ZoneType::Any => Color::rgb(0.45, 0.42, 0.38),
    }
}

/// Create a zone node on the world map
pub fn create_zone_node(
    commands: &mut Commands,
    zone: &WorldZone,
    position: Vec3,
    width: f32,
    height: f32,
) -> Entity {
    let color = zone_type_color(zone.zone_type);
    let border = zone_border_color(zone.zone_type);

    commands.spawn((
        ZoneNode { zone_id: zone.id },
        NodeBundle {
            style: Style {
                width: Val::Px(width),
                height: Val::Px(height),
                border: UiRect::all(Val::Px(3.0)),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            background_color: BackgroundColor(color),
            border_color: BorderColor(border),
            ..default()
        },
        Transform {
            translation: position,
            ..default()
        },
    )).id()
}
