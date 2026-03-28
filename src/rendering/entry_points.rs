use bevy::prelude::*;
use crate::game::{EntryPoint, ZoneType};

#[derive(Component)]
pub struct EntryPointComponent {
    pub entry_id: u32,
}

#[derive(Component)]
pub struct EntryLabel {
    pub entry_id: u32,
}

pub fn entry_type_color(entry_type: ZoneType) -> Color {
    match entry_type {
        ZoneType::Trade => Color::rgb(0.55, 0.4, 0.25),
        ZoneType::Wealth => Color::rgb(0.83, 0.69, 0.22),
        ZoneType::Resource => Color::rgb(0.45, 0.35, 0.3),
        ZoneType::Supply => Color::rgb(0.6, 0.45, 0.3),
        ZoneType::Military => Color::rgb(0.4, 0.35, 0.4),
        ZoneType::Production => Color::rgb(0.6, 0.25, 0.1),
        ZoneType::Recruitment => Color::rgb(0.5, 0.35, 0.2),
        ZoneType::Danger => Color::rgb(0.25, 0.1, 0.08),
        ZoneType::Any => Color::rgb(0.4, 0.4, 0.35),
    }
}

pub fn entry_border_color(entry_type: ZoneType) -> Color {
    match entry_type {
        ZoneType::Trade => Color::rgb(0.72, 0.53, 0.2),
        ZoneType::Wealth => Color::rgb(1.0, 0.85, 0.3),
        ZoneType::Resource => Color::rgb(0.5, 0.4, 0.3),
        ZoneType::Supply => Color::rgb(0.6, 0.45, 0.3),
        ZoneType::Military => Color::rgb(0.6, 0.6, 0.65),
        ZoneType::Production => Color::rgb(0.8, 0.35, 0.1),
        ZoneType::Recruitment => Color::rgb(0.6, 0.4, 0.2),
        ZoneType::Danger => Color::rgb(0.5, 0.15, 0.1),
        ZoneType::Any => Color::rgb(0.5, 0.45, 0.4),
    }
}

pub fn create_entry_point_ui(
    commands: &mut Commands,
    entry: &EntryPoint,
    position: Vec3,
    width: f32,
    height: f32,
) -> Entity {
    let color = entry_type_color(entry.entry_type);
    let border = entry_border_color(entry.entry_type);

    let entry_entity = commands.spawn((
        EntryPointComponent {
            entry_id: entry.id,
        },
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
    )).id();

    commands.entity(entry_entity).insert(Transform::from_translation(position));

    // Entry name
    let name_entity = commands.spawn((
        EntryLabel { entry_id: entry.id },
        TextBundle::from_section(
            &entry.name,
            TextStyle {
                font_size: 10.0,
                color: Color::rgb(0.95, 0.88, 0.65),
                ..default()
            },
        )
    )).id();
    commands.entity(name_entity).set_parent(entry_entity);

    // Entry type badge
    let type_entity = commands.spawn(
        TextBundle::from_section(
            entry.entry_type.synergy_bonus(),
            TextStyle {
                font_size: 8.0,
                color: Color::rgb(0.7, 0.65, 0.5),
                ..default()
            },
        )
    ).id();
    commands.entity(type_entity).set_parent(entry_entity);

    // Description
    let desc_entity = commands.spawn(
        TextBundle::from_section(
            &entry.description,
            TextStyle {
                font_size: 7.0,
                color: Color::rgb(0.6, 0.55, 0.45),
                ..default()
            },
        )
    ).id();
    commands.entity(desc_entity).set_parent(entry_entity);

    // Card count
    let count_str = format!("Cards: {}/{}", entry.cards.len(), entry.max_cards);
    let count_entity = commands.spawn(
        TextBundle::from_section(
            &count_str,
            TextStyle {
                font_size: 8.0,
                color: Color::rgb(0.8, 0.7, 0.5),
                ..default()
            },
        )
    ).id();
    commands.entity(count_entity).set_parent(entry_entity);

    entry_entity
}
