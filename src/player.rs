use crate::actions::Actions;
use crate::GameState;
use crate::sprite_anim::SpriteAnimator;
use crate::actor::*;
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

#[derive(Component, Default, Clone)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(player_inputs))
        ;
    }
}

#[derive(Clone, Default, Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub sprite_animator: SpriteAnimator,
    pub player: Player,
    pub rigidbody: RigidBody,
    pub collider: Collider,
    pub controller: KinematicCharacterController,
    pub actor: Actor,
    pub actor_status: ActorStatus,
}

impl LdtkEntity for PlayerBundle {
    fn bundle_entity(
        _entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        let texture_handle = asset_server.load("sprites/sam1.png");
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(48., 32.), 4, 1, None, None);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        
        PlayerBundle {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                ..Default::default()
            },
            sprite_animator: crate::sprite_anim::SpriteAnimator::new(texture_atlas_handle.clone(), 0, 3, 4, 0.2, true),
            player: Player,
            rigidbody: RigidBody::KinematicPositionBased,
            collider: Collider::capsule_y(7., 7.),
            controller: KinematicCharacterController {
                offset: CharacterLength::Relative(0.1),
                ..Default::default()
            },
            actor: Actor {
                move_speed: 60.,
                drag: 0.1,
                accel: 500., 
                deccel: 1000.,
                gravity: 1000.,
                jump_speed: 1000.,
                jump_time: 0.1,
                move_input: 0.,
                jump_input: false,
            },
            actor_status: ActorStatus {
                grounded: false,
                velocity: Vec2::ZERO,
            }
           
        }
    }
}

fn player_inputs(
    actions: Res<Actions>,
    mut player_query: Query<(&mut Actor, &ActorStatus), With<Player>>,
) {
    let input = Vec2::new(
        actions.player_movement.x,
        actions.player_movement.y,
    );
    for (mut actor, status) in &mut player_query {
        actor.move_input = input.x;
        
        if status.grounded {
            actor.jump_input = actions.jump;
        }
        else {
            actor.jump_input = false;
        }
    }
}