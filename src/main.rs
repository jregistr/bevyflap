use bevy::prelude::*;

fn main() {
    App::build()
    .add_plugins(DefaultPlugins)
    .add_plugin(HelloPlugin)
    .run();
}

struct Person;

struct Name(String);

fn hello_world() {
    println!("hello world!");
}

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
        .add_startup_system(add_people.system())
        .add_system(greet_people.system())
        .insert_resource(GreetTimer(Timer::from_seconds(5.0, true)));
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn()
    .insert(Person)
    .insert(Name(String::from("Jack")));

    commands.spawn()
    .insert(Person)
    .insert(Name(String::from("Forkly")));
}

struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello {}", name.0)
        }
    }
}