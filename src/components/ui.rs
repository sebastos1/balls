use crate::*;

pub fn init(
    commands: &mut Commands,
) {
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
                    left: Val::Px(25.),
                    bottom: Val::Px(25.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|cooldown_container| {
                cooldown_container.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(100.),
                        height: Val::Px(10.),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    background_color: Color::GRAY.into(),
                    ..Default::default()
                });

                cooldown_container.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(100.),
                        height: Val::Px(10.),
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    background_color: Color::GREEN.into(),
                    ..Default::default()
                }).insert(Meter);
            });
        });
}

pub fn update_meters(
    global_charge: Res<GlobalCharge>,
    mut meters: Query<&mut Style, With<Meter>>,
) {
    for mut style in meters.iter_mut() {
        style.width = Val::Px(100. * global_charge.charge / 20.);
    }
}