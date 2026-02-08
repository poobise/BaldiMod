mod acmd;
mod frame;

pub static mut FIGHTER_BALDI_GENERATE_ARTICLE_STAPLER: i32 = 0x1;
pub const STAPLER_STATUS_KIND_SHOOT: i32 = 0x0;

pub mod stapler;

pub fn install() {
    let agent = &mut smashline::Agent::new("lucina");
    acmd::install(agent);
    frame::install(agent);

    agent.install();
    
    stapler::install();
    
}
