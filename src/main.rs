use bevy::prelude::*;

//
#[derive(Component)]
struct Person;
#[derive(Component)]
struct Name(String);

// Component

// System
fn hello_world_system() {
    println!("Hello World!");
}
//
fn startup_system(mut commands: Commands) {
    commands.spawn((Person, Name("Alice".to_string())));
    commands.spawn((Person, Name("Bob".to_string())));
    commands.spawn((Person, Name("Charlie".to_string())));
}

fn greeting_people_system(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello, {}!", name.0);
    }
}

fn main() {
    App::new()
        .add_systems(Startup, startup_system)
        .add_systems(Update, (hello_world_system, greeting_people_system))
        .run();
}
