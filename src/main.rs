use animation::{AnimationIndex, SpriteAnimation, SpriteAnimator, tick_animations};
use bevy::{
    math::NormedVectorSpace, prelude::*, sprite::Anchor, text::FontSmoothing,
    window::WindowResolution,
};
use rts::NavigationDestination;

mod animation;
mod rts;
mod stage;
mod talksim;
mod virtualpet;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(640.0, 480.0)
                            .with_scale_factor_override(1.0),
                        title: String::from("Welcome to Virtual Pet!"),
                        ..default()
                    }),
                    ..default()
                }),
            MeshPickingPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.9, 0.9, 0.9)))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (sprite_movement, animate_walk, tick_animations).chain(),
        )
        .add_systems(Update, (camera_control, handle_click))
        .add_observer(testevent)
        .run();
}

#[derive(Clone, Debug, Resource)]
pub struct RtsController {
    pub selected_entity: Option<Entity>,
}

#[derive(Clone, Debug, Resource)]
pub struct Cursor {
    pub carrying: Option<Entity>,
    pub max_weight: usize,
}

#[derive(Clone, Debug, Component)]
pub struct CarriedByCursor;

#[derive(Clone, Debug, Component)]
pub struct Carriable {
    pub weight: usize,
}

#[derive(Clone, Debug, Component)]
struct CameraController {
    pub scale: f32,
    pub offset: Vec2,
}

#[derive(Clone, Debug, Component)]
struct Hitbox {
    pub size: Vec2,
}
impl Hitbox {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            size: Vec2::new(x, y),
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Camera2d,
        CameraController {
            scale: 0.25,
            offset: Vec2::new(0.0, 0.0),
        },
    ));

    commands.spawn((
        Sprite::from_image(asset_server.load("bg/room1.png")),
        Transform::from_xyz(0., 0., 0.),
    ));
    commands.spawn((
        Sprite {
            // anchor: Anchor(Vec2::new(0.0, -0.5)),
            ..default()
        },
        Anchor(Vec2::new(0.0, -0.5)),
        Transform::from_xyz(0., -30., 1.),
        NavigationDestination(Some(Vec2::new(30., -70.))),
        SpriteAnimator::new(vec![
            SpriteAnimation::new(
                0.5,
                vec![
                    asset_server.load("obj/girl_idle1.png"),
                    asset_server.load("obj/girl_idle2.png"),
                ],
            ),
            SpriteAnimation::new(
                0.3,
                vec![
                    asset_server.load("obj/girl_walk1.png"),
                    asset_server.load("obj/girl_walk2.png"),
                ],
            ),
        ]), // Move,
    ));
    commands.spawn((
        Sprite {
            image: asset_server.load("obj/plate.png"),
            ..default()
        },
        Transform::from_xyz(0., -70., 0.5),
        Carriable { weight: 1 },
        Hitbox {
            size: Vec2::new(10., 10.),
        },
        children![(Sprite {
            image: asset_server.load("obj/food_pellets.png"),
            ..default()
        },)],
    ));
    commands.spawn(textbox(&asset_server));
    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        Transform::from_xyz(0., 0., 0.),
    ));
}

fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Transform, &mut NavigationDestination)>,
) {
    for (mut transform, mut target) in &mut sprite_position {
        if let Some(dest) = target.0 {
            let current = transform.translation.truncate();
            if transform.translation.truncate().distance_squared(dest) < 1.0 {
                target.0 = None;
            } else {
                let dir = current.move_towards(dest, 75. * time.delta_secs());
                transform.translation.x = dir.x;
                transform.translation.y = dir.y;
            }
        }
    }
}

fn animate_walk(
    time: Res<Time>,
    mut sprite_position: Query<(&Transform, &NavigationDestination, &mut SpriteAnimator)>,
) {
    for (transform, target, mut animator) in &mut sprite_position {
        if let Some(dest) = target.0 {
            let dir = (dest - transform.translation.truncate()).normalize_or_zero();
            animator.direction = Dir2::from_xy(dir.x, dir.y).unwrap_or(animator.direction);
            animator.animation_index = AnimationIndex::Walking;
        } else {
            animator.set_animation_index(AnimationIndex::Idle);
        }
    }
}

fn camera_control(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Transform, &mut CameraController)>,
    key: Res<ButtonInput<KeyCode>>,
) {
    for (mut transform, mut controller) in &mut sprite_position {
        if key.pressed(KeyCode::Equal) {
            controller.scale /= 1.0 + 1.0 * time.delta_secs();
        }
        if key.pressed(KeyCode::Minus) {
            controller.scale *= 1.0 + 1.0 * time.delta_secs();
        }
        if key.just_pressed(KeyCode::Digit0) {
            controller.scale = 1.0;
        }
        let speed = 250.0 * controller.scale;
        if key.pressed(KeyCode::ArrowUp) {
            controller.offset.y += speed * time.delta_secs();
        }
        if key.pressed(KeyCode::ArrowDown) {
            controller.offset.y -= speed * time.delta_secs();
        }
        if key.pressed(KeyCode::ArrowRight) {
            controller.offset.x += speed * time.delta_secs();
        }
        if key.pressed(KeyCode::ArrowLeft) {
            controller.offset.x -= speed * time.delta_secs();
        }
        transform.translation.x = controller.offset.x;
        transform.translation.y = controller.offset.y;
        transform.scale = Vec3::new(controller.scale, controller.scale, 1.);
    }
}

fn textbox(asset_server: &AssetServer) -> impl Bundle + use<> {
    let slicer = TextureSlicer {
        border: BorderRect::all(20.0),
        center_scale_mode: SliceScaleMode::Tile { stretch_value: 1.5 },
        sides_scale_mode: SliceScaleMode::Tile { stretch_value: 1.5 },
        max_corner_scale: 2.0,
    };
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            Node {
                width: Val::Percent(80.0),
                min_width: Val::Px(480.0),
                height: Val::Percent(20.0),
                max_height: Val::Px(480.0),
                min_height: Val::Px(240.0),
                border: UiRect::all(Val::Px(4.0)),
                margin: UiRect::all(Val::Px(50.0)),
                padding: UiRect::all(Val::Px(50.0)),
                // horizontally center child text
                justify_content: JustifyContent::Start,
                // vertically center child text
                align_items: AlignItems::Start,
                ..default()
            },
            BorderColor::all(Color::srgb(0.6, 0.7, 0.8)),
            // BorderRadius::MAX,
            BackgroundColor(Color::WHITE.darker(0.1)),
            children![(
                Text::new("Welcome to VirtualPet!\nゲームが下手ですね。\nありがとうございました"),
                TextFont {
                    font: asset_server.load("fonts/unifont-16.0.03.otf"),
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::srgb(0.1, 0.05, 0.02)),
            )]
        )],
    )
}

fn handle_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut commands: Commands,
) {
    let Ok(windows) = windows.single() else {
        return;
    };

    let (camera, camera_transform) = *camera;
    if let Some(pos) = windows
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        if mouse_button_input.pressed(MouseButton::Left) {
            commands.trigger(MoveClick { pos });
        }
    }
}

#[derive(Event)]
struct MoveClick {
    pos: Vec2,
}

fn testevent(
    trigger: Trigger<MoveClick>,
    mut navigators: Query<&mut NavigationDestination>,
    mut carriable: Query<(&Carriable, &Hitbox)>,
    mut commands: Commands,
) {
    // You can access the trigger data via the `Observer`
    let event = trigger.event();

    for (carriable, mut aabb) in carriable.iter_mut() {
        // Perform actions based on the carriable and its AABB
        debug!("Carriable: {:?}, AABB: {:?}", carriable, aabb);
    }

    // Access resources
    for mut i in navigators.iter_mut() {
        i.0 = Some(event.pos);
    }
}
