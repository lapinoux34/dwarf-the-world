use bevy::prelude::*;

#[derive(Component)]
pub struct TopBar;

#[derive(Component)]
pub struct DayCounter;

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
                height: Val::Px(50.0),
                border: UiRect::bottom(Val::Px(2.0)),
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::horizontal(Val::Px(20.0)),
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.1, 0.08, 0.05)),
            border_color: BorderColor(Color::rgb(0.5, 0.4, 0.2)),
            ..default()
        },
    )).id();

    // Game title
    let title_entity = commands.spawn((
        GameTitle,
        TextBundle::from_section(
            "DWARF THE WORLD",
            TextStyle {
                font_size: 18.0,
                color: Color::rgb(1.0, 0.85, 0.4),
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
                color: Color::WHITE,
                ..default()
            },
        )
    )).id();
    commands.entity(day_entity).set_parent(top_bar);

    // Phase indicator
    let phase_entity = commands.spawn((
        PhaseIndicator,
        TextBundle::from_section(
            "Phase: Draw",
            TextStyle {
                font_size: 14.0,
                color: Color::rgb(0.7, 0.9, 0.7),
                ..default()
            },
        )
    )).id();
    commands.entity(phase_entity).set_parent(top_bar);

    // Resources
    let res_entity = commands.spawn((
        ResourcesDisplay,
        TextBundle::from_section(
            "Gold: 0  Ore: 0  Beer: 0  Runes: 0",
            TextStyle {
                font_size: 12.0,
                color: Color::rgb(1.0, 0.9, 0.5),
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
            ..default()
        },
    )).id()
}

pub fn create_end_turn_button(commands: &mut Commands) -> Entity {
    commands.spawn((
        EndTurnButton,
        ButtonBundle {
            style: Style {
                width: Val::Px(120.0),
                height: Val::Px(40.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.3, 0.5, 0.2)),
            border_color: BorderColor(Color::rgb(0.5, 0.7, 0.3)),
            ..default()
        },
    )).id()
}

pub fn update_day_counter(query: &mut Query<&mut Text, With<DayCounter>>, day: u32) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("Day {}", day);
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
    ore: u32,
    beer: u32,
    runes: u32,
) {
    if let Ok(mut text) = query.get_single_mut() {
        text.sections[0].value = format!("Gold: {}  Ore: {}  Beer: {}  Runes: {}", gold, ore, beer, runes);
    }
}
