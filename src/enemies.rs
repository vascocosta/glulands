use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, utils::ldtk_pixel_coords_to_translation_pivoted};

use crate::consts::*;

#[derive(PartialEq, Debug, Default, Component)]
pub(crate) struct Patrol {
    pub points: Vec<Vec2>,
    pub index: usize,
    pub forward: bool,
}

impl LdtkEntity for Patrol {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlasLayout>,
    ) -> Patrol {
        let mut points = Vec::new();
        points.push(ldtk_pixel_coords_to_translation_pivoted(
            entity_instance.px,
            layer_instance.c_hei * layer_instance.grid_size,
            IVec2::new(entity_instance.width, entity_instance.height),
            entity_instance.pivot,
        ));

        let ldtk_patrol_points = entity_instance
            .iter_points_field("patrol")
            .expect("patrol field should be correclty typed");

        for ldtk_point in ldtk_patrol_points {
            let pixel_coords = (ldtk_point.as_vec2() + Vec2::new(0.5, 1.))
                * Vec2::splat(layer_instance.grid_size as f32);
            points.push(ldtk_pixel_coords_to_translation_pivoted(
                pixel_coords.as_ivec2(),
                layer_instance.c_hei * layer_instance.grid_size,
                IVec2::new(entity_instance.width, entity_instance.height),
                entity_instance.pivot,
            ));
        }

        Patrol {
            points,
            index: 1,
            forward: true,
        }
    }
}

#[derive(Default, Component)]
pub(crate) struct Cow;

#[derive(Default, Bundle, LdtkEntity)]
pub(crate) struct CowBundle {
    pub cow: Cow,
    #[sprite_sheet_bundle]
    pub sprite_sheet_bundle: LdtkSpriteSheetBundle,
    #[ldtk_entity]
    pub patrol: Patrol,
}

pub(crate) fn patrol(mut query: Query<(&mut Transform, &mut Patrol)>, time: Res<Time>) {
    for (mut transform, mut patrol) in &mut query {
        if patrol.points.len() <= 1 {
            continue;
        }

        let start = patrol
            .points
            .first()
            .unwrap_or(&Vec2::new(0.0, 0.0))
            .extend(0.0);
        let mut finish = patrol
            .points
            .last()
            .unwrap_or(&Vec2::new(0.0, 0.0))
            .extend(0.0);
        finish.y += GRID_SIZE as f32 / 2.0;
        let direction = (finish - start).normalize();
        let orientation = if patrol.forward { 1.0 } else { -1.0 };

        if (transform.translation.x >= finish.x && patrol.forward)
            || (transform.translation.x <= start.x && !patrol.forward)
        {
            patrol.forward = !patrol.forward;
        }

        transform.translation += orientation * direction * COW_SPEED * time.delta_seconds();
    }
}
