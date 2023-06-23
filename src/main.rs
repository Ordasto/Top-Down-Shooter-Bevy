use bevy::{prelude::*, math::{vec3, vec2}, sprite::collide_aabb};
mod window_manager;
use window_manager::*;
const BACKGROUND_COLOR:Color = Color::rgb(0.15, 0.15, 0.15);


#[derive(Component)]
struct RayCollision;

#[derive(Component)]
struct RayObj;

#[derive(Component)]
struct MouseCursorObj;

#[derive(Component)]
struct PlayerObj;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum GameState {
    MainMenu,
    SettingsMenu,
    #[default]
    InGame,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .insert_resource(ClearColor( BACKGROUND_COLOR ))
        .insert_resource(MousePosition{x:0.0,y:0.0})

        .add_plugins(DefaultPlugins)
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WindowManager)

        .add_startup_system(setup_entities)

        // Ingame systems
        .add_systems((
            player_movement,
            aim_at_mouse,
            firing_test,
            draw_ray
        ).distributive_run_if(in_state(GameState::InGame)))
            
        // // Main Menu systems (when you quit or start the game)
        // .add_systems((
            
        // ).distributive_run_if(in_state(GameState::MainMenu)))

        // // Settings systems (when you press escape)
        // .add_systems((

        // ).distributive_run_if(in_state(GameState::SettingsMenu)))

        .run();
}

fn setup_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // cursor
    let cursor_radius = 4.0;
    // let cursor_mesh:Mesh = shape::Circle::new(cursor_radius).into();
    let cursor_mesh:Mesh = shape::Quad::new(vec2(1.0, 1.0)).into();
    let cursor_material = ColorMaterial::from(Color::rgb(100.,100.,100.));
    let cursor_pos: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    commands.spawn(( 
        ColorMesh2dBundle {
            mesh: meshes.add(cursor_mesh).into(),
            material: materials.add(cursor_material),
            transform: Transform::from_translation(cursor_pos),
            ..default()
        },
        MouseCursorObj,
    ));

    // Player triangle
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

    // Test Collision object
    let quad_mesh:Mesh = shape::Quad::new(vec2(50.0, 20.0)).into();
    let quad_material = ColorMaterial::from(Color::rgb(100.,100.,100.));
    let quad_pos: Vec3 = Vec3::new(100.0, 200.0, 0.0);

    commands.spawn(( 
        ColorMesh2dBundle {
            mesh: meshes.add(quad_mesh).into(),
            material: materials.add(quad_material),
            transform: Transform::from_translation(quad_pos),
            ..default()
        },
        RayCollision
    ));


    // draw ray object
    let quad_mesh:Mesh = shape::Quad::new(vec2(1.0, 1.0)).into();
    let quad_material = ColorMaterial::from(Color::rgb(100.,100.,100.));
    let quad_pos: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    commands.spawn(( 
        ColorMesh2dBundle {
            mesh: meshes.add(quad_mesh).into(),
            material: materials.add(quad_material),
            transform: Transform::from_translation(quad_pos),
            ..default()
        },
        RayObj
    ));

}

fn aim_at_mouse (
    mut look_at_mouse: Query<&mut Transform, With<PlayerObj>>,
    mouse_pos: Res<MousePosition>,
) {
    let mut player = look_at_mouse.get_single_mut().expect("Player obj brokie");

    let x = player.translation.x;
    let y = player.translation.y;

    player.look_at(
        vec3(x, y, 999.9),
        vec3(mouse_pos.x-x, mouse_pos.y-y, 0.0)
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
    collision_objects: Query<(&bevy::sprite::Mesh2dHandle, &Transform), With<RayCollision>>,
    meshes: Res<Assets<Mesh>>,
    mouse_pos: Res<MousePosition>,

    // for debug coloring ray_object and stuff
    mut cursor_obj: Query<&mut Transform, (With<MouseCursorObj>, Without<PlayerObj>, Without<RayCollision>, Without<RayObj>)>,
    mut ray_object: Query<&Handle<ColorMaterial>, With<RayObj>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut ray_obj_material = materials.get_mut(ray_object.single_mut()).unwrap();

    let player = player.single();

    let dir = -vec3(player.translation.x-mouse_pos.x, player.translation.y-mouse_pos.y, player.translation.z).normalize();

    // cursor.translation = player.translation+dir*100.0;
    
    for (mesh_handle, transform) in &collision_objects {
        let mesh = meshes.get(&mesh_handle.0).expect("\n\n[MESH DOESN'T EXIST]\n\n");
        // Simple collision, just aabb collision with the second collision just being a point on the ray of the same distance as the player to the object in question
        // could check on multiple points while getting closer


        let a_pos = player.translation+dir * player.translation.distance(transform.translation);
        let a_size = vec2(1.0, 1.0);
        
        
        let b_aabb = mesh.compute_aabb().unwrap();
        
        let b_pos = transform.translation;
        let b_size = b_aabb.half_extents*2.0;
        
        let collision = collide_aabb::collide(a_pos, a_size, b_pos, vec2(b_size.x, b_size.y));
        
        if collision.is_some() {
            ray_obj_material.color.set_b(0.0);
            ray_obj_material.color.set_g(0.0);
        }else{
            ray_obj_material.color.set_b(1.0);
            ray_obj_material.color.set_g(1.0);
        }
        
        let mut cursor = cursor_obj.single_mut();

        cursor.translation = a_pos*0.95;
        cursor.scale = vec3(3.0,3.0,0.0)
        // cursor.scale = b_aabb.half_extents.into()
        
    }
}

fn draw_ray(
    player: Query<&Transform, With<PlayerObj>>,
    mouse_pos: Res<MousePosition>,
    mut ray_object: Query<&mut Transform, (With<RayObj>, Without<PlayerObj>)>,
) {
    // Sets the ray_obj.translation to the midpoint between the mouse and player
    // then sets the length of the ray_obj to the distance between the player and the cursor
    // finally it makes the ray_obj point towards the mouse position (look_at())
    // this has the effect of drawing a line between the player and the cursor

    // Might want to turn this into a plugin or something, twas an absolute pain to make
    let player = player.single();
    let mut ray_obj = ray_object.single_mut();
    
    let mouse_vec = vec3(mouse_pos.x, mouse_pos.y, 0.0);
    ray_obj.translation = ((mouse_vec - player.translation)/2.0)+player.translation;
    ray_obj.scale.y = player.translation.distance(mouse_vec);

    let rt = ray_obj.translation; // Ray-Translation(rt) local store operation
    ray_obj.look_at(
        vec3(rt.x, rt.y, 999.9),
        vec3(mouse_pos.x-rt.x, mouse_pos.y-rt.y, 0.0)
    );
}
