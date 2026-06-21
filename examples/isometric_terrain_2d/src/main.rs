//! Isometric Terrain 2D Example with bevy_ecs_tiled
//!
//! A 2D isometric terrain rendering example using Bevy and bevy_ecs_tiled.
//! This example loads a Tiled map file (.tmx) with isometric orientation.
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
//! - Space: Cycle between maps
//! - P: Randomize tile at position (5, 5)

use bevy::prelude::*;
use bevy::input::mouse::AccumulatedMouseScroll;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tilemap::prelude::{TilePos, TileStorage, TileTextureIndex, TileBundle, TilemapId};

fn main() {
    App::new()
        // Bevy default plugins: prevent blur effect by changing default sampling
        .add_plugins(DefaultPlugins.build().set(ImagePlugin::default_nearest()))
        // Add bevy_ecs_tiled plugin: bevy_ecs_tilemap::TilemapPlugin will
        // be automatically added as well if it's not already done
        .add_plugins(TiledPlugin::default())
        // Add our systems and run the app!
        .add_systems(Startup, startup)
        .add_systems(Update, (camera_movement, cycle_maps, randomize_tile))
        .run();
}

/// Component to track which map is currently loaded
#[derive(Resource)]
struct CurrentMap {
    index: usize,
    maps: Vec<(String, String)>, // (path, description)
}

/// Setup the scene with a Tiled map
fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn camera
    commands.spawn(Camera2d);

    // List of available maps
    let maps = vec![
        ("maps/isometric/finite_diamond.tmx".to_string(), "Finite Diamond Isometric Map".to_string()),
        ("maps/isometric/infinite_diamond.tmx".to_string(), "Infinite Diamond Isometric Map".to_string()),
    ];

    // Clone maps for the resource
    let maps_clone = maps.clone();

    // Insert current map resource
    commands.insert_resource(CurrentMap { index: 0, maps: maps_clone });

    // Load the first map
    let map_handle: Handle<TiledMapAsset> = asset_server.load(&maps[0].0);
    commands.spawn((
        TiledMap(map_handle),
        TilemapAnchor::Center,
        TilemapRenderSettings {
            render_chunk_size: UVec2::new(64, 1),
            y_sort: true,
        },
    ));

    // Add UI text
    commands.spawn((
        Text2d::new("Isometric Terrain 2D\nbevy_ecs_tiled\nWASD: Move Camera\nMouse Wheel: Zoom\nSpace: Switch Map\nP: Randomize Tile"),
        Transform::from_xyz(10.0, 10.0, 100.0),
    ));
}

/// Cycle through available maps
fn cycle_maps(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut current_map: ResMut<CurrentMap>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    map_query: Query<Entity, With<TiledMap>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        // Despawn current map
        for entity in &map_query {
            commands.entity(entity).despawn();
        }
        
        // Move to next map
        current_map.index = (current_map.index + 1) % current_map.maps.len();
        let path = &current_map.maps[current_map.index].0;
        
        // Load new map
        let map_handle: Handle<TiledMapAsset> = asset_server.load(path);
        commands.spawn((
            TiledMap(map_handle),
            TilemapAnchor::Center,
            TilemapRenderSettings {
                render_chunk_size: UVec2::new(64, 1),
                y_sort: true,
            },
        ));
    }
}

/// Randomize tile at a specific position when P is pressed
/// This demonstrates dynamic tile modification at runtime
fn randomize_tile(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut tilemap_query: Query<(Entity, &mut TileStorage)>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyP) {
        // Find the tilemap entity and its storage
        for (tilemap_entity, mut tile_storage) in &mut tilemap_query {
            // Try to find the first tile in the storage
            // Iterate through all possible positions in the storage
            let size = tile_storage.size;
            let mut found = false;
            
            for x in 0..size.x {
                for y in 0..size.y {
                    let target_pos = TilePos { x, y };
                    
                    if let Some(tile_entity) = tile_storage.get(&target_pos) {
                        // Despawn the old tile
                        commands.entity(tile_entity).despawn();
                        
                        // Remove from storage
                        tile_storage.remove(&target_pos);
                        
                        // Get a random tile index (0-4 based on the kenney-sketch-desert tileset)
                        // The tileset has 5 tiles with IDs 0-4
                        let new_tile_index = (rand::random::<f32>() * 5.0) as u32;
                        
                        // Spawn a new tile with the new texture index
                        commands.spawn(TileBundle {
                            position: target_pos,
                            tilemap_id: TilemapId(tilemap_entity),
                            texture_index: TileTextureIndex(new_tile_index),
                            ..Default::default()
                        });
                        
                        println!("Replaced tile at position ({}, {}) with random index {}", 
                                 target_pos.x, target_pos.y, new_tile_index);
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            
            if !found {
                println!("No tiles found in storage (size: {}x{})", size.x, size.y);
            }
        }
    }
}

/// Camera movement system
fn camera_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    mut projection_query: Query<&mut Projection, With<Camera2d>>,
) {
    // Handle movement
    for mut transform in &mut camera_query {
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
    }
    
    // Handle zoom
    for mut projection in &mut projection_query {
        if let Projection::Orthographic(ref mut orthographic) = *projection {
            let zoom_factor = 1.0 - mouse_scroll.delta.y * 0.1;
            orthographic.scale *= zoom_factor;
            orthographic.scale = orthographic.scale.clamp(0.1, 10.0);
        }
    }
}
