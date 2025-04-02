# bevyapp

# BEVY
- use Entity Component System paradigm.

- Components: Rust structs that implement the Component trait
```
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
```

- Systems: normal Rust functions
```
fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}

```

- Entities: a simple type containing a unique 
```
struct Entity(u64);
```

# Bevy code Organization
- Bundles
- Collections of components that can be added to entities in one go, ensuring consistency and reducing boilerplate.

- Usually used to define a common set of components that can be reused across multiple entities.

- Bevy provides numerous built-in bundles for common functionality
, enhancing both ease of use and code clarity.

- Can also be nested inside other bundles.

# Plugins
- Allow for modular code organization, encapsulating specific features or functionalities.

- Cofigure and extend your Bevy application by implementing
the plugins trait and its build method.

# Vectors
- A vector is an object that has both a direction and a magnitude.

# Cameras
- Cameras determine which part of the game world is visible on the screen to the players.

- To position the camera in a Bevy game, you can manipulate its associated Transform component, which represnts the camera's position, rotation, and scale within the 3D space.

- Bevy provides built-in bundles to add and configure.

# AssetServer & Loading assets
Bevy provides AssetServer to load assets. For 3D models, Bevy has built-in support for GLTF file format

