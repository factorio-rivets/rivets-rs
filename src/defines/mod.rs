#![allow(clippy::enum_variant_names)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::match_same_arms)]
#![allow(non_camel_case_types)]

pub trait Defines<T>: std::ops::Deref {
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
impl std::ops::Deref for inventory {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 45] = [
            1, // fuel
            6, // burnt_result
            1, // chest
            2, // furnace_source
            3, // furnace_result
            4, // furnace_modules
            1, // character_main
            3, // character_guns
            4, // character_ammo
            5, // character_armor
            7, // character_vehicle
            8, // character_trash
            2, // god_main
            1, // editor_main
            3, // editor_guns
            4, // editor_ammo
            5, // editor_armor
            1, // roboport_robot
            2, // roboport_material
            1, // robot_cargo
            2, // robot_repair
            2, // assembling_machine_input
            3, // assembling_machine_output
            4, // assembling_machine_modules
            2, // lab_input
            3, // lab_modules
            2, // mining_drill_modules
            1, // item_main
            5, // rocket_silo_rocket
            6, // rocket_silo_result
            2, // rocket_silo_input
            3, // rocket_silo_output
            4, // rocket_silo_modules
            1, // rocket
            2, // car_trunk
            3, // car_ammo
            1, // cargo_wagon
            1, // turret_ammo
            1, // beacon_modules
            1, // character_corpse
            1, // artillery_turret_ammo
            1, // artillery_wagon_ammo
            2, // spider_trunk
            3, // spider_ammo
            4, // spider_trash
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for inventory {
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
impl std::ops::Deref for transport_line {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 10] = [
            1, // left_line
            2, // right_line
            3, // left_underground_line
            4, // right_underground_line
            3, // secondary_left_line
            4, // secondary_right_line
            5, // left_split_line
            6, // right_split_line
            7, // secondary_left_split_line
            8, // secondary_right_split_line
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for transport_line {
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
impl std::ops::Deref for direction {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 8] = [
            0, // north
            1, // northeast
            2, // east
            3, // southeast
            4, // south
            5, // southwest
            6, // west
            7, // northwest
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for direction {
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
    impl std::ops::Deref for acceleration {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 4] = [
                0, // nothing
                1, // accelerating
                2, // braking
                3, // reversing
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for acceleration {
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
    impl std::ops::Deref for direction {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 3] = [
                0, // left
                1, // straight
                2, // right
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for direction {
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
impl std::ops::Deref for shooting {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 3] = [
            0, // not_shooting
            1, // shooting_enemies
            2, // shooting_selected
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for shooting {
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
impl std::ops::Deref for command {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 9] = [
            1, // attack
            2, // go_to_location
            3, // compound
            4, // group
            5, // attack_area
            6, // wander
            8, // flee
            9, // stop
            7, // build_base
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for command {
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
impl std::ops::Deref for distraction {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 4] = [
            0, // none
            1, // by_enemy
            3, // by_anything
            4, // by_damage
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for distraction {
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
impl std::ops::Deref for compound_command {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 3] = [
            0, // logical_and
            1, // logical_or
            2, // return_last
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for compound_command {
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
impl std::ops::Deref for difficulty {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 3] = [
            0, // easy
            1, // normal
            2, // hard
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for difficulty {
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
    impl std::ops::Deref for recipe_difficulty {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 2] = [
                0, // normal
                1, // expensive
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for recipe_difficulty {
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
    impl std::ops::Deref for technology_difficulty {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 2] = [
                0, // normal
                1, // expensive
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for technology_difficulty {
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
impl std::ops::Deref for events {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 184] = [
            183, // on_player_input_method_changed
            182, // on_cutscene_finished
            181, // on_cutscene_started
            180, // on_entity_color_changed
            179, // on_gui_leave
            178, // on_gui_hover
            177, // on_player_alt_reverse_selected_area
            176, // on_player_reverse_selected_area
            175, // on_equipment_removed
            174, // on_equipment_inserted
            173, // on_entity_logistic_slot_changed
            172, // on_spider_command_completed
            171, // on_player_used_spider_remote
            170, // on_player_configured_spider_remote
            169, // on_cutscene_cancelled
            168, // on_permission_group_added
            167, // on_permission_group_deleted
            166, // on_pre_permission_group_deleted
            165, // on_permission_string_imported
            164, // on_pre_permission_string_imported
            163, // on_permission_group_edited
            162, // on_player_flushed_fluid
            161, // on_player_clicked_gps_tag
            160, // on_entity_destroyed
            159, // on_script_inventory_resized
            158, // on_pre_script_inventory_resized
            157, // on_pre_player_toggled_map_editor
            156, // on_player_set_quick_bar_slot
            155, // on_script_trigger_effect
            154, // on_string_translated
            153, // on_force_cease_fire_changed
            152, // on_force_friends_changed
            151, // on_gui_switch_state_changed
            150, // on_gui_selected_tab_changed
            149, // on_gui_location_changed
            148, // on_gui_confirmed
            147, // on_chart_tag_removed
            146, // on_chart_tag_modified
            145, // on_chart_tag_added
            144, // on_trigger_fired_artillery
            143, // on_build_base_arrived
            142, // on_unit_group_finished_gathering
            141, // on_unit_removed_from_group
            140, // on_unit_added_to_group
            139, // on_unit_group_created
            138, // on_pre_player_removed
            137, // on_entity_spawned
            136, // on_post_entity_died
            135, // on_robot_exploded_cliff
            134, // on_pre_robot_exploded_cliff
            133, // on_pre_chunk_deleted
            132, // on_player_fast_transferred
            131, // on_player_repaired_entity
            130, // on_player_toggled_alt_mode
            129, // on_surface_renamed
            128, // on_surface_imported
            127, // on_game_created_from_scenario
            126, // on_brush_cloned
            125, // on_area_cloned
            124, // on_entity_cloned
            123, // on_player_toggled_map_editor
            122, // on_cancelled_upgrade
            121, // on_marked_for_upgrade
            120, // on_ai_command_completed
            119, // on_script_path_request_finished
            118, // on_rocket_launch_ordered
            117, // on_player_unbanned
            116, // on_player_kicked
            115, // on_player_banned
            114, // on_train_schedule_changed
            113, // on_chunk_deleted
            112, // on_pre_surface_cleared
            111, // on_surface_cleared
            110, // on_pre_player_left_game
            109, // on_player_trash_inventory_changed
            108, // on_forces_merged
            107, // on_land_mine_armed
            106, // on_force_reset
            105, // on_technology_effects_reset
            104, // on_chunk_charted
            103, // on_entity_damaged
            102, // on_player_cancelled_crafting
            101, // on_pre_player_crafted_item
            100, // on_player_display_scale_changed
            99, // on_player_display_resolution_changed
            98, // on_player_pipette
            97, // on_pre_ghost_upgraded
            96, // on_pre_ghost_deconstructed
            95, // on_character_corpse_expired
            94, // on_player_cheat_mode_disabled
            93, // on_player_cheat_mode_enabled
            92, // on_player_unmuted
            91, // on_player_muted
            90, // on_gui_value_changed
            89, // on_gui_closed
            88, // on_gui_opened
            87, // on_mod_item_opened
            86, // on_player_changed_position
            85, // on_worker_robot_expired
            84, // on_combat_robot_expired
            83, // script_raised_set_tiles
            82, // script_raised_teleported
            81, // script_raised_revive
            80, // script_raised_destroy
            79, // script_raised_built
            78, // on_player_demoted
            77, // on_player_promoted
            76, // on_player_used_capsule
            75, // on_player_removed
            74, // on_console_command
            73, // on_console_chat
            72, // on_player_configured_blueprint
            71, // on_player_deconstructed_area
            70, // on_player_setup_blueprint
            69, // on_gui_elem_changed
            68, // on_train_created
            67, // on_player_mined_entity
            66, // on_robot_mined_entity
            65, // on_pre_surface_deleted
            64, // on_surface_deleted
            63, // on_surface_created
            62, // on_difficulty_settings_changed
            61, // on_runtime_mod_setting_changed
            60, // on_gui_selection_state_changed
            59, // on_entity_renamed
            58, // on_player_changed_force
            57, // on_biter_base_built
            56, // on_player_dropped_item
            55, // on_market_item_purchased
            54, // on_selected_entity_changed
            53, // on_player_changed_surface
            52, // on_player_alt_selected_area
            51, // on_player_selected_area
            50, // on_robot_mined_tile
            49, // on_robot_built_tile
            48, // on_player_mined_tile
            47, // on_player_built_tile
            46, // on_player_left_game
            45, // on_player_joined_game
            44, // on_player_respawned
            43, // on_player_died
            42, // on_pre_player_died
            41, // on_player_removed_equipment
            40, // on_player_placed_equipment
            39, // on_player_gun_inventory_changed
            38, // on_player_ammo_inventory_changed
            37, // on_player_armor_inventory_changed
            36, // on_lua_shortcut
            35, // on_cutscene_waypoint_reached
            34, // on_player_main_inventory_changed
            33, // on_entity_settings_pasted
            32, // on_pre_entity_settings_pasted
            31, // on_player_cursor_stack_changed
            30, // on_forces_merging
            29, // on_force_created
            28, // on_player_driving_changed_state
            27, // on_resource_depleted
            26, // on_player_created
            25, // on_train_changed_state
            24, // on_trigger_created_entity
            23, // on_cancelled_deconstruction
            22, // on_marked_for_deconstruction
            21, // on_player_rotated_entity
            20, // on_research_cancelled
            19, // on_research_reversed
            18, // on_research_finished
            17, // on_research_started
            16, // on_robot_mined
            15, // on_robot_pre_mined
            14, // on_robot_built_entity
            13, // on_player_crafted_item
            12, // on_chunk_generated
            11, // on_pre_player_mined_item
            10, // on_rocket_launched
            9, // on_pre_build
            8, // on_player_mined_item
            7, // on_sector_scanned
            6, // on_built_entity
            5, // on_picked_up_item
            4, // on_entity_died
            3, // on_gui_checked_state_changed
            2, // on_gui_text_changed
            1, // on_gui_click
            0, // on_tick
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for events {
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
impl std::ops::Deref for controllers {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 6] = [
            0, // ghost
            1, // character
            2, // god
            4, // editor
            6, // cutscene
            5, // spectator
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for controllers {
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
impl std::ops::Deref for group_state {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 7] = [
            0, // gathering
            1, // moving
            2, // attacking_distraction
            3, // attacking_target
            4, // finished
            5, // pathfinding
            6, // wander_in_group
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for group_state {
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
impl std::ops::Deref for wire_type {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 3] = [
            2, // red
            3, // green
            1, // copper
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for wire_type {
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
impl std::ops::Deref for circuit_connector_id {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 17] = [
            1, // accumulator
            1, // constant_combinator
            1, // container
            1, // linked_container
            1, // programmable_speaker
            1, // rail_signal
            1, // rail_chain_signal
            1, // roboport
            1, // storage_tank
            1, // wall
            1, // electric_pole
            1, // inserter
            1, // lamp
            1, // combinator_input
            2, // combinator_output
            1, // offshore_pump
            1, // pump
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for circuit_connector_id {
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
impl std::ops::Deref for circuit_condition_index {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 8] = [
            1, // inserter_circuit
            2, // inserter_logistic
            1, // lamp
            1, // arithmetic_combinator
            1, // decider_combinator
            1, // constant_combinator
            1, // offshore_pump
            1, // pump
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for circuit_condition_index {
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
impl std::ops::Deref for wire_connection_id {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 3] = [
            0, // electric_pole
            0, // power_switch_left
            1, // power_switch_right
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for wire_connection_id {
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
impl std::ops::Deref for train_state {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 11] = [
            0, // on_the_path
            1, // path_lost
            2, // no_schedule
            3, // no_path
            4, // arrive_signal
            5, // wait_signal
            6, // arrive_station
            7, // wait_station
            8, // manual_control_stop
            9, // manual_control
            10, // destination_full
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for train_state {
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
impl std::ops::Deref for signal_state {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 4] = [
            0, // open
            1, // closed
            2, // reserved
            3, // reserved_by_circuit_network
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for signal_state {
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
impl std::ops::Deref for chain_signal_state {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 4] = [
            0, // none
            1, // all_open
            2, // partially_open
            3, // none_open
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for chain_signal_state {
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
impl std::ops::Deref for rail_direction {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 2] = [
            0, // front
            1, // back
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for rail_direction {
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
impl std::ops::Deref for rail_connection_direction {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 4] = [
            0, // left
            1, // straight
            2, // right
            3, // none
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for rail_connection_direction {
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
        impl std::ops::Deref for circuit_mode_of_operation {
            type Target = u8;
            fn deref(&self) -> &'static u8 {
                const VALUES: [u8; 5] = [
                    3, // none
                    0, // enable_disable
                    1, // set_filters
                    2, // read_hand_contents
                    4, // set_stack_size
                ];
                &VALUES[*self as usize]
            }
        }
        impl crate::defines::Defines<u8> for circuit_mode_of_operation {
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
        impl std::ops::Deref for hand_read_mode {
            type Target = u8;
            fn deref(&self) -> &'static u8 {
                const VALUES: [u8; 2] = [
                    1, // hold
                    0, // pulse
                ];
                &VALUES[*self as usize]
            }
        }
        impl crate::defines::Defines<u8> for hand_read_mode {
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
        impl std::ops::Deref for circuit_mode_of_operation {
            type Target = u8;
            fn deref(&self) -> &'static u8 {
                const VALUES: [u8; 2] = [
                    0, // send_contents
                    1, // set_requests
                ];
                &VALUES[*self as usize]
            }
        }
        impl crate::defines::Defines<u8> for circuit_mode_of_operation {
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
        impl std::ops::Deref for circuit_mode_of_operation {
            type Target = u8;
            fn deref(&self) -> &'static u8 {
                const VALUES: [u8; 1] = [
                    0, // use_colors
                ];
                &VALUES[*self as usize]
            }
        }
        impl crate::defines::Defines<u8> for circuit_mode_of_operation {
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
        impl std::ops::Deref for resource_read_mode {
            type Target = u8;
            fn deref(&self) -> &'static u8 {
                const VALUES: [u8; 2] = [
                    0, // this_miner
                    1, // entire_patch
                ];
                &VALUES[*self as usize]
            }
        }
        impl crate::defines::Defines<u8> for resource_read_mode {
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
        impl std::ops::Deref for content_read_mode {
            type Target = u8;
            fn deref(&self) -> &'static u8 {
                const VALUES: [u8; 2] = [
                    0, // pulse
                    1, // hold
                ];
                &VALUES[*self as usize]
            }
        }
        impl crate::defines::Defines<u8> for content_read_mode {
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
    impl std::ops::Deref for r#type {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 18] = [
                1, // container
                2, // generic_on_off
                3, // inserter
                4, // lamp
                5, // logistic_container
                6, // roboport
                7, // storage_tank
                8, // train_stop
                9, // decider_combinator
                10, // arithmetic_combinator
                11, // constant_combinator
                12, // transport_belt
                13, // accumulator
                14, // rail_signal
                18, // rail_chain_signal
                15, // wall
                16, // mining_drill
                17, // programmable_speaker
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for r#type {
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
impl std::ops::Deref for chunk_generated_status {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 6] = [
            0, // nothing
            10, // custom_tiles
            20, // basic_tiles
            30, // corrected_tiles
            40, // tiles
            50, // entities
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for chunk_generated_status {
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
impl std::ops::Deref for logistic_mode {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 6] = [
            0, // none
            1, // active_provider
            2, // storage
            3, // requester
            4, // passive_provider
            5, // buffer
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for logistic_mode {
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
impl std::ops::Deref for logistic_member_index {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 6] = [
            0, // logistic_container
            1, // vehicle_storage
            0, // character_requester
            1, // character_storage
            2, // character_provider
            0, // generic_on_off_behavior
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for logistic_member_index {
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
    impl std::ops::Deref for entity_filter_mode {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 2] = [
                0, // whitelist
                1, // blacklist
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for entity_filter_mode {
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
    impl std::ops::Deref for tile_filter_mode {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 2] = [
                0, // whitelist
                1, // blacklist
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for tile_filter_mode {
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
    impl std::ops::Deref for tile_selection_mode {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 4] = [
                0, // normal
                1, // always
                2, // never
                3, // only
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for tile_selection_mode {
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
impl std::ops::Deref for alert_type {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 9] = [
            0, // entity_destroyed
            1, // entity_under_attack
            2, // not_enough_construction_robots
            3, // no_material_for_construction
            4, // not_enough_repair_packs
            5, // turret_fire
            6, // custom
            7, // no_storage
            8, // train_out_of_fuel
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for alert_type {
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
impl std::ops::Deref for mouse_button_type {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 4] = [
            1, // none
            2, // left
            4, // right
            8, // middle
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for mouse_button_type {
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
impl std::ops::Deref for input_action {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 203] = [
            45, // activate_copy
            46, // activate_cut
            47, // activate_paste
            227, // add_permission_group
            96, // add_train_station
            195, // admin_action
            164, // alt_reverse_select_area
            162, // alt_select_area
            121, // alt_select_blueprint_entities
            119, // alternative_copy
            2, // begin_mining
            62, // begin_mining_terrain
            60, // build
            159, // build_rail
            152, // build_terrain
            83, // cancel_craft
            144, // cancel_deconstruct
            18, // cancel_new_blueprint
            160, // cancel_research
            145, // cancel_upgrade
            100, // change_active_character_tab
            98, // change_active_item_group_for_crafting
            99, // change_active_item_group_for_filters
            231, // change_active_quick_bar
            146, // change_arithmetic_combinator_parameters
            147, // change_decider_combinator_parameters
            158, // change_entity_label
            157, // change_item_description
            156, // change_item_label
            194, // change_multiplayer_config
            199, // change_picking_state
            149, // change_programmable_speaker_alert_parameters
            150, // change_programmable_speaker_circuit_parameters
            148, // change_programmable_speaker_parameters
            63, // change_riding_state
            77, // change_shooting_state
            97, // change_train_stop_station
            153, // change_train_wait_condition
            154, // change_train_wait_condition_data
            12, // clear_cursor
            9, // connect_rolling_stock
            118, // copy
            20, // copy_entity_settings
            124, // copy_opened_blueprint
            23, // copy_opened_item
            75, // craft
            71, // cursor_split
            70, // cursor_transfer
            155, // custom_input
            33, // cycle_blueprint_book_backwards
            32, // cycle_blueprint_book_forwards
            116, // deconstruct
            43, // delete_blueprint_library
            129, // delete_blueprint_record
            225, // delete_custom_tag
            226, // delete_permission_group
            67, // destroy_item
            22, // destroy_opened_item
            10, // disconnect_rolling_stock
            180, // drag_train_schedule
            181, // drag_train_wait_condition
            128, // drop_blueprint_record
            59, // drop_item
            137, // edit_blueprint_tool_preview
            172, // edit_custom_tag
            173, // edit_permission_group
            139, // export_blueprint
            215, // fast_entity_split
            213, // fast_entity_transfer
            54, // flush_opened_entity_fluid
            198, // flush_opened_entity_specific_fluid
            240, // go_to_train_station
            127, // grab_blueprint_record
            102, // gui_checked_state_changed
            92, // gui_click
            93, // gui_confirmed
            178, // gui_elem_changed
            249, // gui_hover
            250, // gui_leave
            107, // gui_location_changed
            104, // gui_selected_tab_changed
            103, // gui_selection_state_changed
            106, // gui_switch_state_changed
            101, // gui_text_changed
            105, // gui_value_changed
            140, // import_blueprint
            174, // import_blueprint_string
            141, // import_blueprints_filtered
            175, // import_permissions_string
            82, // inventory_split
            73, // inventory_transfer
            15, // launch_rocket
            196, // lua_shortcut
            191, // map_editor_action
            95, // market_offer
            170, // mod_settings_changed
            31, // open_achievements_gui
            57, // open_blueprint_library_gui
            126, // open_blueprint_record
            29, // open_bonus_gui
            7, // open_character_gui
            8, // open_current_vehicle_gui
            69, // open_equipment
            5, // open_gui
            64, // open_item
            40, // open_logistic_gui
            68, // open_mod_item
            65, // open_parent_of_opened_item
            16, // open_production_gui
            14, // open_technology_gui
            56, // open_tips_and_tricks_gui
            221, // open_train_gui
            238, // open_train_station_gui
            30, // open_trains_gui
            21, // paste_entity_settings
            108, // place_equipment
            188, // quick_bar_pick_slot
            189, // quick_bar_set_selected_page
            187, // quick_bar_set_slot
            125, // reassign_blueprint
            138, // remove_cables
            239, // remove_train_station
            13, // reset_assembling_machine
            66, // reset_item
            163, // reverse_select_area
            214, // rotate_entity
            161, // select_area
            120, // select_blueprint_entities
            183, // select_entity_slot
            182, // select_item
            185, // select_mapper_slot
            41, // select_next_valid_gun
            184, // select_tile_slot
            111, // send_spidertron
            207, // set_auto_launch_rocket
            204, // set_autosort_inventory
            212, // set_behavior_mode
            229, // set_car_weapons_control
            86, // set_circuit_condition
            91, // set_circuit_mode_of_operation
            166, // set_controller_logistic_trash_filter_item
            224, // set_deconstruction_item_tile_selection_mode
            223, // set_deconstruction_item_trees_and_rocks_only
            222, // set_entity_color
            171, // set_entity_energy_property
            167, // set_entity_logistic_trash_filter_item
            84, // set_filter
            205, // set_flat_controller_gui
            237, // set_heat_interface_mode
            236, // set_heat_interface_temperature
            168, // set_infinity_container_filter_item
            228, // set_infinity_container_remove_unfiltered_items
            169, // set_infinity_pipe_filter
            220, // set_inserter_max_stack_size
            113, // set_inventory_bar
            248, // set_linked_container_link_i_d
            89, // set_logistic_filter_item
            90, // set_logistic_filter_signal
            244, // set_player_color
            206, // set_recipe_notifications
            230, // set_request_from_buffers
            219, // set_research_finished_stops_game
            87, // set_signal
            234, // set_splitter_priority
            216, // set_train_stopped
            246, // set_trains_limit
            151, // set_vehicle_automatic_targeting_parameters
            78, // setup_assembling_machine
            122, // setup_blueprint
            123, // setup_single_blueprint_record
            80, // smart_pipette
            132, // spawn_item
            81, // stack_split
            72, // stack_transfer
            115, // start_repair
            88, // start_research
            61, // start_walking
            53, // stop_building_by_moving
            211, // switch_connect_to_logistic_network
            208, // switch_constant_combinator_state
            210, // switch_inserter_filter_mode_state
            209, // switch_power_switch_state
            28, // switch_to_rename_stop_gui
            109, // take_equipment
            38, // toggle_deconstruction_item_entity_filter_mode
            39, // toggle_deconstruction_item_tile_filter_mode
            4, // toggle_driving
            37, // toggle_enable_vehicle_logistics_while_moving
            52, // toggle_entity_logistic_requests
            50, // toggle_equipment_movement_bonus
            42, // toggle_map_editor
            51, // toggle_personal_logistic_requests
            49, // toggle_personal_roboport
            24, // toggle_show_entity_info
            197, // translate_string
            48, // undo
            117, // upgrade
            131, // upgrade_opened_blueprint_by_item
            130, // upgrade_opened_blueprint_by_record
            112, // use_artillery_remote
            110, // use_item
            76, // wire_dragging
            94, // write_to_console
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for input_action {
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
impl std::ops::Deref for build_check_type {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 6] = [
            0, // script
            1, // manual
            3, // manual_ghost
            2, // script_ghost
            4, // blueprint_ghost
            5, // ghost_revive
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for build_check_type {
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
impl std::ops::Deref for gui_type {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 20] = [
            0, // none
            1, // entity
            2, // research
            3, // controller
            4, // production
            5, // item
            6, // bonus
            7, // trains
            8, // achievement
            9, // blueprint_library
            10, // equipment
            11, // logistic
            12, // other_player
            14, // permissions
            15, // tutorials
            16, // custom
            17, // server_management
            18, // player_management
            19, // tile
            23, // script_inventory
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for gui_type {
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
impl std::ops::Deref for behavior_result {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 4] = [
            0, // in_progress
            1, // fail
            2, // success
            3, // deleted
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for behavior_result {
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
impl std::ops::Deref for flow_precision_index {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 8] = [
            0, // five_seconds
            1, // one_minute
            2, // ten_minutes
            3, // one_hour
            4, // ten_hours
            5, // fifty_hours
            6, // two_hundred_fifty_hours
            7, // one_thousand_hours
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for flow_precision_index {
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
impl std::ops::Deref for entity_status {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 44] = [
            1, // working
            2, // normal
            38, // no_power
            12, // low_power
            37, // no_fuel
            39, // disabled_by_control_behavior
            41, // opened_by_circuit_network
            40, // closed_by_circuit_network
            42, // disabled_by_script
            44, // marked_for_deconstruction
            3, // not_plugged_in_electric_network
            4, // networks_connected
            5, // networks_disconnected
            6, // charging
            7, // discharging
            8, // fully_charged
            13, // out_of_logistic_network
            15, // no_recipe
            14, // no_ingredients
            19, // no_input_fluid
            16, // no_research_in_progress
            17, // no_minable_resources
            18, // low_input_fluid
            20, // fluid_ingredient_shortage
            22, // full_output
            23, // full_burnt_result_output
            21, // item_ingredient_shortage
            24, // missing_required_fluid
            25, // missing_science_packs
            26, // waiting_for_source_items
            27, // waiting_for_space_in_destination
            28, // preparing_rocket_for_launch
            29, // waiting_to_launch_rocket
            30, // launching_rocket
            31, // no_modules_to_transmit
            32, // recharging_after_power_outage
            33, // waiting_for_target_to_be_built
            34, // waiting_for_train
            35, // no_ammo
            36, // low_temperature
            43, // disabled
            9, // turned_off_during_daytime
            11, // not_connected_to_rail
            10, // cant_divide_segments
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for entity_status {
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
impl std::ops::Deref for rocket_silo_status {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 15] = [
            0, // building_rocket
            1, // create_rocket
            2, // lights_blinking_open
            3, // doors_opening
            4, // doors_opened
            5, // rocket_rising
            6, // arms_advance
            7, // rocket_ready
            8, // launch_starting
            9, // engine_starting
            10, // arms_retract
            11, // rocket_flying
            12, // lights_blinking_close
            13, // doors_closing
            14, // launch_started
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for rocket_silo_status {
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
impl std::ops::Deref for render_mode {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 3] = [
            1, // game
            2, // chart
            3, // chart_zoomed_in
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for render_mode {
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
impl std::ops::Deref for input_method {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 2] = [
            0, // keyboard_and_mouse
            1, // game_controller
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for input_method {
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
impl std::ops::Deref for game_controller_interaction {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 3] = [
            0, // always
            2, // never
            1, // normal
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for game_controller_interaction {
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
impl std::ops::Deref for rich_text_setting {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 3] = [
            17, // enabled
            0, // disabled
            30, // highlight
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for rich_text_setting {
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
impl std::ops::Deref for relative_gui_position {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 4] = [
            0, // top
            1, // bottom
            2, // left
            3, // right
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for relative_gui_position {
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
impl std::ops::Deref for relative_gui_type {
    type Target = str;
    fn deref(&self) -> &'static str {
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
}
impl crate::defines::Defines<&'static str> for relative_gui_type {
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
impl std::ops::Deref for disconnect_reason {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 11] = [
            0, // quit
            1, // dropped
            2, // reconnect
            3, // wrong_input
            4, // desync_limit_reached
            5, // cannot_keep_up
            6, // afk
            7, // kicked
            8, // kicked_and_deleted
            9, // banned
            11, // switching_servers
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for disconnect_reason {
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
    impl std::ops::Deref for achievement {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 17] = [
                0, // achievement
                0, // build-entity-achievement
                0, // combat-robot-count
                0, // construct-with-robots-achievement
                0, // deconstruct-with-robots-achievement
                0, // deliver-by-robots-achievement
                0, // dont-build-entity-achievement
                0, // dont-craft-manually-achievement
                0, // dont-use-entity-in-energy-production-achievement
                0, // finish-the-game-achievement
                0, // group-attack-achievement
                0, // kill-achievement
                0, // player-damaged-achievement
                0, // produce-achievement
                0, // produce-per-hour-achievement
                0, // research-achievement
                0, // train-path-achievement
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for achievement {
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
    impl std::ops::Deref for ambient_sound {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // ambient-sound
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for ambient_sound {
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
    impl std::ops::Deref for ammo_category {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // ammo-category
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for ammo_category {
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
    impl std::ops::Deref for animation {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // animation
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for animation {
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
    impl std::ops::Deref for autoplace_control {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // autoplace-control
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for autoplace_control {
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
    impl std::ops::Deref for custom_input {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // custom-input
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for custom_input {
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
    impl std::ops::Deref for damage_type {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // damage-type
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for damage_type {
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
    impl std::ops::Deref for decorative {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // optimized-decorative
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for decorative {
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
    impl std::ops::Deref for editor_controller {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // editor-controller
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for editor_controller {
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
    impl std::ops::Deref for entity {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 104] = [
                0, // accumulator
                0, // ammo-turret
                0, // arithmetic-combinator
                0, // arrow
                0, // artillery-flare
                0, // artillery-projectile
                0, // artillery-turret
                0, // artillery-wagon
                0, // assembling-machine
                0, // beacon
                0, // beam
                0, // boiler
                0, // burner-generator
                0, // car
                0, // cargo-wagon
                0, // character
                0, // character-corpse
                0, // cliff
                0, // combat-robot
                0, // constant-combinator
                0, // construction-robot
                0, // container
                0, // corpse
                0, // curved-rail
                0, // decider-combinator
                0, // deconstructible-tile-proxy
                0, // electric-energy-interface
                0, // electric-pole
                0, // electric-turret
                0, // entity-ghost
                0, // explosion
                0, // fire
                0, // fish
                0, // flame-thrower-explosion
                0, // fluid-turret
                0, // fluid-wagon
                0, // flying-text
                0, // furnace
                0, // gate
                0, // generator
                0, // heat-interface
                0, // heat-pipe
                0, // highlight-box
                0, // infinity-container
                0, // infinity-pipe
                0, // inserter
                0, // item-entity
                0, // item-request-proxy
                0, // lab
                0, // lamp
                0, // land-mine
                0, // leaf-particle
                0, // linked-belt
                0, // linked-container
                0, // loader
                0, // loader-1x1
                0, // locomotive
                0, // logistic-container
                0, // logistic-robot
                0, // market
                0, // mining-drill
                0, // offshore-pump
                0, // particle
                0, // particle-source
                0, // pipe
                0, // pipe-to-ground
                0, // player-port
                0, // power-switch
                0, // programmable-speaker
                0, // projectile
                0, // pump
                0, // radar
                0, // rail-chain-signal
                0, // rail-remnants
                0, // rail-signal
                0, // reactor
                0, // resource
                0, // roboport
                0, // rocket-silo
                0, // rocket-silo-rocket
                0, // rocket-silo-rocket-shadow
                0, // simple-entity
                0, // simple-entity-with-force
                0, // simple-entity-with-owner
                0, // smoke
                0, // smoke-with-trigger
                0, // solar-panel
                0, // speech-bubble
                0, // spider-leg
                0, // spider-vehicle
                0, // splitter
                0, // sticker
                0, // storage-tank
                0, // straight-rail
                0, // stream
                0, // tile-ghost
                0, // train-stop
                0, // transport-belt
                0, // tree
                0, // turret
                0, // underground-belt
                0, // unit
                0, // unit-spawner
                0, // wall
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for entity {
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
    impl std::ops::Deref for equipment {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 9] = [
                0, // active-defense-equipment
                0, // battery-equipment
                0, // belt-immunity-equipment
                0, // energy-shield-equipment
                0, // generator-equipment
                0, // movement-bonus-equipment
                0, // night-vision-equipment
                0, // roboport-equipment
                0, // solar-panel-equipment
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for equipment {
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
    impl std::ops::Deref for equipment_category {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // equipment-category
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for equipment_category {
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
    impl std::ops::Deref for equipment_grid {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // equipment-grid
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for equipment_grid {
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
    impl std::ops::Deref for fluid {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // fluid
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for fluid {
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
    impl std::ops::Deref for font {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // font
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for font {
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
    impl std::ops::Deref for fuel_category {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // fuel-category
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for fuel_category {
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
    impl std::ops::Deref for god_controller {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // god-controller
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for god_controller {
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
    impl std::ops::Deref for gui_style {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // gui-style
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for gui_style {
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
    impl std::ops::Deref for item {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 21] = [
                0, // ammo
                0, // armor
                0, // blueprint
                0, // blueprint-book
                0, // capsule
                0, // copy-paste-tool
                0, // deconstruction-item
                0, // gun
                0, // item
                0, // item-with-entity-data
                0, // item-with-inventory
                0, // item-with-label
                0, // item-with-tags
                0, // mining-tool
                0, // module
                0, // rail-planner
                0, // repair-tool
                0, // selection-tool
                0, // spidertron-remote
                0, // tool
                0, // upgrade-item
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for item {
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
    impl std::ops::Deref for item_group {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // item-group
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for item_group {
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
    impl std::ops::Deref for item_subgroup {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // item-subgroup
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for item_subgroup {
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
    impl std::ops::Deref for map_gen_presets {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // map-gen-presets
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for map_gen_presets {
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
    impl std::ops::Deref for map_settings {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // map-settings
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for map_settings {
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
    impl std::ops::Deref for module_category {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // module-category
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for module_category {
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
    impl std::ops::Deref for mouse_cursor {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // mouse-cursor
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for mouse_cursor {
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
    impl std::ops::Deref for noise_expression {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // noise-expression
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for noise_expression {
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
    impl std::ops::Deref for noise_layer {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // noise-layer
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for noise_layer {
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
    impl std::ops::Deref for particle {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // optimized-particle
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for particle {
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
    impl std::ops::Deref for recipe {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // recipe
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for recipe {
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
    impl std::ops::Deref for recipe_category {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // recipe-category
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for recipe_category {
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
    impl std::ops::Deref for resource_category {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // resource-category
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for resource_category {
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
    impl std::ops::Deref for shortcut {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // shortcut
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for shortcut {
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
    impl std::ops::Deref for sound {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // sound
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for sound {
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
    impl std::ops::Deref for spectator_controller {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // spectator-controller
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for spectator_controller {
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
    impl std::ops::Deref for sprite {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // sprite
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for sprite {
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
    impl std::ops::Deref for technology {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // technology
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for technology {
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
    impl std::ops::Deref for tile {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // tile
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for tile {
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
    impl std::ops::Deref for tile_effect {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // tile-effect
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for tile_effect {
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
    impl std::ops::Deref for tips_and_tricks_item {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // tips-and-tricks-item
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for tips_and_tricks_item {
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
    impl std::ops::Deref for tips_and_tricks_item_category {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // tips-and-tricks-item-category
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for tips_and_tricks_item_category {
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
    impl std::ops::Deref for trigger_target_type {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // trigger-target-type
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for trigger_target_type {
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
    impl std::ops::Deref for trivial_smoke {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // trivial-smoke
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for trivial_smoke {
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
    impl std::ops::Deref for tutorial {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // tutorial
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for tutorial {
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
    impl std::ops::Deref for utility_constants {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // utility-constants
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for utility_constants {
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
    impl std::ops::Deref for utility_sounds {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // utility-sounds
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for utility_sounds {
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
    impl std::ops::Deref for utility_sprites {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // utility-sprites
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for utility_sprites {
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
    impl std::ops::Deref for virtual_signal {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // virtual-signal
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for virtual_signal {
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
    impl std::ops::Deref for wind_sound {
        type Target = u8;
        fn deref(&self) -> &'static u8 {
            const VALUES: [u8; 1] = [
                0, // wind-sound
            ];
            &VALUES[*self as usize]
        }
    }
    impl crate::defines::Defines<u8> for wind_sound {
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
impl std::ops::Deref for print_sound {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 3] = [
            1, // always
            0, // never
            2, // use_player_settings
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for print_sound {
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
impl std::ops::Deref for print_skip {
    type Target = u8;
    fn deref(&self) -> &'static u8 {
        const VALUES: [u8; 3] = [
            0, // never
            1, // if_redundant
            2, // if_visible
        ];
        &VALUES[*self as usize]
    }
}
impl crate::defines::Defines<u8> for print_skip {
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


