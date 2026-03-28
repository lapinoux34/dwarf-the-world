mod data;
mod game;
mod logging;
mod rendering;

use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};
use game::{GameState, Phase};
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
    state: GameState,
}

#[derive(Component)]
struct CardInHand;

fn main() {
    // Use process::exit to ensure at_exit runs AND we capture the exit code
    let exit_code = std::panic::catch_unwind(|| {
        logging::setup_logging();

        // Panic hook for main thread panics
        std::panic::set_hook(Box::new(|panic_info| {
            let msg = format!("PANIC: {}", panic_info);
            logging::log_error(&msg);
        }));

        logging::log_info("Starting Dwarf The World...");
        logging::log_info(&format!("DISCORD_WEBHOOK_URL={}",
            if std::env::var("DISCORD_WEBHOOK_URL").unwrap_or_default().is_empty() {
                "NOT SET".to_string()
            } else { "SET".to_string() }
        ));

        let cards = get_starter_cards();
        logging::log_info(&format!("Loaded {} cards", cards.len()));

        let game_state = GameState::new(cards);
        logging::log_info("GameState created, entering App::run()...");

        App::new()
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "⚒ DWARF THE WORLD ⚒ - GAME WINDOW".to_string(),
                            resolution: bevy::window::WindowResolution::new(1280.0, 720.0),
                            resizable: true,
                            decorations: true,
                            transparent: false,
                            position: bevy::window::WindowPosition::Centered,
                            ..Default::default()
                        }),
                        ..Default::default()
                    })
            )
            .insert_resource(GameResource { state: game_state })
            .add_systems(Startup, setup)
            .add_systems(Update, (
                handle_card_click,
                handle_entry_click,
                handle_end_turn,
                advance_phase_system,
                update_ui,
            ))
            .run();

        logging::log_error("WINDOW DID NOT APPEAR - App::run() returned but no window was shown. GPU/driver crash or Bevy initialization failure.");
        0
    }).unwrap_or_else(|_| {
        logging::log_error("Process panicked before App::run()");
        101
    });

    logging::log_info(&format!("Process exiting with code: {}", exit_code));
    std::process::exit(exit_code);
}

fn setup(
    mut commands: Commands,
    mut game_res: ResMut<GameResource>,
) {
    commands.spawn(Camera2dBundle::default());
    logging::log_info("Camera spawned");

    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: BackgroundColor(Color::rgb(0.05, 0.03, 0.02)),
        ..default()
    });
    logging::log_info("Background spawned");

    create_top_bar(&mut commands);
    logging::log_info("Top bar created");

    let entry_width = 175.0;
    let entry_height = 125.0;
    let start_x = -370.0;
    let start_y = 70.0;
    let gap_x = 190.0;
    let gap_y = 145.0;
    let cols = 4;

    for (i, entry) in game_res.state.entry_points.iter().enumerate() {
        let row = i / cols;
        let col = i % cols;
        let x = start_x + (col as f32) * gap_x;
        let y = start_y - (row as f32) * gap_y;
        create_entry_point_ui(&mut commands, entry, Vec3::new(x, y, 0.0), entry_width, entry_height);
    }
    logging::log_info(&format!("{} entry points spawned", game_res.state.entry_points.len()));

    create_hand_area(&mut commands);
    create_end_turn_button(&mut commands);
    logging::log_info("Hand area and end turn button created");

    game_res.state.draw_cards(3);
    game_res.state.phase = Phase::Day;
    logging::log_info("Initial hand drawn, phase set to Day");

    spawn_hand(&mut commands, &mut game_res);
}

fn spawn_hand(commands: &mut Commands, game_res: &mut GameResource) {
    let hand_size = game_res.state.hand.len();
    if hand_size == 0 { return; }
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
    logging::log_info(&format!("{} cards spawned in hand", hand_size));
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
            let entry_id = entry_comp.entry_id;
            if let Some(ref card) = game_res.state.selected_card {
                let card_clone = card.clone();
                game_res.state.play_card(&card_clone, entry_id);
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
            game_res.state.collect_resources();
            game_res.state.advance_day();
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
    mut ui_queries: ParamSet<(
        Query<&mut Text, With<DayCounter>>,
        Query<&mut Text, With<DarknessIndicator>>,
        Query<&mut Text, With<PhaseIndicator>>,
        Query<&mut Text, With<ResourcesDisplay>>,
    )>,
) {
    let state = &game_res.state;

    update_day_counter(&mut ui_queries.p0(), state.day);
    update_darkness_indicator(&mut ui_queries.p1(), state.darkness_level);

    let phase_str = match state.phase {
        Phase::Dawn => "Dawn",
        Phase::Day => "Day",
        Phase::Dusk => "Dusk",
        Phase::Night => "Night",
        Phase::EndTurn => "End",
    };
    update_phase_indicator(&mut ui_queries.p2(), phase_str);

    update_resources_display(
        &mut ui_queries.p3(),
        state.resources.gold,
        state.resources.ore,
        state.resources.beer,
        state.resources.food,
    );
}
