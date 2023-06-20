use bevy::{prelude::*, window::WindowResized, math::vec3, diagnostic::Diagnostics};

const BACKGROUND_COLOR:Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Component)]
struct FpsCounter;

#[derive(Component)]
struct MouseCursorObj;

#[derive(Component)]
struct PlayerObj;

#[derive(Resource)]
struct WindowSize {
    width:f32,
    height:f32
}

fn main() {
    App::new()
        .insert_resource(ClearColor( BACKGROUND_COLOR ))
        .insert_resource(WindowSize{width:100.0,height:100.0})
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup_window)
        .add_startup_system(setup_entities)
        .add_startup_system(setup_ui)
        .add_system(window_resized)
        .add_system(player_movement)
        .add_system(aim_at_mouse)
        .add_system(firing_test)
        .add_system(update_fps_counter)
        .run();
}

fn setup_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let ball_radius = 8.0;
    let ball_mesh:Mesh = shape::Circle::new(ball_radius).into();
    let ball_material = ColorMaterial::from(Color::rgb(100.,100.,100.));
    let ball_pos: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    commands.spawn(( 
        ColorMesh2dBundle {
            mesh: meshes.add(ball_mesh).into(),
            material: materials.add(ball_material),
            transform: Transform::from_translation(ball_pos),
            ..default()
        },
        MouseCursorObj,
    ));

    let triangle_mesh:Mesh = shape::RegularPolygon::new(20.0, 3).into();
    let ball_material = ColorMaterial::from(Color::rgb(100.,100.,100.));
    let ball_pos: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    commands.spawn(( 
        ColorMesh2dBundle {
            mesh: meshes.add(triangle_mesh).into(),
            material: materials.add(ball_material),
            transform: Transform::from_translation(ball_pos),
            ..default()
        },
        PlayerObj,
    ));
}



fn aim_at_mouse (
    mut mouse: EventReader<CursorMoved>,
    mut look_at_mouse: Query<&mut Transform, (With<PlayerObj>, Without<MouseCursorObj>)>,
    mut cursor_obj: Query<&mut Transform, (With<MouseCursorObj>, Without<PlayerObj>)>,
    window_size: Res<WindowSize>,
) {
    let mut cursor_obj = cursor_obj.get_single_mut().expect("cursor Object brokie");
    let mut player = look_at_mouse.get_single_mut().expect("Player obj brokie");

    for ev in mouse.iter() {

        cursor_obj.translation.x = ev.position.x - window_size.width/2.0;
        cursor_obj.translation.y = ev.position.y - window_size.height/2.0;

    }

    let x = player.translation.x;
    let y = player.translation.y;

    player.look_at(
        vec3(x, y, 999.9),
        vec3(cursor_obj.translation.x-x, cursor_obj.translation.y-y, 0.0)
    );

}

fn player_movement(
    // player comp query
    mut player: Query<&mut Transform, With<PlayerObj>>,
    // camera to move
    mut camera: Query<&mut Transform, (With<Camera>, Without<PlayerObj>)>,
    // Keyboard input events
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut player = player.get_single_mut().expect("No players or more than one player");
    let mut camera = camera.get_single_mut().expect("No camera or more than one camera");

    // to normalise the speed when going diagonal, add keypress speeds to a vector than normalise the vector, then add this vector the the player.translation
    // This vec3 just allows me to normalise speed when going diagonal to avoid diagonal movement being faster then normal movement
    let mut velocity = vec3(0.0, 0.0, 0.0);

    // players speed, might move to a resource to allow it to be modified by other functions
    let speed = 5.0;
    
    if keyboard_input.pressed(KeyCode::W) {
        velocity.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        velocity.y -= 1.0
    }
    if keyboard_input.pressed(KeyCode::D) {
        velocity.x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        velocity.x -= 1.0;
    }

    player.translation += velocity.normalize_or_zero() * speed;
    
    // Loose camera follow could be done by checking dist from player to camera and applying velocity*speed to them if it's larger than a arbitrary follow dist thingy
    // for dist theres probably an inbuilt function or pythagoras or just check x and y and move them seperatly

    // Cba right now it doesn't matter as much as getting normal functionality going like shooting and stuff (do it if i can't be bothered with other shit)
    // I also just realised i don't have an actual plan for what this game is about lmao, just mindlessly making games for some fucking reason

    
}

fn firing_test(
    player: Query<&Transform, With<PlayerObj>>,
    cursor_obj: Query<&Transform, With<MouseCursorObj>>,
    
) {
    
}






// Should probably move the fps counter functions to a plugin or whatever
fn update_fps_counter(
    mut fps_counter_qur: Query<&mut Text, With<FpsCounter>>,
    diag: Res<Diagnostics>,
) {
    // MY EYES
    let fps = diag.get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS).and_then(|fps| fps.average()).unwrap_or(0.0);
    let mut fps_counter = fps_counter_qur.get_single_mut().expect("fps counter doesn't exist");
    fps_counter.sections[0].value = format!("fps:{:.5}",fps).to_string();
}


fn setup_ui(
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


fn setup_window(
    mut resize_event: EventReader<WindowResized>,
    mut window_size: ResMut<WindowSize>,
) {
    // let window = resize_event.iter().nth(0).expect("Window Setup Failed");
    for i in resize_event.iter() {
        window_size.width = i.width;
        window_size.height = i.height;
    }
}


fn window_resized(
    mut resize_event: EventReader<WindowResized>,
    mut window_size: ResMut<WindowSize>,
) {
    for i in resize_event.iter() {
        window_size.width = i.width;
        window_size.height = i.height;
    }
}
