#![allow(clippy::enum_variant_names)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::match_same_arms)]
#![allow(non_camel_case_types)]

pub trait Defines<T> {
    fn value(&self) -> T;
    fn key(&self) -> &'static str;
    fn iter() -> &'static [Self] where Self: std::marker::Sized;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum inventory {
    fuel,
    burnt_result,
    chest,
    furnace_source,
    furnace_result,
    furnace_modules,
    character_main,
    character_guns,
    character_ammo,
    character_armor,
    character_vehicle,
    character_trash,
    god_main,
    editor_main,
    editor_guns,
    editor_ammo,
    editor_armor,
    roboport_robot,
    roboport_material,
    robot_cargo,
    robot_repair,
    assembling_machine_input,
    assembling_machine_output,
    assembling_machine_modules,
    lab_input,
    lab_modules,
    mining_drill_modules,
    item_main,
    rocket_silo_rocket,
    rocket_silo_result,
    rocket_silo_input,
    rocket_silo_output,
    rocket_silo_modules,
    rocket,
    car_trunk,
    car_ammo,
    cargo_wagon,
    turret_ammo,
    beacon_modules,
    character_corpse,
    artillery_turret_ammo,
    artillery_wagon_ammo,
    spider_trunk,
    spider_ammo,
    spider_trash,
}
impl crate::defines::Defines<u8> for inventory {
    fn value(&self) -> u8 {
        match self {
            Self::fuel => 1,
            Self::burnt_result => 6,
            Self::chest => 1,
            Self::furnace_source => 2,
            Self::furnace_result => 3,
            Self::furnace_modules => 4,
            Self::character_main => 1,
            Self::character_guns => 3,
            Self::character_ammo => 4,
            Self::character_armor => 5,
            Self::character_vehicle => 7,
            Self::character_trash => 8,
            Self::god_main => 2,
            Self::editor_main => 1,
            Self::editor_guns => 3,
            Self::editor_ammo => 4,
            Self::editor_armor => 5,
            Self::roboport_robot => 1,
            Self::roboport_material => 2,
            Self::robot_cargo => 1,
            Self::robot_repair => 2,
            Self::assembling_machine_input => 2,
            Self::assembling_machine_output => 3,
            Self::assembling_machine_modules => 4,
            Self::lab_input => 2,
            Self::lab_modules => 3,
            Self::mining_drill_modules => 2,
            Self::item_main => 1,
            Self::rocket_silo_rocket => 5,
            Self::rocket_silo_result => 6,
            Self::rocket_silo_input => 2,
            Self::rocket_silo_output => 3,
            Self::rocket_silo_modules => 4,
            Self::rocket => 1,
            Self::car_trunk => 2,
            Self::car_ammo => 3,
            Self::cargo_wagon => 1,
            Self::turret_ammo => 1,
            Self::beacon_modules => 1,
            Self::character_corpse => 1,
            Self::artillery_turret_ammo => 1,
            Self::artillery_wagon_ammo => 1,
            Self::spider_trunk => 2,
            Self::spider_ammo => 3,
            Self::spider_trash => 4,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::fuel => "fuel",
            Self::burnt_result => "burnt_result",
            Self::chest => "chest",
            Self::furnace_source => "furnace_source",
            Self::furnace_result => "furnace_result",
            Self::furnace_modules => "furnace_modules",
            Self::character_main => "character_main",
            Self::character_guns => "character_guns",
            Self::character_ammo => "character_ammo",
            Self::character_armor => "character_armor",
            Self::character_vehicle => "character_vehicle",
            Self::character_trash => "character_trash",
            Self::god_main => "god_main",
            Self::editor_main => "editor_main",
            Self::editor_guns => "editor_guns",
            Self::editor_ammo => "editor_ammo",
            Self::editor_armor => "editor_armor",
            Self::roboport_robot => "roboport_robot",
            Self::roboport_material => "roboport_material",
            Self::robot_cargo => "robot_cargo",
            Self::robot_repair => "robot_repair",
            Self::assembling_machine_input => "assembling_machine_input",
            Self::assembling_machine_output => "assembling_machine_output",
            Self::assembling_machine_modules => "assembling_machine_modules",
            Self::lab_input => "lab_input",
            Self::lab_modules => "lab_modules",
            Self::mining_drill_modules => "mining_drill_modules",
            Self::item_main => "item_main",
            Self::rocket_silo_rocket => "rocket_silo_rocket",
            Self::rocket_silo_result => "rocket_silo_result",
            Self::rocket_silo_input => "rocket_silo_input",
            Self::rocket_silo_output => "rocket_silo_output",
            Self::rocket_silo_modules => "rocket_silo_modules",
            Self::rocket => "rocket",
            Self::car_trunk => "car_trunk",
            Self::car_ammo => "car_ammo",
            Self::cargo_wagon => "cargo_wagon",
            Self::turret_ammo => "turret_ammo",
            Self::beacon_modules => "beacon_modules",
            Self::character_corpse => "character_corpse",
            Self::artillery_turret_ammo => "artillery_turret_ammo",
            Self::artillery_wagon_ammo => "artillery_wagon_ammo",
            Self::spider_trunk => "spider_trunk",
            Self::spider_ammo => "spider_ammo",
            Self::spider_trash => "spider_trash",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::fuel,
            Self::burnt_result,
            Self::chest,
            Self::furnace_source,
            Self::furnace_result,
            Self::furnace_modules,
            Self::character_main,
            Self::character_guns,
            Self::character_ammo,
            Self::character_armor,
            Self::character_vehicle,
            Self::character_trash,
            Self::god_main,
            Self::editor_main,
            Self::editor_guns,
            Self::editor_ammo,
            Self::editor_armor,
            Self::roboport_robot,
            Self::roboport_material,
            Self::robot_cargo,
            Self::robot_repair,
            Self::assembling_machine_input,
            Self::assembling_machine_output,
            Self::assembling_machine_modules,
            Self::lab_input,
            Self::lab_modules,
            Self::mining_drill_modules,
            Self::item_main,
            Self::rocket_silo_rocket,
            Self::rocket_silo_result,
            Self::rocket_silo_input,
            Self::rocket_silo_output,
            Self::rocket_silo_modules,
            Self::rocket,
            Self::car_trunk,
            Self::car_ammo,
            Self::cargo_wagon,
            Self::turret_ammo,
            Self::beacon_modules,
            Self::character_corpse,
            Self::artillery_turret_ammo,
            Self::artillery_wagon_ammo,
            Self::spider_trunk,
            Self::spider_ammo,
            Self::spider_trash,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum transport_line {
    left_line,
    right_line,
    left_underground_line,
    right_underground_line,
    secondary_left_line,
    secondary_right_line,
    left_split_line,
    right_split_line,
    secondary_left_split_line,
    secondary_right_split_line,
}
impl crate::defines::Defines<u8> for transport_line {
    fn value(&self) -> u8 {
        match self {
            Self::left_line => 1,
            Self::right_line => 2,
            Self::left_underground_line => 3,
            Self::right_underground_line => 4,
            Self::secondary_left_line => 3,
            Self::secondary_right_line => 4,
            Self::left_split_line => 5,
            Self::right_split_line => 6,
            Self::secondary_left_split_line => 7,
            Self::secondary_right_split_line => 8,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::left_line => "left_line",
            Self::right_line => "right_line",
            Self::left_underground_line => "left_underground_line",
            Self::right_underground_line => "right_underground_line",
            Self::secondary_left_line => "secondary_left_line",
            Self::secondary_right_line => "secondary_right_line",
            Self::left_split_line => "left_split_line",
            Self::right_split_line => "right_split_line",
            Self::secondary_left_split_line => "secondary_left_split_line",
            Self::secondary_right_split_line => "secondary_right_split_line",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::left_line,
            Self::right_line,
            Self::left_underground_line,
            Self::right_underground_line,
            Self::secondary_left_line,
            Self::secondary_right_line,
            Self::left_split_line,
            Self::right_split_line,
            Self::secondary_left_split_line,
            Self::secondary_right_split_line,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum direction {
    north,
    northeast,
    east,
    southeast,
    south,
    southwest,
    west,
    northwest,
}
impl crate::defines::Defines<u8> for direction {
    fn value(&self) -> u8 {
        match self {
            Self::north => 0,
            Self::northeast => 1,
            Self::east => 2,
            Self::southeast => 3,
            Self::south => 4,
            Self::southwest => 5,
            Self::west => 6,
            Self::northwest => 7,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::north => "north",
            Self::northeast => "northeast",
            Self::east => "east",
            Self::southeast => "southeast",
            Self::south => "south",
            Self::southwest => "southwest",
            Self::west => "west",
            Self::northwest => "northwest",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::north,
            Self::northeast,
            Self::east,
            Self::southeast,
            Self::south,
            Self::southwest,
            Self::west,
            Self::northwest,
        ]
    }
}

pub mod riding {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum acceleration {
        nothing,
        accelerating,
        braking,
        reversing,
    }
    impl crate::defines::Defines<u8> for acceleration {
        fn value(&self) -> u8 {
            match self {
                Self::nothing => 0,
                Self::accelerating => 1,
                Self::braking => 2,
                Self::reversing => 3,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::nothing => "nothing",
                Self::accelerating => "accelerating",
                Self::braking => "braking",
                Self::reversing => "reversing",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::nothing,
                Self::accelerating,
                Self::braking,
                Self::reversing,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum direction {
        left,
        straight,
        right,
    }
    impl crate::defines::Defines<u8> for direction {
        fn value(&self) -> u8 {
            match self {
                Self::left => 0,
                Self::straight => 1,
                Self::right => 2,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::left => "left",
                Self::straight => "straight",
                Self::right => "right",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::left,
                Self::straight,
                Self::right,
            ]
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum shooting {
    not_shooting,
    shooting_enemies,
    shooting_selected,
}
impl crate::defines::Defines<u8> for shooting {
    fn value(&self) -> u8 {
        match self {
            Self::not_shooting => 0,
            Self::shooting_enemies => 1,
            Self::shooting_selected => 2,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::not_shooting => "not_shooting",
            Self::shooting_enemies => "shooting_enemies",
            Self::shooting_selected => "shooting_selected",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::not_shooting,
            Self::shooting_enemies,
            Self::shooting_selected,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum command {
    attack,
    go_to_location,
    compound,
    group,
    attack_area,
    wander,
    flee,
    stop,
    build_base,
}
impl crate::defines::Defines<u8> for command {
    fn value(&self) -> u8 {
        match self {
            Self::attack => 1,
            Self::go_to_location => 2,
            Self::compound => 3,
            Self::group => 4,
            Self::attack_area => 5,
            Self::wander => 6,
            Self::flee => 8,
            Self::stop => 9,
            Self::build_base => 7,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::attack => "attack",
            Self::go_to_location => "go_to_location",
            Self::compound => "compound",
            Self::group => "group",
            Self::attack_area => "attack_area",
            Self::wander => "wander",
            Self::flee => "flee",
            Self::stop => "stop",
            Self::build_base => "build_base",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::attack,
            Self::go_to_location,
            Self::compound,
            Self::group,
            Self::attack_area,
            Self::wander,
            Self::flee,
            Self::stop,
            Self::build_base,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum distraction {
    none,
    by_enemy,
    by_anything,
    by_damage,
}
impl crate::defines::Defines<u8> for distraction {
    fn value(&self) -> u8 {
        match self {
            Self::none => 0,
            Self::by_enemy => 1,
            Self::by_anything => 3,
            Self::by_damage => 4,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::none => "none",
            Self::by_enemy => "by_enemy",
            Self::by_anything => "by_anything",
            Self::by_damage => "by_damage",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::none,
            Self::by_enemy,
            Self::by_anything,
            Self::by_damage,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum compound_command {
    logical_and,
    logical_or,
    return_last,
}
impl crate::defines::Defines<u8> for compound_command {
    fn value(&self) -> u8 {
        match self {
            Self::logical_and => 0,
            Self::logical_or => 1,
            Self::return_last => 2,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::logical_and => "logical_and",
            Self::logical_or => "logical_or",
            Self::return_last => "return_last",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::logical_and,
            Self::logical_or,
            Self::return_last,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum difficulty {
    easy,
    normal,
    hard,
}
impl crate::defines::Defines<u8> for difficulty {
    fn value(&self) -> u8 {
        match self {
            Self::easy => 0,
            Self::normal => 1,
            Self::hard => 2,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::easy => "easy",
            Self::normal => "normal",
            Self::hard => "hard",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::easy,
            Self::normal,
            Self::hard,
        ]
    }
}

pub mod difficulty_settings {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum recipe_difficulty {
        normal,
        expensive,
    }
    impl crate::defines::Defines<u8> for recipe_difficulty {
        fn value(&self) -> u8 {
            match self {
                Self::normal => 0,
                Self::expensive => 1,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::normal => "normal",
                Self::expensive => "expensive",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::normal,
                Self::expensive,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum technology_difficulty {
        normal,
        expensive,
    }
    impl crate::defines::Defines<u8> for technology_difficulty {
        fn value(&self) -> u8 {
            match self {
                Self::normal => 0,
                Self::expensive => 1,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::normal => "normal",
                Self::expensive => "expensive",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::normal,
                Self::expensive,
            ]
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum events {
    on_player_input_method_changed,
    on_cutscene_finished,
    on_cutscene_started,
    on_entity_color_changed,
    on_gui_leave,
    on_gui_hover,
    on_player_alt_reverse_selected_area,
    on_player_reverse_selected_area,
    on_equipment_removed,
    on_equipment_inserted,
    on_entity_logistic_slot_changed,
    on_spider_command_completed,
    on_player_used_spider_remote,
    on_player_configured_spider_remote,
    on_cutscene_cancelled,
    on_permission_group_added,
    on_permission_group_deleted,
    on_pre_permission_group_deleted,
    on_permission_string_imported,
    on_pre_permission_string_imported,
    on_permission_group_edited,
    on_player_flushed_fluid,
    on_player_clicked_gps_tag,
    on_entity_destroyed,
    on_script_inventory_resized,
    on_pre_script_inventory_resized,
    on_pre_player_toggled_map_editor,
    on_player_set_quick_bar_slot,
    on_script_trigger_effect,
    on_string_translated,
    on_force_cease_fire_changed,
    on_force_friends_changed,
    on_gui_switch_state_changed,
    on_gui_selected_tab_changed,
    on_gui_location_changed,
    on_gui_confirmed,
    on_chart_tag_removed,
    on_chart_tag_modified,
    on_chart_tag_added,
    on_trigger_fired_artillery,
    on_build_base_arrived,
    on_unit_group_finished_gathering,
    on_unit_removed_from_group,
    on_unit_added_to_group,
    on_unit_group_created,
    on_pre_player_removed,
    on_entity_spawned,
    on_post_entity_died,
    on_robot_exploded_cliff,
    on_pre_robot_exploded_cliff,
    on_pre_chunk_deleted,
    on_player_fast_transferred,
    on_player_repaired_entity,
    on_player_toggled_alt_mode,
    on_surface_renamed,
    on_surface_imported,
    on_game_created_from_scenario,
    on_brush_cloned,
    on_area_cloned,
    on_entity_cloned,
    on_player_toggled_map_editor,
    on_cancelled_upgrade,
    on_marked_for_upgrade,
    on_ai_command_completed,
    on_script_path_request_finished,
    on_rocket_launch_ordered,
    on_player_unbanned,
    on_player_kicked,
    on_player_banned,
    on_train_schedule_changed,
    on_chunk_deleted,
    on_pre_surface_cleared,
    on_surface_cleared,
    on_pre_player_left_game,
    on_player_trash_inventory_changed,
    on_forces_merged,
    on_land_mine_armed,
    on_force_reset,
    on_technology_effects_reset,
    on_chunk_charted,
    on_entity_damaged,
    on_player_cancelled_crafting,
    on_pre_player_crafted_item,
    on_player_display_scale_changed,
    on_player_display_resolution_changed,
    on_player_pipette,
    on_pre_ghost_upgraded,
    on_pre_ghost_deconstructed,
    on_character_corpse_expired,
    on_player_cheat_mode_disabled,
    on_player_cheat_mode_enabled,
    on_player_unmuted,
    on_player_muted,
    on_gui_value_changed,
    on_gui_closed,
    on_gui_opened,
    on_mod_item_opened,
    on_player_changed_position,
    on_worker_robot_expired,
    on_combat_robot_expired,
    script_raised_set_tiles,
    script_raised_teleported,
    script_raised_revive,
    script_raised_destroy,
    script_raised_built,
    on_player_demoted,
    on_player_promoted,
    on_player_used_capsule,
    on_player_removed,
    on_console_command,
    on_console_chat,
    on_player_configured_blueprint,
    on_player_deconstructed_area,
    on_player_setup_blueprint,
    on_gui_elem_changed,
    on_train_created,
    on_player_mined_entity,
    on_robot_mined_entity,
    on_pre_surface_deleted,
    on_surface_deleted,
    on_surface_created,
    on_difficulty_settings_changed,
    on_runtime_mod_setting_changed,
    on_gui_selection_state_changed,
    on_entity_renamed,
    on_player_changed_force,
    on_biter_base_built,
    on_player_dropped_item,
    on_market_item_purchased,
    on_selected_entity_changed,
    on_player_changed_surface,
    on_player_alt_selected_area,
    on_player_selected_area,
    on_robot_mined_tile,
    on_robot_built_tile,
    on_player_mined_tile,
    on_player_built_tile,
    on_player_left_game,
    on_player_joined_game,
    on_player_respawned,
    on_player_died,
    on_pre_player_died,
    on_player_removed_equipment,
    on_player_placed_equipment,
    on_player_gun_inventory_changed,
    on_player_ammo_inventory_changed,
    on_player_armor_inventory_changed,
    on_lua_shortcut,
    on_cutscene_waypoint_reached,
    on_player_main_inventory_changed,
    on_entity_settings_pasted,
    on_pre_entity_settings_pasted,
    on_player_cursor_stack_changed,
    on_forces_merging,
    on_force_created,
    on_player_driving_changed_state,
    on_resource_depleted,
    on_player_created,
    on_train_changed_state,
    on_trigger_created_entity,
    on_cancelled_deconstruction,
    on_marked_for_deconstruction,
    on_player_rotated_entity,
    on_research_cancelled,
    on_research_reversed,
    on_research_finished,
    on_research_started,
    on_robot_mined,
    on_robot_pre_mined,
    on_robot_built_entity,
    on_player_crafted_item,
    on_chunk_generated,
    on_pre_player_mined_item,
    on_rocket_launched,
    on_pre_build,
    on_player_mined_item,
    on_sector_scanned,
    on_built_entity,
    on_picked_up_item,
    on_entity_died,
    on_gui_checked_state_changed,
    on_gui_text_changed,
    on_gui_click,
    on_tick,
}
impl crate::defines::Defines<u8> for events {
    fn value(&self) -> u8 {
        match self {
            Self::on_player_input_method_changed => 183,
            Self::on_cutscene_finished => 182,
            Self::on_cutscene_started => 181,
            Self::on_entity_color_changed => 180,
            Self::on_gui_leave => 179,
            Self::on_gui_hover => 178,
            Self::on_player_alt_reverse_selected_area => 177,
            Self::on_player_reverse_selected_area => 176,
            Self::on_equipment_removed => 175,
            Self::on_equipment_inserted => 174,
            Self::on_entity_logistic_slot_changed => 173,
            Self::on_spider_command_completed => 172,
            Self::on_player_used_spider_remote => 171,
            Self::on_player_configured_spider_remote => 170,
            Self::on_cutscene_cancelled => 169,
            Self::on_permission_group_added => 168,
            Self::on_permission_group_deleted => 167,
            Self::on_pre_permission_group_deleted => 166,
            Self::on_permission_string_imported => 165,
            Self::on_pre_permission_string_imported => 164,
            Self::on_permission_group_edited => 163,
            Self::on_player_flushed_fluid => 162,
            Self::on_player_clicked_gps_tag => 161,
            Self::on_entity_destroyed => 160,
            Self::on_script_inventory_resized => 159,
            Self::on_pre_script_inventory_resized => 158,
            Self::on_pre_player_toggled_map_editor => 157,
            Self::on_player_set_quick_bar_slot => 156,
            Self::on_script_trigger_effect => 155,
            Self::on_string_translated => 154,
            Self::on_force_cease_fire_changed => 153,
            Self::on_force_friends_changed => 152,
            Self::on_gui_switch_state_changed => 151,
            Self::on_gui_selected_tab_changed => 150,
            Self::on_gui_location_changed => 149,
            Self::on_gui_confirmed => 148,
            Self::on_chart_tag_removed => 147,
            Self::on_chart_tag_modified => 146,
            Self::on_chart_tag_added => 145,
            Self::on_trigger_fired_artillery => 144,
            Self::on_build_base_arrived => 143,
            Self::on_unit_group_finished_gathering => 142,
            Self::on_unit_removed_from_group => 141,
            Self::on_unit_added_to_group => 140,
            Self::on_unit_group_created => 139,
            Self::on_pre_player_removed => 138,
            Self::on_entity_spawned => 137,
            Self::on_post_entity_died => 136,
            Self::on_robot_exploded_cliff => 135,
            Self::on_pre_robot_exploded_cliff => 134,
            Self::on_pre_chunk_deleted => 133,
            Self::on_player_fast_transferred => 132,
            Self::on_player_repaired_entity => 131,
            Self::on_player_toggled_alt_mode => 130,
            Self::on_surface_renamed => 129,
            Self::on_surface_imported => 128,
            Self::on_game_created_from_scenario => 127,
            Self::on_brush_cloned => 126,
            Self::on_area_cloned => 125,
            Self::on_entity_cloned => 124,
            Self::on_player_toggled_map_editor => 123,
            Self::on_cancelled_upgrade => 122,
            Self::on_marked_for_upgrade => 121,
            Self::on_ai_command_completed => 120,
            Self::on_script_path_request_finished => 119,
            Self::on_rocket_launch_ordered => 118,
            Self::on_player_unbanned => 117,
            Self::on_player_kicked => 116,
            Self::on_player_banned => 115,
            Self::on_train_schedule_changed => 114,
            Self::on_chunk_deleted => 113,
            Self::on_pre_surface_cleared => 112,
            Self::on_surface_cleared => 111,
            Self::on_pre_player_left_game => 110,
            Self::on_player_trash_inventory_changed => 109,
            Self::on_forces_merged => 108,
            Self::on_land_mine_armed => 107,
            Self::on_force_reset => 106,
            Self::on_technology_effects_reset => 105,
            Self::on_chunk_charted => 104,
            Self::on_entity_damaged => 103,
            Self::on_player_cancelled_crafting => 102,
            Self::on_pre_player_crafted_item => 101,
            Self::on_player_display_scale_changed => 100,
            Self::on_player_display_resolution_changed => 99,
            Self::on_player_pipette => 98,
            Self::on_pre_ghost_upgraded => 97,
            Self::on_pre_ghost_deconstructed => 96,
            Self::on_character_corpse_expired => 95,
            Self::on_player_cheat_mode_disabled => 94,
            Self::on_player_cheat_mode_enabled => 93,
            Self::on_player_unmuted => 92,
            Self::on_player_muted => 91,
            Self::on_gui_value_changed => 90,
            Self::on_gui_closed => 89,
            Self::on_gui_opened => 88,
            Self::on_mod_item_opened => 87,
            Self::on_player_changed_position => 86,
            Self::on_worker_robot_expired => 85,
            Self::on_combat_robot_expired => 84,
            Self::script_raised_set_tiles => 83,
            Self::script_raised_teleported => 82,
            Self::script_raised_revive => 81,
            Self::script_raised_destroy => 80,
            Self::script_raised_built => 79,
            Self::on_player_demoted => 78,
            Self::on_player_promoted => 77,
            Self::on_player_used_capsule => 76,
            Self::on_player_removed => 75,
            Self::on_console_command => 74,
            Self::on_console_chat => 73,
            Self::on_player_configured_blueprint => 72,
            Self::on_player_deconstructed_area => 71,
            Self::on_player_setup_blueprint => 70,
            Self::on_gui_elem_changed => 69,
            Self::on_train_created => 68,
            Self::on_player_mined_entity => 67,
            Self::on_robot_mined_entity => 66,
            Self::on_pre_surface_deleted => 65,
            Self::on_surface_deleted => 64,
            Self::on_surface_created => 63,
            Self::on_difficulty_settings_changed => 62,
            Self::on_runtime_mod_setting_changed => 61,
            Self::on_gui_selection_state_changed => 60,
            Self::on_entity_renamed => 59,
            Self::on_player_changed_force => 58,
            Self::on_biter_base_built => 57,
            Self::on_player_dropped_item => 56,
            Self::on_market_item_purchased => 55,
            Self::on_selected_entity_changed => 54,
            Self::on_player_changed_surface => 53,
            Self::on_player_alt_selected_area => 52,
            Self::on_player_selected_area => 51,
            Self::on_robot_mined_tile => 50,
            Self::on_robot_built_tile => 49,
            Self::on_player_mined_tile => 48,
            Self::on_player_built_tile => 47,
            Self::on_player_left_game => 46,
            Self::on_player_joined_game => 45,
            Self::on_player_respawned => 44,
            Self::on_player_died => 43,
            Self::on_pre_player_died => 42,
            Self::on_player_removed_equipment => 41,
            Self::on_player_placed_equipment => 40,
            Self::on_player_gun_inventory_changed => 39,
            Self::on_player_ammo_inventory_changed => 38,
            Self::on_player_armor_inventory_changed => 37,
            Self::on_lua_shortcut => 36,
            Self::on_cutscene_waypoint_reached => 35,
            Self::on_player_main_inventory_changed => 34,
            Self::on_entity_settings_pasted => 33,
            Self::on_pre_entity_settings_pasted => 32,
            Self::on_player_cursor_stack_changed => 31,
            Self::on_forces_merging => 30,
            Self::on_force_created => 29,
            Self::on_player_driving_changed_state => 28,
            Self::on_resource_depleted => 27,
            Self::on_player_created => 26,
            Self::on_train_changed_state => 25,
            Self::on_trigger_created_entity => 24,
            Self::on_cancelled_deconstruction => 23,
            Self::on_marked_for_deconstruction => 22,
            Self::on_player_rotated_entity => 21,
            Self::on_research_cancelled => 20,
            Self::on_research_reversed => 19,
            Self::on_research_finished => 18,
            Self::on_research_started => 17,
            Self::on_robot_mined => 16,
            Self::on_robot_pre_mined => 15,
            Self::on_robot_built_entity => 14,
            Self::on_player_crafted_item => 13,
            Self::on_chunk_generated => 12,
            Self::on_pre_player_mined_item => 11,
            Self::on_rocket_launched => 10,
            Self::on_pre_build => 9,
            Self::on_player_mined_item => 8,
            Self::on_sector_scanned => 7,
            Self::on_built_entity => 6,
            Self::on_picked_up_item => 5,
            Self::on_entity_died => 4,
            Self::on_gui_checked_state_changed => 3,
            Self::on_gui_text_changed => 2,
            Self::on_gui_click => 1,
            Self::on_tick => 0,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::on_player_input_method_changed => "on_player_input_method_changed",
            Self::on_cutscene_finished => "on_cutscene_finished",
            Self::on_cutscene_started => "on_cutscene_started",
            Self::on_entity_color_changed => "on_entity_color_changed",
            Self::on_gui_leave => "on_gui_leave",
            Self::on_gui_hover => "on_gui_hover",
            Self::on_player_alt_reverse_selected_area => "on_player_alt_reverse_selected_area",
            Self::on_player_reverse_selected_area => "on_player_reverse_selected_area",
            Self::on_equipment_removed => "on_equipment_removed",
            Self::on_equipment_inserted => "on_equipment_inserted",
            Self::on_entity_logistic_slot_changed => "on_entity_logistic_slot_changed",
            Self::on_spider_command_completed => "on_spider_command_completed",
            Self::on_player_used_spider_remote => "on_player_used_spider_remote",
            Self::on_player_configured_spider_remote => "on_player_configured_spider_remote",
            Self::on_cutscene_cancelled => "on_cutscene_cancelled",
            Self::on_permission_group_added => "on_permission_group_added",
            Self::on_permission_group_deleted => "on_permission_group_deleted",
            Self::on_pre_permission_group_deleted => "on_pre_permission_group_deleted",
            Self::on_permission_string_imported => "on_permission_string_imported",
            Self::on_pre_permission_string_imported => "on_pre_permission_string_imported",
            Self::on_permission_group_edited => "on_permission_group_edited",
            Self::on_player_flushed_fluid => "on_player_flushed_fluid",
            Self::on_player_clicked_gps_tag => "on_player_clicked_gps_tag",
            Self::on_entity_destroyed => "on_entity_destroyed",
            Self::on_script_inventory_resized => "on_script_inventory_resized",
            Self::on_pre_script_inventory_resized => "on_pre_script_inventory_resized",
            Self::on_pre_player_toggled_map_editor => "on_pre_player_toggled_map_editor",
            Self::on_player_set_quick_bar_slot => "on_player_set_quick_bar_slot",
            Self::on_script_trigger_effect => "on_script_trigger_effect",
            Self::on_string_translated => "on_string_translated",
            Self::on_force_cease_fire_changed => "on_force_cease_fire_changed",
            Self::on_force_friends_changed => "on_force_friends_changed",
            Self::on_gui_switch_state_changed => "on_gui_switch_state_changed",
            Self::on_gui_selected_tab_changed => "on_gui_selected_tab_changed",
            Self::on_gui_location_changed => "on_gui_location_changed",
            Self::on_gui_confirmed => "on_gui_confirmed",
            Self::on_chart_tag_removed => "on_chart_tag_removed",
            Self::on_chart_tag_modified => "on_chart_tag_modified",
            Self::on_chart_tag_added => "on_chart_tag_added",
            Self::on_trigger_fired_artillery => "on_trigger_fired_artillery",
            Self::on_build_base_arrived => "on_build_base_arrived",
            Self::on_unit_group_finished_gathering => "on_unit_group_finished_gathering",
            Self::on_unit_removed_from_group => "on_unit_removed_from_group",
            Self::on_unit_added_to_group => "on_unit_added_to_group",
            Self::on_unit_group_created => "on_unit_group_created",
            Self::on_pre_player_removed => "on_pre_player_removed",
            Self::on_entity_spawned => "on_entity_spawned",
            Self::on_post_entity_died => "on_post_entity_died",
            Self::on_robot_exploded_cliff => "on_robot_exploded_cliff",
            Self::on_pre_robot_exploded_cliff => "on_pre_robot_exploded_cliff",
            Self::on_pre_chunk_deleted => "on_pre_chunk_deleted",
            Self::on_player_fast_transferred => "on_player_fast_transferred",
            Self::on_player_repaired_entity => "on_player_repaired_entity",
            Self::on_player_toggled_alt_mode => "on_player_toggled_alt_mode",
            Self::on_surface_renamed => "on_surface_renamed",
            Self::on_surface_imported => "on_surface_imported",
            Self::on_game_created_from_scenario => "on_game_created_from_scenario",
            Self::on_brush_cloned => "on_brush_cloned",
            Self::on_area_cloned => "on_area_cloned",
            Self::on_entity_cloned => "on_entity_cloned",
            Self::on_player_toggled_map_editor => "on_player_toggled_map_editor",
            Self::on_cancelled_upgrade => "on_cancelled_upgrade",
            Self::on_marked_for_upgrade => "on_marked_for_upgrade",
            Self::on_ai_command_completed => "on_ai_command_completed",
            Self::on_script_path_request_finished => "on_script_path_request_finished",
            Self::on_rocket_launch_ordered => "on_rocket_launch_ordered",
            Self::on_player_unbanned => "on_player_unbanned",
            Self::on_player_kicked => "on_player_kicked",
            Self::on_player_banned => "on_player_banned",
            Self::on_train_schedule_changed => "on_train_schedule_changed",
            Self::on_chunk_deleted => "on_chunk_deleted",
            Self::on_pre_surface_cleared => "on_pre_surface_cleared",
            Self::on_surface_cleared => "on_surface_cleared",
            Self::on_pre_player_left_game => "on_pre_player_left_game",
            Self::on_player_trash_inventory_changed => "on_player_trash_inventory_changed",
            Self::on_forces_merged => "on_forces_merged",
            Self::on_land_mine_armed => "on_land_mine_armed",
            Self::on_force_reset => "on_force_reset",
            Self::on_technology_effects_reset => "on_technology_effects_reset",
            Self::on_chunk_charted => "on_chunk_charted",
            Self::on_entity_damaged => "on_entity_damaged",
            Self::on_player_cancelled_crafting => "on_player_cancelled_crafting",
            Self::on_pre_player_crafted_item => "on_pre_player_crafted_item",
            Self::on_player_display_scale_changed => "on_player_display_scale_changed",
            Self::on_player_display_resolution_changed => "on_player_display_resolution_changed",
            Self::on_player_pipette => "on_player_pipette",
            Self::on_pre_ghost_upgraded => "on_pre_ghost_upgraded",
            Self::on_pre_ghost_deconstructed => "on_pre_ghost_deconstructed",
            Self::on_character_corpse_expired => "on_character_corpse_expired",
            Self::on_player_cheat_mode_disabled => "on_player_cheat_mode_disabled",
            Self::on_player_cheat_mode_enabled => "on_player_cheat_mode_enabled",
            Self::on_player_unmuted => "on_player_unmuted",
            Self::on_player_muted => "on_player_muted",
            Self::on_gui_value_changed => "on_gui_value_changed",
            Self::on_gui_closed => "on_gui_closed",
            Self::on_gui_opened => "on_gui_opened",
            Self::on_mod_item_opened => "on_mod_item_opened",
            Self::on_player_changed_position => "on_player_changed_position",
            Self::on_worker_robot_expired => "on_worker_robot_expired",
            Self::on_combat_robot_expired => "on_combat_robot_expired",
            Self::script_raised_set_tiles => "script_raised_set_tiles",
            Self::script_raised_teleported => "script_raised_teleported",
            Self::script_raised_revive => "script_raised_revive",
            Self::script_raised_destroy => "script_raised_destroy",
            Self::script_raised_built => "script_raised_built",
            Self::on_player_demoted => "on_player_demoted",
            Self::on_player_promoted => "on_player_promoted",
            Self::on_player_used_capsule => "on_player_used_capsule",
            Self::on_player_removed => "on_player_removed",
            Self::on_console_command => "on_console_command",
            Self::on_console_chat => "on_console_chat",
            Self::on_player_configured_blueprint => "on_player_configured_blueprint",
            Self::on_player_deconstructed_area => "on_player_deconstructed_area",
            Self::on_player_setup_blueprint => "on_player_setup_blueprint",
            Self::on_gui_elem_changed => "on_gui_elem_changed",
            Self::on_train_created => "on_train_created",
            Self::on_player_mined_entity => "on_player_mined_entity",
            Self::on_robot_mined_entity => "on_robot_mined_entity",
            Self::on_pre_surface_deleted => "on_pre_surface_deleted",
            Self::on_surface_deleted => "on_surface_deleted",
            Self::on_surface_created => "on_surface_created",
            Self::on_difficulty_settings_changed => "on_difficulty_settings_changed",
            Self::on_runtime_mod_setting_changed => "on_runtime_mod_setting_changed",
            Self::on_gui_selection_state_changed => "on_gui_selection_state_changed",
            Self::on_entity_renamed => "on_entity_renamed",
            Self::on_player_changed_force => "on_player_changed_force",
            Self::on_biter_base_built => "on_biter_base_built",
            Self::on_player_dropped_item => "on_player_dropped_item",
            Self::on_market_item_purchased => "on_market_item_purchased",
            Self::on_selected_entity_changed => "on_selected_entity_changed",
            Self::on_player_changed_surface => "on_player_changed_surface",
            Self::on_player_alt_selected_area => "on_player_alt_selected_area",
            Self::on_player_selected_area => "on_player_selected_area",
            Self::on_robot_mined_tile => "on_robot_mined_tile",
            Self::on_robot_built_tile => "on_robot_built_tile",
            Self::on_player_mined_tile => "on_player_mined_tile",
            Self::on_player_built_tile => "on_player_built_tile",
            Self::on_player_left_game => "on_player_left_game",
            Self::on_player_joined_game => "on_player_joined_game",
            Self::on_player_respawned => "on_player_respawned",
            Self::on_player_died => "on_player_died",
            Self::on_pre_player_died => "on_pre_player_died",
            Self::on_player_removed_equipment => "on_player_removed_equipment",
            Self::on_player_placed_equipment => "on_player_placed_equipment",
            Self::on_player_gun_inventory_changed => "on_player_gun_inventory_changed",
            Self::on_player_ammo_inventory_changed => "on_player_ammo_inventory_changed",
            Self::on_player_armor_inventory_changed => "on_player_armor_inventory_changed",
            Self::on_lua_shortcut => "on_lua_shortcut",
            Self::on_cutscene_waypoint_reached => "on_cutscene_waypoint_reached",
            Self::on_player_main_inventory_changed => "on_player_main_inventory_changed",
            Self::on_entity_settings_pasted => "on_entity_settings_pasted",
            Self::on_pre_entity_settings_pasted => "on_pre_entity_settings_pasted",
            Self::on_player_cursor_stack_changed => "on_player_cursor_stack_changed",
            Self::on_forces_merging => "on_forces_merging",
            Self::on_force_created => "on_force_created",
            Self::on_player_driving_changed_state => "on_player_driving_changed_state",
            Self::on_resource_depleted => "on_resource_depleted",
            Self::on_player_created => "on_player_created",
            Self::on_train_changed_state => "on_train_changed_state",
            Self::on_trigger_created_entity => "on_trigger_created_entity",
            Self::on_cancelled_deconstruction => "on_cancelled_deconstruction",
            Self::on_marked_for_deconstruction => "on_marked_for_deconstruction",
            Self::on_player_rotated_entity => "on_player_rotated_entity",
            Self::on_research_cancelled => "on_research_cancelled",
            Self::on_research_reversed => "on_research_reversed",
            Self::on_research_finished => "on_research_finished",
            Self::on_research_started => "on_research_started",
            Self::on_robot_mined => "on_robot_mined",
            Self::on_robot_pre_mined => "on_robot_pre_mined",
            Self::on_robot_built_entity => "on_robot_built_entity",
            Self::on_player_crafted_item => "on_player_crafted_item",
            Self::on_chunk_generated => "on_chunk_generated",
            Self::on_pre_player_mined_item => "on_pre_player_mined_item",
            Self::on_rocket_launched => "on_rocket_launched",
            Self::on_pre_build => "on_pre_build",
            Self::on_player_mined_item => "on_player_mined_item",
            Self::on_sector_scanned => "on_sector_scanned",
            Self::on_built_entity => "on_built_entity",
            Self::on_picked_up_item => "on_picked_up_item",
            Self::on_entity_died => "on_entity_died",
            Self::on_gui_checked_state_changed => "on_gui_checked_state_changed",
            Self::on_gui_text_changed => "on_gui_text_changed",
            Self::on_gui_click => "on_gui_click",
            Self::on_tick => "on_tick",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::on_player_input_method_changed,
            Self::on_cutscene_finished,
            Self::on_cutscene_started,
            Self::on_entity_color_changed,
            Self::on_gui_leave,
            Self::on_gui_hover,
            Self::on_player_alt_reverse_selected_area,
            Self::on_player_reverse_selected_area,
            Self::on_equipment_removed,
            Self::on_equipment_inserted,
            Self::on_entity_logistic_slot_changed,
            Self::on_spider_command_completed,
            Self::on_player_used_spider_remote,
            Self::on_player_configured_spider_remote,
            Self::on_cutscene_cancelled,
            Self::on_permission_group_added,
            Self::on_permission_group_deleted,
            Self::on_pre_permission_group_deleted,
            Self::on_permission_string_imported,
            Self::on_pre_permission_string_imported,
            Self::on_permission_group_edited,
            Self::on_player_flushed_fluid,
            Self::on_player_clicked_gps_tag,
            Self::on_entity_destroyed,
            Self::on_script_inventory_resized,
            Self::on_pre_script_inventory_resized,
            Self::on_pre_player_toggled_map_editor,
            Self::on_player_set_quick_bar_slot,
            Self::on_script_trigger_effect,
            Self::on_string_translated,
            Self::on_force_cease_fire_changed,
            Self::on_force_friends_changed,
            Self::on_gui_switch_state_changed,
            Self::on_gui_selected_tab_changed,
            Self::on_gui_location_changed,
            Self::on_gui_confirmed,
            Self::on_chart_tag_removed,
            Self::on_chart_tag_modified,
            Self::on_chart_tag_added,
            Self::on_trigger_fired_artillery,
            Self::on_build_base_arrived,
            Self::on_unit_group_finished_gathering,
            Self::on_unit_removed_from_group,
            Self::on_unit_added_to_group,
            Self::on_unit_group_created,
            Self::on_pre_player_removed,
            Self::on_entity_spawned,
            Self::on_post_entity_died,
            Self::on_robot_exploded_cliff,
            Self::on_pre_robot_exploded_cliff,
            Self::on_pre_chunk_deleted,
            Self::on_player_fast_transferred,
            Self::on_player_repaired_entity,
            Self::on_player_toggled_alt_mode,
            Self::on_surface_renamed,
            Self::on_surface_imported,
            Self::on_game_created_from_scenario,
            Self::on_brush_cloned,
            Self::on_area_cloned,
            Self::on_entity_cloned,
            Self::on_player_toggled_map_editor,
            Self::on_cancelled_upgrade,
            Self::on_marked_for_upgrade,
            Self::on_ai_command_completed,
            Self::on_script_path_request_finished,
            Self::on_rocket_launch_ordered,
            Self::on_player_unbanned,
            Self::on_player_kicked,
            Self::on_player_banned,
            Self::on_train_schedule_changed,
            Self::on_chunk_deleted,
            Self::on_pre_surface_cleared,
            Self::on_surface_cleared,
            Self::on_pre_player_left_game,
            Self::on_player_trash_inventory_changed,
            Self::on_forces_merged,
            Self::on_land_mine_armed,
            Self::on_force_reset,
            Self::on_technology_effects_reset,
            Self::on_chunk_charted,
            Self::on_entity_damaged,
            Self::on_player_cancelled_crafting,
            Self::on_pre_player_crafted_item,
            Self::on_player_display_scale_changed,
            Self::on_player_display_resolution_changed,
            Self::on_player_pipette,
            Self::on_pre_ghost_upgraded,
            Self::on_pre_ghost_deconstructed,
            Self::on_character_corpse_expired,
            Self::on_player_cheat_mode_disabled,
            Self::on_player_cheat_mode_enabled,
            Self::on_player_unmuted,
            Self::on_player_muted,
            Self::on_gui_value_changed,
            Self::on_gui_closed,
            Self::on_gui_opened,
            Self::on_mod_item_opened,
            Self::on_player_changed_position,
            Self::on_worker_robot_expired,
            Self::on_combat_robot_expired,
            Self::script_raised_set_tiles,
            Self::script_raised_teleported,
            Self::script_raised_revive,
            Self::script_raised_destroy,
            Self::script_raised_built,
            Self::on_player_demoted,
            Self::on_player_promoted,
            Self::on_player_used_capsule,
            Self::on_player_removed,
            Self::on_console_command,
            Self::on_console_chat,
            Self::on_player_configured_blueprint,
            Self::on_player_deconstructed_area,
            Self::on_player_setup_blueprint,
            Self::on_gui_elem_changed,
            Self::on_train_created,
            Self::on_player_mined_entity,
            Self::on_robot_mined_entity,
            Self::on_pre_surface_deleted,
            Self::on_surface_deleted,
            Self::on_surface_created,
            Self::on_difficulty_settings_changed,
            Self::on_runtime_mod_setting_changed,
            Self::on_gui_selection_state_changed,
            Self::on_entity_renamed,
            Self::on_player_changed_force,
            Self::on_biter_base_built,
            Self::on_player_dropped_item,
            Self::on_market_item_purchased,
            Self::on_selected_entity_changed,
            Self::on_player_changed_surface,
            Self::on_player_alt_selected_area,
            Self::on_player_selected_area,
            Self::on_robot_mined_tile,
            Self::on_robot_built_tile,
            Self::on_player_mined_tile,
            Self::on_player_built_tile,
            Self::on_player_left_game,
            Self::on_player_joined_game,
            Self::on_player_respawned,
            Self::on_player_died,
            Self::on_pre_player_died,
            Self::on_player_removed_equipment,
            Self::on_player_placed_equipment,
            Self::on_player_gun_inventory_changed,
            Self::on_player_ammo_inventory_changed,
            Self::on_player_armor_inventory_changed,
            Self::on_lua_shortcut,
            Self::on_cutscene_waypoint_reached,
            Self::on_player_main_inventory_changed,
            Self::on_entity_settings_pasted,
            Self::on_pre_entity_settings_pasted,
            Self::on_player_cursor_stack_changed,
            Self::on_forces_merging,
            Self::on_force_created,
            Self::on_player_driving_changed_state,
            Self::on_resource_depleted,
            Self::on_player_created,
            Self::on_train_changed_state,
            Self::on_trigger_created_entity,
            Self::on_cancelled_deconstruction,
            Self::on_marked_for_deconstruction,
            Self::on_player_rotated_entity,
            Self::on_research_cancelled,
            Self::on_research_reversed,
            Self::on_research_finished,
            Self::on_research_started,
            Self::on_robot_mined,
            Self::on_robot_pre_mined,
            Self::on_robot_built_entity,
            Self::on_player_crafted_item,
            Self::on_chunk_generated,
            Self::on_pre_player_mined_item,
            Self::on_rocket_launched,
            Self::on_pre_build,
            Self::on_player_mined_item,
            Self::on_sector_scanned,
            Self::on_built_entity,
            Self::on_picked_up_item,
            Self::on_entity_died,
            Self::on_gui_checked_state_changed,
            Self::on_gui_text_changed,
            Self::on_gui_click,
            Self::on_tick,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum controllers {
    ghost,
    character,
    god,
    editor,
    cutscene,
    spectator,
}
impl crate::defines::Defines<u8> for controllers {
    fn value(&self) -> u8 {
        match self {
            Self::ghost => 0,
            Self::character => 1,
            Self::god => 2,
            Self::editor => 4,
            Self::cutscene => 6,
            Self::spectator => 5,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::ghost => "ghost",
            Self::character => "character",
            Self::god => "god",
            Self::editor => "editor",
            Self::cutscene => "cutscene",
            Self::spectator => "spectator",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::ghost,
            Self::character,
            Self::god,
            Self::editor,
            Self::cutscene,
            Self::spectator,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum group_state {
    gathering,
    moving,
    attacking_distraction,
    attacking_target,
    finished,
    pathfinding,
    wander_in_group,
}
impl crate::defines::Defines<u8> for group_state {
    fn value(&self) -> u8 {
        match self {
            Self::gathering => 0,
            Self::moving => 1,
            Self::attacking_distraction => 2,
            Self::attacking_target => 3,
            Self::finished => 4,
            Self::pathfinding => 5,
            Self::wander_in_group => 6,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::gathering => "gathering",
            Self::moving => "moving",
            Self::attacking_distraction => "attacking_distraction",
            Self::attacking_target => "attacking_target",
            Self::finished => "finished",
            Self::pathfinding => "pathfinding",
            Self::wander_in_group => "wander_in_group",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::gathering,
            Self::moving,
            Self::attacking_distraction,
            Self::attacking_target,
            Self::finished,
            Self::pathfinding,
            Self::wander_in_group,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum wire_type {
    red,
    green,
    copper,
}
impl crate::defines::Defines<u8> for wire_type {
    fn value(&self) -> u8 {
        match self {
            Self::red => 2,
            Self::green => 3,
            Self::copper => 1,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::red => "red",
            Self::green => "green",
            Self::copper => "copper",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::red,
            Self::green,
            Self::copper,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum circuit_connector_id {
    accumulator,
    constant_combinator,
    container,
    linked_container,
    programmable_speaker,
    rail_signal,
    rail_chain_signal,
    roboport,
    storage_tank,
    wall,
    electric_pole,
    inserter,
    lamp,
    combinator_input,
    combinator_output,
    offshore_pump,
    pump,
}
impl crate::defines::Defines<u8> for circuit_connector_id {
    fn value(&self) -> u8 {
        match self {
            Self::accumulator => 1,
            Self::constant_combinator => 1,
            Self::container => 1,
            Self::linked_container => 1,
            Self::programmable_speaker => 1,
            Self::rail_signal => 1,
            Self::rail_chain_signal => 1,
            Self::roboport => 1,
            Self::storage_tank => 1,
            Self::wall => 1,
            Self::electric_pole => 1,
            Self::inserter => 1,
            Self::lamp => 1,
            Self::combinator_input => 1,
            Self::combinator_output => 2,
            Self::offshore_pump => 1,
            Self::pump => 1,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::accumulator => "accumulator",
            Self::constant_combinator => "constant_combinator",
            Self::container => "container",
            Self::linked_container => "linked_container",
            Self::programmable_speaker => "programmable_speaker",
            Self::rail_signal => "rail_signal",
            Self::rail_chain_signal => "rail_chain_signal",
            Self::roboport => "roboport",
            Self::storage_tank => "storage_tank",
            Self::wall => "wall",
            Self::electric_pole => "electric_pole",
            Self::inserter => "inserter",
            Self::lamp => "lamp",
            Self::combinator_input => "combinator_input",
            Self::combinator_output => "combinator_output",
            Self::offshore_pump => "offshore_pump",
            Self::pump => "pump",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::accumulator,
            Self::constant_combinator,
            Self::container,
            Self::linked_container,
            Self::programmable_speaker,
            Self::rail_signal,
            Self::rail_chain_signal,
            Self::roboport,
            Self::storage_tank,
            Self::wall,
            Self::electric_pole,
            Self::inserter,
            Self::lamp,
            Self::combinator_input,
            Self::combinator_output,
            Self::offshore_pump,
            Self::pump,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum circuit_condition_index {
    inserter_circuit,
    inserter_logistic,
    lamp,
    arithmetic_combinator,
    decider_combinator,
    constant_combinator,
    offshore_pump,
    pump,
}
impl crate::defines::Defines<u8> for circuit_condition_index {
    fn value(&self) -> u8 {
        match self {
            Self::inserter_circuit => 1,
            Self::inserter_logistic => 2,
            Self::lamp => 1,
            Self::arithmetic_combinator => 1,
            Self::decider_combinator => 1,
            Self::constant_combinator => 1,
            Self::offshore_pump => 1,
            Self::pump => 1,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::inserter_circuit => "inserter_circuit",
            Self::inserter_logistic => "inserter_logistic",
            Self::lamp => "lamp",
            Self::arithmetic_combinator => "arithmetic_combinator",
            Self::decider_combinator => "decider_combinator",
            Self::constant_combinator => "constant_combinator",
            Self::offshore_pump => "offshore_pump",
            Self::pump => "pump",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::inserter_circuit,
            Self::inserter_logistic,
            Self::lamp,
            Self::arithmetic_combinator,
            Self::decider_combinator,
            Self::constant_combinator,
            Self::offshore_pump,
            Self::pump,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum wire_connection_id {
    electric_pole,
    power_switch_left,
    power_switch_right,
}
impl crate::defines::Defines<u8> for wire_connection_id {
    fn value(&self) -> u8 {
        match self {
            Self::electric_pole => 0,
            Self::power_switch_left => 0,
            Self::power_switch_right => 1,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::electric_pole => "electric_pole",
            Self::power_switch_left => "power_switch_left",
            Self::power_switch_right => "power_switch_right",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::electric_pole,
            Self::power_switch_left,
            Self::power_switch_right,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum train_state {
    on_the_path,
    path_lost,
    no_schedule,
    no_path,
    arrive_signal,
    wait_signal,
    arrive_station,
    wait_station,
    manual_control_stop,
    manual_control,
    destination_full,
}
impl crate::defines::Defines<u8> for train_state {
    fn value(&self) -> u8 {
        match self {
            Self::on_the_path => 0,
            Self::path_lost => 1,
            Self::no_schedule => 2,
            Self::no_path => 3,
            Self::arrive_signal => 4,
            Self::wait_signal => 5,
            Self::arrive_station => 6,
            Self::wait_station => 7,
            Self::manual_control_stop => 8,
            Self::manual_control => 9,
            Self::destination_full => 10,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::on_the_path => "on_the_path",
            Self::path_lost => "path_lost",
            Self::no_schedule => "no_schedule",
            Self::no_path => "no_path",
            Self::arrive_signal => "arrive_signal",
            Self::wait_signal => "wait_signal",
            Self::arrive_station => "arrive_station",
            Self::wait_station => "wait_station",
            Self::manual_control_stop => "manual_control_stop",
            Self::manual_control => "manual_control",
            Self::destination_full => "destination_full",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::on_the_path,
            Self::path_lost,
            Self::no_schedule,
            Self::no_path,
            Self::arrive_signal,
            Self::wait_signal,
            Self::arrive_station,
            Self::wait_station,
            Self::manual_control_stop,
            Self::manual_control,
            Self::destination_full,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum signal_state {
    open,
    closed,
    reserved,
    reserved_by_circuit_network,
}
impl crate::defines::Defines<u8> for signal_state {
    fn value(&self) -> u8 {
        match self {
            Self::open => 0,
            Self::closed => 1,
            Self::reserved => 2,
            Self::reserved_by_circuit_network => 3,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::open => "open",
            Self::closed => "closed",
            Self::reserved => "reserved",
            Self::reserved_by_circuit_network => "reserved_by_circuit_network",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::open,
            Self::closed,
            Self::reserved,
            Self::reserved_by_circuit_network,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum chain_signal_state {
    none,
    all_open,
    partially_open,
    none_open,
}
impl crate::defines::Defines<u8> for chain_signal_state {
    fn value(&self) -> u8 {
        match self {
            Self::none => 0,
            Self::all_open => 1,
            Self::partially_open => 2,
            Self::none_open => 3,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::none => "none",
            Self::all_open => "all_open",
            Self::partially_open => "partially_open",
            Self::none_open => "none_open",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::none,
            Self::all_open,
            Self::partially_open,
            Self::none_open,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rail_direction {
    front,
    back,
}
impl crate::defines::Defines<u8> for rail_direction {
    fn value(&self) -> u8 {
        match self {
            Self::front => 0,
            Self::back => 1,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::front => "front",
            Self::back => "back",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::front,
            Self::back,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rail_connection_direction {
    left,
    straight,
    right,
    none,
}
impl crate::defines::Defines<u8> for rail_connection_direction {
    fn value(&self) -> u8 {
        match self {
            Self::left => 0,
            Self::straight => 1,
            Self::right => 2,
            Self::none => 3,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::left => "left",
            Self::straight => "straight",
            Self::right => "right",
            Self::none => "none",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::left,
            Self::straight,
            Self::right,
            Self::none,
        ]
    }
}

pub mod control_behavior {
    pub mod inserter {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum circuit_mode_of_operation {
            none,
            enable_disable,
            set_filters,
            read_hand_contents,
            set_stack_size,
        }
        impl crate::defines::Defines<u8> for circuit_mode_of_operation {
            fn value(&self) -> u8 {
                match self {
                    Self::none => 3,
                    Self::enable_disable => 0,
                    Self::set_filters => 1,
                    Self::read_hand_contents => 2,
                    Self::set_stack_size => 4,
                }
            }
            fn key(&self) -> &'static str {
                match self {
                    Self::none => "none",
                    Self::enable_disable => "enable_disable",
                    Self::set_filters => "set_filters",
                    Self::read_hand_contents => "read_hand_contents",
                    Self::set_stack_size => "set_stack_size",
                }
            }
            fn iter() -> &'static [Self] {
                &[
                    Self::none,
                    Self::enable_disable,
                    Self::set_filters,
                    Self::read_hand_contents,
                    Self::set_stack_size,
                ]
            }
        }
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum hand_read_mode {
            hold,
            pulse,
        }
        impl crate::defines::Defines<u8> for hand_read_mode {
            fn value(&self) -> u8 {
                match self {
                    Self::hold => 1,
                    Self::pulse => 0,
                }
            }
            fn key(&self) -> &'static str {
                match self {
                    Self::hold => "hold",
                    Self::pulse => "pulse",
                }
            }
            fn iter() -> &'static [Self] {
                &[
                    Self::hold,
                    Self::pulse,
                ]
            }
        }
    }
    pub mod logistic_container {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum circuit_mode_of_operation {
            send_contents,
            set_requests,
        }
        impl crate::defines::Defines<u8> for circuit_mode_of_operation {
            fn value(&self) -> u8 {
                match self {
                    Self::send_contents => 0,
                    Self::set_requests => 1,
                }
            }
            fn key(&self) -> &'static str {
                match self {
                    Self::send_contents => "send_contents",
                    Self::set_requests => "set_requests",
                }
            }
            fn iter() -> &'static [Self] {
                &[
                    Self::send_contents,
                    Self::set_requests,
                ]
            }
        }
    }
    pub mod lamp {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum circuit_mode_of_operation {
            use_colors,
        }
        impl crate::defines::Defines<u8> for circuit_mode_of_operation {
            fn value(&self) -> u8 {
                match self {
                    Self::use_colors => 0,
                }
            }
            fn key(&self) -> &'static str {
                match self {
                    Self::use_colors => "use_colors",
                }
            }
            fn iter() -> &'static [Self] {
                &[
                    Self::use_colors,
                ]
            }
        }
    }
    pub mod mining_drill {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum resource_read_mode {
            this_miner,
            entire_patch,
        }
        impl crate::defines::Defines<u8> for resource_read_mode {
            fn value(&self) -> u8 {
                match self {
                    Self::this_miner => 0,
                    Self::entire_patch => 1,
                }
            }
            fn key(&self) -> &'static str {
                match self {
                    Self::this_miner => "this_miner",
                    Self::entire_patch => "entire_patch",
                }
            }
            fn iter() -> &'static [Self] {
                &[
                    Self::this_miner,
                    Self::entire_patch,
                ]
            }
        }
    }
    pub mod transport_belt {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum content_read_mode {
            pulse,
            hold,
        }
        impl crate::defines::Defines<u8> for content_read_mode {
            fn value(&self) -> u8 {
                match self {
                    Self::pulse => 0,
                    Self::hold => 1,
                }
            }
            fn key(&self) -> &'static str {
                match self {
                    Self::pulse => "pulse",
                    Self::hold => "hold",
                }
            }
            fn iter() -> &'static [Self] {
                &[
                    Self::pulse,
                    Self::hold,
                ]
            }
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum r#type {
        container,
        generic_on_off,
        inserter,
        lamp,
        logistic_container,
        roboport,
        storage_tank,
        train_stop,
        decider_combinator,
        arithmetic_combinator,
        constant_combinator,
        transport_belt,
        accumulator,
        rail_signal,
        rail_chain_signal,
        wall,
        mining_drill,
        programmable_speaker,
    }
    impl crate::defines::Defines<u8> for r#type {
        fn value(&self) -> u8 {
            match self {
                Self::container => 1,
                Self::generic_on_off => 2,
                Self::inserter => 3,
                Self::lamp => 4,
                Self::logistic_container => 5,
                Self::roboport => 6,
                Self::storage_tank => 7,
                Self::train_stop => 8,
                Self::decider_combinator => 9,
                Self::arithmetic_combinator => 10,
                Self::constant_combinator => 11,
                Self::transport_belt => 12,
                Self::accumulator => 13,
                Self::rail_signal => 14,
                Self::rail_chain_signal => 18,
                Self::wall => 15,
                Self::mining_drill => 16,
                Self::programmable_speaker => 17,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::container => "container",
                Self::generic_on_off => "generic_on_off",
                Self::inserter => "inserter",
                Self::lamp => "lamp",
                Self::logistic_container => "logistic_container",
                Self::roboport => "roboport",
                Self::storage_tank => "storage_tank",
                Self::train_stop => "train_stop",
                Self::decider_combinator => "decider_combinator",
                Self::arithmetic_combinator => "arithmetic_combinator",
                Self::constant_combinator => "constant_combinator",
                Self::transport_belt => "transport_belt",
                Self::accumulator => "accumulator",
                Self::rail_signal => "rail_signal",
                Self::rail_chain_signal => "rail_chain_signal",
                Self::wall => "wall",
                Self::mining_drill => "mining_drill",
                Self::programmable_speaker => "programmable_speaker",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::container,
                Self::generic_on_off,
                Self::inserter,
                Self::lamp,
                Self::logistic_container,
                Self::roboport,
                Self::storage_tank,
                Self::train_stop,
                Self::decider_combinator,
                Self::arithmetic_combinator,
                Self::constant_combinator,
                Self::transport_belt,
                Self::accumulator,
                Self::rail_signal,
                Self::rail_chain_signal,
                Self::wall,
                Self::mining_drill,
                Self::programmable_speaker,
            ]
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum chunk_generated_status {
    nothing,
    custom_tiles,
    basic_tiles,
    corrected_tiles,
    tiles,
    entities,
}
impl crate::defines::Defines<u8> for chunk_generated_status {
    fn value(&self) -> u8 {
        match self {
            Self::nothing => 0,
            Self::custom_tiles => 10,
            Self::basic_tiles => 20,
            Self::corrected_tiles => 30,
            Self::tiles => 40,
            Self::entities => 50,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::nothing => "nothing",
            Self::custom_tiles => "custom_tiles",
            Self::basic_tiles => "basic_tiles",
            Self::corrected_tiles => "corrected_tiles",
            Self::tiles => "tiles",
            Self::entities => "entities",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::nothing,
            Self::custom_tiles,
            Self::basic_tiles,
            Self::corrected_tiles,
            Self::tiles,
            Self::entities,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum logistic_mode {
    none,
    active_provider,
    storage,
    requester,
    passive_provider,
    buffer,
}
impl crate::defines::Defines<u8> for logistic_mode {
    fn value(&self) -> u8 {
        match self {
            Self::none => 0,
            Self::active_provider => 1,
            Self::storage => 2,
            Self::requester => 3,
            Self::passive_provider => 4,
            Self::buffer => 5,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::none => "none",
            Self::active_provider => "active_provider",
            Self::storage => "storage",
            Self::requester => "requester",
            Self::passive_provider => "passive_provider",
            Self::buffer => "buffer",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::none,
            Self::active_provider,
            Self::storage,
            Self::requester,
            Self::passive_provider,
            Self::buffer,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum logistic_member_index {
    logistic_container,
    vehicle_storage,
    character_requester,
    character_storage,
    character_provider,
    generic_on_off_behavior,
}
impl crate::defines::Defines<u8> for logistic_member_index {
    fn value(&self) -> u8 {
        match self {
            Self::logistic_container => 0,
            Self::vehicle_storage => 1,
            Self::character_requester => 0,
            Self::character_storage => 1,
            Self::character_provider => 2,
            Self::generic_on_off_behavior => 0,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::logistic_container => "logistic_container",
            Self::vehicle_storage => "vehicle_storage",
            Self::character_requester => "character_requester",
            Self::character_storage => "character_storage",
            Self::character_provider => "character_provider",
            Self::generic_on_off_behavior => "generic_on_off_behavior",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::logistic_container,
            Self::vehicle_storage,
            Self::character_requester,
            Self::character_storage,
            Self::character_provider,
            Self::generic_on_off_behavior,
        ]
    }
}

pub mod deconstruction_item {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum entity_filter_mode {
        whitelist,
        blacklist,
    }
    impl crate::defines::Defines<u8> for entity_filter_mode {
        fn value(&self) -> u8 {
            match self {
                Self::whitelist => 0,
                Self::blacklist => 1,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::whitelist => "whitelist",
                Self::blacklist => "blacklist",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::whitelist,
                Self::blacklist,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum tile_filter_mode {
        whitelist,
        blacklist,
    }
    impl crate::defines::Defines<u8> for tile_filter_mode {
        fn value(&self) -> u8 {
            match self {
                Self::whitelist => 0,
                Self::blacklist => 1,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::whitelist => "whitelist",
                Self::blacklist => "blacklist",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::whitelist,
                Self::blacklist,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum tile_selection_mode {
        normal,
        always,
        never,
        only,
    }
    impl crate::defines::Defines<u8> for tile_selection_mode {
        fn value(&self) -> u8 {
            match self {
                Self::normal => 0,
                Self::always => 1,
                Self::never => 2,
                Self::only => 3,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::normal => "normal",
                Self::always => "always",
                Self::never => "never",
                Self::only => "only",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::normal,
                Self::always,
                Self::never,
                Self::only,
            ]
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum alert_type {
    entity_destroyed,
    entity_under_attack,
    not_enough_construction_robots,
    no_material_for_construction,
    not_enough_repair_packs,
    turret_fire,
    custom,
    no_storage,
    train_out_of_fuel,
}
impl crate::defines::Defines<u8> for alert_type {
    fn value(&self) -> u8 {
        match self {
            Self::entity_destroyed => 0,
            Self::entity_under_attack => 1,
            Self::not_enough_construction_robots => 2,
            Self::no_material_for_construction => 3,
            Self::not_enough_repair_packs => 4,
            Self::turret_fire => 5,
            Self::custom => 6,
            Self::no_storage => 7,
            Self::train_out_of_fuel => 8,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::entity_destroyed => "entity_destroyed",
            Self::entity_under_attack => "entity_under_attack",
            Self::not_enough_construction_robots => "not_enough_construction_robots",
            Self::no_material_for_construction => "no_material_for_construction",
            Self::not_enough_repair_packs => "not_enough_repair_packs",
            Self::turret_fire => "turret_fire",
            Self::custom => "custom",
            Self::no_storage => "no_storage",
            Self::train_out_of_fuel => "train_out_of_fuel",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::entity_destroyed,
            Self::entity_under_attack,
            Self::not_enough_construction_robots,
            Self::no_material_for_construction,
            Self::not_enough_repair_packs,
            Self::turret_fire,
            Self::custom,
            Self::no_storage,
            Self::train_out_of_fuel,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum mouse_button_type {
    none,
    left,
    right,
    middle,
}
impl crate::defines::Defines<u8> for mouse_button_type {
    fn value(&self) -> u8 {
        match self {
            Self::none => 1,
            Self::left => 2,
            Self::right => 4,
            Self::middle => 8,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::none => "none",
            Self::left => "left",
            Self::right => "right",
            Self::middle => "middle",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::none,
            Self::left,
            Self::right,
            Self::middle,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum input_action {
    activate_copy,
    activate_cut,
    activate_paste,
    add_permission_group,
    add_train_station,
    admin_action,
    alt_reverse_select_area,
    alt_select_area,
    alt_select_blueprint_entities,
    alternative_copy,
    begin_mining,
    begin_mining_terrain,
    build,
    build_rail,
    build_terrain,
    cancel_craft,
    cancel_deconstruct,
    cancel_new_blueprint,
    cancel_research,
    cancel_upgrade,
    change_active_character_tab,
    change_active_item_group_for_crafting,
    change_active_item_group_for_filters,
    change_active_quick_bar,
    change_arithmetic_combinator_parameters,
    change_decider_combinator_parameters,
    change_entity_label,
    change_item_description,
    change_item_label,
    change_multiplayer_config,
    change_picking_state,
    change_programmable_speaker_alert_parameters,
    change_programmable_speaker_circuit_parameters,
    change_programmable_speaker_parameters,
    change_riding_state,
    change_shooting_state,
    change_train_stop_station,
    change_train_wait_condition,
    change_train_wait_condition_data,
    clear_cursor,
    connect_rolling_stock,
    copy,
    copy_entity_settings,
    copy_opened_blueprint,
    copy_opened_item,
    craft,
    cursor_split,
    cursor_transfer,
    custom_input,
    cycle_blueprint_book_backwards,
    cycle_blueprint_book_forwards,
    deconstruct,
    delete_blueprint_library,
    delete_blueprint_record,
    delete_custom_tag,
    delete_permission_group,
    destroy_item,
    destroy_opened_item,
    disconnect_rolling_stock,
    drag_train_schedule,
    drag_train_wait_condition,
    drop_blueprint_record,
    drop_item,
    edit_blueprint_tool_preview,
    edit_custom_tag,
    edit_permission_group,
    export_blueprint,
    fast_entity_split,
    fast_entity_transfer,
    flush_opened_entity_fluid,
    flush_opened_entity_specific_fluid,
    go_to_train_station,
    grab_blueprint_record,
    gui_checked_state_changed,
    gui_click,
    gui_confirmed,
    gui_elem_changed,
    gui_hover,
    gui_leave,
    gui_location_changed,
    gui_selected_tab_changed,
    gui_selection_state_changed,
    gui_switch_state_changed,
    gui_text_changed,
    gui_value_changed,
    import_blueprint,
    import_blueprint_string,
    import_blueprints_filtered,
    import_permissions_string,
    inventory_split,
    inventory_transfer,
    launch_rocket,
    lua_shortcut,
    map_editor_action,
    market_offer,
    mod_settings_changed,
    open_achievements_gui,
    open_blueprint_library_gui,
    open_blueprint_record,
    open_bonus_gui,
    open_character_gui,
    open_current_vehicle_gui,
    open_equipment,
    open_gui,
    open_item,
    open_logistic_gui,
    open_mod_item,
    open_parent_of_opened_item,
    open_production_gui,
    open_technology_gui,
    open_tips_and_tricks_gui,
    open_train_gui,
    open_train_station_gui,
    open_trains_gui,
    paste_entity_settings,
    place_equipment,
    quick_bar_pick_slot,
    quick_bar_set_selected_page,
    quick_bar_set_slot,
    reassign_blueprint,
    remove_cables,
    remove_train_station,
    reset_assembling_machine,
    reset_item,
    reverse_select_area,
    rotate_entity,
    select_area,
    select_blueprint_entities,
    select_entity_slot,
    select_item,
    select_mapper_slot,
    select_next_valid_gun,
    select_tile_slot,
    send_spidertron,
    set_auto_launch_rocket,
    set_autosort_inventory,
    set_behavior_mode,
    set_car_weapons_control,
    set_circuit_condition,
    set_circuit_mode_of_operation,
    set_controller_logistic_trash_filter_item,
    set_deconstruction_item_tile_selection_mode,
    set_deconstruction_item_trees_and_rocks_only,
    set_entity_color,
    set_entity_energy_property,
    set_entity_logistic_trash_filter_item,
    set_filter,
    set_flat_controller_gui,
    set_heat_interface_mode,
    set_heat_interface_temperature,
    set_infinity_container_filter_item,
    set_infinity_container_remove_unfiltered_items,
    set_infinity_pipe_filter,
    set_inserter_max_stack_size,
    set_inventory_bar,
    set_linked_container_link_i_d,
    set_logistic_filter_item,
    set_logistic_filter_signal,
    set_player_color,
    set_recipe_notifications,
    set_request_from_buffers,
    set_research_finished_stops_game,
    set_signal,
    set_splitter_priority,
    set_train_stopped,
    set_trains_limit,
    set_vehicle_automatic_targeting_parameters,
    setup_assembling_machine,
    setup_blueprint,
    setup_single_blueprint_record,
    smart_pipette,
    spawn_item,
    stack_split,
    stack_transfer,
    start_repair,
    start_research,
    start_walking,
    stop_building_by_moving,
    switch_connect_to_logistic_network,
    switch_constant_combinator_state,
    switch_inserter_filter_mode_state,
    switch_power_switch_state,
    switch_to_rename_stop_gui,
    take_equipment,
    toggle_deconstruction_item_entity_filter_mode,
    toggle_deconstruction_item_tile_filter_mode,
    toggle_driving,
    toggle_enable_vehicle_logistics_while_moving,
    toggle_entity_logistic_requests,
    toggle_equipment_movement_bonus,
    toggle_map_editor,
    toggle_personal_logistic_requests,
    toggle_personal_roboport,
    toggle_show_entity_info,
    translate_string,
    undo,
    upgrade,
    upgrade_opened_blueprint_by_item,
    upgrade_opened_blueprint_by_record,
    use_artillery_remote,
    use_item,
    wire_dragging,
    write_to_console,
}
impl crate::defines::Defines<u8> for input_action {
    fn value(&self) -> u8 {
        match self {
            Self::activate_copy => 45,
            Self::activate_cut => 46,
            Self::activate_paste => 47,
            Self::add_permission_group => 227,
            Self::add_train_station => 96,
            Self::admin_action => 195,
            Self::alt_reverse_select_area => 164,
            Self::alt_select_area => 162,
            Self::alt_select_blueprint_entities => 121,
            Self::alternative_copy => 119,
            Self::begin_mining => 2,
            Self::begin_mining_terrain => 62,
            Self::build => 60,
            Self::build_rail => 159,
            Self::build_terrain => 152,
            Self::cancel_craft => 83,
            Self::cancel_deconstruct => 144,
            Self::cancel_new_blueprint => 18,
            Self::cancel_research => 160,
            Self::cancel_upgrade => 145,
            Self::change_active_character_tab => 100,
            Self::change_active_item_group_for_crafting => 98,
            Self::change_active_item_group_for_filters => 99,
            Self::change_active_quick_bar => 231,
            Self::change_arithmetic_combinator_parameters => 146,
            Self::change_decider_combinator_parameters => 147,
            Self::change_entity_label => 158,
            Self::change_item_description => 157,
            Self::change_item_label => 156,
            Self::change_multiplayer_config => 194,
            Self::change_picking_state => 199,
            Self::change_programmable_speaker_alert_parameters => 149,
            Self::change_programmable_speaker_circuit_parameters => 150,
            Self::change_programmable_speaker_parameters => 148,
            Self::change_riding_state => 63,
            Self::change_shooting_state => 77,
            Self::change_train_stop_station => 97,
            Self::change_train_wait_condition => 153,
            Self::change_train_wait_condition_data => 154,
            Self::clear_cursor => 12,
            Self::connect_rolling_stock => 9,
            Self::copy => 118,
            Self::copy_entity_settings => 20,
            Self::copy_opened_blueprint => 124,
            Self::copy_opened_item => 23,
            Self::craft => 75,
            Self::cursor_split => 71,
            Self::cursor_transfer => 70,
            Self::custom_input => 155,
            Self::cycle_blueprint_book_backwards => 33,
            Self::cycle_blueprint_book_forwards => 32,
            Self::deconstruct => 116,
            Self::delete_blueprint_library => 43,
            Self::delete_blueprint_record => 129,
            Self::delete_custom_tag => 225,
            Self::delete_permission_group => 226,
            Self::destroy_item => 67,
            Self::destroy_opened_item => 22,
            Self::disconnect_rolling_stock => 10,
            Self::drag_train_schedule => 180,
            Self::drag_train_wait_condition => 181,
            Self::drop_blueprint_record => 128,
            Self::drop_item => 59,
            Self::edit_blueprint_tool_preview => 137,
            Self::edit_custom_tag => 172,
            Self::edit_permission_group => 173,
            Self::export_blueprint => 139,
            Self::fast_entity_split => 215,
            Self::fast_entity_transfer => 213,
            Self::flush_opened_entity_fluid => 54,
            Self::flush_opened_entity_specific_fluid => 198,
            Self::go_to_train_station => 240,
            Self::grab_blueprint_record => 127,
            Self::gui_checked_state_changed => 102,
            Self::gui_click => 92,
            Self::gui_confirmed => 93,
            Self::gui_elem_changed => 178,
            Self::gui_hover => 249,
            Self::gui_leave => 250,
            Self::gui_location_changed => 107,
            Self::gui_selected_tab_changed => 104,
            Self::gui_selection_state_changed => 103,
            Self::gui_switch_state_changed => 106,
            Self::gui_text_changed => 101,
            Self::gui_value_changed => 105,
            Self::import_blueprint => 140,
            Self::import_blueprint_string => 174,
            Self::import_blueprints_filtered => 141,
            Self::import_permissions_string => 175,
            Self::inventory_split => 82,
            Self::inventory_transfer => 73,
            Self::launch_rocket => 15,
            Self::lua_shortcut => 196,
            Self::map_editor_action => 191,
            Self::market_offer => 95,
            Self::mod_settings_changed => 170,
            Self::open_achievements_gui => 31,
            Self::open_blueprint_library_gui => 57,
            Self::open_blueprint_record => 126,
            Self::open_bonus_gui => 29,
            Self::open_character_gui => 7,
            Self::open_current_vehicle_gui => 8,
            Self::open_equipment => 69,
            Self::open_gui => 5,
            Self::open_item => 64,
            Self::open_logistic_gui => 40,
            Self::open_mod_item => 68,
            Self::open_parent_of_opened_item => 65,
            Self::open_production_gui => 16,
            Self::open_technology_gui => 14,
            Self::open_tips_and_tricks_gui => 56,
            Self::open_train_gui => 221,
            Self::open_train_station_gui => 238,
            Self::open_trains_gui => 30,
            Self::paste_entity_settings => 21,
            Self::place_equipment => 108,
            Self::quick_bar_pick_slot => 188,
            Self::quick_bar_set_selected_page => 189,
            Self::quick_bar_set_slot => 187,
            Self::reassign_blueprint => 125,
            Self::remove_cables => 138,
            Self::remove_train_station => 239,
            Self::reset_assembling_machine => 13,
            Self::reset_item => 66,
            Self::reverse_select_area => 163,
            Self::rotate_entity => 214,
            Self::select_area => 161,
            Self::select_blueprint_entities => 120,
            Self::select_entity_slot => 183,
            Self::select_item => 182,
            Self::select_mapper_slot => 185,
            Self::select_next_valid_gun => 41,
            Self::select_tile_slot => 184,
            Self::send_spidertron => 111,
            Self::set_auto_launch_rocket => 207,
            Self::set_autosort_inventory => 204,
            Self::set_behavior_mode => 212,
            Self::set_car_weapons_control => 229,
            Self::set_circuit_condition => 86,
            Self::set_circuit_mode_of_operation => 91,
            Self::set_controller_logistic_trash_filter_item => 166,
            Self::set_deconstruction_item_tile_selection_mode => 224,
            Self::set_deconstruction_item_trees_and_rocks_only => 223,
            Self::set_entity_color => 222,
            Self::set_entity_energy_property => 171,
            Self::set_entity_logistic_trash_filter_item => 167,
            Self::set_filter => 84,
            Self::set_flat_controller_gui => 205,
            Self::set_heat_interface_mode => 237,
            Self::set_heat_interface_temperature => 236,
            Self::set_infinity_container_filter_item => 168,
            Self::set_infinity_container_remove_unfiltered_items => 228,
            Self::set_infinity_pipe_filter => 169,
            Self::set_inserter_max_stack_size => 220,
            Self::set_inventory_bar => 113,
            Self::set_linked_container_link_i_d => 248,
            Self::set_logistic_filter_item => 89,
            Self::set_logistic_filter_signal => 90,
            Self::set_player_color => 244,
            Self::set_recipe_notifications => 206,
            Self::set_request_from_buffers => 230,
            Self::set_research_finished_stops_game => 219,
            Self::set_signal => 87,
            Self::set_splitter_priority => 234,
            Self::set_train_stopped => 216,
            Self::set_trains_limit => 246,
            Self::set_vehicle_automatic_targeting_parameters => 151,
            Self::setup_assembling_machine => 78,
            Self::setup_blueprint => 122,
            Self::setup_single_blueprint_record => 123,
            Self::smart_pipette => 80,
            Self::spawn_item => 132,
            Self::stack_split => 81,
            Self::stack_transfer => 72,
            Self::start_repair => 115,
            Self::start_research => 88,
            Self::start_walking => 61,
            Self::stop_building_by_moving => 53,
            Self::switch_connect_to_logistic_network => 211,
            Self::switch_constant_combinator_state => 208,
            Self::switch_inserter_filter_mode_state => 210,
            Self::switch_power_switch_state => 209,
            Self::switch_to_rename_stop_gui => 28,
            Self::take_equipment => 109,
            Self::toggle_deconstruction_item_entity_filter_mode => 38,
            Self::toggle_deconstruction_item_tile_filter_mode => 39,
            Self::toggle_driving => 4,
            Self::toggle_enable_vehicle_logistics_while_moving => 37,
            Self::toggle_entity_logistic_requests => 52,
            Self::toggle_equipment_movement_bonus => 50,
            Self::toggle_map_editor => 42,
            Self::toggle_personal_logistic_requests => 51,
            Self::toggle_personal_roboport => 49,
            Self::toggle_show_entity_info => 24,
            Self::translate_string => 197,
            Self::undo => 48,
            Self::upgrade => 117,
            Self::upgrade_opened_blueprint_by_item => 131,
            Self::upgrade_opened_blueprint_by_record => 130,
            Self::use_artillery_remote => 112,
            Self::use_item => 110,
            Self::wire_dragging => 76,
            Self::write_to_console => 94,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::activate_copy => "activate_copy",
            Self::activate_cut => "activate_cut",
            Self::activate_paste => "activate_paste",
            Self::add_permission_group => "add_permission_group",
            Self::add_train_station => "add_train_station",
            Self::admin_action => "admin_action",
            Self::alt_reverse_select_area => "alt_reverse_select_area",
            Self::alt_select_area => "alt_select_area",
            Self::alt_select_blueprint_entities => "alt_select_blueprint_entities",
            Self::alternative_copy => "alternative_copy",
            Self::begin_mining => "begin_mining",
            Self::begin_mining_terrain => "begin_mining_terrain",
            Self::build => "build",
            Self::build_rail => "build_rail",
            Self::build_terrain => "build_terrain",
            Self::cancel_craft => "cancel_craft",
            Self::cancel_deconstruct => "cancel_deconstruct",
            Self::cancel_new_blueprint => "cancel_new_blueprint",
            Self::cancel_research => "cancel_research",
            Self::cancel_upgrade => "cancel_upgrade",
            Self::change_active_character_tab => "change_active_character_tab",
            Self::change_active_item_group_for_crafting => "change_active_item_group_for_crafting",
            Self::change_active_item_group_for_filters => "change_active_item_group_for_filters",
            Self::change_active_quick_bar => "change_active_quick_bar",
            Self::change_arithmetic_combinator_parameters => "change_arithmetic_combinator_parameters",
            Self::change_decider_combinator_parameters => "change_decider_combinator_parameters",
            Self::change_entity_label => "change_entity_label",
            Self::change_item_description => "change_item_description",
            Self::change_item_label => "change_item_label",
            Self::change_multiplayer_config => "change_multiplayer_config",
            Self::change_picking_state => "change_picking_state",
            Self::change_programmable_speaker_alert_parameters => "change_programmable_speaker_alert_parameters",
            Self::change_programmable_speaker_circuit_parameters => "change_programmable_speaker_circuit_parameters",
            Self::change_programmable_speaker_parameters => "change_programmable_speaker_parameters",
            Self::change_riding_state => "change_riding_state",
            Self::change_shooting_state => "change_shooting_state",
            Self::change_train_stop_station => "change_train_stop_station",
            Self::change_train_wait_condition => "change_train_wait_condition",
            Self::change_train_wait_condition_data => "change_train_wait_condition_data",
            Self::clear_cursor => "clear_cursor",
            Self::connect_rolling_stock => "connect_rolling_stock",
            Self::copy => "copy",
            Self::copy_entity_settings => "copy_entity_settings",
            Self::copy_opened_blueprint => "copy_opened_blueprint",
            Self::copy_opened_item => "copy_opened_item",
            Self::craft => "craft",
            Self::cursor_split => "cursor_split",
            Self::cursor_transfer => "cursor_transfer",
            Self::custom_input => "custom_input",
            Self::cycle_blueprint_book_backwards => "cycle_blueprint_book_backwards",
            Self::cycle_blueprint_book_forwards => "cycle_blueprint_book_forwards",
            Self::deconstruct => "deconstruct",
            Self::delete_blueprint_library => "delete_blueprint_library",
            Self::delete_blueprint_record => "delete_blueprint_record",
            Self::delete_custom_tag => "delete_custom_tag",
            Self::delete_permission_group => "delete_permission_group",
            Self::destroy_item => "destroy_item",
            Self::destroy_opened_item => "destroy_opened_item",
            Self::disconnect_rolling_stock => "disconnect_rolling_stock",
            Self::drag_train_schedule => "drag_train_schedule",
            Self::drag_train_wait_condition => "drag_train_wait_condition",
            Self::drop_blueprint_record => "drop_blueprint_record",
            Self::drop_item => "drop_item",
            Self::edit_blueprint_tool_preview => "edit_blueprint_tool_preview",
            Self::edit_custom_tag => "edit_custom_tag",
            Self::edit_permission_group => "edit_permission_group",
            Self::export_blueprint => "export_blueprint",
            Self::fast_entity_split => "fast_entity_split",
            Self::fast_entity_transfer => "fast_entity_transfer",
            Self::flush_opened_entity_fluid => "flush_opened_entity_fluid",
            Self::flush_opened_entity_specific_fluid => "flush_opened_entity_specific_fluid",
            Self::go_to_train_station => "go_to_train_station",
            Self::grab_blueprint_record => "grab_blueprint_record",
            Self::gui_checked_state_changed => "gui_checked_state_changed",
            Self::gui_click => "gui_click",
            Self::gui_confirmed => "gui_confirmed",
            Self::gui_elem_changed => "gui_elem_changed",
            Self::gui_hover => "gui_hover",
            Self::gui_leave => "gui_leave",
            Self::gui_location_changed => "gui_location_changed",
            Self::gui_selected_tab_changed => "gui_selected_tab_changed",
            Self::gui_selection_state_changed => "gui_selection_state_changed",
            Self::gui_switch_state_changed => "gui_switch_state_changed",
            Self::gui_text_changed => "gui_text_changed",
            Self::gui_value_changed => "gui_value_changed",
            Self::import_blueprint => "import_blueprint",
            Self::import_blueprint_string => "import_blueprint_string",
            Self::import_blueprints_filtered => "import_blueprints_filtered",
            Self::import_permissions_string => "import_permissions_string",
            Self::inventory_split => "inventory_split",
            Self::inventory_transfer => "inventory_transfer",
            Self::launch_rocket => "launch_rocket",
            Self::lua_shortcut => "lua_shortcut",
            Self::map_editor_action => "map_editor_action",
            Self::market_offer => "market_offer",
            Self::mod_settings_changed => "mod_settings_changed",
            Self::open_achievements_gui => "open_achievements_gui",
            Self::open_blueprint_library_gui => "open_blueprint_library_gui",
            Self::open_blueprint_record => "open_blueprint_record",
            Self::open_bonus_gui => "open_bonus_gui",
            Self::open_character_gui => "open_character_gui",
            Self::open_current_vehicle_gui => "open_current_vehicle_gui",
            Self::open_equipment => "open_equipment",
            Self::open_gui => "open_gui",
            Self::open_item => "open_item",
            Self::open_logistic_gui => "open_logistic_gui",
            Self::open_mod_item => "open_mod_item",
            Self::open_parent_of_opened_item => "open_parent_of_opened_item",
            Self::open_production_gui => "open_production_gui",
            Self::open_technology_gui => "open_technology_gui",
            Self::open_tips_and_tricks_gui => "open_tips_and_tricks_gui",
            Self::open_train_gui => "open_train_gui",
            Self::open_train_station_gui => "open_train_station_gui",
            Self::open_trains_gui => "open_trains_gui",
            Self::paste_entity_settings => "paste_entity_settings",
            Self::place_equipment => "place_equipment",
            Self::quick_bar_pick_slot => "quick_bar_pick_slot",
            Self::quick_bar_set_selected_page => "quick_bar_set_selected_page",
            Self::quick_bar_set_slot => "quick_bar_set_slot",
            Self::reassign_blueprint => "reassign_blueprint",
            Self::remove_cables => "remove_cables",
            Self::remove_train_station => "remove_train_station",
            Self::reset_assembling_machine => "reset_assembling_machine",
            Self::reset_item => "reset_item",
            Self::reverse_select_area => "reverse_select_area",
            Self::rotate_entity => "rotate_entity",
            Self::select_area => "select_area",
            Self::select_blueprint_entities => "select_blueprint_entities",
            Self::select_entity_slot => "select_entity_slot",
            Self::select_item => "select_item",
            Self::select_mapper_slot => "select_mapper_slot",
            Self::select_next_valid_gun => "select_next_valid_gun",
            Self::select_tile_slot => "select_tile_slot",
            Self::send_spidertron => "send_spidertron",
            Self::set_auto_launch_rocket => "set_auto_launch_rocket",
            Self::set_autosort_inventory => "set_autosort_inventory",
            Self::set_behavior_mode => "set_behavior_mode",
            Self::set_car_weapons_control => "set_car_weapons_control",
            Self::set_circuit_condition => "set_circuit_condition",
            Self::set_circuit_mode_of_operation => "set_circuit_mode_of_operation",
            Self::set_controller_logistic_trash_filter_item => "set_controller_logistic_trash_filter_item",
            Self::set_deconstruction_item_tile_selection_mode => "set_deconstruction_item_tile_selection_mode",
            Self::set_deconstruction_item_trees_and_rocks_only => "set_deconstruction_item_trees_and_rocks_only",
            Self::set_entity_color => "set_entity_color",
            Self::set_entity_energy_property => "set_entity_energy_property",
            Self::set_entity_logistic_trash_filter_item => "set_entity_logistic_trash_filter_item",
            Self::set_filter => "set_filter",
            Self::set_flat_controller_gui => "set_flat_controller_gui",
            Self::set_heat_interface_mode => "set_heat_interface_mode",
            Self::set_heat_interface_temperature => "set_heat_interface_temperature",
            Self::set_infinity_container_filter_item => "set_infinity_container_filter_item",
            Self::set_infinity_container_remove_unfiltered_items => "set_infinity_container_remove_unfiltered_items",
            Self::set_infinity_pipe_filter => "set_infinity_pipe_filter",
            Self::set_inserter_max_stack_size => "set_inserter_max_stack_size",
            Self::set_inventory_bar => "set_inventory_bar",
            Self::set_linked_container_link_i_d => "set_linked_container_link_i_d",
            Self::set_logistic_filter_item => "set_logistic_filter_item",
            Self::set_logistic_filter_signal => "set_logistic_filter_signal",
            Self::set_player_color => "set_player_color",
            Self::set_recipe_notifications => "set_recipe_notifications",
            Self::set_request_from_buffers => "set_request_from_buffers",
            Self::set_research_finished_stops_game => "set_research_finished_stops_game",
            Self::set_signal => "set_signal",
            Self::set_splitter_priority => "set_splitter_priority",
            Self::set_train_stopped => "set_train_stopped",
            Self::set_trains_limit => "set_trains_limit",
            Self::set_vehicle_automatic_targeting_parameters => "set_vehicle_automatic_targeting_parameters",
            Self::setup_assembling_machine => "setup_assembling_machine",
            Self::setup_blueprint => "setup_blueprint",
            Self::setup_single_blueprint_record => "setup_single_blueprint_record",
            Self::smart_pipette => "smart_pipette",
            Self::spawn_item => "spawn_item",
            Self::stack_split => "stack_split",
            Self::stack_transfer => "stack_transfer",
            Self::start_repair => "start_repair",
            Self::start_research => "start_research",
            Self::start_walking => "start_walking",
            Self::stop_building_by_moving => "stop_building_by_moving",
            Self::switch_connect_to_logistic_network => "switch_connect_to_logistic_network",
            Self::switch_constant_combinator_state => "switch_constant_combinator_state",
            Self::switch_inserter_filter_mode_state => "switch_inserter_filter_mode_state",
            Self::switch_power_switch_state => "switch_power_switch_state",
            Self::switch_to_rename_stop_gui => "switch_to_rename_stop_gui",
            Self::take_equipment => "take_equipment",
            Self::toggle_deconstruction_item_entity_filter_mode => "toggle_deconstruction_item_entity_filter_mode",
            Self::toggle_deconstruction_item_tile_filter_mode => "toggle_deconstruction_item_tile_filter_mode",
            Self::toggle_driving => "toggle_driving",
            Self::toggle_enable_vehicle_logistics_while_moving => "toggle_enable_vehicle_logistics_while_moving",
            Self::toggle_entity_logistic_requests => "toggle_entity_logistic_requests",
            Self::toggle_equipment_movement_bonus => "toggle_equipment_movement_bonus",
            Self::toggle_map_editor => "toggle_map_editor",
            Self::toggle_personal_logistic_requests => "toggle_personal_logistic_requests",
            Self::toggle_personal_roboport => "toggle_personal_roboport",
            Self::toggle_show_entity_info => "toggle_show_entity_info",
            Self::translate_string => "translate_string",
            Self::undo => "undo",
            Self::upgrade => "upgrade",
            Self::upgrade_opened_blueprint_by_item => "upgrade_opened_blueprint_by_item",
            Self::upgrade_opened_blueprint_by_record => "upgrade_opened_blueprint_by_record",
            Self::use_artillery_remote => "use_artillery_remote",
            Self::use_item => "use_item",
            Self::wire_dragging => "wire_dragging",
            Self::write_to_console => "write_to_console",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::activate_copy,
            Self::activate_cut,
            Self::activate_paste,
            Self::add_permission_group,
            Self::add_train_station,
            Self::admin_action,
            Self::alt_reverse_select_area,
            Self::alt_select_area,
            Self::alt_select_blueprint_entities,
            Self::alternative_copy,
            Self::begin_mining,
            Self::begin_mining_terrain,
            Self::build,
            Self::build_rail,
            Self::build_terrain,
            Self::cancel_craft,
            Self::cancel_deconstruct,
            Self::cancel_new_blueprint,
            Self::cancel_research,
            Self::cancel_upgrade,
            Self::change_active_character_tab,
            Self::change_active_item_group_for_crafting,
            Self::change_active_item_group_for_filters,
            Self::change_active_quick_bar,
            Self::change_arithmetic_combinator_parameters,
            Self::change_decider_combinator_parameters,
            Self::change_entity_label,
            Self::change_item_description,
            Self::change_item_label,
            Self::change_multiplayer_config,
            Self::change_picking_state,
            Self::change_programmable_speaker_alert_parameters,
            Self::change_programmable_speaker_circuit_parameters,
            Self::change_programmable_speaker_parameters,
            Self::change_riding_state,
            Self::change_shooting_state,
            Self::change_train_stop_station,
            Self::change_train_wait_condition,
            Self::change_train_wait_condition_data,
            Self::clear_cursor,
            Self::connect_rolling_stock,
            Self::copy,
            Self::copy_entity_settings,
            Self::copy_opened_blueprint,
            Self::copy_opened_item,
            Self::craft,
            Self::cursor_split,
            Self::cursor_transfer,
            Self::custom_input,
            Self::cycle_blueprint_book_backwards,
            Self::cycle_blueprint_book_forwards,
            Self::deconstruct,
            Self::delete_blueprint_library,
            Self::delete_blueprint_record,
            Self::delete_custom_tag,
            Self::delete_permission_group,
            Self::destroy_item,
            Self::destroy_opened_item,
            Self::disconnect_rolling_stock,
            Self::drag_train_schedule,
            Self::drag_train_wait_condition,
            Self::drop_blueprint_record,
            Self::drop_item,
            Self::edit_blueprint_tool_preview,
            Self::edit_custom_tag,
            Self::edit_permission_group,
            Self::export_blueprint,
            Self::fast_entity_split,
            Self::fast_entity_transfer,
            Self::flush_opened_entity_fluid,
            Self::flush_opened_entity_specific_fluid,
            Self::go_to_train_station,
            Self::grab_blueprint_record,
            Self::gui_checked_state_changed,
            Self::gui_click,
            Self::gui_confirmed,
            Self::gui_elem_changed,
            Self::gui_hover,
            Self::gui_leave,
            Self::gui_location_changed,
            Self::gui_selected_tab_changed,
            Self::gui_selection_state_changed,
            Self::gui_switch_state_changed,
            Self::gui_text_changed,
            Self::gui_value_changed,
            Self::import_blueprint,
            Self::import_blueprint_string,
            Self::import_blueprints_filtered,
            Self::import_permissions_string,
            Self::inventory_split,
            Self::inventory_transfer,
            Self::launch_rocket,
            Self::lua_shortcut,
            Self::map_editor_action,
            Self::market_offer,
            Self::mod_settings_changed,
            Self::open_achievements_gui,
            Self::open_blueprint_library_gui,
            Self::open_blueprint_record,
            Self::open_bonus_gui,
            Self::open_character_gui,
            Self::open_current_vehicle_gui,
            Self::open_equipment,
            Self::open_gui,
            Self::open_item,
            Self::open_logistic_gui,
            Self::open_mod_item,
            Self::open_parent_of_opened_item,
            Self::open_production_gui,
            Self::open_technology_gui,
            Self::open_tips_and_tricks_gui,
            Self::open_train_gui,
            Self::open_train_station_gui,
            Self::open_trains_gui,
            Self::paste_entity_settings,
            Self::place_equipment,
            Self::quick_bar_pick_slot,
            Self::quick_bar_set_selected_page,
            Self::quick_bar_set_slot,
            Self::reassign_blueprint,
            Self::remove_cables,
            Self::remove_train_station,
            Self::reset_assembling_machine,
            Self::reset_item,
            Self::reverse_select_area,
            Self::rotate_entity,
            Self::select_area,
            Self::select_blueprint_entities,
            Self::select_entity_slot,
            Self::select_item,
            Self::select_mapper_slot,
            Self::select_next_valid_gun,
            Self::select_tile_slot,
            Self::send_spidertron,
            Self::set_auto_launch_rocket,
            Self::set_autosort_inventory,
            Self::set_behavior_mode,
            Self::set_car_weapons_control,
            Self::set_circuit_condition,
            Self::set_circuit_mode_of_operation,
            Self::set_controller_logistic_trash_filter_item,
            Self::set_deconstruction_item_tile_selection_mode,
            Self::set_deconstruction_item_trees_and_rocks_only,
            Self::set_entity_color,
            Self::set_entity_energy_property,
            Self::set_entity_logistic_trash_filter_item,
            Self::set_filter,
            Self::set_flat_controller_gui,
            Self::set_heat_interface_mode,
            Self::set_heat_interface_temperature,
            Self::set_infinity_container_filter_item,
            Self::set_infinity_container_remove_unfiltered_items,
            Self::set_infinity_pipe_filter,
            Self::set_inserter_max_stack_size,
            Self::set_inventory_bar,
            Self::set_linked_container_link_i_d,
            Self::set_logistic_filter_item,
            Self::set_logistic_filter_signal,
            Self::set_player_color,
            Self::set_recipe_notifications,
            Self::set_request_from_buffers,
            Self::set_research_finished_stops_game,
            Self::set_signal,
            Self::set_splitter_priority,
            Self::set_train_stopped,
            Self::set_trains_limit,
            Self::set_vehicle_automatic_targeting_parameters,
            Self::setup_assembling_machine,
            Self::setup_blueprint,
            Self::setup_single_blueprint_record,
            Self::smart_pipette,
            Self::spawn_item,
            Self::stack_split,
            Self::stack_transfer,
            Self::start_repair,
            Self::start_research,
            Self::start_walking,
            Self::stop_building_by_moving,
            Self::switch_connect_to_logistic_network,
            Self::switch_constant_combinator_state,
            Self::switch_inserter_filter_mode_state,
            Self::switch_power_switch_state,
            Self::switch_to_rename_stop_gui,
            Self::take_equipment,
            Self::toggle_deconstruction_item_entity_filter_mode,
            Self::toggle_deconstruction_item_tile_filter_mode,
            Self::toggle_driving,
            Self::toggle_enable_vehicle_logistics_while_moving,
            Self::toggle_entity_logistic_requests,
            Self::toggle_equipment_movement_bonus,
            Self::toggle_map_editor,
            Self::toggle_personal_logistic_requests,
            Self::toggle_personal_roboport,
            Self::toggle_show_entity_info,
            Self::translate_string,
            Self::undo,
            Self::upgrade,
            Self::upgrade_opened_blueprint_by_item,
            Self::upgrade_opened_blueprint_by_record,
            Self::use_artillery_remote,
            Self::use_item,
            Self::wire_dragging,
            Self::write_to_console,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum build_check_type {
    script,
    manual,
    manual_ghost,
    script_ghost,
    blueprint_ghost,
    ghost_revive,
}
impl crate::defines::Defines<u8> for build_check_type {
    fn value(&self) -> u8 {
        match self {
            Self::script => 0,
            Self::manual => 1,
            Self::manual_ghost => 3,
            Self::script_ghost => 2,
            Self::blueprint_ghost => 4,
            Self::ghost_revive => 5,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::script => "script",
            Self::manual => "manual",
            Self::manual_ghost => "manual_ghost",
            Self::script_ghost => "script_ghost",
            Self::blueprint_ghost => "blueprint_ghost",
            Self::ghost_revive => "ghost_revive",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::script,
            Self::manual,
            Self::manual_ghost,
            Self::script_ghost,
            Self::blueprint_ghost,
            Self::ghost_revive,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum gui_type {
    none,
    entity,
    research,
    controller,
    production,
    item,
    bonus,
    trains,
    achievement,
    blueprint_library,
    equipment,
    logistic,
    other_player,
    permissions,
    tutorials,
    custom,
    server_management,
    player_management,
    tile,
    script_inventory,
}
impl crate::defines::Defines<u8> for gui_type {
    fn value(&self) -> u8 {
        match self {
            Self::none => 0,
            Self::entity => 1,
            Self::research => 2,
            Self::controller => 3,
            Self::production => 4,
            Self::item => 5,
            Self::bonus => 6,
            Self::trains => 7,
            Self::achievement => 8,
            Self::blueprint_library => 9,
            Self::equipment => 10,
            Self::logistic => 11,
            Self::other_player => 12,
            Self::permissions => 14,
            Self::tutorials => 15,
            Self::custom => 16,
            Self::server_management => 17,
            Self::player_management => 18,
            Self::tile => 19,
            Self::script_inventory => 23,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::none => "none",
            Self::entity => "entity",
            Self::research => "research",
            Self::controller => "controller",
            Self::production => "production",
            Self::item => "item",
            Self::bonus => "bonus",
            Self::trains => "trains",
            Self::achievement => "achievement",
            Self::blueprint_library => "blueprint_library",
            Self::equipment => "equipment",
            Self::logistic => "logistic",
            Self::other_player => "other_player",
            Self::permissions => "permissions",
            Self::tutorials => "tutorials",
            Self::custom => "custom",
            Self::server_management => "server_management",
            Self::player_management => "player_management",
            Self::tile => "tile",
            Self::script_inventory => "script_inventory",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::none,
            Self::entity,
            Self::research,
            Self::controller,
            Self::production,
            Self::item,
            Self::bonus,
            Self::trains,
            Self::achievement,
            Self::blueprint_library,
            Self::equipment,
            Self::logistic,
            Self::other_player,
            Self::permissions,
            Self::tutorials,
            Self::custom,
            Self::server_management,
            Self::player_management,
            Self::tile,
            Self::script_inventory,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum behavior_result {
    in_progress,
    fail,
    success,
    deleted,
}
impl crate::defines::Defines<u8> for behavior_result {
    fn value(&self) -> u8 {
        match self {
            Self::in_progress => 0,
            Self::fail => 1,
            Self::success => 2,
            Self::deleted => 3,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::in_progress => "in_progress",
            Self::fail => "fail",
            Self::success => "success",
            Self::deleted => "deleted",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::in_progress,
            Self::fail,
            Self::success,
            Self::deleted,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum flow_precision_index {
    five_seconds,
    one_minute,
    ten_minutes,
    one_hour,
    ten_hours,
    fifty_hours,
    two_hundred_fifty_hours,
    one_thousand_hours,
}
impl crate::defines::Defines<u8> for flow_precision_index {
    fn value(&self) -> u8 {
        match self {
            Self::five_seconds => 0,
            Self::one_minute => 1,
            Self::ten_minutes => 2,
            Self::one_hour => 3,
            Self::ten_hours => 4,
            Self::fifty_hours => 5,
            Self::two_hundred_fifty_hours => 6,
            Self::one_thousand_hours => 7,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::five_seconds => "five_seconds",
            Self::one_minute => "one_minute",
            Self::ten_minutes => "ten_minutes",
            Self::one_hour => "one_hour",
            Self::ten_hours => "ten_hours",
            Self::fifty_hours => "fifty_hours",
            Self::two_hundred_fifty_hours => "two_hundred_fifty_hours",
            Self::one_thousand_hours => "one_thousand_hours",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::five_seconds,
            Self::one_minute,
            Self::ten_minutes,
            Self::one_hour,
            Self::ten_hours,
            Self::fifty_hours,
            Self::two_hundred_fifty_hours,
            Self::one_thousand_hours,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum entity_status {
    working,
    normal,
    no_power,
    low_power,
    no_fuel,
    disabled_by_control_behavior,
    opened_by_circuit_network,
    closed_by_circuit_network,
    disabled_by_script,
    marked_for_deconstruction,
    not_plugged_in_electric_network,
    networks_connected,
    networks_disconnected,
    charging,
    discharging,
    fully_charged,
    out_of_logistic_network,
    no_recipe,
    no_ingredients,
    no_input_fluid,
    no_research_in_progress,
    no_minable_resources,
    low_input_fluid,
    fluid_ingredient_shortage,
    full_output,
    full_burnt_result_output,
    item_ingredient_shortage,
    missing_required_fluid,
    missing_science_packs,
    waiting_for_source_items,
    waiting_for_space_in_destination,
    preparing_rocket_for_launch,
    waiting_to_launch_rocket,
    launching_rocket,
    no_modules_to_transmit,
    recharging_after_power_outage,
    waiting_for_target_to_be_built,
    waiting_for_train,
    no_ammo,
    low_temperature,
    disabled,
    turned_off_during_daytime,
    not_connected_to_rail,
    cant_divide_segments,
}
impl crate::defines::Defines<u8> for entity_status {
    fn value(&self) -> u8 {
        match self {
            Self::working => 1,
            Self::normal => 2,
            Self::no_power => 38,
            Self::low_power => 12,
            Self::no_fuel => 37,
            Self::disabled_by_control_behavior => 39,
            Self::opened_by_circuit_network => 41,
            Self::closed_by_circuit_network => 40,
            Self::disabled_by_script => 42,
            Self::marked_for_deconstruction => 44,
            Self::not_plugged_in_electric_network => 3,
            Self::networks_connected => 4,
            Self::networks_disconnected => 5,
            Self::charging => 6,
            Self::discharging => 7,
            Self::fully_charged => 8,
            Self::out_of_logistic_network => 13,
            Self::no_recipe => 15,
            Self::no_ingredients => 14,
            Self::no_input_fluid => 19,
            Self::no_research_in_progress => 16,
            Self::no_minable_resources => 17,
            Self::low_input_fluid => 18,
            Self::fluid_ingredient_shortage => 20,
            Self::full_output => 22,
            Self::full_burnt_result_output => 23,
            Self::item_ingredient_shortage => 21,
            Self::missing_required_fluid => 24,
            Self::missing_science_packs => 25,
            Self::waiting_for_source_items => 26,
            Self::waiting_for_space_in_destination => 27,
            Self::preparing_rocket_for_launch => 28,
            Self::waiting_to_launch_rocket => 29,
            Self::launching_rocket => 30,
            Self::no_modules_to_transmit => 31,
            Self::recharging_after_power_outage => 32,
            Self::waiting_for_target_to_be_built => 33,
            Self::waiting_for_train => 34,
            Self::no_ammo => 35,
            Self::low_temperature => 36,
            Self::disabled => 43,
            Self::turned_off_during_daytime => 9,
            Self::not_connected_to_rail => 11,
            Self::cant_divide_segments => 10,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::working => "working",
            Self::normal => "normal",
            Self::no_power => "no_power",
            Self::low_power => "low_power",
            Self::no_fuel => "no_fuel",
            Self::disabled_by_control_behavior => "disabled_by_control_behavior",
            Self::opened_by_circuit_network => "opened_by_circuit_network",
            Self::closed_by_circuit_network => "closed_by_circuit_network",
            Self::disabled_by_script => "disabled_by_script",
            Self::marked_for_deconstruction => "marked_for_deconstruction",
            Self::not_plugged_in_electric_network => "not_plugged_in_electric_network",
            Self::networks_connected => "networks_connected",
            Self::networks_disconnected => "networks_disconnected",
            Self::charging => "charging",
            Self::discharging => "discharging",
            Self::fully_charged => "fully_charged",
            Self::out_of_logistic_network => "out_of_logistic_network",
            Self::no_recipe => "no_recipe",
            Self::no_ingredients => "no_ingredients",
            Self::no_input_fluid => "no_input_fluid",
            Self::no_research_in_progress => "no_research_in_progress",
            Self::no_minable_resources => "no_minable_resources",
            Self::low_input_fluid => "low_input_fluid",
            Self::fluid_ingredient_shortage => "fluid_ingredient_shortage",
            Self::full_output => "full_output",
            Self::full_burnt_result_output => "full_burnt_result_output",
            Self::item_ingredient_shortage => "item_ingredient_shortage",
            Self::missing_required_fluid => "missing_required_fluid",
            Self::missing_science_packs => "missing_science_packs",
            Self::waiting_for_source_items => "waiting_for_source_items",
            Self::waiting_for_space_in_destination => "waiting_for_space_in_destination",
            Self::preparing_rocket_for_launch => "preparing_rocket_for_launch",
            Self::waiting_to_launch_rocket => "waiting_to_launch_rocket",
            Self::launching_rocket => "launching_rocket",
            Self::no_modules_to_transmit => "no_modules_to_transmit",
            Self::recharging_after_power_outage => "recharging_after_power_outage",
            Self::waiting_for_target_to_be_built => "waiting_for_target_to_be_built",
            Self::waiting_for_train => "waiting_for_train",
            Self::no_ammo => "no_ammo",
            Self::low_temperature => "low_temperature",
            Self::disabled => "disabled",
            Self::turned_off_during_daytime => "turned_off_during_daytime",
            Self::not_connected_to_rail => "not_connected_to_rail",
            Self::cant_divide_segments => "cant_divide_segments",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::working,
            Self::normal,
            Self::no_power,
            Self::low_power,
            Self::no_fuel,
            Self::disabled_by_control_behavior,
            Self::opened_by_circuit_network,
            Self::closed_by_circuit_network,
            Self::disabled_by_script,
            Self::marked_for_deconstruction,
            Self::not_plugged_in_electric_network,
            Self::networks_connected,
            Self::networks_disconnected,
            Self::charging,
            Self::discharging,
            Self::fully_charged,
            Self::out_of_logistic_network,
            Self::no_recipe,
            Self::no_ingredients,
            Self::no_input_fluid,
            Self::no_research_in_progress,
            Self::no_minable_resources,
            Self::low_input_fluid,
            Self::fluid_ingredient_shortage,
            Self::full_output,
            Self::full_burnt_result_output,
            Self::item_ingredient_shortage,
            Self::missing_required_fluid,
            Self::missing_science_packs,
            Self::waiting_for_source_items,
            Self::waiting_for_space_in_destination,
            Self::preparing_rocket_for_launch,
            Self::waiting_to_launch_rocket,
            Self::launching_rocket,
            Self::no_modules_to_transmit,
            Self::recharging_after_power_outage,
            Self::waiting_for_target_to_be_built,
            Self::waiting_for_train,
            Self::no_ammo,
            Self::low_temperature,
            Self::disabled,
            Self::turned_off_during_daytime,
            Self::not_connected_to_rail,
            Self::cant_divide_segments,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rocket_silo_status {
    building_rocket,
    create_rocket,
    lights_blinking_open,
    doors_opening,
    doors_opened,
    rocket_rising,
    arms_advance,
    rocket_ready,
    launch_starting,
    engine_starting,
    arms_retract,
    rocket_flying,
    lights_blinking_close,
    doors_closing,
    launch_started,
}
impl crate::defines::Defines<u8> for rocket_silo_status {
    fn value(&self) -> u8 {
        match self {
            Self::building_rocket => 0,
            Self::create_rocket => 1,
            Self::lights_blinking_open => 2,
            Self::doors_opening => 3,
            Self::doors_opened => 4,
            Self::rocket_rising => 5,
            Self::arms_advance => 6,
            Self::rocket_ready => 7,
            Self::launch_starting => 8,
            Self::engine_starting => 9,
            Self::arms_retract => 10,
            Self::rocket_flying => 11,
            Self::lights_blinking_close => 12,
            Self::doors_closing => 13,
            Self::launch_started => 14,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::building_rocket => "building_rocket",
            Self::create_rocket => "create_rocket",
            Self::lights_blinking_open => "lights_blinking_open",
            Self::doors_opening => "doors_opening",
            Self::doors_opened => "doors_opened",
            Self::rocket_rising => "rocket_rising",
            Self::arms_advance => "arms_advance",
            Self::rocket_ready => "rocket_ready",
            Self::launch_starting => "launch_starting",
            Self::engine_starting => "engine_starting",
            Self::arms_retract => "arms_retract",
            Self::rocket_flying => "rocket_flying",
            Self::lights_blinking_close => "lights_blinking_close",
            Self::doors_closing => "doors_closing",
            Self::launch_started => "launch_started",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::building_rocket,
            Self::create_rocket,
            Self::lights_blinking_open,
            Self::doors_opening,
            Self::doors_opened,
            Self::rocket_rising,
            Self::arms_advance,
            Self::rocket_ready,
            Self::launch_starting,
            Self::engine_starting,
            Self::arms_retract,
            Self::rocket_flying,
            Self::lights_blinking_close,
            Self::doors_closing,
            Self::launch_started,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum render_mode {
    game,
    chart,
    chart_zoomed_in,
}
impl crate::defines::Defines<u8> for render_mode {
    fn value(&self) -> u8 {
        match self {
            Self::game => 1,
            Self::chart => 2,
            Self::chart_zoomed_in => 3,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::game => "game",
            Self::chart => "chart",
            Self::chart_zoomed_in => "chart_zoomed_in",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::game,
            Self::chart,
            Self::chart_zoomed_in,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum input_method {
    keyboard_and_mouse,
    game_controller,
}
impl crate::defines::Defines<u8> for input_method {
    fn value(&self) -> u8 {
        match self {
            Self::keyboard_and_mouse => 0,
            Self::game_controller => 1,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::keyboard_and_mouse => "keyboard_and_mouse",
            Self::game_controller => "game_controller",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::keyboard_and_mouse,
            Self::game_controller,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum game_controller_interaction {
    always,
    never,
    normal,
}
impl crate::defines::Defines<u8> for game_controller_interaction {
    fn value(&self) -> u8 {
        match self {
            Self::always => 0,
            Self::never => 2,
            Self::normal => 1,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::always => "always",
            Self::never => "never",
            Self::normal => "normal",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::always,
            Self::never,
            Self::normal,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rich_text_setting {
    enabled,
    disabled,
    highlight,
}
impl crate::defines::Defines<u8> for rich_text_setting {
    fn value(&self) -> u8 {
        match self {
            Self::enabled => 17,
            Self::disabled => 0,
            Self::highlight => 30,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::enabled => "enabled",
            Self::disabled => "disabled",
            Self::highlight => "highlight",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::enabled,
            Self::disabled,
            Self::highlight,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum relative_gui_position {
    top,
    bottom,
    left,
    right,
}
impl crate::defines::Defines<u8> for relative_gui_position {
    fn value(&self) -> u8 {
        match self {
            Self::top => 0,
            Self::bottom => 1,
            Self::left => 2,
            Self::right => 3,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::top => "top",
            Self::bottom => "bottom",
            Self::left => "left",
            Self::right => "right",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::top,
            Self::bottom,
            Self::left,
            Self::right,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum relative_gui_type {
    blueprint_library_gui,
    production_gui,
    train_stop_gui,
    bonus_gui,
    tile_variations_gui,
    trains_gui,
    achievement_gui,
    furnace_gui,
    permissions_gui,
    logistic_gui,
    heat_interface_gui,
    controller_gui,
    script_inventory_gui,
    server_config_gui,
    armor_gui,
    admin_gui,
    burner_equipment_gui,
    other_player_gui,
    rename_stop_gui,
    entity_with_energy_source_gui,
    loader_gui,
    blueprint_book_gui,
    item_with_inventory_gui,
    decider_combinator_gui,
    programmable_speaker_gui,
    equipment_grid_gui,
    spider_vehicle_gui,
    deconstruction_item_gui,
    mining_drill_gui,
    upgrade_item_gui,
    transport_belt_gui,
    blueprint_setup_gui,
    inserter_gui,
    assembling_machine_gui,
    splitter_gui,
    lamp_gui,
    infinity_pipe_gui,
    pipe_gui,
    standalone_character_gui,
    lab_gui,
    generic_on_off_entity_gui,
    wall_gui,
    storage_tank_gui,
    power_switch_gui,
    rail_signal_gui,
    rail_chain_signal_gui,
    beacon_gui,
    accumulator_gui,
    reactor_gui,
    car_gui,
    container_gui,
    linked_container_gui,
    assembling_machine_select_recipe_gui,
    electric_network_gui,
    train_gui,
    rocket_silo_gui,
    roboport_gui,
    arithmetic_combinator_gui,
    constant_combinator_gui,
    electric_energy_interface_gui,
    market_gui,
    additional_entity_info_gui,
    resource_entity_gui,
    entity_variations_gui,
}
impl crate::defines::Defines<&'static str> for relative_gui_type {
    fn value(&self) -> &'static str {
        match self {
            Self::blueprint_library_gui => "blueprint-library-gui",
            Self::production_gui => "production-gui",
            Self::train_stop_gui => "train-stop-gui",
            Self::bonus_gui => "bonus-gui",
            Self::tile_variations_gui => "tile-variations-gui",
            Self::trains_gui => "trains-gui",
            Self::achievement_gui => "achievement-gui",
            Self::furnace_gui => "furnace-gui",
            Self::permissions_gui => "permissions-gui",
            Self::logistic_gui => "logistic-gui",
            Self::heat_interface_gui => "heat-interface-gui",
            Self::controller_gui => "controller-gui",
            Self::script_inventory_gui => "script-inventory-gui",
            Self::server_config_gui => "server-config-gui",
            Self::armor_gui => "armor-gui",
            Self::admin_gui => "admin-gui",
            Self::burner_equipment_gui => "burner-equipment-gui",
            Self::other_player_gui => "other-player-gui",
            Self::rename_stop_gui => "rename-stop-gui",
            Self::entity_with_energy_source_gui => "entity-with-energy-source-gui",
            Self::loader_gui => "loader-gui",
            Self::blueprint_book_gui => "blueprint-book-gui",
            Self::item_with_inventory_gui => "item-with-inventory-gui",
            Self::decider_combinator_gui => "decider-combinator-gui",
            Self::programmable_speaker_gui => "programmable-speaker-gui",
            Self::equipment_grid_gui => "equipment-grid-gui",
            Self::spider_vehicle_gui => "spider-vehicle-gui",
            Self::deconstruction_item_gui => "deconstruction-item-gui",
            Self::mining_drill_gui => "mining-drill-gui",
            Self::upgrade_item_gui => "upgrade-item-gui",
            Self::transport_belt_gui => "transport-belt-gui",
            Self::blueprint_setup_gui => "blueprint-setup-gui",
            Self::inserter_gui => "inserter-gui",
            Self::assembling_machine_gui => "assembling-machine-gui",
            Self::splitter_gui => "splitter-gui",
            Self::lamp_gui => "lamp-gui",
            Self::infinity_pipe_gui => "infinity-pipe-gui",
            Self::pipe_gui => "pipe-gui",
            Self::standalone_character_gui => "standalone-character-gui",
            Self::lab_gui => "lab-gui",
            Self::generic_on_off_entity_gui => "generic-on-off-entity-gui",
            Self::wall_gui => "wall-gui",
            Self::storage_tank_gui => "storage-tank-gui",
            Self::power_switch_gui => "power-switch-gui",
            Self::rail_signal_gui => "rail-signal-gui",
            Self::rail_chain_signal_gui => "rail-chain-signal-gui",
            Self::beacon_gui => "beacon-gui",
            Self::accumulator_gui => "accumulator-gui",
            Self::reactor_gui => "reactor-gui",
            Self::car_gui => "car-gui",
            Self::container_gui => "container-gui",
            Self::linked_container_gui => "linked-container-gui",
            Self::assembling_machine_select_recipe_gui => "assembling-machine-select-recipe-gui",
            Self::electric_network_gui => "electric-network-gui",
            Self::train_gui => "train-gui",
            Self::rocket_silo_gui => "rocket-silo-gui",
            Self::roboport_gui => "roboport-gui",
            Self::arithmetic_combinator_gui => "arithmetic-combinator-gui",
            Self::constant_combinator_gui => "constant-combinator-gui",
            Self::electric_energy_interface_gui => "electric-energy-interface-gui",
            Self::market_gui => "market-gui",
            Self::additional_entity_info_gui => "additional-entity-info-gui",
            Self::resource_entity_gui => "resource-entity-gui",
            Self::entity_variations_gui => "entity-variations-gui",
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::blueprint_library_gui => "blueprint_library_gui",
            Self::production_gui => "production_gui",
            Self::train_stop_gui => "train_stop_gui",
            Self::bonus_gui => "bonus_gui",
            Self::tile_variations_gui => "tile_variations_gui",
            Self::trains_gui => "trains_gui",
            Self::achievement_gui => "achievement_gui",
            Self::furnace_gui => "furnace_gui",
            Self::permissions_gui => "permissions_gui",
            Self::logistic_gui => "logistic_gui",
            Self::heat_interface_gui => "heat_interface_gui",
            Self::controller_gui => "controller_gui",
            Self::script_inventory_gui => "script_inventory_gui",
            Self::server_config_gui => "server_config_gui",
            Self::armor_gui => "armor_gui",
            Self::admin_gui => "admin_gui",
            Self::burner_equipment_gui => "burner_equipment_gui",
            Self::other_player_gui => "other_player_gui",
            Self::rename_stop_gui => "rename_stop_gui",
            Self::entity_with_energy_source_gui => "entity_with_energy_source_gui",
            Self::loader_gui => "loader_gui",
            Self::blueprint_book_gui => "blueprint_book_gui",
            Self::item_with_inventory_gui => "item_with_inventory_gui",
            Self::decider_combinator_gui => "decider_combinator_gui",
            Self::programmable_speaker_gui => "programmable_speaker_gui",
            Self::equipment_grid_gui => "equipment_grid_gui",
            Self::spider_vehicle_gui => "spider_vehicle_gui",
            Self::deconstruction_item_gui => "deconstruction_item_gui",
            Self::mining_drill_gui => "mining_drill_gui",
            Self::upgrade_item_gui => "upgrade_item_gui",
            Self::transport_belt_gui => "transport_belt_gui",
            Self::blueprint_setup_gui => "blueprint_setup_gui",
            Self::inserter_gui => "inserter_gui",
            Self::assembling_machine_gui => "assembling_machine_gui",
            Self::splitter_gui => "splitter_gui",
            Self::lamp_gui => "lamp_gui",
            Self::infinity_pipe_gui => "infinity_pipe_gui",
            Self::pipe_gui => "pipe_gui",
            Self::standalone_character_gui => "standalone_character_gui",
            Self::lab_gui => "lab_gui",
            Self::generic_on_off_entity_gui => "generic_on_off_entity_gui",
            Self::wall_gui => "wall_gui",
            Self::storage_tank_gui => "storage_tank_gui",
            Self::power_switch_gui => "power_switch_gui",
            Self::rail_signal_gui => "rail_signal_gui",
            Self::rail_chain_signal_gui => "rail_chain_signal_gui",
            Self::beacon_gui => "beacon_gui",
            Self::accumulator_gui => "accumulator_gui",
            Self::reactor_gui => "reactor_gui",
            Self::car_gui => "car_gui",
            Self::container_gui => "container_gui",
            Self::linked_container_gui => "linked_container_gui",
            Self::assembling_machine_select_recipe_gui => "assembling_machine_select_recipe_gui",
            Self::electric_network_gui => "electric_network_gui",
            Self::train_gui => "train_gui",
            Self::rocket_silo_gui => "rocket_silo_gui",
            Self::roboport_gui => "roboport_gui",
            Self::arithmetic_combinator_gui => "arithmetic_combinator_gui",
            Self::constant_combinator_gui => "constant_combinator_gui",
            Self::electric_energy_interface_gui => "electric_energy_interface_gui",
            Self::market_gui => "market_gui",
            Self::additional_entity_info_gui => "additional_entity_info_gui",
            Self::resource_entity_gui => "resource_entity_gui",
            Self::entity_variations_gui => "entity_variations_gui",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::blueprint_library_gui,
            Self::production_gui,
            Self::train_stop_gui,
            Self::bonus_gui,
            Self::tile_variations_gui,
            Self::trains_gui,
            Self::achievement_gui,
            Self::furnace_gui,
            Self::permissions_gui,
            Self::logistic_gui,
            Self::heat_interface_gui,
            Self::controller_gui,
            Self::script_inventory_gui,
            Self::server_config_gui,
            Self::armor_gui,
            Self::admin_gui,
            Self::burner_equipment_gui,
            Self::other_player_gui,
            Self::rename_stop_gui,
            Self::entity_with_energy_source_gui,
            Self::loader_gui,
            Self::blueprint_book_gui,
            Self::item_with_inventory_gui,
            Self::decider_combinator_gui,
            Self::programmable_speaker_gui,
            Self::equipment_grid_gui,
            Self::spider_vehicle_gui,
            Self::deconstruction_item_gui,
            Self::mining_drill_gui,
            Self::upgrade_item_gui,
            Self::transport_belt_gui,
            Self::blueprint_setup_gui,
            Self::inserter_gui,
            Self::assembling_machine_gui,
            Self::splitter_gui,
            Self::lamp_gui,
            Self::infinity_pipe_gui,
            Self::pipe_gui,
            Self::standalone_character_gui,
            Self::lab_gui,
            Self::generic_on_off_entity_gui,
            Self::wall_gui,
            Self::storage_tank_gui,
            Self::power_switch_gui,
            Self::rail_signal_gui,
            Self::rail_chain_signal_gui,
            Self::beacon_gui,
            Self::accumulator_gui,
            Self::reactor_gui,
            Self::car_gui,
            Self::container_gui,
            Self::linked_container_gui,
            Self::assembling_machine_select_recipe_gui,
            Self::electric_network_gui,
            Self::train_gui,
            Self::rocket_silo_gui,
            Self::roboport_gui,
            Self::arithmetic_combinator_gui,
            Self::constant_combinator_gui,
            Self::electric_energy_interface_gui,
            Self::market_gui,
            Self::additional_entity_info_gui,
            Self::resource_entity_gui,
            Self::entity_variations_gui,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum disconnect_reason {
    quit,
    dropped,
    reconnect,
    wrong_input,
    desync_limit_reached,
    cannot_keep_up,
    afk,
    kicked,
    kicked_and_deleted,
    banned,
    switching_servers,
}
impl crate::defines::Defines<u8> for disconnect_reason {
    fn value(&self) -> u8 {
        match self {
            Self::quit => 0,
            Self::dropped => 1,
            Self::reconnect => 2,
            Self::wrong_input => 3,
            Self::desync_limit_reached => 4,
            Self::cannot_keep_up => 5,
            Self::afk => 6,
            Self::kicked => 7,
            Self::kicked_and_deleted => 8,
            Self::banned => 9,
            Self::switching_servers => 11,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::quit => "quit",
            Self::dropped => "dropped",
            Self::reconnect => "reconnect",
            Self::wrong_input => "wrong_input",
            Self::desync_limit_reached => "desync_limit_reached",
            Self::cannot_keep_up => "cannot_keep_up",
            Self::afk => "afk",
            Self::kicked => "kicked",
            Self::kicked_and_deleted => "kicked_and_deleted",
            Self::banned => "banned",
            Self::switching_servers => "switching_servers",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::quit,
            Self::dropped,
            Self::reconnect,
            Self::wrong_input,
            Self::desync_limit_reached,
            Self::cannot_keep_up,
            Self::afk,
            Self::kicked,
            Self::kicked_and_deleted,
            Self::banned,
            Self::switching_servers,
        ]
    }
}

pub mod prototypes {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum achievement {
        achievement,
        build_entity_achievement,
        combat_robot_count,
        construct_with_robots_achievement,
        deconstruct_with_robots_achievement,
        deliver_by_robots_achievement,
        dont_build_entity_achievement,
        dont_craft_manually_achievement,
        dont_use_entity_in_energy_production_achievement,
        finish_the_game_achievement,
        group_attack_achievement,
        kill_achievement,
        player_damaged_achievement,
        produce_achievement,
        produce_per_hour_achievement,
        research_achievement,
        train_path_achievement,
    }
    impl crate::defines::Defines<u8> for achievement {
        fn value(&self) -> u8 {
            match self {
                Self::achievement => 0,
                Self::build_entity_achievement => 0,
                Self::combat_robot_count => 0,
                Self::construct_with_robots_achievement => 0,
                Self::deconstruct_with_robots_achievement => 0,
                Self::deliver_by_robots_achievement => 0,
                Self::dont_build_entity_achievement => 0,
                Self::dont_craft_manually_achievement => 0,
                Self::dont_use_entity_in_energy_production_achievement => 0,
                Self::finish_the_game_achievement => 0,
                Self::group_attack_achievement => 0,
                Self::kill_achievement => 0,
                Self::player_damaged_achievement => 0,
                Self::produce_achievement => 0,
                Self::produce_per_hour_achievement => 0,
                Self::research_achievement => 0,
                Self::train_path_achievement => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::achievement => "achievement",
                Self::build_entity_achievement => "build-entity-achievement",
                Self::combat_robot_count => "combat-robot-count",
                Self::construct_with_robots_achievement => "construct-with-robots-achievement",
                Self::deconstruct_with_robots_achievement => "deconstruct-with-robots-achievement",
                Self::deliver_by_robots_achievement => "deliver-by-robots-achievement",
                Self::dont_build_entity_achievement => "dont-build-entity-achievement",
                Self::dont_craft_manually_achievement => "dont-craft-manually-achievement",
                Self::dont_use_entity_in_energy_production_achievement => "dont-use-entity-in-energy-production-achievement",
                Self::finish_the_game_achievement => "finish-the-game-achievement",
                Self::group_attack_achievement => "group-attack-achievement",
                Self::kill_achievement => "kill-achievement",
                Self::player_damaged_achievement => "player-damaged-achievement",
                Self::produce_achievement => "produce-achievement",
                Self::produce_per_hour_achievement => "produce-per-hour-achievement",
                Self::research_achievement => "research-achievement",
                Self::train_path_achievement => "train-path-achievement",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::achievement,
                Self::build_entity_achievement,
                Self::combat_robot_count,
                Self::construct_with_robots_achievement,
                Self::deconstruct_with_robots_achievement,
                Self::deliver_by_robots_achievement,
                Self::dont_build_entity_achievement,
                Self::dont_craft_manually_achievement,
                Self::dont_use_entity_in_energy_production_achievement,
                Self::finish_the_game_achievement,
                Self::group_attack_achievement,
                Self::kill_achievement,
                Self::player_damaged_achievement,
                Self::produce_achievement,
                Self::produce_per_hour_achievement,
                Self::research_achievement,
                Self::train_path_achievement,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum ambient_sound {
        ambient_sound,
    }
    impl crate::defines::Defines<u8> for ambient_sound {
        fn value(&self) -> u8 {
            match self {
                Self::ambient_sound => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::ambient_sound => "ambient-sound",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::ambient_sound,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum ammo_category {
        ammo_category,
    }
    impl crate::defines::Defines<u8> for ammo_category {
        fn value(&self) -> u8 {
            match self {
                Self::ammo_category => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::ammo_category => "ammo-category",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::ammo_category,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum animation {
        animation,
    }
    impl crate::defines::Defines<u8> for animation {
        fn value(&self) -> u8 {
            match self {
                Self::animation => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::animation => "animation",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::animation,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum autoplace_control {
        autoplace_control,
    }
    impl crate::defines::Defines<u8> for autoplace_control {
        fn value(&self) -> u8 {
            match self {
                Self::autoplace_control => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::autoplace_control => "autoplace-control",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::autoplace_control,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum custom_input {
        custom_input,
    }
    impl crate::defines::Defines<u8> for custom_input {
        fn value(&self) -> u8 {
            match self {
                Self::custom_input => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::custom_input => "custom-input",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::custom_input,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum damage_type {
        damage_type,
    }
    impl crate::defines::Defines<u8> for damage_type {
        fn value(&self) -> u8 {
            match self {
                Self::damage_type => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::damage_type => "damage-type",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::damage_type,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum decorative {
        optimized_decorative,
    }
    impl crate::defines::Defines<u8> for decorative {
        fn value(&self) -> u8 {
            match self {
                Self::optimized_decorative => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::optimized_decorative => "optimized-decorative",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::optimized_decorative,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum editor_controller {
        editor_controller,
    }
    impl crate::defines::Defines<u8> for editor_controller {
        fn value(&self) -> u8 {
            match self {
                Self::editor_controller => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::editor_controller => "editor-controller",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::editor_controller,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum entity {
        accumulator,
        ammo_turret,
        arithmetic_combinator,
        arrow,
        artillery_flare,
        artillery_projectile,
        artillery_turret,
        artillery_wagon,
        assembling_machine,
        beacon,
        beam,
        boiler,
        burner_generator,
        car,
        cargo_wagon,
        character,
        character_corpse,
        cliff,
        combat_robot,
        constant_combinator,
        construction_robot,
        container,
        corpse,
        curved_rail,
        decider_combinator,
        deconstructible_tile_proxy,
        electric_energy_interface,
        electric_pole,
        electric_turret,
        entity_ghost,
        explosion,
        fire,
        fish,
        flame_thrower_explosion,
        fluid_turret,
        fluid_wagon,
        flying_text,
        furnace,
        gate,
        generator,
        heat_interface,
        heat_pipe,
        highlight_box,
        infinity_container,
        infinity_pipe,
        inserter,
        item_entity,
        item_request_proxy,
        lab,
        lamp,
        land_mine,
        leaf_particle,
        linked_belt,
        linked_container,
        loader,
        loader_1x1,
        locomotive,
        logistic_container,
        logistic_robot,
        market,
        mining_drill,
        offshore_pump,
        particle,
        particle_source,
        pipe,
        pipe_to_ground,
        player_port,
        power_switch,
        programmable_speaker,
        projectile,
        pump,
        radar,
        rail_chain_signal,
        rail_remnants,
        rail_signal,
        reactor,
        resource,
        roboport,
        rocket_silo,
        rocket_silo_rocket,
        rocket_silo_rocket_shadow,
        simple_entity,
        simple_entity_with_force,
        simple_entity_with_owner,
        smoke,
        smoke_with_trigger,
        solar_panel,
        speech_bubble,
        spider_leg,
        spider_vehicle,
        splitter,
        sticker,
        storage_tank,
        straight_rail,
        stream,
        tile_ghost,
        train_stop,
        transport_belt,
        tree,
        turret,
        underground_belt,
        unit,
        unit_spawner,
        wall,
    }
    impl crate::defines::Defines<u8> for entity {
        fn value(&self) -> u8 {
            match self {
                Self::accumulator => 0,
                Self::ammo_turret => 0,
                Self::arithmetic_combinator => 0,
                Self::arrow => 0,
                Self::artillery_flare => 0,
                Self::artillery_projectile => 0,
                Self::artillery_turret => 0,
                Self::artillery_wagon => 0,
                Self::assembling_machine => 0,
                Self::beacon => 0,
                Self::beam => 0,
                Self::boiler => 0,
                Self::burner_generator => 0,
                Self::car => 0,
                Self::cargo_wagon => 0,
                Self::character => 0,
                Self::character_corpse => 0,
                Self::cliff => 0,
                Self::combat_robot => 0,
                Self::constant_combinator => 0,
                Self::construction_robot => 0,
                Self::container => 0,
                Self::corpse => 0,
                Self::curved_rail => 0,
                Self::decider_combinator => 0,
                Self::deconstructible_tile_proxy => 0,
                Self::electric_energy_interface => 0,
                Self::electric_pole => 0,
                Self::electric_turret => 0,
                Self::entity_ghost => 0,
                Self::explosion => 0,
                Self::fire => 0,
                Self::fish => 0,
                Self::flame_thrower_explosion => 0,
                Self::fluid_turret => 0,
                Self::fluid_wagon => 0,
                Self::flying_text => 0,
                Self::furnace => 0,
                Self::gate => 0,
                Self::generator => 0,
                Self::heat_interface => 0,
                Self::heat_pipe => 0,
                Self::highlight_box => 0,
                Self::infinity_container => 0,
                Self::infinity_pipe => 0,
                Self::inserter => 0,
                Self::item_entity => 0,
                Self::item_request_proxy => 0,
                Self::lab => 0,
                Self::lamp => 0,
                Self::land_mine => 0,
                Self::leaf_particle => 0,
                Self::linked_belt => 0,
                Self::linked_container => 0,
                Self::loader => 0,
                Self::loader_1x1 => 0,
                Self::locomotive => 0,
                Self::logistic_container => 0,
                Self::logistic_robot => 0,
                Self::market => 0,
                Self::mining_drill => 0,
                Self::offshore_pump => 0,
                Self::particle => 0,
                Self::particle_source => 0,
                Self::pipe => 0,
                Self::pipe_to_ground => 0,
                Self::player_port => 0,
                Self::power_switch => 0,
                Self::programmable_speaker => 0,
                Self::projectile => 0,
                Self::pump => 0,
                Self::radar => 0,
                Self::rail_chain_signal => 0,
                Self::rail_remnants => 0,
                Self::rail_signal => 0,
                Self::reactor => 0,
                Self::resource => 0,
                Self::roboport => 0,
                Self::rocket_silo => 0,
                Self::rocket_silo_rocket => 0,
                Self::rocket_silo_rocket_shadow => 0,
                Self::simple_entity => 0,
                Self::simple_entity_with_force => 0,
                Self::simple_entity_with_owner => 0,
                Self::smoke => 0,
                Self::smoke_with_trigger => 0,
                Self::solar_panel => 0,
                Self::speech_bubble => 0,
                Self::spider_leg => 0,
                Self::spider_vehicle => 0,
                Self::splitter => 0,
                Self::sticker => 0,
                Self::storage_tank => 0,
                Self::straight_rail => 0,
                Self::stream => 0,
                Self::tile_ghost => 0,
                Self::train_stop => 0,
                Self::transport_belt => 0,
                Self::tree => 0,
                Self::turret => 0,
                Self::underground_belt => 0,
                Self::unit => 0,
                Self::unit_spawner => 0,
                Self::wall => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::accumulator => "accumulator",
                Self::ammo_turret => "ammo-turret",
                Self::arithmetic_combinator => "arithmetic-combinator",
                Self::arrow => "arrow",
                Self::artillery_flare => "artillery-flare",
                Self::artillery_projectile => "artillery-projectile",
                Self::artillery_turret => "artillery-turret",
                Self::artillery_wagon => "artillery-wagon",
                Self::assembling_machine => "assembling-machine",
                Self::beacon => "beacon",
                Self::beam => "beam",
                Self::boiler => "boiler",
                Self::burner_generator => "burner-generator",
                Self::car => "car",
                Self::cargo_wagon => "cargo-wagon",
                Self::character => "character",
                Self::character_corpse => "character-corpse",
                Self::cliff => "cliff",
                Self::combat_robot => "combat-robot",
                Self::constant_combinator => "constant-combinator",
                Self::construction_robot => "construction-robot",
                Self::container => "container",
                Self::corpse => "corpse",
                Self::curved_rail => "curved-rail",
                Self::decider_combinator => "decider-combinator",
                Self::deconstructible_tile_proxy => "deconstructible-tile-proxy",
                Self::electric_energy_interface => "electric-energy-interface",
                Self::electric_pole => "electric-pole",
                Self::electric_turret => "electric-turret",
                Self::entity_ghost => "entity-ghost",
                Self::explosion => "explosion",
                Self::fire => "fire",
                Self::fish => "fish",
                Self::flame_thrower_explosion => "flame-thrower-explosion",
                Self::fluid_turret => "fluid-turret",
                Self::fluid_wagon => "fluid-wagon",
                Self::flying_text => "flying-text",
                Self::furnace => "furnace",
                Self::gate => "gate",
                Self::generator => "generator",
                Self::heat_interface => "heat-interface",
                Self::heat_pipe => "heat-pipe",
                Self::highlight_box => "highlight-box",
                Self::infinity_container => "infinity-container",
                Self::infinity_pipe => "infinity-pipe",
                Self::inserter => "inserter",
                Self::item_entity => "item-entity",
                Self::item_request_proxy => "item-request-proxy",
                Self::lab => "lab",
                Self::lamp => "lamp",
                Self::land_mine => "land-mine",
                Self::leaf_particle => "leaf-particle",
                Self::linked_belt => "linked-belt",
                Self::linked_container => "linked-container",
                Self::loader => "loader",
                Self::loader_1x1 => "loader-1x1",
                Self::locomotive => "locomotive",
                Self::logistic_container => "logistic-container",
                Self::logistic_robot => "logistic-robot",
                Self::market => "market",
                Self::mining_drill => "mining-drill",
                Self::offshore_pump => "offshore-pump",
                Self::particle => "particle",
                Self::particle_source => "particle-source",
                Self::pipe => "pipe",
                Self::pipe_to_ground => "pipe-to-ground",
                Self::player_port => "player-port",
                Self::power_switch => "power-switch",
                Self::programmable_speaker => "programmable-speaker",
                Self::projectile => "projectile",
                Self::pump => "pump",
                Self::radar => "radar",
                Self::rail_chain_signal => "rail-chain-signal",
                Self::rail_remnants => "rail-remnants",
                Self::rail_signal => "rail-signal",
                Self::reactor => "reactor",
                Self::resource => "resource",
                Self::roboport => "roboport",
                Self::rocket_silo => "rocket-silo",
                Self::rocket_silo_rocket => "rocket-silo-rocket",
                Self::rocket_silo_rocket_shadow => "rocket-silo-rocket-shadow",
                Self::simple_entity => "simple-entity",
                Self::simple_entity_with_force => "simple-entity-with-force",
                Self::simple_entity_with_owner => "simple-entity-with-owner",
                Self::smoke => "smoke",
                Self::smoke_with_trigger => "smoke-with-trigger",
                Self::solar_panel => "solar-panel",
                Self::speech_bubble => "speech-bubble",
                Self::spider_leg => "spider-leg",
                Self::spider_vehicle => "spider-vehicle",
                Self::splitter => "splitter",
                Self::sticker => "sticker",
                Self::storage_tank => "storage-tank",
                Self::straight_rail => "straight-rail",
                Self::stream => "stream",
                Self::tile_ghost => "tile-ghost",
                Self::train_stop => "train-stop",
                Self::transport_belt => "transport-belt",
                Self::tree => "tree",
                Self::turret => "turret",
                Self::underground_belt => "underground-belt",
                Self::unit => "unit",
                Self::unit_spawner => "unit-spawner",
                Self::wall => "wall",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::accumulator,
                Self::ammo_turret,
                Self::arithmetic_combinator,
                Self::arrow,
                Self::artillery_flare,
                Self::artillery_projectile,
                Self::artillery_turret,
                Self::artillery_wagon,
                Self::assembling_machine,
                Self::beacon,
                Self::beam,
                Self::boiler,
                Self::burner_generator,
                Self::car,
                Self::cargo_wagon,
                Self::character,
                Self::character_corpse,
                Self::cliff,
                Self::combat_robot,
                Self::constant_combinator,
                Self::construction_robot,
                Self::container,
                Self::corpse,
                Self::curved_rail,
                Self::decider_combinator,
                Self::deconstructible_tile_proxy,
                Self::electric_energy_interface,
                Self::electric_pole,
                Self::electric_turret,
                Self::entity_ghost,
                Self::explosion,
                Self::fire,
                Self::fish,
                Self::flame_thrower_explosion,
                Self::fluid_turret,
                Self::fluid_wagon,
                Self::flying_text,
                Self::furnace,
                Self::gate,
                Self::generator,
                Self::heat_interface,
                Self::heat_pipe,
                Self::highlight_box,
                Self::infinity_container,
                Self::infinity_pipe,
                Self::inserter,
                Self::item_entity,
                Self::item_request_proxy,
                Self::lab,
                Self::lamp,
                Self::land_mine,
                Self::leaf_particle,
                Self::linked_belt,
                Self::linked_container,
                Self::loader,
                Self::loader_1x1,
                Self::locomotive,
                Self::logistic_container,
                Self::logistic_robot,
                Self::market,
                Self::mining_drill,
                Self::offshore_pump,
                Self::particle,
                Self::particle_source,
                Self::pipe,
                Self::pipe_to_ground,
                Self::player_port,
                Self::power_switch,
                Self::programmable_speaker,
                Self::projectile,
                Self::pump,
                Self::radar,
                Self::rail_chain_signal,
                Self::rail_remnants,
                Self::rail_signal,
                Self::reactor,
                Self::resource,
                Self::roboport,
                Self::rocket_silo,
                Self::rocket_silo_rocket,
                Self::rocket_silo_rocket_shadow,
                Self::simple_entity,
                Self::simple_entity_with_force,
                Self::simple_entity_with_owner,
                Self::smoke,
                Self::smoke_with_trigger,
                Self::solar_panel,
                Self::speech_bubble,
                Self::spider_leg,
                Self::spider_vehicle,
                Self::splitter,
                Self::sticker,
                Self::storage_tank,
                Self::straight_rail,
                Self::stream,
                Self::tile_ghost,
                Self::train_stop,
                Self::transport_belt,
                Self::tree,
                Self::turret,
                Self::underground_belt,
                Self::unit,
                Self::unit_spawner,
                Self::wall,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum equipment {
        active_defense_equipment,
        battery_equipment,
        belt_immunity_equipment,
        energy_shield_equipment,
        generator_equipment,
        movement_bonus_equipment,
        night_vision_equipment,
        roboport_equipment,
        solar_panel_equipment,
    }
    impl crate::defines::Defines<u8> for equipment {
        fn value(&self) -> u8 {
            match self {
                Self::active_defense_equipment => 0,
                Self::battery_equipment => 0,
                Self::belt_immunity_equipment => 0,
                Self::energy_shield_equipment => 0,
                Self::generator_equipment => 0,
                Self::movement_bonus_equipment => 0,
                Self::night_vision_equipment => 0,
                Self::roboport_equipment => 0,
                Self::solar_panel_equipment => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::active_defense_equipment => "active-defense-equipment",
                Self::battery_equipment => "battery-equipment",
                Self::belt_immunity_equipment => "belt-immunity-equipment",
                Self::energy_shield_equipment => "energy-shield-equipment",
                Self::generator_equipment => "generator-equipment",
                Self::movement_bonus_equipment => "movement-bonus-equipment",
                Self::night_vision_equipment => "night-vision-equipment",
                Self::roboport_equipment => "roboport-equipment",
                Self::solar_panel_equipment => "solar-panel-equipment",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::active_defense_equipment,
                Self::battery_equipment,
                Self::belt_immunity_equipment,
                Self::energy_shield_equipment,
                Self::generator_equipment,
                Self::movement_bonus_equipment,
                Self::night_vision_equipment,
                Self::roboport_equipment,
                Self::solar_panel_equipment,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum equipment_category {
        equipment_category,
    }
    impl crate::defines::Defines<u8> for equipment_category {
        fn value(&self) -> u8 {
            match self {
                Self::equipment_category => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::equipment_category => "equipment-category",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::equipment_category,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum equipment_grid {
        equipment_grid,
    }
    impl crate::defines::Defines<u8> for equipment_grid {
        fn value(&self) -> u8 {
            match self {
                Self::equipment_grid => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::equipment_grid => "equipment-grid",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::equipment_grid,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum fluid {
        fluid,
    }
    impl crate::defines::Defines<u8> for fluid {
        fn value(&self) -> u8 {
            match self {
                Self::fluid => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::fluid => "fluid",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::fluid,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum font {
        font,
    }
    impl crate::defines::Defines<u8> for font {
        fn value(&self) -> u8 {
            match self {
                Self::font => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::font => "font",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::font,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum fuel_category {
        fuel_category,
    }
    impl crate::defines::Defines<u8> for fuel_category {
        fn value(&self) -> u8 {
            match self {
                Self::fuel_category => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::fuel_category => "fuel-category",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::fuel_category,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum god_controller {
        god_controller,
    }
    impl crate::defines::Defines<u8> for god_controller {
        fn value(&self) -> u8 {
            match self {
                Self::god_controller => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::god_controller => "god-controller",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::god_controller,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum gui_style {
        gui_style,
    }
    impl crate::defines::Defines<u8> for gui_style {
        fn value(&self) -> u8 {
            match self {
                Self::gui_style => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::gui_style => "gui-style",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::gui_style,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum item {
        ammo,
        armor,
        blueprint,
        blueprint_book,
        capsule,
        copy_paste_tool,
        deconstruction_item,
        gun,
        item,
        item_with_entity_data,
        item_with_inventory,
        item_with_label,
        item_with_tags,
        mining_tool,
        module,
        rail_planner,
        repair_tool,
        selection_tool,
        spidertron_remote,
        tool,
        upgrade_item,
    }
    impl crate::defines::Defines<u8> for item {
        fn value(&self) -> u8 {
            match self {
                Self::ammo => 0,
                Self::armor => 0,
                Self::blueprint => 0,
                Self::blueprint_book => 0,
                Self::capsule => 0,
                Self::copy_paste_tool => 0,
                Self::deconstruction_item => 0,
                Self::gun => 0,
                Self::item => 0,
                Self::item_with_entity_data => 0,
                Self::item_with_inventory => 0,
                Self::item_with_label => 0,
                Self::item_with_tags => 0,
                Self::mining_tool => 0,
                Self::module => 0,
                Self::rail_planner => 0,
                Self::repair_tool => 0,
                Self::selection_tool => 0,
                Self::spidertron_remote => 0,
                Self::tool => 0,
                Self::upgrade_item => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::ammo => "ammo",
                Self::armor => "armor",
                Self::blueprint => "blueprint",
                Self::blueprint_book => "blueprint-book",
                Self::capsule => "capsule",
                Self::copy_paste_tool => "copy-paste-tool",
                Self::deconstruction_item => "deconstruction-item",
                Self::gun => "gun",
                Self::item => "item",
                Self::item_with_entity_data => "item-with-entity-data",
                Self::item_with_inventory => "item-with-inventory",
                Self::item_with_label => "item-with-label",
                Self::item_with_tags => "item-with-tags",
                Self::mining_tool => "mining-tool",
                Self::module => "module",
                Self::rail_planner => "rail-planner",
                Self::repair_tool => "repair-tool",
                Self::selection_tool => "selection-tool",
                Self::spidertron_remote => "spidertron-remote",
                Self::tool => "tool",
                Self::upgrade_item => "upgrade-item",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::ammo,
                Self::armor,
                Self::blueprint,
                Self::blueprint_book,
                Self::capsule,
                Self::copy_paste_tool,
                Self::deconstruction_item,
                Self::gun,
                Self::item,
                Self::item_with_entity_data,
                Self::item_with_inventory,
                Self::item_with_label,
                Self::item_with_tags,
                Self::mining_tool,
                Self::module,
                Self::rail_planner,
                Self::repair_tool,
                Self::selection_tool,
                Self::spidertron_remote,
                Self::tool,
                Self::upgrade_item,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum item_group {
        item_group,
    }
    impl crate::defines::Defines<u8> for item_group {
        fn value(&self) -> u8 {
            match self {
                Self::item_group => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::item_group => "item-group",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::item_group,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum item_subgroup {
        item_subgroup,
    }
    impl crate::defines::Defines<u8> for item_subgroup {
        fn value(&self) -> u8 {
            match self {
                Self::item_subgroup => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::item_subgroup => "item-subgroup",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::item_subgroup,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum map_gen_presets {
        map_gen_presets,
    }
    impl crate::defines::Defines<u8> for map_gen_presets {
        fn value(&self) -> u8 {
            match self {
                Self::map_gen_presets => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::map_gen_presets => "map-gen-presets",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::map_gen_presets,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum map_settings {
        map_settings,
    }
    impl crate::defines::Defines<u8> for map_settings {
        fn value(&self) -> u8 {
            match self {
                Self::map_settings => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::map_settings => "map-settings",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::map_settings,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum module_category {
        module_category,
    }
    impl crate::defines::Defines<u8> for module_category {
        fn value(&self) -> u8 {
            match self {
                Self::module_category => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::module_category => "module-category",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::module_category,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum mouse_cursor {
        mouse_cursor,
    }
    impl crate::defines::Defines<u8> for mouse_cursor {
        fn value(&self) -> u8 {
            match self {
                Self::mouse_cursor => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::mouse_cursor => "mouse-cursor",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::mouse_cursor,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum noise_expression {
        noise_expression,
    }
    impl crate::defines::Defines<u8> for noise_expression {
        fn value(&self) -> u8 {
            match self {
                Self::noise_expression => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::noise_expression => "noise-expression",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::noise_expression,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum noise_layer {
        noise_layer,
    }
    impl crate::defines::Defines<u8> for noise_layer {
        fn value(&self) -> u8 {
            match self {
                Self::noise_layer => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::noise_layer => "noise-layer",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::noise_layer,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum particle {
        optimized_particle,
    }
    impl crate::defines::Defines<u8> for particle {
        fn value(&self) -> u8 {
            match self {
                Self::optimized_particle => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::optimized_particle => "optimized-particle",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::optimized_particle,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum recipe {
        recipe,
    }
    impl crate::defines::Defines<u8> for recipe {
        fn value(&self) -> u8 {
            match self {
                Self::recipe => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::recipe => "recipe",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::recipe,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum recipe_category {
        recipe_category,
    }
    impl crate::defines::Defines<u8> for recipe_category {
        fn value(&self) -> u8 {
            match self {
                Self::recipe_category => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::recipe_category => "recipe-category",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::recipe_category,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum resource_category {
        resource_category,
    }
    impl crate::defines::Defines<u8> for resource_category {
        fn value(&self) -> u8 {
            match self {
                Self::resource_category => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::resource_category => "resource-category",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::resource_category,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum shortcut {
        shortcut,
    }
    impl crate::defines::Defines<u8> for shortcut {
        fn value(&self) -> u8 {
            match self {
                Self::shortcut => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::shortcut => "shortcut",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::shortcut,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum sound {
        sound,
    }
    impl crate::defines::Defines<u8> for sound {
        fn value(&self) -> u8 {
            match self {
                Self::sound => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::sound => "sound",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::sound,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum spectator_controller {
        spectator_controller,
    }
    impl crate::defines::Defines<u8> for spectator_controller {
        fn value(&self) -> u8 {
            match self {
                Self::spectator_controller => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::spectator_controller => "spectator-controller",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::spectator_controller,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum sprite {
        sprite,
    }
    impl crate::defines::Defines<u8> for sprite {
        fn value(&self) -> u8 {
            match self {
                Self::sprite => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::sprite => "sprite",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::sprite,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum technology {
        technology,
    }
    impl crate::defines::Defines<u8> for technology {
        fn value(&self) -> u8 {
            match self {
                Self::technology => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::technology => "technology",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::technology,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum tile {
        tile,
    }
    impl crate::defines::Defines<u8> for tile {
        fn value(&self) -> u8 {
            match self {
                Self::tile => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::tile => "tile",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::tile,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum tile_effect {
        tile_effect,
    }
    impl crate::defines::Defines<u8> for tile_effect {
        fn value(&self) -> u8 {
            match self {
                Self::tile_effect => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::tile_effect => "tile-effect",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::tile_effect,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum tips_and_tricks_item {
        tips_and_tricks_item,
    }
    impl crate::defines::Defines<u8> for tips_and_tricks_item {
        fn value(&self) -> u8 {
            match self {
                Self::tips_and_tricks_item => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::tips_and_tricks_item => "tips-and-tricks-item",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::tips_and_tricks_item,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum tips_and_tricks_item_category {
        tips_and_tricks_item_category,
    }
    impl crate::defines::Defines<u8> for tips_and_tricks_item_category {
        fn value(&self) -> u8 {
            match self {
                Self::tips_and_tricks_item_category => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::tips_and_tricks_item_category => "tips-and-tricks-item-category",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::tips_and_tricks_item_category,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum trigger_target_type {
        trigger_target_type,
    }
    impl crate::defines::Defines<u8> for trigger_target_type {
        fn value(&self) -> u8 {
            match self {
                Self::trigger_target_type => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::trigger_target_type => "trigger-target-type",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::trigger_target_type,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum trivial_smoke {
        trivial_smoke,
    }
    impl crate::defines::Defines<u8> for trivial_smoke {
        fn value(&self) -> u8 {
            match self {
                Self::trivial_smoke => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::trivial_smoke => "trivial-smoke",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::trivial_smoke,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum tutorial {
        tutorial,
    }
    impl crate::defines::Defines<u8> for tutorial {
        fn value(&self) -> u8 {
            match self {
                Self::tutorial => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::tutorial => "tutorial",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::tutorial,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum utility_constants {
        utility_constants,
    }
    impl crate::defines::Defines<u8> for utility_constants {
        fn value(&self) -> u8 {
            match self {
                Self::utility_constants => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::utility_constants => "utility-constants",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::utility_constants,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum utility_sounds {
        utility_sounds,
    }
    impl crate::defines::Defines<u8> for utility_sounds {
        fn value(&self) -> u8 {
            match self {
                Self::utility_sounds => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::utility_sounds => "utility-sounds",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::utility_sounds,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum utility_sprites {
        utility_sprites,
    }
    impl crate::defines::Defines<u8> for utility_sprites {
        fn value(&self) -> u8 {
            match self {
                Self::utility_sprites => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::utility_sprites => "utility-sprites",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::utility_sprites,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum virtual_signal {
        virtual_signal,
    }
    impl crate::defines::Defines<u8> for virtual_signal {
        fn value(&self) -> u8 {
            match self {
                Self::virtual_signal => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::virtual_signal => "virtual-signal",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::virtual_signal,
            ]
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum wind_sound {
        wind_sound,
    }
    impl crate::defines::Defines<u8> for wind_sound {
        fn value(&self) -> u8 {
            match self {
                Self::wind_sound => 0,
            }
        }
        fn key(&self) -> &'static str {
            match self {
                Self::wind_sound => "wind-sound",
            }
        }
        fn iter() -> &'static [Self] {
            &[
                Self::wind_sound,
            ]
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum print_sound {
    always,
    never,
    use_player_settings,
}
impl crate::defines::Defines<u8> for print_sound {
    fn value(&self) -> u8 {
        match self {
            Self::always => 1,
            Self::never => 0,
            Self::use_player_settings => 2,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::always => "always",
            Self::never => "never",
            Self::use_player_settings => "use_player_settings",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::always,
            Self::never,
            Self::use_player_settings,
        ]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum print_skip {
    never,
    if_redundant,
    if_visible,
}
impl crate::defines::Defines<u8> for print_skip {
    fn value(&self) -> u8 {
        match self {
            Self::never => 0,
            Self::if_redundant => 1,
            Self::if_visible => 2,
        }
    }
    fn key(&self) -> &'static str {
        match self {
            Self::never => "never",
            Self::if_redundant => "if_redundant",
            Self::if_visible => "if_visible",
        }
    }
    fn iter() -> &'static [Self] {
        &[
            Self::never,
            Self::if_redundant,
            Self::if_visible,
        ]
    }
}


