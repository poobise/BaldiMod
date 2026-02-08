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
let stock_count = FighterInformation::stock_count();
let item_list: Vec<i32> = vec![1,2,3];


unsafe extern "C" fn game_speciallw(agent: &mut L2CAgentBase) {

}

unsafe extern "C" fn effect_speciallw(agent: &mut L2CAgentBase) {

}

unsafe extern "C" fn sound_speciallw(agent: &mut L2CAgentBase) {

}

unsafe extern "C" fn expression_speciallw(agent: &mut L2CAgentBase) {

}

pub fn install(agent: &mut smashline::Agent) {
	agent.acmd("game_speciallw", game_speciallw, Priority::Default);
	agent.acmd("effect_speciallw", effect_speciallw, Priority::Default);
	agent.acmd("sound_speciallw", sound_speciallw, Priority::Default);
	agent.acmd("expression_speciallw", expression_speciallw, Priority::Default);
}