use bevy::prelude::*;
use crate::game::{BoardLocation, LocationZone};

#[derive(Component)]
pub struct BoardZone {
    pub location_id: u32,
}

#[derive(Component)]
pub struct ZoneLabel {
    pub location_id: u32,
}

pub fn location_zone_color(zone: LocationZone) -> Color {
    match zone {
        LocationZone::MineEntrance => Color::rgb(0.4, 0.35, 0.3),
        LocationZone::Forge => Color::rgb(0.6, 0.25, 0.1),
        LocationZone::Tavern => Color::rgb(0.5, 0.35, 0.2),
        LocationZone::MountainPeak => Color::rgb(0.5, 0.55, 0.6),
        LocationZone::UndergroundCavern => Color::rgb(0.2, 0.15, 0.25),
    }
}

pub fn create_board_zone(
    commands: &mut Commands,
    location: &BoardLocation,
    position: Vec3,
    width: f32,
    height: f32,
) -> Entity {
    let color = location_zone_color(location.zone);

    let zone_entity = commands.spawn((
        BoardZone {
            location_id: location.id,
        },
        NodeBundle {
            style: Style {
                width: Val::Px(width),
                height: Val::Px(height),
                border: UiRect::all(Val::Px(3.0)),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            background_color: BackgroundColor(color),
            border_color: BorderColor(Color::rgb(0.6, 0.5, 0.3)),
            ..default()
        },
        Transform {
            translation: position,
            ..default()
        },
    )).id();

    // Zone name label
    let name_entity = commands.spawn((
        ZoneLabel { location_id: location.id },
        TextBundle::from_section(
            &location.name,
            TextStyle {
                font_size: 14.0,
                color: Color::rgb(1.0, 0.9, 0.7),
                ..default()
            },
        )
    )).id();
    commands.entity(name_entity).set_parent(zone_entity);

    // Description
    let desc_entity = commands.spawn(
        TextBundle::from_section(
            &location.description,
            TextStyle {
                font_size: 9.0,
                color: Color::rgb(0.8, 0.75, 0.6),
                ..default()
            },
        )
    ).id();
    commands.entity(desc_entity).set_parent(zone_entity);

    // Card count placeholder
    let count_entity = commands.spawn(
        TextBundle::from_section(
            "Cards: 0",
            TextStyle {
                font_size: 10.0,
                color: Color::WHITE,
                ..default()
            },
        )
    ).id();
    commands.entity(count_entity).set_parent(zone_entity);

    zone_entity
}
