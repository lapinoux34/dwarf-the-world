use bevy::prelude::*;
use crate::game::{Card, CardType, DwarfFaction, entry_point::EntryType};

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
        CardType::Settler => Color::rgb(0.45, 0.4, 0.35),   // Stone
        CardType::Builder => Color::rgb(0.6, 0.35, 0.2),   // Wood/forge
        CardType::Defender => Color::rgb(0.4, 0.45, 0.5),  // Steel
        CardType::Resource => Color::rgb(0.5, 0.45, 0.3),  // Earth
        CardType::Hero => {
            match faction {
                Some(DwarfFaction::Erebor) => Color::rgb(0.83, 0.69, 0.22),  // Gold
                Some(DwarfFaction::Moria) => Color::rgb(0.4, 0.3, 0.2),      // Dark bronze
                Some(DwarfFaction::Dale) => Color::rgb(0.55, 0.4, 0.3),      // Warm brown
                Some(DwarfFaction::IronHills) => Color::rgb(0.5, 0.5, 0.55), // Silver
                None => Color::rgb(0.5, 0.4, 0.2),
            }
        }
        CardType::Ally => Color::rgb(0.3, 0.5, 0.4),   // Elven green
        CardType::Spell => Color::rgb(0.4, 0.3, 0.6),  // Mystic purple
        CardType::Monster => Color::rgb(0.35, 0.05, 0.05),  // Dark blood
        CardType::Event => Color::rgb(0.6, 0.4, 0.1),       // Torch orange
    }
}

pub fn card_border_color(card_type: CardType) -> Color {
    match card_type {
        CardType::Hero => Color::rgb(0.83, 0.69, 0.22),  // Gold
        CardType::Monster => Color::rgb(0.5, 0.0, 0.0), // Dark red
        _ => Color::rgb(0.55, 0.45, 0.35),              // Bronze
    }
}

pub fn entry_type_badge_color(entry_type: EntryType) -> Color {
    match entry_type {
        EntryType::Trade => Color::rgb(0.55, 0.4, 0.25),
        EntryType::Wealth => Color::rgb(0.83, 0.69, 0.22),
        EntryType::Resource => Color::rgb(0.45, 0.35, 0.3),
        EntryType::Supply => Color::rgb(0.6, 0.45, 0.3),
        EntryType::Military => Color::rgb(0.4, 0.35, 0.4),
        EntryType::Production => Color::rgb(0.6, 0.25, 0.1),
        EntryType::Recruitment => Color::rgb(0.5, 0.35, 0.2),
        EntryType::Any => Color::rgb(0.4, 0.4, 0.35),
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
    let entry_color = entry_type_badge_color(card.entry_type);

    let card_entity = commands.spawn((
        CardComponent {
            card: card.clone(),
            is_selected: false,
        },
        NodeBundle {
            style: Style {
                width: Val::Px(95.0),
                height: Val::Px(130.0),
                border: UiRect::all(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(3.0)),
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
                font_size: 8.0,
                color: Color::WHITE,
                ..default()
            },
        )
    ).id();
    commands.entity(name_entity).set_parent(card_entity);

    // Cost display
    let cost_str = format!("{}G", card.cost.gold);
    let cost_entity = commands.spawn(
        TextBundle::from_section(
            &cost_str,
            TextStyle {
                font_size: 9.0,
                color: if card.card_type == CardType::Monster {
                    Color::rgb(1.0, 0.3, 0.3)
                } else {
                    Color::rgb(0.7, 0.85, 1.0)
                },
                ..default()
            },
        )
    ).id();
    commands.entity(cost_entity).set_parent(card_entity);

    // Entry type badge
    let entry_entity = commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Px(50.0),
                height: Val::Px(12.0),
                margin: UiRect::vertical(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(entry_color),
            ..default()
        },
    ).id();
    commands.entity(entry_entity).set_parent(card_entity);

    // Art placeholder
    let art_entity = commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(45.0),
                margin: UiRect::vertical(Val::Px(1.0)),
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.12, 0.08, 0.05)),
            ..default()
        },
    ).id();
    commands.entity(art_entity).set_parent(card_entity);

    // Stats (if attacker/defender)
    let stats_text = match (card.attack, card.defense) {
        (Some(atk), Some(def)) => format!("⚔{} 🛡{}", atk, def),
        _ => String::new(),
    };

    if !stats_text.is_empty() {
        let stats_entity = commands.spawn(
            TextBundle::from_section(
                &stats_text,
                TextStyle {
                    font_size: 8.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
        ).id();
        commands.entity(stats_entity).set_parent(card_entity);
    }

    card_entity
}
