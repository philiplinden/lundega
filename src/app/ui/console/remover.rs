use bevy::prelude::*;
use bevy_console::{AddConsoleCommand, ConsoleCommand};
use clap::Parser;

pub(super) fn plugin(app: &mut App) {
    app.add_console_command::<RemoveEntityCommand, _>(remove_entity_command);
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "remove-entity")]
struct RemoveEntityCommand {
    /// Entity to remove
    entity_id: u32,
}

fn remove_entity_command(mut log: ConsoleCommand<RemoveEntityCommand>, mut commands: Commands) {
    if let Some(Ok(RemoveEntityCommand { entity_id })) = log.take() {
        commands.entity(Entity::from_raw(entity_id)).despawn_recursive();
        log.ok();
    }
}
