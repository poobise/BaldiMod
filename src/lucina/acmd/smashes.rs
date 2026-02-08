use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*},
        lib::lua_const::*
    },
    smashline::*,
    smash_script::*
};

/*
if macros::is_excute(fighter) {
    macros::ATTACK(fighter, /* dont edit */
        0, /* id */
        0, /* part (dont edit)*/
        Hash40::new("arml"), /* bone */
        67.0 /* Damage, changed from 12.0 */, 
        361, /* angle */
        80,  0, 30, /* knockback growth, fixed knockback, base knockback */
        3.0 /* Size, changed from 3.0 */, 
        3.2, 0.0, 0.0, /* position */
        Some(50.0), Some(0.0), Some(0.0), /* position 2 */
        1.0, /* hitlag */
        1.0, /* sdi */
        *ATTACK_SETOFF_KIND_ON, /* clang rebound (ignore) */
        *ATTACK_LR_CHECK_F, /* facing restriction (ignore) */
        false, /* set weight */
        0, /* shield damage (max 2)*/
        0.0, /* trip chance */
        0, /* rehit (for multihits, how long til a hitbox can hit again)*/
        false, /* reflectable */
        false, /* absorbable */
        false, /* flinchless */
        false, /* disable hitlag */
        true, /* direct hitbox (true if from the character, false if on weapon)*/
        *COLLISION_SITUATION_MASK_GA, /* ground or air */
        *COLLISION_CATEGORY_MASK_ALL, /* hitbits (ignore) */
        *COLLISION_PART_MASK_ALL, /* collision part (ignore) */
        false, /* friendly fire */
        Hash40::new("collision_attr_normal"), /* effect */
        *ATTACK_SOUND_LEVEL_M, /* sfx volume/level (s, m, or l)*/
        *COLLISION_SOUND_ATTR_PUNCH, /* sfx type */
        *ATTACK_REGION_PUNCH
    ); /* type (mostly for spirits only)*/
}
*/

unsafe extern "C" fn game_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(agent.lua_state_agent, 10.0);
    for _ in 0..5 {
        if macros::is_excute(agent) {
            macros::ATTACK(agent, 0, 0, Hash40::new("sword1"), 15.0, 361, 73, 0, 65, 3.5, 1.0, 0.0, 2.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 1, 0, Hash40::new("armr"), 15.0, 361, 73, 0, 65, 3.0, 0.0, 1.0, 0.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 2, 0, Hash40::new("bust"), 15.0, 361, 73, 0, 65, 2.0, 0.0, 0.0, 0.0, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(agent, 3, 0, Hash40::new("sword1"), 15.0, 361, 73, 0, 65, 3.5, 1.0, 0.0, 7.3, None, None, None, 0.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_attack_height_all(agent.module_accessor, smash::app::AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        }
        wait(agent.lua_state_agent, 2.0);
        if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        }
    }
    frame(agent.lua_state_agent, 20.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_smash_flash"), Hash40::new("top"), -0.0, 8, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE4_ON_arg29(agent, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0, 0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("sword1"), 0, 0, 0, 90, 0, 90, 1, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(agent.lua_state_agent, 13.0);
    for _ in 0..5 {
        if macros::is_excute(agent) {
            macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 8, 0, 0, 0, 180, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
            macros::LAST_EFFECT_SET_RATE(agent, 1.4);
        }
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::AFTER_IMAGE_OFF(agent, 3);
    }
}

unsafe extern "C" fn sound_attacks4(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::STOP_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
    wait(agent.lua_state_agent, 4.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_lucina_attack05"));
        macros::PLAY_SE(agent, Hash40::new("se_lucina_smash_s01"));
    }
}

unsafe extern "C" fn expression_attacks4(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 3.0);
    execute(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        AttackModule::set_attack_reference_joint_id(agent.module_accessor, Hash40::new("haver"), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Z), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_Y), smash::app::AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(agent.lua_state_agent, 8.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 10.0);
    if macros::is_excute(agent) {
        macros::RUMBLE_HIT(agent, Hash40::new("rbkind_slashl"), 0);
    }
}

//side smash charge
unsafe extern "C" fn effect_attacks4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), 5, 0, 0, 0, 0, 0, 1, 15, 0, 4, 0, 0, 0, false);
    }
    wait(agent.lua_state_agent, 5.0);
    macros::EFFECT(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sword1"), 0, -0.0, 8, 0, 0, 0, 1, 2, 2, 4, 0, 0, 0, true);
}

unsafe extern "C" fn sound_attacks4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_smash_start_02"));
    }
}
/* 
unsafe extern "C" fn expression_attacks4charge(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        crate::lucina::acmd::smashes::skyline_smash::app::sv_module_access::physics(*MA_MSC_CMD_PHYSICS_START_CHARGE, 0.9, 0.8, -1, 0.9, 0.8, -1, Hash40::new("invalid"));
        macros::AREA_WIND_2ND_arg10(agent, 0, 0.5, 100, 3, 0.2, 0, 12, 24, 24, 80);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 61.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}
*/
pub fn install(agent: &mut smashline::Agent) {
    agent.acmd("game_attacks4", game_attacks4, Priority::Default);
    agent.acmd("effect_attacks4", effect_attacks4, Priority::Default);
    agent.acmd("sound_attacks4", sound_attacks4, Priority::Default);
    agent.acmd("expression_attacks4", expression_attacks4, Priority::Default);

    agent.acmd("effect_attacks4charge", effect_attacks4charge, Priority::Default);
    agent.acmd("sound_attacks4charge", sound_attacks4charge, Priority::Default);
}