use bevy::prelude::*;

#[derive(Component)]
pub struct TopBar;

#[derive(Component)]
pub struct DayCounter;

#[derive(Component)]
pub struct DarknessIndicator;

#[derive(Component)]
pub struct PhaseIndicator;

#[derive(Component)]
pub struct ResourcesDisplay;

#[derive(Component)]
pub struct EndTurnButton;

#[derive(Component)]
pub struct HandArea;

#[derive(Component)]
pub struct GameTitle;

pub fn create_top_bar(commands: &mut Commands) -> Entity {
    let top_bar = commands.spawn((
        TopBar,
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(55.0),
                border: UiRect::bottom(Val::Px(2.0)),
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::horizontal(Val::Px(20.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.18, 0.11, 0.05)),  // Deep brown
            border_color: BorderColor(Color::rgb(0.72, 0.53, 0.2)),          // Bronze border
            ..default()
        },
    )).id();

    // Game title
    let title_entity = commands.spawn((
        GameTitle,
        TextBundle::from_section(
            "⚒ DWARF THE WORLD ⚒",
            TextStyle {
                font_size: 20.0,
                color: Color::rgb(0.83, 0.69, 0.22),  // Gold
                ..default()
            },
        )
    )).id();
    commands.entity(title_entity).set_parent(top_bar);

    // Day counter
    let day_entity = commands.spawn((
        DayCounter,
        TextBundle::from_section(
            "Day 1",
            TextStyle {
                font_size: 16.0,
                color: Color::rgb(0.95, 0.9, 0.7),  // Warm white
                ..default()
            },
        )
    )).id();
    commands.entity(day_entity).set_parent(top_bar);

    // Darkness indicator
    let dark_entity = commands.spawn((
        DarknessIndicator,
        TextBundle::from_section(
            "🌅",
            TextStyle {
                font_size: 16.0,
                ..default()
            },
        )
    )).id();
    commands.entity(dark_entity).set_parent(top_bar);

    // Phase indicator
    let phase_entity = commands.spawn((
        PhaseIndicator,
        TextBundle::from_section(
            "Draw Phase",
            TextStyle {
                font_size: 12.0,
                color: Color::rgb(0.6, 0.85, 0.6),  // Pale green
                ..default()
            },
        )
    )).id();
    commands.entity(phase_entity).set_parent(top_bar);

    // Resources
    let res_entity = commands.spawn((
        ResourcesDisplay,
        TextBundle::from_section(
            "💰 Gold: 0  ⚗ Mithril: 0  🍺 Provisions: 0  🔮 Runestones: 0",
            TextStyle {
                font_size: 11.0,
                color: Color::rgb(0.9, 0.75, 0.4),  // Gold text
                ..default()
            },
        )
    )).id();
    commands.entity(res_entity).set_parent(top_bar);

    top_bar
}

pub fn create_hand_area(commands: &mut Commands) -> Entity {
    commands.spawn((
        HandArea,
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Px(170.0),
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.05, 0.03, 0.02, 0.8)),  // Near black, slightly transparent
            ..default()
        },
    )).id()
}

pub fn create_end_turn_button(commands: &mut Commands) -> Entity {
    commands.spawn((
        EndTurnButton,
        ButtonBundle {
            style: Style {
                width: Val::Px(130.0),
                height: Val::Px(45.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.25, 0.15, 0.05)),  // Dark bronze
            border_color: BorderColor(Color::rgb(0.83, 0.69, 0.22)),            // Gold border
            ..default()
        },
    )).id()
}

pub fn update_day_counter(query: &mut Query<&mut Text, With<DayCounter>>, day: u32) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("Day {}", day);
    }
}

pub fn update_darkness_indicator(query: &mut Query<&mut Text, With<DarknessIndicator>>, darkness: f32) {
    if let Ok(mut text) = query.get_single_mut() {
        let emoji = if darkness < 0.3 {
            "🌅" // Dawn
        } else if darkness < 0.5 {
            "🌄" // Sunrise/Sunset
        } else if darkness < 0.7 {
            "🌙" // Moon
        } else {
            "🌑" // New Moon (darkness)
        };
        text.sections[0].value = emoji.to_string();
    }
}

pub fn update_phase_indicator(query: &mut Query<&mut Text, With<PhaseIndicator>>, phase: &str) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("Phase: {}", phase);
    }
}

pub fn update_resources_display(
    query: &mut Query<&mut Text, With<ResourcesDisplay>>,
    gold: u32,
    mithril: u32,
    provisions: u32,
    runestones: u32,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!(
            "💰 Gold: {}  ⚗ Mithril: {}  🍺 Provisions: {}  🔮 Runestones: {}",
            gold, mithril, provisions, runestones
        );
    }
}
