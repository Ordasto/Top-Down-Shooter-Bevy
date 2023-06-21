use bevy::{prelude::*, window::WindowResized, math::{vec3}, diagnostic::Diagnostics};

#[derive(Component)]
struct FpsCounter;

#[derive(Resource)]
pub struct WindowSize {
    width:f32,
    height:f32
}

#[derive(Resource)]
pub struct MousePosition {
    pub x:f32,
    pub y:f32,
}
pub struct WindowManager;

impl Plugin for WindowManager {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(WindowSize{width:100.0,height:100.0})
        .insert_resource(MousePosition{x:0.0,y:0.0})
        .add_startup_system(setup_window)
        .add_startup_system(setup_fps_counter)
        .add_system(mouse_pos_updater)
        .add_system(window_resize_handler)
        .add_system(update_fps_counter);
    }
}


fn update_fps_counter(
    mut fps_counter_qur: Query<&mut Text, With<FpsCounter>>,
    diag: Res<Diagnostics>,
) {
    // MY EYES
    let fps = diag.get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS).and_then(|fps| fps.average()).unwrap_or(0.0);
    let mut fps_counter = fps_counter_qur.get_single_mut().expect("fps counter doesn't exist");
    // the "{:.5}" is the number of decimal places to display ("{:.3}" would display 3 decimal places)
    fps_counter.sections[0].value = format!("fps:{:.5}",fps).to_string();
}

fn setup_fps_counter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Res<WindowSize>
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 20.0,
        color: Color::WHITE,
    };

    let text_alignment = TextAlignment::Left;

    commands.spawn((
        Text2dBundle{
            text: Text::from_section("fps:", text_style.clone()).with_alignment(text_alignment),
            transform: Transform::from_translation( vec3(-window.width/2.0, window.height/2.0, 0.0)),
            text_anchor: bevy::sprite::Anchor::TopLeft,
            ..default()
        },
        FpsCounter,
    ));
}


fn mouse_pos_updater(
    mut mouse: EventReader<CursorMoved>,
    mut mouse_pos: ResMut<MousePosition>,
    window_size: Res<WindowSize>,
) {
    for ev in mouse.iter() {
        mouse_pos.x = ev.position.x - window_size.width/2.0;
        mouse_pos.y = ev.position.y - window_size.height/2.0;
    }
}

fn setup_window(
    mut resize_event: EventReader<WindowResized>,
    mut window_size: ResMut<WindowSize>,
) {
    for i in resize_event.iter() {
        window_size.width = i.width;
        window_size.height = i.height;
    }
}

fn window_resize_handler(
    mut resize_event: EventReader<WindowResized>,
    mut window_size: ResMut<WindowSize>,
) {
    for i in resize_event.iter() {
        window_size.width = i.width;
        window_size.height = i.height;
    }
}
