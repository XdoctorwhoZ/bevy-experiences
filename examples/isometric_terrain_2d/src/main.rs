//! Isometric Terrain 2D Example
//!
//! A simple 2D isometric terrain rendering example using Bevy.
//! 
//! To run:
//! ```bash
//! cd examples/isometric_terrain_2d
//! cargo run
//! ```
//!
//! Controls:
//! - WASD / Arrow keys: Move camera
//! - Mouse Wheel: Zoom in/out

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Isometric Terrain 2D".into(),
                resolution: (800.0, 600.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, camera_movement)
        .run();
}

/// Terrain configuration
const TERRAIN_WIDTH: usize = 20;
const TERRAIN_HEIGHT: usize = 15;
const TILE_SIZE: f32 = 64.0;

/// Isometric tile dimensions
const TILE_HALF_WIDTH: f32 = TILE_SIZE / 2.0;
const TILE_HALF_HEIGHT: f32 = TILE_SIZE / 2.0;

/// Colors for different terrain types
const GRASS_COLOR: Color = Color::srgb(0.2, 0.6, 0.2);
const WATER_COLOR: Color = Color::srgb(0.1, 0.3, 0.8);
const SAND_COLOR: Color = Color::srgb(0.8, 0.7, 0.4);
const MOUNTAIN_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);

/// Terrain tile type
#[derive(Clone, Copy, PartialEq, Debug)]
enum TerrainType {
    Grass,
    Water,
    Sand,
    Mountain,
}

impl TerrainType {
    fn color(&self) -> Color {
        match self {
            TerrainType::Grass => GRASS_COLOR,
            TerrainType::Water => WATER_COLOR,
            TerrainType::Sand => SAND_COLOR,
            TerrainType::Mountain => MOUNTAIN_COLOR,
        }
    }
}

/// Generate terrain data with some pattern
fn generate_terrain() -> Vec<Vec<TerrainType>> {
    let mut terrain = vec![vec![TerrainType::Grass; TERRAIN_WIDTH]; TERRAIN_HEIGHT];
    
    // Add some water in the corners
    for y in 0..3 {
        for x in 0..5 {
            terrain[y][x] = TerrainType::Water;
        }
    }
    
    // Add some sand near water
    for y in 2..5 {
        for x in 4..8 {
            terrain[y][x] = TerrainType::Sand;
        }
    }
    
    // Add some mountains
    for y in 8..12 {
        for x in 10..15 {
            terrain[y][x] = TerrainType::Mountain;
        }
    }
    
    terrain
}

/// Convert isometric screen coordinates to world coordinates
fn isometric_to_world(iso_x: f32, iso_y: f32) -> (f32, f32) {
    let world_x = (iso_x - iso_y) / 2.0;
    let world_y = (iso_x + iso_y) / 2.0;
    (world_x, world_y)
}

/// Setup the scene
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn camera
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(10.0),
            ..default()
        },
        transform: Transform::from_xyz(
            TERRAIN_WIDTH as f32 * TILE_HALF_WIDTH,
            TERRAIN_HEIGHT as f32 * TILE_HALF_HEIGHT,
            0.0,
        ),
        ..default()
    });

    // Generate terrain
    let terrain = generate_terrain();
    
    // Spawn terrain tiles
    for y in 0..TERRAIN_HEIGHT {
        for x in 0..TERRAIN_WIDTH {
            let terrain_type = terrain[y][x];
            
            // Calculate isometric position
            let iso_x = (x as f32 - y as f32) * TILE_HALF_WIDTH;
            let iso_y = (x as f32 + y as f32) * TILE_HALF_HEIGHT;
            
            // Convert to world coordinates
            let (world_x, world_y) = isometric_to_world(iso_x, iso_y);
            
            // Spawn tile as a rectangle
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: terrain_type.color(),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(world_x, world_y, 0.0),
                ..default()
            });
            
            // Add a slight border to make tiles visible
            commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(world_x, world_y, 0.1),
                ..default()
            });
        }
    }
    
    // Add some UI text
    commands.spawn(Text2dBundle {
        text: Text::from_section(
            "Isometric Terrain 2D\nWASD: Move Camera\nMouse Wheel: Zoom",
            TextStyle {
                font: asset_server.load("embedded://bevy/default_font.ttf"),
                font_size: 24.0,
                color: Color::WHITE,
            },
        ),
        transform: Transform::from_xyz(10.0, 10.0, 100.0),
        ..default()
    });
}

/// Camera movement system
fn camera_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_wheel: Res<Events<MouseWheel>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
) {
    for (mut transform, mut projection) in &mut query {
        let mut direction = Vec3::ZERO;
        
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        
        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * 500.0 * 0.016;
        }
        
        // Handle zoom
        for event in mouse_wheel.read() {
            let zoom_factor = 1.0 - event.y * 0.1;
            projection.scale *= zoom_factor;
        }
    }
}
