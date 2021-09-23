use serde::Deserialize;

use crate::{structures::MapInterface, utils::bitmasker::get_bitmask_8bit, Point};

#[derive(Clone, Debug, Deserialize)]
pub struct DungeonTilesetBitmaskValues {
    wall_left: Vec<i64>,
    wall_left_d: Vec<i64>,

    wall_right: Vec<i64>,
    wall_right_d: Vec<i64>,

    wall_bottom: Vec<i64>,
    wall_top: Vec<i64>,

    wall_bw: Vec<i64>,
    wall_bw_end_top: Vec<i64>,
    wall_bw_end_bottom: Vec<i64>,

    wall_end_top_wide: Vec<i64>,
    wall_end_bottom_wide: Vec<i64>,

    corner_tl_outer: Vec<i64>,
    corner_tl_outer_c: Vec<i64>,
    corner_tl_inner: Vec<i64>,

    corner_tr_outer: Vec<i64>,
    corner_tr_outer_c: Vec<i64>,
    corner_tr_inner: Vec<i64>,

    corner_bl_outer: Vec<i64>,
    corner_bl_outer_c: Vec<i64>,
    corner_bl_inner: Vec<i64>,

    corner_br_outer: Vec<i64>,
    corner_br_outer_c: Vec<i64>,
    corner_br_inner: Vec<i64>,
    corner_br_inner_long: Vec<i64>,

    pillar_l: Vec<i64>,
    pillar_r: Vec<i64>,
    pillar_m: Vec<i64>,
}

// Takes coords, bitmask values from json, and map
// Loop the coords, get f (bitmask value) and check what array contains it
#[allow(clippy::too_many_lines)]
pub fn mask<T>(coords: &[Point], data: &DungeonTilesetBitmaskValues, map: &mut Box<T>)
where
    T: MapInterface + ?Sized,
{
    for coord in coords {
        let f = get_bitmask_8bit(coord, map);

        // Walls
        if data.wall_left.contains(&f) {
            map.set_matrix_value(coord, 1);
            continue;
        }
        if data.wall_left_d.contains(&f) {
            map.set_matrix_value(coord, 28);
            continue;
        }
        if data.wall_right.contains(&f) {
            map.set_matrix_value(coord, 0);
            continue;
        }
        if data.wall_right_d.contains(&f) {
            map.set_matrix_value(coord, 27);
            continue;
        }
        if data.wall_bottom.contains(&f) {
            map.set_matrix_value(coord, 13);
            continue;
        }
        if data.wall_top.contains(&f) {
            map.set_matrix_value(coord, 13);
            continue;
        }
        if data.wall_bw.contains(&f) {
            map.set_matrix_value(coord, 20);
            continue;
        }
        if data.wall_bw_end_top.contains(&f) {
            map.set_matrix_value(coord, 21);
            continue;
        }
        if data.wall_bw_end_bottom.contains(&f) {
            map.set_matrix_value(coord, 22);
            continue;
        }
        if data.wall_end_top_wide.contains(&f) {
            map.set_matrix_value(coord, 33);
        }
        if data.wall_end_bottom_wide.contains(&f) {
            map.set_matrix_value(coord, 34);
        }

        // Corner TL
        if data.corner_tl_outer.contains(&f) {
            map.set_matrix_value(coord, 11);
            continue;
        }
        if data.corner_tl_outer_c.contains(&f) {
            map.set_matrix_value(coord, 23);
            continue;
        }
        if data.corner_tl_inner.contains(&f) {
            map.set_matrix_value(coord, 18);
            continue;
        }

        // Corner TR
        if data.corner_tr_outer.contains(&f) {
            map.set_matrix_value(coord, 12);
            continue;
        }
        if data.corner_tr_outer_c.contains(&f) {
            map.set_matrix_value(coord, 26);
            continue;
        }
        if data.corner_tr_inner.contains(&f) {
            map.set_matrix_value(coord, 19);
            continue;
        }

        // Corner BL
        if data.corner_bl_outer.contains(&f) {
            map.set_matrix_value(coord, 17);
            continue;
        }
        if data.corner_bl_outer_c.contains(&f) {
            map.set_matrix_value(coord, 24);
            continue;
        }
        if data.corner_bl_inner.contains(&f) {
            map.set_matrix_value(coord, 14);
            continue;
        }

        // Corner BR
        if data.corner_br_outer.contains(&f) {
            map.set_matrix_value(coord, 16);
            continue;
        }
        if data.corner_br_outer_c.contains(&f) {
            map.set_matrix_value(coord, 25);
            continue;
        }
        if data.corner_br_inner.contains(&f) {
            map.set_matrix_value(coord, 15);
            continue;
        }
        if data.corner_br_inner_long.contains(&f) {
            map.set_matrix_value(coord, 35);
            continue;
        }

        // Pillars
        if data.pillar_l.contains(&f) {
            map.set_matrix_value(coord, 30);
            continue;
        }
        if data.pillar_r.contains(&f) {
            map.set_matrix_value(coord, 31);
            continue;
        }
        if data.pillar_m.contains(&f) {
            map.set_matrix_value(coord, 29);
            continue;
        }
    }
}
