use {
    smash::{
        lua2cpp::*,
        app::lua_bind::*,
        lib::lua_const::*
    },
    crate::custom::global_fighter_frame
};

// What used to be known as a "Once-Per-Fighter-Frame". On-Line functions can be set to run on any status condition.
unsafe extern "C" fn marth_on_main(fighter: &mut L2CFighterCommon) {
    unsafe {
        // Calls the global fighter frame
        global_fighter_frame(fighter);
        // Gives marth unlimited jumps.
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
    }
}

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(smashline::Main, marth_on_main);
}