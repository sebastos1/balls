use crate::*;
use bevy::utils::HashSet;

pub fn collision(
    mut collisions: EventReader<CollisionEvent>,
    balls: Query<(Entity, &Player)>,
    grounds: Query<Entity, With<Ground>>,
) {
    let grounds = grounds.iter().collect::<HashSet<_>>();
    for collision in collisions.read() {
        match collision {
            CollisionEvent::Started(entity1, entity2, _) => {
                // info!("Collision START: {:?} + {:?}", entity1, entity2);
                for (ball_entity, _) in balls.iter() {
                    if *entity2 == ball_entity && grounds.contains(entity1) {
                        // ball.grounded = true;
                    }
                }
            },
            CollisionEvent::Stopped(entity1, entity2, _) => {
                // info!("Collision STOP: {:?} - {:?}", entity1, entity2);
                for (ball_entity, _) in balls.iter() {
                    if *entity2 == ball_entity && grounds.contains(entity1) {
                        // ball.grounded = false;
                    }
                }
            }
        }
    }
}