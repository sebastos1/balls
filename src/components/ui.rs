use crate::*;

pub fn init(commands: &mut Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {

            // Crosshair container
            parent.spawn(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|crosshair_container| {
                // Crosshair
                crosshair_container.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(5.),
                        height: Val::Px(5.),
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    background_color: Color::WHITE.into(),
                    ..Default::default()
                });
            });

            // Cooldown meter container
            parent.spawn(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Px(5.),
                    bottom: Val::Px(5.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|cooldown_container| {
                cooldown_container.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(100.),
                        height: Val::Px(10.),
                        ..Default::default()
                    },
                    background_color: Color::RED.into(),
                    ..Default::default()
                }).insert(Meter {
                    max: 1.,
                });
            });
        });
}

pub fn update_meters(
    ball: Query<&Ball>,
    mut query: Query<(&Meter, &mut Style), With<Meter>>
) {
    let ball = ball.single();
    let (meter, mut style) = query.single_mut();

    info!("{:?}", style.width);

    style.width = Val::Px(100. - (100. * ball.cooldown / meter.max));
}