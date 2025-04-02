use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Position{
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(pub String);