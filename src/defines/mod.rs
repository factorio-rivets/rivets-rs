#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::match_same_arms)]
#![allow(non_camel_case_types)]
#![allow(unreachable_patterns)]
#![allow(missing_docs)]

pub trait Define<const COUNT: usize>: std::ops::Deref {
    fn variants() -> &'static [Self; COUNT]
    where
        Self: Sized;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum inventory {
    #[value = 1]
    fuel,
    #[value = 6]
    burnt_result,
    #[value = 1]
    chest,
    #[value = 2]
    furnace_source,
    #[value = 3]
    furnace_result,
    #[value = 4]
    furnace_modules,
    #[value = 1]
    character_main,
    #[value = 3]
    character_guns,
    #[value = 4]
    character_ammo,
    #[value = 5]
    character_armor,
    #[value = 7]
    character_vehicle,
    #[value = 8]
    character_trash,
    #[value = 2]
    god_main,
    #[value = 1]
    editor_main,
    #[value = 3]
    editor_guns,
    #[value = 4]
    editor_ammo,
    #[value = 5]
    editor_armor,
    #[value = 1]
    roboport_robot,
    #[value = 2]
    roboport_material,
    #[value = 1]
    robot_cargo,
    #[value = 2]
    robot_repair,
    #[value = 2]
    assembling_machine_input,
    #[value = 3]
    assembling_machine_output,
    #[value = 4]
    assembling_machine_modules,
    #[value = 2]
    lab_input,
    #[value = 3]
    lab_modules,
    #[value = 2]
    mining_drill_modules,
    #[value = 1]
    item_main,
    #[value = 5]
    rocket_silo_rocket,
    #[value = 6]
    rocket_silo_result,
    #[value = 2]
    rocket_silo_input,
    #[value = 3]
    rocket_silo_output,
    #[value = 4]
    rocket_silo_modules,
    #[value = 1]
    rocket,
    #[value = 2]
    car_trunk,
    #[value = 3]
    car_ammo,
    #[value = 1]
    cargo_wagon,
    #[value = 1]
    turret_ammo,
    #[value = 1]
    beacon_modules,
    #[value = 1]
    character_corpse,
    #[value = 1]
    artillery_turret_ammo,
    #[value = 1]
    artillery_wagon_ammo,
    #[value = 2]
    spider_trunk,
    #[value = 3]
    spider_ammo,
    #[value = 4]
    spider_trash,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum transport_line {
    #[value = 1]
    left_line,
    #[value = 2]
    right_line,
    #[value = 3]
    left_underground_line,
    #[value = 4]
    right_underground_line,
    #[value = 3]
    secondary_left_line,
    #[value = 4]
    secondary_right_line,
    #[value = 5]
    left_split_line,
    #[value = 6]
    right_split_line,
    #[value = 7]
    secondary_left_split_line,
    #[value = 8]
    secondary_right_split_line,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum direction {
    #[value = 0]
    north,
    #[value = 1]
    northeast,
    #[value = 2]
    east,
    #[value = 3]
    southeast,
    #[value = 4]
    south,
    #[value = 5]
    southwest,
    #[value = 6]
    west,
    #[value = 7]
    northwest,
}

pub mod riding {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum acceleration {
        #[value = 0]
        nothing,
        #[value = 1]
        accelerating,
        #[value = 2]
        braking,
        #[value = 3]
        reversing,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum direction {
        #[value = 0]
        left,
        #[value = 1]
        straight,
        #[value = 2]
        right,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum shooting {
    #[value = 0]
    not_shooting,
    #[value = 1]
    shooting_enemies,
    #[value = 2]
    shooting_selected,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum command {
    #[value = 1]
    attack,
    #[value = 2]
    go_to_location,
    #[value = 3]
    compound,
    #[value = 4]
    group,
    #[value = 5]
    attack_area,
    #[value = 6]
    wander,
    #[value = 8]
    flee,
    #[value = 9]
    stop,
    #[value = 7]
    build_base,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum distraction {
    #[value = 0]
    none,
    #[value = 1]
    by_enemy,
    #[value = 3]
    by_anything,
    #[value = 4]
    by_damage,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum compound_command {
    #[value = 0]
    logical_and,
    #[value = 1]
    logical_or,
    #[value = 2]
    return_last,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum difficulty {
    #[value = 0]
    easy,
    #[value = 1]
    normal,
    #[value = 2]
    hard,
}

pub mod difficulty_settings {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum recipe_difficulty {
        #[value = 0]
        normal,
        #[value = 1]
        expensive,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum technology_difficulty {
        #[value = 0]
        normal,
        #[value = 1]
        expensive,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum events {
    #[value = 183]
    on_player_input_method_changed,
    #[value = 182]
    on_cutscene_finished,
    #[value = 181]
    on_cutscene_started,
    #[value = 180]
    on_entity_color_changed,
    #[value = 179]
    on_gui_leave,
    #[value = 178]
    on_gui_hover,
    #[value = 177]
    on_player_alt_reverse_selected_area,
    #[value = 176]
    on_player_reverse_selected_area,
    #[value = 175]
    on_equipment_removed,
    #[value = 174]
    on_equipment_inserted,
    #[value = 173]
    on_entity_logistic_slot_changed,
    #[value = 172]
    on_spider_command_completed,
    #[value = 171]
    on_player_used_spider_remote,
    #[value = 170]
    on_player_configured_spider_remote,
    #[value = 169]
    on_cutscene_cancelled,
    #[value = 168]
    on_permission_group_added,
    #[value = 167]
    on_permission_group_deleted,
    #[value = 166]
    on_pre_permission_group_deleted,
    #[value = 165]
    on_permission_string_imported,
    #[value = 164]
    on_pre_permission_string_imported,
    #[value = 163]
    on_permission_group_edited,
    #[value = 162]
    on_player_flushed_fluid,
    #[value = 161]
    on_player_clicked_gps_tag,
    #[value = 160]
    on_entity_destroyed,
    #[value = 159]
    on_script_inventory_resized,
    #[value = 158]
    on_pre_script_inventory_resized,
    #[value = 157]
    on_pre_player_toggled_map_editor,
    #[value = 156]
    on_player_set_quick_bar_slot,
    #[value = 155]
    on_script_trigger_effect,
    #[value = 154]
    on_string_translated,
    #[value = 153]
    on_force_cease_fire_changed,
    #[value = 152]
    on_force_friends_changed,
    #[value = 151]
    on_gui_switch_state_changed,
    #[value = 150]
    on_gui_selected_tab_changed,
    #[value = 149]
    on_gui_location_changed,
    #[value = 148]
    on_gui_confirmed,
    #[value = 147]
    on_chart_tag_removed,
    #[value = 146]
    on_chart_tag_modified,
    #[value = 145]
    on_chart_tag_added,
    #[value = 144]
    on_trigger_fired_artillery,
    #[value = 143]
    on_build_base_arrived,
    #[value = 142]
    on_unit_group_finished_gathering,
    #[value = 141]
    on_unit_removed_from_group,
    #[value = 140]
    on_unit_added_to_group,
    #[value = 139]
    on_unit_group_created,
    #[value = 138]
    on_pre_player_removed,
    #[value = 137]
    on_entity_spawned,
    #[value = 136]
    on_post_entity_died,
    #[value = 135]
    on_robot_exploded_cliff,
    #[value = 134]
    on_pre_robot_exploded_cliff,
    #[value = 133]
    on_pre_chunk_deleted,
    #[value = 132]
    on_player_fast_transferred,
    #[value = 131]
    on_player_repaired_entity,
    #[value = 130]
    on_player_toggled_alt_mode,
    #[value = 129]
    on_surface_renamed,
    #[value = 128]
    on_surface_imported,
    #[value = 127]
    on_game_created_from_scenario,
    #[value = 126]
    on_brush_cloned,
    #[value = 125]
    on_area_cloned,
    #[value = 124]
    on_entity_cloned,
    #[value = 123]
    on_player_toggled_map_editor,
    #[value = 122]
    on_cancelled_upgrade,
    #[value = 121]
    on_marked_for_upgrade,
    #[value = 120]
    on_ai_command_completed,
    #[value = 119]
    on_script_path_request_finished,
    #[value = 118]
    on_rocket_launch_ordered,
    #[value = 117]
    on_player_unbanned,
    #[value = 116]
    on_player_kicked,
    #[value = 115]
    on_player_banned,
    #[value = 114]
    on_train_schedule_changed,
    #[value = 113]
    on_chunk_deleted,
    #[value = 112]
    on_pre_surface_cleared,
    #[value = 111]
    on_surface_cleared,
    #[value = 110]
    on_pre_player_left_game,
    #[value = 109]
    on_player_trash_inventory_changed,
    #[value = 108]
    on_forces_merged,
    #[value = 107]
    on_land_mine_armed,
    #[value = 106]
    on_force_reset,
    #[value = 105]
    on_technology_effects_reset,
    #[value = 104]
    on_chunk_charted,
    #[value = 103]
    on_entity_damaged,
    #[value = 102]
    on_player_cancelled_crafting,
    #[value = 101]
    on_pre_player_crafted_item,
    #[value = 100]
    on_player_display_scale_changed,
    #[value = 99]
    on_player_display_resolution_changed,
    #[value = 98]
    on_player_pipette,
    #[value = 97]
    on_pre_ghost_upgraded,
    #[value = 96]
    on_pre_ghost_deconstructed,
    #[value = 95]
    on_character_corpse_expired,
    #[value = 94]
    on_player_cheat_mode_disabled,
    #[value = 93]
    on_player_cheat_mode_enabled,
    #[value = 92]
    on_player_unmuted,
    #[value = 91]
    on_player_muted,
    #[value = 90]
    on_gui_value_changed,
    #[value = 89]
    on_gui_closed,
    #[value = 88]
    on_gui_opened,
    #[value = 87]
    on_mod_item_opened,
    #[value = 86]
    on_player_changed_position,
    #[value = 85]
    on_worker_robot_expired,
    #[value = 84]
    on_combat_robot_expired,
    #[value = 83]
    script_raised_set_tiles,
    #[value = 82]
    script_raised_teleported,
    #[value = 81]
    script_raised_revive,
    #[value = 80]
    script_raised_destroy,
    #[value = 79]
    script_raised_built,
    #[value = 78]
    on_player_demoted,
    #[value = 77]
    on_player_promoted,
    #[value = 76]
    on_player_used_capsule,
    #[value = 75]
    on_player_removed,
    #[value = 74]
    on_console_command,
    #[value = 73]
    on_console_chat,
    #[value = 72]
    on_player_configured_blueprint,
    #[value = 71]
    on_player_deconstructed_area,
    #[value = 70]
    on_player_setup_blueprint,
    #[value = 69]
    on_gui_elem_changed,
    #[value = 68]
    on_train_created,
    #[value = 67]
    on_player_mined_entity,
    #[value = 66]
    on_robot_mined_entity,
    #[value = 65]
    on_pre_surface_deleted,
    #[value = 64]
    on_surface_deleted,
    #[value = 63]
    on_surface_created,
    #[value = 62]
    on_difficulty_settings_changed,
    #[value = 61]
    on_runtime_mod_setting_changed,
    #[value = 60]
    on_gui_selection_state_changed,
    #[value = 59]
    on_entity_renamed,
    #[value = 58]
    on_player_changed_force,
    #[value = 57]
    on_biter_base_built,
    #[value = 56]
    on_player_dropped_item,
    #[value = 55]
    on_market_item_purchased,
    #[value = 54]
    on_selected_entity_changed,
    #[value = 53]
    on_player_changed_surface,
    #[value = 52]
    on_player_alt_selected_area,
    #[value = 51]
    on_player_selected_area,
    #[value = 50]
    on_robot_mined_tile,
    #[value = 49]
    on_robot_built_tile,
    #[value = 48]
    on_player_mined_tile,
    #[value = 47]
    on_player_built_tile,
    #[value = 46]
    on_player_left_game,
    #[value = 45]
    on_player_joined_game,
    #[value = 44]
    on_player_respawned,
    #[value = 43]
    on_player_died,
    #[value = 42]
    on_pre_player_died,
    #[value = 41]
    on_player_removed_equipment,
    #[value = 40]
    on_player_placed_equipment,
    #[value = 39]
    on_player_gun_inventory_changed,
    #[value = 38]
    on_player_ammo_inventory_changed,
    #[value = 37]
    on_player_armor_inventory_changed,
    #[value = 36]
    on_lua_shortcut,
    #[value = 35]
    on_cutscene_waypoint_reached,
    #[value = 34]
    on_player_main_inventory_changed,
    #[value = 33]
    on_entity_settings_pasted,
    #[value = 32]
    on_pre_entity_settings_pasted,
    #[value = 31]
    on_player_cursor_stack_changed,
    #[value = 30]
    on_forces_merging,
    #[value = 29]
    on_force_created,
    #[value = 28]
    on_player_driving_changed_state,
    #[value = 27]
    on_resource_depleted,
    #[value = 26]
    on_player_created,
    #[value = 25]
    on_train_changed_state,
    #[value = 24]
    on_trigger_created_entity,
    #[value = 23]
    on_cancelled_deconstruction,
    #[value = 22]
    on_marked_for_deconstruction,
    #[value = 21]
    on_player_rotated_entity,
    #[value = 20]
    on_research_cancelled,
    #[value = 19]
    on_research_reversed,
    #[value = 18]
    on_research_finished,
    #[value = 17]
    on_research_started,
    #[value = 16]
    on_robot_mined,
    #[value = 15]
    on_robot_pre_mined,
    #[value = 14]
    on_robot_built_entity,
    #[value = 13]
    on_player_crafted_item,
    #[value = 12]
    on_chunk_generated,
    #[value = 11]
    on_pre_player_mined_item,
    #[value = 10]
    on_rocket_launched,
    #[value = 9]
    on_pre_build,
    #[value = 8]
    on_player_mined_item,
    #[value = 7]
    on_sector_scanned,
    #[value = 6]
    on_built_entity,
    #[value = 5]
    on_picked_up_item,
    #[value = 4]
    on_entity_died,
    #[value = 3]
    on_gui_checked_state_changed,
    #[value = 2]
    on_gui_text_changed,
    #[value = 1]
    on_gui_click,
    #[value = 0]
    on_tick,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum controllers {
    #[value = 0]
    ghost,
    #[value = 1]
    character,
    #[value = 2]
    god,
    #[value = 4]
    editor,
    #[value = 6]
    cutscene,
    #[value = 5]
    spectator,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum group_state {
    #[value = 0]
    gathering,
    #[value = 1]
    moving,
    #[value = 2]
    attacking_distraction,
    #[value = 3]
    attacking_target,
    #[value = 4]
    finished,
    #[value = 5]
    pathfinding,
    #[value = 6]
    wander_in_group,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum wire_type {
    #[value = 2]
    red,
    #[value = 3]
    green,
    #[value = 1]
    copper,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum circuit_connector_id {
    #[value = 1]
    accumulator,
    #[value = 1]
    constant_combinator,
    #[value = 1]
    container,
    #[value = 1]
    linked_container,
    #[value = 1]
    programmable_speaker,
    #[value = 1]
    rail_signal,
    #[value = 1]
    rail_chain_signal,
    #[value = 1]
    roboport,
    #[value = 1]
    storage_tank,
    #[value = 1]
    wall,
    #[value = 1]
    electric_pole,
    #[value = 1]
    inserter,
    #[value = 1]
    lamp,
    #[value = 1]
    combinator_input,
    #[value = 2]
    combinator_output,
    #[value = 1]
    offshore_pump,
    #[value = 1]
    pump,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum circuit_condition_index {
    #[value = 1]
    inserter_circuit,
    #[value = 2]
    inserter_logistic,
    #[value = 1]
    lamp,
    #[value = 1]
    arithmetic_combinator,
    #[value = 1]
    decider_combinator,
    #[value = 1]
    constant_combinator,
    #[value = 1]
    offshore_pump,
    #[value = 1]
    pump,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum wire_connection_id {
    #[value = 0]
    electric_pole,
    #[value = 0]
    power_switch_left,
    #[value = 1]
    power_switch_right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum train_state {
    #[value = 0]
    on_the_path,
    #[value = 1]
    path_lost,
    #[value = 2]
    no_schedule,
    #[value = 3]
    no_path,
    #[value = 4]
    arrive_signal,
    #[value = 5]
    wait_signal,
    #[value = 6]
    arrive_station,
    #[value = 7]
    wait_station,
    #[value = 8]
    manual_control_stop,
    #[value = 9]
    manual_control,
    #[value = 10]
    destination_full,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum signal_state {
    #[value = 0]
    open,
    #[value = 1]
    closed,
    #[value = 2]
    reserved,
    #[value = 3]
    reserved_by_circuit_network,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum chain_signal_state {
    #[value = 0]
    none,
    #[value = 1]
    all_open,
    #[value = 2]
    partially_open,
    #[value = 3]
    none_open,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum rail_direction {
    #[value = 0]
    front,
    #[value = 1]
    back,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum rail_connection_direction {
    #[value = 0]
    left,
    #[value = 1]
    straight,
    #[value = 2]
    right,
    #[value = 3]
    none,
}

pub mod control_behavior {
    use super::*;

    pub mod inserter {
        use super::*;

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
        #[factorio_define(kind = u8)]
        pub enum circuit_mode_of_operation {
            #[value = 3]
            none,
            #[value = 0]
            enable_disable,
            #[value = 1]
            set_filters,
            #[value = 2]
            read_hand_contents,
            #[value = 4]
            set_stack_size,
        }

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
        #[factorio_define(kind = u8)]
        pub enum hand_read_mode {
            #[value = 1]
            hold,
            #[value = 0]
            pulse,
        }
    }
    pub mod logistic_container {
        use super::*;

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
        #[factorio_define(kind = u8)]
        pub enum circuit_mode_of_operation {
            #[value = 0]
            send_contents,
            #[value = 1]
            set_requests,
        }
    }
    pub mod lamp {
        use super::*;

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
        #[factorio_define(kind = u8)]
        pub enum circuit_mode_of_operation {
            #[value = 0]
            use_colors,
        }
    }
    pub mod mining_drill {
        use super::*;

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
        #[factorio_define(kind = u8)]
        pub enum resource_read_mode {
            #[value = 0]
            this_miner,
            #[value = 1]
            entire_patch,
        }
    }
    pub mod transport_belt {
        use super::*;

        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
        #[factorio_define(kind = u8)]
        pub enum content_read_mode {
            #[value = 0]
            pulse,
            #[value = 1]
            hold,
        }
    }
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum r#type {
        #[value = 1]
        container,
        #[value = 2]
        generic_on_off,
        #[value = 3]
        inserter,
        #[value = 4]
        lamp,
        #[value = 5]
        logistic_container,
        #[value = 6]
        roboport,
        #[value = 7]
        storage_tank,
        #[value = 8]
        train_stop,
        #[value = 9]
        decider_combinator,
        #[value = 10]
        arithmetic_combinator,
        #[value = 11]
        constant_combinator,
        #[value = 12]
        transport_belt,
        #[value = 13]
        accumulator,
        #[value = 14]
        rail_signal,
        #[value = 18]
        rail_chain_signal,
        #[value = 15]
        wall,
        #[value = 16]
        mining_drill,
        #[value = 17]
        programmable_speaker,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum chunk_generated_status {
    #[value = 0]
    nothing,
    #[value = 10]
    custom_tiles,
    #[value = 20]
    basic_tiles,
    #[value = 30]
    corrected_tiles,
    #[value = 40]
    tiles,
    #[value = 50]
    entities,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum logistic_mode {
    #[value = 0]
    none,
    #[value = 1]
    active_provider,
    #[value = 2]
    storage,
    #[value = 3]
    requester,
    #[value = 4]
    passive_provider,
    #[value = 5]
    buffer,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum logistic_member_index {
    #[value = 0]
    logistic_container,
    #[value = 1]
    vehicle_storage,
    #[value = 0]
    character_requester,
    #[value = 1]
    character_storage,
    #[value = 2]
    character_provider,
    #[value = 0]
    generic_on_off_behavior,
}

pub mod deconstruction_item {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum entity_filter_mode {
        #[value = 0]
        whitelist,
        #[value = 1]
        blacklist,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum tile_filter_mode {
        #[value = 0]
        whitelist,
        #[value = 1]
        blacklist,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum tile_selection_mode {
        #[value = 0]
        normal,
        #[value = 1]
        always,
        #[value = 2]
        never,
        #[value = 3]
        only,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum alert_type {
    #[value = 0]
    entity_destroyed,
    #[value = 1]
    entity_under_attack,
    #[value = 2]
    not_enough_construction_robots,
    #[value = 3]
    no_material_for_construction,
    #[value = 4]
    not_enough_repair_packs,
    #[value = 5]
    turret_fire,
    #[value = 6]
    custom,
    #[value = 7]
    no_storage,
    #[value = 8]
    train_out_of_fuel,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum mouse_button_type {
    #[value = 1]
    none,
    #[value = 2]
    left,
    #[value = 4]
    right,
    #[value = 8]
    middle,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum input_action {
    #[value = 45]
    activate_copy,
    #[value = 46]
    activate_cut,
    #[value = 47]
    activate_paste,
    #[value = 227]
    add_permission_group,
    #[value = 96]
    add_train_station,
    #[value = 195]
    admin_action,
    #[value = 164]
    alt_reverse_select_area,
    #[value = 162]
    alt_select_area,
    #[value = 121]
    alt_select_blueprint_entities,
    #[value = 119]
    alternative_copy,
    #[value = 2]
    begin_mining,
    #[value = 62]
    begin_mining_terrain,
    #[value = 60]
    build,
    #[value = 159]
    build_rail,
    #[value = 152]
    build_terrain,
    #[value = 83]
    cancel_craft,
    #[value = 144]
    cancel_deconstruct,
    #[value = 18]
    cancel_new_blueprint,
    #[value = 160]
    cancel_research,
    #[value = 145]
    cancel_upgrade,
    #[value = 100]
    change_active_character_tab,
    #[value = 98]
    change_active_item_group_for_crafting,
    #[value = 99]
    change_active_item_group_for_filters,
    #[value = 231]
    change_active_quick_bar,
    #[value = 146]
    change_arithmetic_combinator_parameters,
    #[value = 147]
    change_decider_combinator_parameters,
    #[value = 158]
    change_entity_label,
    #[value = 157]
    change_item_description,
    #[value = 156]
    change_item_label,
    #[value = 194]
    change_multiplayer_config,
    #[value = 199]
    change_picking_state,
    #[value = 149]
    change_programmable_speaker_alert_parameters,
    #[value = 150]
    change_programmable_speaker_circuit_parameters,
    #[value = 148]
    change_programmable_speaker_parameters,
    #[value = 63]
    change_riding_state,
    #[value = 77]
    change_shooting_state,
    #[value = 97]
    change_train_stop_station,
    #[value = 153]
    change_train_wait_condition,
    #[value = 154]
    change_train_wait_condition_data,
    #[value = 12]
    clear_cursor,
    #[value = 9]
    connect_rolling_stock,
    #[value = 118]
    copy,
    #[value = 20]
    copy_entity_settings,
    #[value = 124]
    copy_opened_blueprint,
    #[value = 23]
    copy_opened_item,
    #[value = 75]
    craft,
    #[value = 71]
    cursor_split,
    #[value = 70]
    cursor_transfer,
    #[value = 155]
    custom_input,
    #[value = 33]
    cycle_blueprint_book_backwards,
    #[value = 32]
    cycle_blueprint_book_forwards,
    #[value = 116]
    deconstruct,
    #[value = 43]
    delete_blueprint_library,
    #[value = 129]
    delete_blueprint_record,
    #[value = 225]
    delete_custom_tag,
    #[value = 226]
    delete_permission_group,
    #[value = 67]
    destroy_item,
    #[value = 22]
    destroy_opened_item,
    #[value = 10]
    disconnect_rolling_stock,
    #[value = 180]
    drag_train_schedule,
    #[value = 181]
    drag_train_wait_condition,
    #[value = 128]
    drop_blueprint_record,
    #[value = 59]
    drop_item,
    #[value = 137]
    edit_blueprint_tool_preview,
    #[value = 172]
    edit_custom_tag,
    #[value = 173]
    edit_permission_group,
    #[value = 139]
    export_blueprint,
    #[value = 215]
    fast_entity_split,
    #[value = 213]
    fast_entity_transfer,
    #[value = 54]
    flush_opened_entity_fluid,
    #[value = 198]
    flush_opened_entity_specific_fluid,
    #[value = 240]
    go_to_train_station,
    #[value = 127]
    grab_blueprint_record,
    #[value = 102]
    gui_checked_state_changed,
    #[value = 92]
    gui_click,
    #[value = 93]
    gui_confirmed,
    #[value = 178]
    gui_elem_changed,
    #[value = 249]
    gui_hover,
    #[value = 250]
    gui_leave,
    #[value = 107]
    gui_location_changed,
    #[value = 104]
    gui_selected_tab_changed,
    #[value = 103]
    gui_selection_state_changed,
    #[value = 106]
    gui_switch_state_changed,
    #[value = 101]
    gui_text_changed,
    #[value = 105]
    gui_value_changed,
    #[value = 140]
    import_blueprint,
    #[value = 174]
    import_blueprint_string,
    #[value = 141]
    import_blueprints_filtered,
    #[value = 175]
    import_permissions_string,
    #[value = 82]
    inventory_split,
    #[value = 73]
    inventory_transfer,
    #[value = 15]
    launch_rocket,
    #[value = 196]
    lua_shortcut,
    #[value = 191]
    map_editor_action,
    #[value = 95]
    market_offer,
    #[value = 170]
    mod_settings_changed,
    #[value = 31]
    open_achievements_gui,
    #[value = 57]
    open_blueprint_library_gui,
    #[value = 126]
    open_blueprint_record,
    #[value = 29]
    open_bonus_gui,
    #[value = 7]
    open_character_gui,
    #[value = 8]
    open_current_vehicle_gui,
    #[value = 69]
    open_equipment,
    #[value = 5]
    open_gui,
    #[value = 64]
    open_item,
    #[value = 40]
    open_logistic_gui,
    #[value = 68]
    open_mod_item,
    #[value = 65]
    open_parent_of_opened_item,
    #[value = 16]
    open_production_gui,
    #[value = 14]
    open_technology_gui,
    #[value = 56]
    open_tips_and_tricks_gui,
    #[value = 221]
    open_train_gui,
    #[value = 238]
    open_train_station_gui,
    #[value = 30]
    open_trains_gui,
    #[value = 21]
    paste_entity_settings,
    #[value = 108]
    place_equipment,
    #[value = 188]
    quick_bar_pick_slot,
    #[value = 189]
    quick_bar_set_selected_page,
    #[value = 187]
    quick_bar_set_slot,
    #[value = 125]
    reassign_blueprint,
    #[value = 138]
    remove_cables,
    #[value = 239]
    remove_train_station,
    #[value = 13]
    reset_assembling_machine,
    #[value = 66]
    reset_item,
    #[value = 163]
    reverse_select_area,
    #[value = 214]
    rotate_entity,
    #[value = 161]
    select_area,
    #[value = 120]
    select_blueprint_entities,
    #[value = 183]
    select_entity_slot,
    #[value = 182]
    select_item,
    #[value = 185]
    select_mapper_slot,
    #[value = 41]
    select_next_valid_gun,
    #[value = 184]
    select_tile_slot,
    #[value = 111]
    send_spidertron,
    #[value = 207]
    set_auto_launch_rocket,
    #[value = 204]
    set_autosort_inventory,
    #[value = 212]
    set_behavior_mode,
    #[value = 229]
    set_car_weapons_control,
    #[value = 86]
    set_circuit_condition,
    #[value = 91]
    set_circuit_mode_of_operation,
    #[value = 166]
    set_controller_logistic_trash_filter_item,
    #[value = 224]
    set_deconstruction_item_tile_selection_mode,
    #[value = 223]
    set_deconstruction_item_trees_and_rocks_only,
    #[value = 222]
    set_entity_color,
    #[value = 171]
    set_entity_energy_property,
    #[value = 167]
    set_entity_logistic_trash_filter_item,
    #[value = 84]
    set_filter,
    #[value = 205]
    set_flat_controller_gui,
    #[value = 237]
    set_heat_interface_mode,
    #[value = 236]
    set_heat_interface_temperature,
    #[value = 168]
    set_infinity_container_filter_item,
    #[value = 228]
    set_infinity_container_remove_unfiltered_items,
    #[value = 169]
    set_infinity_pipe_filter,
    #[value = 220]
    set_inserter_max_stack_size,
    #[value = 113]
    set_inventory_bar,
    #[value = 248]
    set_linked_container_link_i_d,
    #[value = 89]
    set_logistic_filter_item,
    #[value = 90]
    set_logistic_filter_signal,
    #[value = 244]
    set_player_color,
    #[value = 206]
    set_recipe_notifications,
    #[value = 230]
    set_request_from_buffers,
    #[value = 219]
    set_research_finished_stops_game,
    #[value = 87]
    set_signal,
    #[value = 234]
    set_splitter_priority,
    #[value = 216]
    set_train_stopped,
    #[value = 246]
    set_trains_limit,
    #[value = 151]
    set_vehicle_automatic_targeting_parameters,
    #[value = 78]
    setup_assembling_machine,
    #[value = 122]
    setup_blueprint,
    #[value = 123]
    setup_single_blueprint_record,
    #[value = 80]
    smart_pipette,
    #[value = 132]
    spawn_item,
    #[value = 81]
    stack_split,
    #[value = 72]
    stack_transfer,
    #[value = 115]
    start_repair,
    #[value = 88]
    start_research,
    #[value = 61]
    start_walking,
    #[value = 53]
    stop_building_by_moving,
    #[value = 211]
    switch_connect_to_logistic_network,
    #[value = 208]
    switch_constant_combinator_state,
    #[value = 210]
    switch_inserter_filter_mode_state,
    #[value = 209]
    switch_power_switch_state,
    #[value = 28]
    switch_to_rename_stop_gui,
    #[value = 109]
    take_equipment,
    #[value = 38]
    toggle_deconstruction_item_entity_filter_mode,
    #[value = 39]
    toggle_deconstruction_item_tile_filter_mode,
    #[value = 4]
    toggle_driving,
    #[value = 37]
    toggle_enable_vehicle_logistics_while_moving,
    #[value = 52]
    toggle_entity_logistic_requests,
    #[value = 50]
    toggle_equipment_movement_bonus,
    #[value = 42]
    toggle_map_editor,
    #[value = 51]
    toggle_personal_logistic_requests,
    #[value = 49]
    toggle_personal_roboport,
    #[value = 24]
    toggle_show_entity_info,
    #[value = 197]
    translate_string,
    #[value = 48]
    undo,
    #[value = 117]
    upgrade,
    #[value = 131]
    upgrade_opened_blueprint_by_item,
    #[value = 130]
    upgrade_opened_blueprint_by_record,
    #[value = 112]
    use_artillery_remote,
    #[value = 110]
    use_item,
    #[value = 76]
    wire_dragging,
    #[value = 94]
    write_to_console,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum build_check_type {
    #[value = 0]
    script,
    #[value = 1]
    manual,
    #[value = 3]
    manual_ghost,
    #[value = 2]
    script_ghost,
    #[value = 4]
    blueprint_ghost,
    #[value = 5]
    ghost_revive,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum gui_type {
    #[value = 0]
    none,
    #[value = 1]
    entity,
    #[value = 2]
    research,
    #[value = 3]
    controller,
    #[value = 4]
    production,
    #[value = 5]
    item,
    #[value = 6]
    bonus,
    #[value = 7]
    trains,
    #[value = 8]
    achievement,
    #[value = 9]
    blueprint_library,
    #[value = 10]
    equipment,
    #[value = 11]
    logistic,
    #[value = 12]
    other_player,
    #[value = 14]
    permissions,
    #[value = 15]
    tutorials,
    #[value = 16]
    custom,
    #[value = 17]
    server_management,
    #[value = 18]
    player_management,
    #[value = 19]
    tile,
    #[value = 23]
    script_inventory,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum behavior_result {
    #[value = 0]
    in_progress,
    #[value = 1]
    fail,
    #[value = 2]
    success,
    #[value = 3]
    deleted,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum flow_precision_index {
    #[value = 0]
    five_seconds,
    #[value = 1]
    one_minute,
    #[value = 2]
    ten_minutes,
    #[value = 3]
    one_hour,
    #[value = 4]
    ten_hours,
    #[value = 5]
    fifty_hours,
    #[value = 6]
    two_hundred_fifty_hours,
    #[value = 7]
    one_thousand_hours,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum entity_status {
    #[value = 1]
    working,
    #[value = 2]
    normal,
    #[value = 38]
    no_power,
    #[value = 12]
    low_power,
    #[value = 37]
    no_fuel,
    #[value = 39]
    disabled_by_control_behavior,
    #[value = 41]
    opened_by_circuit_network,
    #[value = 40]
    closed_by_circuit_network,
    #[value = 42]
    disabled_by_script,
    #[value = 44]
    marked_for_deconstruction,
    #[value = 3]
    not_plugged_in_electric_network,
    #[value = 4]
    networks_connected,
    #[value = 5]
    networks_disconnected,
    #[value = 6]
    charging,
    #[value = 7]
    discharging,
    #[value = 8]
    fully_charged,
    #[value = 13]
    out_of_logistic_network,
    #[value = 15]
    no_recipe,
    #[value = 14]
    no_ingredients,
    #[value = 19]
    no_input_fluid,
    #[value = 16]
    no_research_in_progress,
    #[value = 17]
    no_minable_resources,
    #[value = 18]
    low_input_fluid,
    #[value = 20]
    fluid_ingredient_shortage,
    #[value = 22]
    full_output,
    #[value = 23]
    full_burnt_result_output,
    #[value = 21]
    item_ingredient_shortage,
    #[value = 24]
    missing_required_fluid,
    #[value = 25]
    missing_science_packs,
    #[value = 26]
    waiting_for_source_items,
    #[value = 27]
    waiting_for_space_in_destination,
    #[value = 28]
    preparing_rocket_for_launch,
    #[value = 29]
    waiting_to_launch_rocket,
    #[value = 30]
    launching_rocket,
    #[value = 31]
    no_modules_to_transmit,
    #[value = 32]
    recharging_after_power_outage,
    #[value = 33]
    waiting_for_target_to_be_built,
    #[value = 34]
    waiting_for_train,
    #[value = 35]
    no_ammo,
    #[value = 36]
    low_temperature,
    #[value = 43]
    disabled,
    #[value = 9]
    turned_off_during_daytime,
    #[value = 11]
    not_connected_to_rail,
    #[value = 10]
    cant_divide_segments,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum rocket_silo_status {
    #[value = 0]
    building_rocket,
    #[value = 1]
    create_rocket,
    #[value = 2]
    lights_blinking_open,
    #[value = 3]
    doors_opening,
    #[value = 4]
    doors_opened,
    #[value = 5]
    rocket_rising,
    #[value = 6]
    arms_advance,
    #[value = 7]
    rocket_ready,
    #[value = 8]
    launch_starting,
    #[value = 9]
    engine_starting,
    #[value = 10]
    arms_retract,
    #[value = 11]
    rocket_flying,
    #[value = 12]
    lights_blinking_close,
    #[value = 13]
    doors_closing,
    #[value = 14]
    launch_started,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum render_mode {
    #[value = 1]
    game,
    #[value = 2]
    chart,
    #[value = 3]
    chart_zoomed_in,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum input_method {
    #[value = 0]
    keyboard_and_mouse,
    #[value = 1]
    game_controller,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum game_controller_interaction {
    #[value = 0]
    always,
    #[value = 2]
    never,
    #[value = 1]
    normal,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum rich_text_setting {
    #[value = 17]
    enabled,
    #[value = 0]
    disabled,
    #[value = 30]
    highlight,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum relative_gui_position {
    #[value = 0]
    top,
    #[value = 1]
    bottom,
    #[value = 2]
    left,
    #[value = 3]
    right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = str)]
pub enum relative_gui_type {
    #[value = "blueprint-library-gui"]
    blueprint_library_gui,
    #[value = "production-gui"]
    production_gui,
    #[value = "train-stop-gui"]
    train_stop_gui,
    #[value = "bonus-gui"]
    bonus_gui,
    #[value = "tile-variations-gui"]
    tile_variations_gui,
    #[value = "trains-gui"]
    trains_gui,
    #[value = "achievement-gui"]
    achievement_gui,
    #[value = "furnace-gui"]
    furnace_gui,
    #[value = "permissions-gui"]
    permissions_gui,
    #[value = "logistic-gui"]
    logistic_gui,
    #[value = "heat-interface-gui"]
    heat_interface_gui,
    #[value = "controller-gui"]
    controller_gui,
    #[value = "script-inventory-gui"]
    script_inventory_gui,
    #[value = "server-config-gui"]
    server_config_gui,
    #[value = "armor-gui"]
    armor_gui,
    #[value = "admin-gui"]
    admin_gui,
    #[value = "burner-equipment-gui"]
    burner_equipment_gui,
    #[value = "other-player-gui"]
    other_player_gui,
    #[value = "rename-stop-gui"]
    rename_stop_gui,
    #[value = "entity-with-energy-source-gui"]
    entity_with_energy_source_gui,
    #[value = "loader-gui"]
    loader_gui,
    #[value = "blueprint-book-gui"]
    blueprint_book_gui,
    #[value = "item-with-inventory-gui"]
    item_with_inventory_gui,
    #[value = "decider-combinator-gui"]
    decider_combinator_gui,
    #[value = "programmable-speaker-gui"]
    programmable_speaker_gui,
    #[value = "equipment-grid-gui"]
    equipment_grid_gui,
    #[value = "spider-vehicle-gui"]
    spider_vehicle_gui,
    #[value = "deconstruction-item-gui"]
    deconstruction_item_gui,
    #[value = "mining-drill-gui"]
    mining_drill_gui,
    #[value = "upgrade-item-gui"]
    upgrade_item_gui,
    #[value = "transport-belt-gui"]
    transport_belt_gui,
    #[value = "blueprint-setup-gui"]
    blueprint_setup_gui,
    #[value = "inserter-gui"]
    inserter_gui,
    #[value = "assembling-machine-gui"]
    assembling_machine_gui,
    #[value = "splitter-gui"]
    splitter_gui,
    #[value = "lamp-gui"]
    lamp_gui,
    #[value = "infinity-pipe-gui"]
    infinity_pipe_gui,
    #[value = "pipe-gui"]
    pipe_gui,
    #[value = "standalone-character-gui"]
    standalone_character_gui,
    #[value = "lab-gui"]
    lab_gui,
    #[value = "generic-on-off-entity-gui"]
    generic_on_off_entity_gui,
    #[value = "wall-gui"]
    wall_gui,
    #[value = "storage-tank-gui"]
    storage_tank_gui,
    #[value = "power-switch-gui"]
    power_switch_gui,
    #[value = "rail-signal-gui"]
    rail_signal_gui,
    #[value = "rail-chain-signal-gui"]
    rail_chain_signal_gui,
    #[value = "beacon-gui"]
    beacon_gui,
    #[value = "accumulator-gui"]
    accumulator_gui,
    #[value = "reactor-gui"]
    reactor_gui,
    #[value = "car-gui"]
    car_gui,
    #[value = "container-gui"]
    container_gui,
    #[value = "linked-container-gui"]
    linked_container_gui,
    #[value = "assembling-machine-select-recipe-gui"]
    assembling_machine_select_recipe_gui,
    #[value = "electric-network-gui"]
    electric_network_gui,
    #[value = "train-gui"]
    train_gui,
    #[value = "rocket-silo-gui"]
    rocket_silo_gui,
    #[value = "roboport-gui"]
    roboport_gui,
    #[value = "arithmetic-combinator-gui"]
    arithmetic_combinator_gui,
    #[value = "constant-combinator-gui"]
    constant_combinator_gui,
    #[value = "electric-energy-interface-gui"]
    electric_energy_interface_gui,
    #[value = "market-gui"]
    market_gui,
    #[value = "additional-entity-info-gui"]
    additional_entity_info_gui,
    #[value = "resource-entity-gui"]
    resource_entity_gui,
    #[value = "entity-variations-gui"]
    entity_variations_gui,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum disconnect_reason {
    #[value = 0]
    quit,
    #[value = 1]
    dropped,
    #[value = 2]
    reconnect,
    #[value = 3]
    wrong_input,
    #[value = 4]
    desync_limit_reached,
    #[value = 5]
    cannot_keep_up,
    #[value = 6]
    afk,
    #[value = 7]
    kicked,
    #[value = 8]
    kicked_and_deleted,
    #[value = 9]
    banned,
    #[value = 11]
    switching_servers,
}

pub mod prototypes {
    use super::*;

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum achievement {
        #[value = 0]
        achievement,
        #[value = 0]
        build_entity_achievement,
        #[value = 0]
        combat_robot_count,
        #[value = 0]
        construct_with_robots_achievement,
        #[value = 0]
        deconstruct_with_robots_achievement,
        #[value = 0]
        deliver_by_robots_achievement,
        #[value = 0]
        dont_build_entity_achievement,
        #[value = 0]
        dont_craft_manually_achievement,
        #[value = 0]
        dont_use_entity_in_energy_production_achievement,
        #[value = 0]
        finish_the_game_achievement,
        #[value = 0]
        group_attack_achievement,
        #[value = 0]
        kill_achievement,
        #[value = 0]
        player_damaged_achievement,
        #[value = 0]
        produce_achievement,
        #[value = 0]
        produce_per_hour_achievement,
        #[value = 0]
        research_achievement,
        #[value = 0]
        train_path_achievement,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum ambient_sound {
        #[value = 0]
        ambient_sound,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum ammo_category {
        #[value = 0]
        ammo_category,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum animation {
        #[value = 0]
        animation,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum autoplace_control {
        #[value = 0]
        autoplace_control,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum custom_input {
        #[value = 0]
        custom_input,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum damage_type {
        #[value = 0]
        damage_type,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum decorative {
        #[value = 0]
        optimized_decorative,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum editor_controller {
        #[value = 0]
        editor_controller,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum entity {
        #[value = 0]
        accumulator,
        #[value = 0]
        ammo_turret,
        #[value = 0]
        arithmetic_combinator,
        #[value = 0]
        arrow,
        #[value = 0]
        artillery_flare,
        #[value = 0]
        artillery_projectile,
        #[value = 0]
        artillery_turret,
        #[value = 0]
        artillery_wagon,
        #[value = 0]
        assembling_machine,
        #[value = 0]
        beacon,
        #[value = 0]
        beam,
        #[value = 0]
        boiler,
        #[value = 0]
        burner_generator,
        #[value = 0]
        car,
        #[value = 0]
        cargo_wagon,
        #[value = 0]
        character,
        #[value = 0]
        character_corpse,
        #[value = 0]
        cliff,
        #[value = 0]
        combat_robot,
        #[value = 0]
        constant_combinator,
        #[value = 0]
        construction_robot,
        #[value = 0]
        container,
        #[value = 0]
        corpse,
        #[value = 0]
        curved_rail,
        #[value = 0]
        decider_combinator,
        #[value = 0]
        deconstructible_tile_proxy,
        #[value = 0]
        electric_energy_interface,
        #[value = 0]
        electric_pole,
        #[value = 0]
        electric_turret,
        #[value = 0]
        entity_ghost,
        #[value = 0]
        explosion,
        #[value = 0]
        fire,
        #[value = 0]
        fish,
        #[value = 0]
        flame_thrower_explosion,
        #[value = 0]
        fluid_turret,
        #[value = 0]
        fluid_wagon,
        #[value = 0]
        flying_text,
        #[value = 0]
        furnace,
        #[value = 0]
        gate,
        #[value = 0]
        generator,
        #[value = 0]
        heat_interface,
        #[value = 0]
        heat_pipe,
        #[value = 0]
        highlight_box,
        #[value = 0]
        infinity_container,
        #[value = 0]
        infinity_pipe,
        #[value = 0]
        inserter,
        #[value = 0]
        item_entity,
        #[value = 0]
        item_request_proxy,
        #[value = 0]
        lab,
        #[value = 0]
        lamp,
        #[value = 0]
        land_mine,
        #[value = 0]
        leaf_particle,
        #[value = 0]
        linked_belt,
        #[value = 0]
        linked_container,
        #[value = 0]
        loader,
        #[value = 0]
        loader_1x1,
        #[value = 0]
        locomotive,
        #[value = 0]
        logistic_container,
        #[value = 0]
        logistic_robot,
        #[value = 0]
        market,
        #[value = 0]
        mining_drill,
        #[value = 0]
        offshore_pump,
        #[value = 0]
        particle,
        #[value = 0]
        particle_source,
        #[value = 0]
        pipe,
        #[value = 0]
        pipe_to_ground,
        #[value = 0]
        player_port,
        #[value = 0]
        power_switch,
        #[value = 0]
        programmable_speaker,
        #[value = 0]
        projectile,
        #[value = 0]
        pump,
        #[value = 0]
        radar,
        #[value = 0]
        rail_chain_signal,
        #[value = 0]
        rail_remnants,
        #[value = 0]
        rail_signal,
        #[value = 0]
        reactor,
        #[value = 0]
        resource,
        #[value = 0]
        roboport,
        #[value = 0]
        rocket_silo,
        #[value = 0]
        rocket_silo_rocket,
        #[value = 0]
        rocket_silo_rocket_shadow,
        #[value = 0]
        simple_entity,
        #[value = 0]
        simple_entity_with_force,
        #[value = 0]
        simple_entity_with_owner,
        #[value = 0]
        smoke,
        #[value = 0]
        smoke_with_trigger,
        #[value = 0]
        solar_panel,
        #[value = 0]
        speech_bubble,
        #[value = 0]
        spider_leg,
        #[value = 0]
        spider_vehicle,
        #[value = 0]
        splitter,
        #[value = 0]
        sticker,
        #[value = 0]
        storage_tank,
        #[value = 0]
        straight_rail,
        #[value = 0]
        stream,
        #[value = 0]
        tile_ghost,
        #[value = 0]
        train_stop,
        #[value = 0]
        transport_belt,
        #[value = 0]
        tree,
        #[value = 0]
        turret,
        #[value = 0]
        underground_belt,
        #[value = 0]
        unit,
        #[value = 0]
        unit_spawner,
        #[value = 0]
        wall,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum equipment {
        #[value = 0]
        active_defense_equipment,
        #[value = 0]
        battery_equipment,
        #[value = 0]
        belt_immunity_equipment,
        #[value = 0]
        energy_shield_equipment,
        #[value = 0]
        generator_equipment,
        #[value = 0]
        movement_bonus_equipment,
        #[value = 0]
        night_vision_equipment,
        #[value = 0]
        roboport_equipment,
        #[value = 0]
        solar_panel_equipment,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum equipment_category {
        #[value = 0]
        equipment_category,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum equipment_grid {
        #[value = 0]
        equipment_grid,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum fluid {
        #[value = 0]
        fluid,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum font {
        #[value = 0]
        font,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum fuel_category {
        #[value = 0]
        fuel_category,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum god_controller {
        #[value = 0]
        god_controller,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum gui_style {
        #[value = 0]
        gui_style,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum item {
        #[value = 0]
        ammo,
        #[value = 0]
        armor,
        #[value = 0]
        blueprint,
        #[value = 0]
        blueprint_book,
        #[value = 0]
        capsule,
        #[value = 0]
        copy_paste_tool,
        #[value = 0]
        deconstruction_item,
        #[value = 0]
        gun,
        #[value = 0]
        item,
        #[value = 0]
        item_with_entity_data,
        #[value = 0]
        item_with_inventory,
        #[value = 0]
        item_with_label,
        #[value = 0]
        item_with_tags,
        #[value = 0]
        mining_tool,
        #[value = 0]
        module,
        #[value = 0]
        rail_planner,
        #[value = 0]
        repair_tool,
        #[value = 0]
        selection_tool,
        #[value = 0]
        spidertron_remote,
        #[value = 0]
        tool,
        #[value = 0]
        upgrade_item,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum item_group {
        #[value = 0]
        item_group,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum item_subgroup {
        #[value = 0]
        item_subgroup,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum map_gen_presets {
        #[value = 0]
        map_gen_presets,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum map_settings {
        #[value = 0]
        map_settings,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum module_category {
        #[value = 0]
        module_category,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum mouse_cursor {
        #[value = 0]
        mouse_cursor,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum noise_expression {
        #[value = 0]
        noise_expression,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum noise_layer {
        #[value = 0]
        noise_layer,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum particle {
        #[value = 0]
        optimized_particle,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum recipe {
        #[value = 0]
        recipe,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum recipe_category {
        #[value = 0]
        recipe_category,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum resource_category {
        #[value = 0]
        resource_category,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum shortcut {
        #[value = 0]
        shortcut,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum sound {
        #[value = 0]
        sound,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum spectator_controller {
        #[value = 0]
        spectator_controller,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum sprite {
        #[value = 0]
        sprite,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum technology {
        #[value = 0]
        technology,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum tile {
        #[value = 0]
        tile,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum tile_effect {
        #[value = 0]
        tile_effect,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum tips_and_tricks_item {
        #[value = 0]
        tips_and_tricks_item,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum tips_and_tricks_item_category {
        #[value = 0]
        tips_and_tricks_item_category,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum trigger_target_type {
        #[value = 0]
        trigger_target_type,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum trivial_smoke {
        #[value = 0]
        trivial_smoke,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum tutorial {
        #[value = 0]
        tutorial,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum utility_constants {
        #[value = 0]
        utility_constants,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum utility_sounds {
        #[value = 0]
        utility_sounds,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum utility_sprites {
        #[value = 0]
        utility_sprites,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum virtual_signal {
        #[value = 0]
        virtual_signal,
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
    #[factorio_define(kind = u8)]
    pub enum wind_sound {
        #[value = 0]
        wind_sound,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum print_sound {
    #[value = 1]
    always,
    #[value = 0]
    never,
    #[value = 2]
    use_player_settings,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, rivets_macros::FactorioDefine)]
#[factorio_define(kind = u8)]
pub enum print_skip {
    #[value = 0]
    never,
    #[value = 1]
    if_redundant,
    #[value = 2]
    if_visible,
}
