use bevy::prelude::*;
use crate::game::{Card, CardType};

#[derive(Component)]
pub struct CardComponent {
    pub card: Card,
    pub is_selected: bool,
}

#[derive(Component)]
pub struct CardInHand;

#[derive(Component)]
pub struct CardOnBoard {
    pub location_id: u32,
}

pub fn card_color(card_type: CardType) -> Color {
    match card_type {
        CardType::Dwarf => Color::rgb(0.2, 0.4, 0.8),
        CardType::Monster => Color::rgb(0.8, 0.2, 0.2),
        CardType::Event => Color::rgb(0.8, 0.6, 0.1),
        CardType::Location => Color::rgb(0.3, 0.7, 0.3),
        CardType::Resource => Color::rgb(0.9, 0.7, 0.1),
    }
}

pub fn card_border_color(card_type: CardType) -> Color {
    match card_type {
        CardType::Dwarf => Color::rgb(0.8, 0.6, 0.2),
        CardType::Monster => Color::rgb(0.6, 0.1, 0.1),
        _ => Color::rgb(0.3, 0.3, 0.3),
    }
}

pub fn create_card_ui(
    commands: &mut Commands,
    card: &Card,
    position: Vec3,
    scale: Vec3,
) -> Entity {
    let color = card_color(card.card_type);
    let border_color = card_border_color(card.card_type);

    let card_entity = commands.spawn((
        CardComponent {
            card: card.clone(),
            is_selected: false,
        },
        NodeBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(140.0),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            background_color: BackgroundColor(color),
            border_color: BorderColor(border_color),
            ..default()
        },
        Transform {
            translation: position,
            scale,
            ..default()
        },
    )).id();

    // Card name
    let name_entity = commands.spawn(
        TextBundle::from_section(
            &card.name,
            TextStyle {
                font_size: 10.0,
                color: Color::WHITE,
                ..default()
            },
        )
    ).id();
    commands.entity(name_entity).set_parent(card_entity);

    // Cost
    let cost_entity = commands.spawn(
        TextBundle::from_section(
            format!("{}", card.cost),
            TextStyle {
                font_size: 12.0,
                color: Color::rgb(1.0, 0.8, 0.0),
                ..default()
            },
        )
    ).id();
    commands.entity(cost_entity).set_parent(card_entity);

    // Art placeholder
    let art_entity = commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                margin: UiRect::vertical(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.15, 0.1, 0.2)),
            ..default()
        },
    ).id();
    commands.entity(art_entity).set_parent(card_entity);

    // ATK / DEF
    let atk_text = card.attack.map_or("--".to_string(), |a| a.to_string());
    let def_text = card.defense.map_or("--".to_string(), |d| d.to_string());

    let stats_entity = commands.spawn(
        TextBundle::from_section(
            format!("ATK:{} DEF:{}", atk_text, def_text),
            TextStyle {
                font_size: 9.0,
                color: Color::WHITE,
                ..default()
            },
        )
    ).id();
    commands.entity(stats_entity).set_parent(card_entity);

    card_entity
}
