use bevy::{prelude::*, window::WindowResized, math::vec3};

const BACKGROUND_COLOR:Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Component)]
struct LookAtMouse();

#[derive(Component)]
struct MouseCursorObj();

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
        .add_startup_system(setup_window)
        .add_startup_system(setup_entities)
        .add_system(window_resized)
        .add_system(aim_at_mouse)
        .add_system(player_movement)
        
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
        MouseCursorObj{},
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
        LookAtMouse{},
    ));
}



fn aim_at_mouse (
    mut mouse: EventReader<CursorMoved>,
    mut look_at_mouse: Query<(&mut Transform), (With<LookAtMouse>, Without<MouseCursorObj>)>,
    mut cursor_obj: Query<(&mut Transform), (With<MouseCursorObj>, Without<LookAtMouse>)>,
    window_size: Res<WindowSize>,
) {

    for ev in mouse.iter() {

        let mouse_x = ev.position.x - window_size.width/2.0;
        let mouse_y = ev.position.y - window_size.height/2.0;

        for mut trans in &mut cursor_obj{
            trans.translation.x = mouse_x;
            trans.translation.y = mouse_y;
        }

        for mut trans in &mut look_at_mouse {
            let x = trans.translation.x;
            let y = trans.translation.y;
            trans.look_at(vec3(x, y, 999.9), vec3(mouse_x, mouse_y, 0.0));
        }
    }
}

fn player_movement(
    // player comp quer,
    // Keyboard input events
    // camera to move
) {

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
