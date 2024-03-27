use crate::*;
use bevy_ggrs::AddRollbackCommandExtension;
use crate::components::arena::ARENA_LENGTH;
use crate::components::textures::striped_texture;

const RADIUS: f32 = 2.25 / 10. * 5. / 2.;

pub fn init(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    images: &mut ResMut<Assets<Image>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    macro_rules! striped_material {
        ($stripe_color:expr) => {
            materials.add(StandardMaterial {
                base_color_texture: Some(images.add(striped_texture($stripe_color))),
                ..default()
            })
        };
    }
    
    // u8, vec3 with middle field being radius
    macro_rules! spawn_ball {
        ($number:expr, $position:expr) => {
            let ball_finish = Ball::new($number);

            let material = match ball_finish.ball_type {
                BallType::Solid => materials.add(ball_finish.color),
                BallType::Striped => striped_material!(ball_finish.color),
                BallType::Cue => materials.add(Color::WHITE),
                BallType::Eight => materials.add(Color::BLACK),
            };

            let mut ball = commands.spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(RADIUS).mesh().uv(32, 16)),
                material,
                transform: Transform::from_translation($position),
                ..default()
            });
        
            ball.insert(ball_finish)
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(RADIUS))
                .insert(Friction::coefficient(1.))
                .insert(Velocity::linear(Vec3::ZERO))
                .insert(Restitution::coefficient(0.5))
                .insert(Damping { linear_damping: 0.9, angular_damping: 0.9, });
            ball.add_rollback();

            if $number == 0 {
                ball.insert(Player { turn: 0, charge: 0., });
                ball.insert(ActiveEvents::COLLISION_EVENTS);
            }
        };
    }

    // spawn white ball on one side
    spawn_ball!(0, Vec3::new(-ARENA_LENGTH / 4., RADIUS, 0.));

    // center of other side
    let anchor = Vec3::new(ARENA_LENGTH / 4., RADIUS, 0.);

    let row_x_offset = (3.0f32.sqrt()) * RADIUS; //sqrt(3)r
    let row_z_offset = -RADIUS;
    let column_offset = 2. * RADIUS;

    let mut positions: Vec<Vec3> = Vec::new();
    for ball_number in 1..16 {
        let (row, col) = calc_pos(ball_number);
        let mut pos = anchor;
        pos += Vec3::new(row_x_offset, 0., row_z_offset) * row; 
        pos += Vec3::new(0., 0., column_offset) * col;
        positions.push(pos);
    }

    use rand::thread_rng;
    use rand::seq::SliceRandom;

    let mut rng = thread_rng();
    positions.shuffle(&mut rng);

    for (i, pos) in positions.iter().enumerate() {
        spawn_ball!(i as u8 + 1, *pos);
    }

    fn calc_pos(number: u8) -> (f32, f32) {
        // overkill, but since every row has one more ball than the previous one, we can use:
        //      k(k+1)/2 >= n
        // to get the row, and:
        //      n - k(k-1)/2
        // to get the column
        let n = number as f32;
        let row = (((8. * n + 1.).sqrt() - 1.) / 2.).ceil();
        let column = n - (row * (row - 1.) / 2.).floor();
        (row -1., column -1.)
    }
}