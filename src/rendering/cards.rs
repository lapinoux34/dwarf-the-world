use bevy::prelude::*;
use crate::game::card::{Card, CardType, DwarfFaction};
use crate::game::EntryType;

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

pub fn card_color(card_type: CardType, faction: Option<DwarfFaction>) -> Color {
    match card_type {
        CardType::Settler | CardType::Builder | CardType::Defender | CardType::Resource | CardType::Hero => {
            match faction {
                Some(DwarfFaction::Erebor) => Color::rgb(0.72, 0.53, 0.15),  // Gold
                Some(DwarfFaction::Moria) => Color::rgb(0.35, 0.25, 0.12),     // Dark bronze
                Some(DwarfFaction::Dale) => Color::rgb(0.55, 0.35, 0.25),     // Warm brown
                Some(DwarfFaction::IronHills) => Color::rgb(0.45, 0.45, 0.5),  // Mithril
                None => Color::rgb(0.5, 0.4, 0.2),
            }
        }
        CardType::Ally => Color::rgb(0.3, 0.5, 0.3),     // Forest green
        CardType::Spell => Color::rgb(0.3, 0.3, 0.6),    // Magic blue
        CardType::Monster => Color::rgb(0.35, 0.05, 0.05),  // Dark blood red
        CardType::Event => Color::rgb(0.6, 0.4, 0.1),       // Torch orange
    }
}

pub fn card_border_color(card_type: CardType) -> Color {
    match card_type {
        CardType::Settler | CardType::Builder | CardType::Defender | CardType::Resource | CardType::Hero => {
            Color::rgb(0.83, 0.69, 0.22)     // Gold border
        }
        CardType::Ally => Color::rgb(0.4, 0.6, 0.4),      // Green border
        CardType::Spell => Color::rgb(0.4, 0.4, 0.8),     // Blue border
        CardType::Monster => Color::rgb(0.5, 0.0, 0.0),      // Dark red border
        CardType::Event => Color::rgb(0.8, 0.5, 0.2),       // Orange border
    }
}

pub fn card_tier_color(tier: u32) -> Color {
    match tier {
        1 => Color::rgb(0.5, 0.5, 0.5),      // Grey (common)
        2 => Color::rgb(0.3, 0.6, 0.3),      // Green (uncommon)
        3 => Color::rgb(0.3, 0.4, 0.7),      // Blue (rare)
        4 => Color::rgb(0.6, 0.4, 0.8),      // Purple (epic)
        5 => Color::rgb(0.8, 0.6, 0.2),      // Gold (legendary)
        _ => Color::rgb(0.5, 0.5, 0.5),
    }
}

pub fn create_card_ui(
    commands: &mut Commands,
    card: &Card,
    position: Vec3,
    scale: Vec3,
) -> Entity {
    let color = card_color(card.card_type, card.faction);
    let border_color = card_border_color(card.card_type);
    let tier_color = card_tier_color(card.tier);

    let card_entity = commands.spawn((
        CardComponent {
            card: card.clone(),
            is_selected: false,
        },
        NodeBundle {
            style: Style {
                width: Val::Px(100.0),
                height: Val::Px(140.0),
                border: UiRect::all(Val::Px(3.0)),
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
                font_size: 9.0,
                color: Color::WHITE,
                ..default()
            },
        )
    ).id();
    commands.entity(name_entity).set_parent(card_entity);

    // Cost gem
    let cost_entity = commands.spawn(
        TextBundle::from_section(
            format!("{}", card.cost),
            TextStyle {
                font_size: 11.0,
                color: if matches!(card.card_type, CardType::Monster) {
                    Color::rgb(1.0, 0.3, 0.3)  // Red for monsters
                } else {
                    Color::rgb(0.7, 0.85, 1.0) // Blue for dwarves
                },
                ..default()
            },
        )
    ).id();
    commands.entity(cost_entity).set_parent(card_entity);

    // Art placeholder with faction-themed gradient
    let art_entity = commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(55.0),
                margin: UiRect::vertical(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.15, 0.1, 0.05)),
            ..default()
        },
    ).id();
    commands.entity(art_entity).set_parent(card_entity);

    // ATK / DEF stats
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

    // Tier indicator
    let tier_str = match card.tier {
        1 => "★",
        2 => "★★",
        3 => "★★★",
        4 => "★★★★",
        5 => "★★★★★",
        _ => "",
    };
    let tier_entity = commands.spawn(
        TextBundle::from_section(
            tier_str,
            TextStyle {
                font_size: 8.0,
                color: tier_color,
                ..default()
            },
        )
    ).id();
    commands.entity(tier_entity).set_parent(card_entity);

    card_entity
}

pub fn highlight_card(card_query: &mut Query<(&CardComponent, &mut BackgroundColor, &mut BorderColor)>, card_id: u32) {
    for (card_comp, mut bg, mut border) in card_query.iter_mut() {
        if card_comp.card.id == card_id {
            *border = BorderColor(Color::rgb(1.0, 0.9, 0.5)); // Bright gold when selected
        }
    }
}
