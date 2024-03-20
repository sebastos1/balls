use crate::*;
use bevy::utils::HashSet;

pub fn collision(
    mut collisions: EventReader<CollisionEvent>,
    mut ball: Query<(Entity, &mut Ball)>,
    grounds: Query<Entity, With<Ground>>,
) {
    let (ball_entity, mut ball) = ball.single_mut();
    let grounds = grounds.iter().collect::<HashSet<_>>();

    ball.grounded = false;

    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(entity1, entity2, _) => {
                info!("Collision started between {:?} and {:?}", entity1, entity2);
                info!("Ground entities: {:?}", grounds);

                if grounds.contains(entity1) && entity2 == &ball_entity {
                    ball.grounded = true;
                }
            },
            CollisionEvent::Stopped(entity1, entity2, _) => {
                info!("Collision stopped between {:?} and {:?}", entity1, entity2);
            }
        }
    }
}