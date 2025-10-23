use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_sprite3d::*;

#[derive(States, Hash, Clone, PartialEq, Eq, Debug, Default)]
enum GameState {
    #[default]
    Loading,
    Ready,
}

#[derive(Resource, Default)]
struct ImageAssets {
    image: Handle<Image>,
    layout: Handle<TextureAtlasLayout>,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct SpinMarker;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(Sprite3dPlugin)
        .init_state::<GameState>()
        .insert_resource(ImageAssets::default())
        .add_systems(Startup, load_assets)
        .add_systems(Update, setup.run_if(in_state(GameState::Loading)))
        .add_systems(
            Update,
            (animate_sprite, spin_cube).run_if(in_state(GameState::Ready)),
        )
        .run();
}

fn load_assets(
    asset_server: Res<AssetServer>,
    mut assets: ResMut<ImageAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    assets.image = asset_server.load("gabe-idle-run.png");
    assets.layout = texture_atlases.add(TextureAtlasLayout::from_grid(
        UVec2::new(24, 24),
        7,
        1,
        None,
        None,
    ));
}

fn setup(
    asset_server: Res<AssetServer>,
    assets: Res<ImageAssets>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut sprite_params: Sprite3dParams,
) {
    if asset_server.get_load_state(assets.image.id()) != Some(LoadState::Loaded) {
        return;
    }

    next_state.set(GameState::Ready);

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    let texture_atlas = TextureAtlas {
        layout: assets.layout.clone(),
        index: 0,
    };

    commands
        .spawn(
            SpriteCube3d {
                image: assets.image.clone(),
                pixels_per_metre: 32.0,
                alpha_mode: AlphaMode::Opaque,
                unlit: true,
                transform: Transform::from_xyz(0.0, 1.0, 0.0),
                emissive: LinearRgba::WHITE,
                ..default()
            }
            .bundle_with_atlas(&mut sprite_params, texture_atlas),
        )
        .insert(AnimationTimer(Timer::from_seconds(0.08, TimerMode::Repeating)))
        .insert(SpinMarker);

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1200.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 6.0, 4.0),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 7000.0,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            -std::f32::consts::FRAC_PI_4,
            -std::f32::consts::FRAC_PI_6,
        )),
        ..default()
    });
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas, &TextureAtlas3dData)>,
) {
    for (mut timer, mut atlas, data) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = (atlas.index + 1) % data.keys.len();
        }
    }
}

fn spin_cube(time: Res<Time>, mut query: Query<&mut Transform, With<SpinMarker>>) {
    for mut transform in &mut query {
        transform.rotate_y(time.delta_seconds() * 1.8);
        transform.rotate_x(time.delta_seconds() * 0.9);
    }
}
