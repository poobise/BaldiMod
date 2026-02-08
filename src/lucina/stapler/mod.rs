use {
    smash::lib::lua_const::*,
    crate::lucina::*
};

mod acmd;
mod status;

pub fn install() {
    println!("hey dont crash");
    unsafe {
        FIGHTER_BALDI_GENERATE_ARTICLE_STAPLER += smashline::clone_weapon("krool",*WEAPON_KIND_KROOL_IRONBALL,"lucina","stapler",false);
    }
    println!("poopy butt");
    let agent = &mut smashline::Agent::new("lucina_stapler");
    acmd::install(agent);
    status::install(agent);

    agent.install();

}