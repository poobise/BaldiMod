use {
    smash::lib::lua_const::*,
    crate::lucina::*
};

mod acmd;
mod status;

pub fn install() {
    
    unsafe {
        println!("hey dont crash");
        FIGHTER_BALDI_GENERATE_ARTICLE_STAPLER += smashline::clone_weapon("krool",*WEAPON_KIND_KROOL_IRONBALL,"lucina","stapler",false);
    }
    
    let agent = &mut smashline::Agent::new("lucina_stapler");
    acmd::install(agent);
    status::install(agent);

    agent.install();

}