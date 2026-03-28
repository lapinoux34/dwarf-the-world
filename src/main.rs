mod data;
mod game;
mod rendering;

use bevy::prelude::*;
use game::{GameState, Phase, CardType};
use rendering::{CardComponent, BoardZone, EndTurnButton, DayCounter, DarknessIndicator, PhaseIndicator, ResourcesDisplay};
use rendering::ui::{create_top_bar, create_hand_area, create_end_turn_button, update_day_counter, update_darkness_indicator, update_phase_indicator, update_resources_display};
use rendering::board::create_board_zone;
use rendering::cards::create_card_ui;
use data::{get_starter_cards, get_starter_locations};

#[derive(Resource)]
struct GameResource {
    state: GameState,
    locations: Vec<game::BoardLocation>,
}

#[derive(Component)]
struct CardInHand;

fn main() {
    let cards = get_starter_cards();
    let locations = get_starter_locations();
    let game_state = GameState::new(cards.clone(), locations.clone());

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameResource {
            state: game_state,
            locations,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_card_click,
            handle_zone_click,
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

    // Top bar
    create_top_bar(&mut commands);

    // Board zones - Tolkien locations
    let zone_width = 190.0;
    let zone_height = 145.0;
    let start_x = -420.0;
    let start_y = 50.0;
    let gap_x = 210.0;
    let gap_y = 170.0;

    for (i, loc) in game_res.locations.iter().enumerate() {
        let row = i / 3;
        let col = i % 3;
        let x = start_x + (col as f32) * gap_x;
        let y = start_y - (row as f32) * gap_y;
        create_board_zone(&mut commands, loc, Vec3::new(x, y, 0.0), zone_width, zone_height);
    }

    // Hand area
    create_hand_area(&mut commands);

    // End turn button
    create_end_turn_button(&mut commands);

    // Initial draw
    game_res.state.draw_card();
    game_res.state.phase = Phase::Play;

    // Spawn initial hand
    spawn_hand(&mut commands, &mut game_res);
}

fn spawn_hand(commands: &mut Commands, game_res: &mut GameResource) {
    let hand_size = game_res.state.hand.len();
    if hand_size == 0 {
        return;
    }
    let card_width = 105.0;
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

fn handle_zone_click(
    mut game_res: ResMut<GameResource>,
    zone_query: Query<(&BoardZone, &Interaction), Changed<Interaction>>,
) {
    for (zone, interaction) in zone_query.iter() {
        if *interaction == Interaction::Pressed {
            if let Some(ref card) = game_res.state.selected_card {
                if card.card_type == CardType::Dwarf {
                    let card_clone = card.clone();
                    game_res.state.play_card(&card_clone, zone.location_id);
                    game_res.state.selected_card = None;
                }
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
            game_res.state.phase = Phase::Combat;

            let mut dead_cards: Vec<(u32, u32)> = Vec::new();

            for (&loc_id, cards) in game_res.state.board.iter_mut() {
                let mut dwarves: Vec<_> = cards.iter().filter(|c| c.card_type == CardType::Dwarf).cloned().collect();
                let mut monsters: Vec<_> = cards.iter().filter(|c| c.card_type == CardType::Monster).cloned().collect();

                let (dwarf_results, monster_results) = game::resolve_combat_on_location(&mut dwarves, &mut monsters);

                for result in dwarf_results.iter().chain(monster_results.iter()) {
                    if result.destroyed {
                        dead_cards.push((result.card.id, loc_id));
                    }
                }
            }

            for (card_id, loc_id) in dead_cards {
                if let Some(cards) = game_res.state.board.get_mut(&loc_id) {
                    cards.retain(|c| c.id != card_id);
                }
            }

            game_res.state.end_turn();
            game_res.state.draw_card();
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

    // Calculate darkness level based on day (0-300)
    let darkness = (state.day as f32 / 300.0).min(1.0);
    update_darkness_indicator(&mut dark_query, darkness);

    let phase_str = match state.phase {
        Phase::Draw => "Draw",
        Phase::Play => "Play",
        Phase::Combat => "Combat",
        Phase::EndTurn => "End Turn",
    };
    update_phase_indicator(&mut phase_query, phase_str);

    update_resources_display(
        &mut resources_query,
        state.resources.gold,
        state.resources.mithril,
        state.resources.provisions,
        state.resources.runestones,
    );
}
