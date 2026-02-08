use {
    smash::{
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue},
        lua2cpp::*,
    },
    smashline::{Main, *},
    crate::lucina::*
};

pub unsafe extern "C" fn empty_status(_agent: &mut L2CAgentBase) -> L2CValue {
    0.into()
}

unsafe extern "C" fn redshell_frame(_weapon: &mut smashline::L2CWeaponCommon) {
    /* 
    let mut life = 100;
    life -= 1;
    if life < 0 {
        //0.into()
    }
    */
}

pub unsafe extern "C" fn stapler_fly_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor,
        SituationKind(*SITUATION_KIND_AIR),
        *WEAPON_KINETIC_TYPE_NORMAL,
        *GROUND_CORRECT_KIND_AIR as u32,
        GroundCliffCheckKind(0),
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

pub unsafe extern "C" fn stapler_fly_init(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_FLAG_SWALLOWED) {
        //snap_to_owner(weapon,Hash40::new("have"),Hash40::new("haver"));
        println!("poo poo!")
    }
    0.into()
}

pub unsafe extern "C" fn stapler_fly_main(weapon: &mut smashline::L2CWeaponCommon) -> L2CValue {
    ModelModule::set_visibility(weapon.module_accessor, true);
    0.into()
}
/*
unsafe extern "C" fn redshell_fly_exec(weapon: &mut smashline::L2CWeaponCommon) -> smashline::L2CValue {
    if eff != *BATTLE_OBJECT_ID_INVALID {
        let rot_x = PostureModule::rot_x(weapon.module_accessor,0);
        EffectModule::set_rot(weapon.module_accessor, eff as u32, &Vector3f::new(rot_x, 0.0, 0.0));
    }

    0.into()
}
*/

pub fn install(agent: &mut smashline::Agent) {
    agent.on_line(Main, redshell_frame);

    //snap_to_owner(weapon,Hash40::new("have"),Hash40::new("haver"));

    agent.status(Pre, STAPLER_STATUS_KIND_SHOOT, stapler_fly_pre);
	agent.status(Init, STAPLER_STATUS_KIND_SHOOT, stapler_fly_init);
	agent.status(Main, STAPLER_STATUS_KIND_SHOOT, stapler_fly_main);
	agent.status(Exec, STAPLER_STATUS_KIND_SHOOT, empty_status);
	agent.status(End, STAPLER_STATUS_KIND_SHOOT, empty_status);

}