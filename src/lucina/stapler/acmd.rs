use {
    smash::{
        app::{
            lua_bind::*,
        },
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};


unsafe extern "C" fn game_fly(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 5.0, 65, 40, 0, 60, 3.25, 0.0, 0.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 45, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
}


pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_fly", game_fly, Priority::Default);
}