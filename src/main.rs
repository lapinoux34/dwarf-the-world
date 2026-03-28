mod data;
mod game;
mod rendering;

use bevy::prelude::*;
use game::{Card, CardType, EntryType, Phase};
use rendering::{
    CardComponent, EntryPointComponent, EndTurnButton,
    DayCounter, DarknessIndicator, PhaseIndicator, ResourcesDisplay,
};
use rendering::ui::{
    create_top_bar, create_hand_area, create_end_turn_button,
    update_day_counter, update_darkness_indicator, update_phase_indicator,
    update_resources_display,
};
use rendering::entry_points::create_entry_point_ui;
use rendering::cards::create_card_ui;
use data::get_starter_cards;

#[derive(Resource)]
struct GameResource {
    state: game::GameState,
}

#[derive(Component)]
struct CardInHand;

fn main() {
    let cards = get_starter_cards();
    let game_state = game::GameState::new(cards);

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameResource {
            state: game_state,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_card_click,
            handle_entry_click,
            handle_end_turn,
            advance_phase_system,
            update_ui,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut game_res: ResMut<GameResource>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Dark atmospheric background
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: BackgroundColor(Color::rgb(0.05, 0.03, 0.02)),  // Cave darkness
        ..default()
    });

    // Top bar with resources
    create_top_bar(&mut commands);

    // World map - 8 entry points in a grid
    let entry_width = 180.0;
    let entry_height = 130.0;
    let start_x = -380.0;
    let start_y = 80.0;
    let gap_x = 200.0;
    let gap_y = 150.0;
    let cols = 4;

    for (i, entry) in game_res.state.entry_points.iter().enumerate() {
        let row = i / cols;
        let col = i % cols;
        let x = start_x + (col as f32) * gap_x;
        let y = start_y - (row as f32) * gap_y;
        create_entry_point_ui(&mut commands, entry, Vec3::new(x, y, 0.0), entry_width, entry_height);
    }

    // Hand area at bottom
    create_hand_area(&mut commands);

    // End turn button
    create_end_turn_button(&mut commands);

    // Initial draw - start in Day phase
    game_res.state.draw_cards(3);
    game_res.state.phase = Phase::Day;

    // Spawn initial hand
    spawn_hand(&mut commands, &mut game_res);
}

fn spawn_hand(commands: &mut Commands, game_res: &mut GameResource) {
    let hand_size = game_res.state.hand.len();
    if hand_size == 0 {
        return;
    }
    let card_width = 95.0;
    let total_width = (hand_size as f32) * card_width;
    let start_x = -total_width / 2.0 + card_width / 2.0;

    for (i, card) in game_res.state.hand.iter().enumerate() {
        let x = start_x + (i as f32) * card_width;
        let entity = create_card_ui(
            commands,
            card,
            Vec3::new(x, -280.0, 1.0),
            Vec3::ONE,
        );
        commands.entity(entity).insert(CardInHand);
    }
}

fn handle_card_click(
    mut game_res: ResMut<GameResource>,
    card_query: Query<(&CardComponent, &Interaction), Changed<Interaction>>,
) {
    for (card_comp, interaction) in card_query.iter() {
        if *interaction == Interaction::Pressed {
            game_res.state.selected_card = Some(card_comp.card.clone());
        }
    }
}

fn handle_entry_click(
    mut game_res: ResMut<GameResource>,
    entry_query: Query<(&EntryPointComponent, &Interaction), Changed<Interaction>>,
) {
    for (entry_comp, interaction) in entry_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Some(ref card) = game_res.state.selected_card {
                game_res.state.selected_entry = Some(entry_comp.entry_id);
                let card_clone = card.clone();
                game_res.state.play_card(&card_clone, entry_comp.entry_id);
            }
        }
    }
}

fn handle_end_turn(
    mut game_res: ResMut<GameResource>,
    button_query: Query<&Interaction, (With<EndTurnButton>, Changed<Interaction>)>,
) {
    for interaction in button_query.iter() {
        if *interaction == Interaction::Pressed {
            // Collect resources
            game_res.state.collect_resources();

            // Advance day
            game_res.state.advance_day();

            // Draw new hand
            game_res.state.draw_cards(3);
            game_res.state.phase = Phase::Day;
        }
    }
}

fn advance_phase_system(
    mut game_res: ResMut<GameResource>,
    mut phase_timer: Local<f32>,
    time: Res<Time>,
) {
    *phase_timer += time.delta_seconds();
    if *phase_timer > 0.5 {
        *phase_timer = 0.0;
        game_res.state.advance_phase();
    }
}

fn update_ui(
    game_res: Res<GameResource>,
    mut day_query: Query<&mut Text, With<DayCounter>>,
    mut dark_query: Query<&mut Text, With<DarknessIndicator>>,
    mut phase_query: Query<&mut Text, With<PhaseIndicator>>,
    mut resources_query: Query<&mut Text, With<ResourcesDisplay>>,
) {
    let state = &game_res.state;

    update_day_counter(&mut day_query, state.day);
    update_darkness_indicator(&mut dark_query, state.darkness_level);

    let phase_str = match state.phase {
        Phase::Dawn => "Dawn",
        Phase::Day => "Day",
        Phase::Dusk => "Dusk",
        Phase::Night => "Night",
        Phase::EndTurn => "End",
    };
    update_phase_indicator(&mut phase_query, phase_str);

    update_resources_display(
        &mut resources_query,
        state.resources.gold,
        state.resources.ore,
        state.resources.beer,
        state.resources.food,
    );
}
