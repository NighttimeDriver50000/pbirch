use crate::battle::{Current, DamageContext};
use crate::formats::{AbsoluteTarget, RelativeTarget};
use crate::hooks;
use vdex::moves::{self, Effect, Move};

pub fn get_targets(user: &Current, mov: &'static Move) -> Vec<RelativeTarget> {
    match mov.target {
        moves::Target::SpecificMove
            | moves::Target::SelectedPokemonReuseStolen
            | moves::Target::UserOrAlly
            | moves::Target::RandomOpponent
            | moves::Target::SelectedPokemon
            => vec![user.borrow().hooks.targeting.call(user, mov.target)],
        moves::Target::Ally
            => vec![RelativeTarget::Ally],
        moves::Target::UsersField
            => vec![RelativeTarget::User, RelativeTarget::Ally],
        moves::Target::OpponentsField | moves::Target::AllOpponents
            => vec![RelativeTarget::OpponentForward, RelativeTarget::OpponentAcross],
        moves::Target::User
            => vec![RelativeTarget::User],
        moves::Target::AllOtherPokemon
            => vec![RelativeTarget::Ally, RelativeTarget::OpponentForward,
                RelativeTarget::OpponentAcross],
        moves::Target::EntireField
            => vec![RelativeTarget::User, RelativeTarget::Ally,
                RelativeTarget::OpponentForward, RelativeTarget::OpponentAcross],
    }
}

pub fn execute_move<F, R>(
    user: &Current, slot: u8, mov: &'static Move,
    mut resolve_targets: F, rng: &mut R
) -> bool where F: FnMut(&Vec<AbsoluteTarget>) -> Vec<Current>, R: rand::Rng {
    let mut abs_targets = Vec::new();
    for rel_target in get_targets(user, mov) {
        abs_targets.push(rel_target.absolute(&user.borrow().position));
    }
    let targets = resolve_targets(&abs_targets);
    let target_count = targets.len().min(std::u8::MAX as usize) as u8;
    if target_count < 1 {
        return false;
    }
    if slot < 4 {
        let perm = &user.borrow().perm;
        if perm.borrow().pp[slot as usize] > 0 {
            perm.borrow_mut().pp[slot as usize] -= 1;
        } else {
            return false;
        }
    }
    let create_context = |target: &Current, rng: &mut R| -> DamageContext {
        DamageContext::new_basic(user, target, slot, mov, target_count, rng)
    };
    match mov.effect {
        Effect::RegularDamage
            | Effect::SleepTarget
            | Effect::ChancePoisonTarget
            | Effect::HealUserHalfInflicted
            | Effect::ChanceBurnTarget
            | Effect::ChanceFreezeTarget
            | Effect::ChanceParalyzeTarget
            | Effect::NeverMisses
            | Effect::LowerTargetAttack
            | Effect::LowerTargetDefense
            | Effect::LowerTargetSpeed
            | Effect::LowerTargetAccuracy
            | Effect::LowerTargetEvasion
            | Effect::ChanceFlinchTarget
            | Effect::HealUserByHalfMaxHP
            | Effect::PayDay
            | Effect::IncreasedCritical
            | Effect::QuarterRecoil
            | Effect::ConfuseTarget
            | Effect::LowerTargetAttack2
            | Effect::LowerTargetDefense2
            | Effect::LowerTargetSpeed2
            | Effect::LowerTargetSpecialDefense2
            | Effect::PoisonTarget
            | Effect::ParalyzeTarget
            | Effect::ChanceLowerTargetAttack
            | Effect::ChanceLowerTargetDefense
            | Effect::ChanceLowerTargetSpeed
            | Effect::ChanceLowerTargetSpecialAttack
            | Effect::ChanceLowerTargetSpecialDefense
            | Effect::ChanceLowerTargetAccuracy
            | Effect::ChanceConfuseTarget
            | Effect::VitalThrow
            | Effect::Fast
        => {
            for target in targets {
                create_context(&target, rng).execute_basic_move(rng);
            }
        },
        Effect::FaintUser => {
            // TODO: faint user
            for target in targets {
                let key = hooks::HookKey::new_move(0, mov.id, 0);
                target.borrow_mut().hooks.defense_modifiers.overlay
                    .insert(key, hooks::DamageHook(|_| 0.5));
                create_context(&target, rng).execute_basic_move(rng);
                target.borrow_mut().hooks.defense_modifiers.overlay
                    .remove(&key);
            }
        },
        Effect::DreamEater => {
            if target_count != 1 || !targets[0].borrow().is_asleep() {
                return false;
            }
            create_context(&targets[0], rng).execute_basic_move(rng);
        },
        Effect::MirrorMove => {
            // TODO: implement
        },
        Effect::RaiseUserAttack
            | Effect::RaiseUserDefense
            | Effect::RaiseUserSpecialAttack
            | Effect::RaiseUserEvasion
            | Effect::RaiseUserAttack2
            | Effect::RaiseUserDefense2
            | Effect::RaiseUserSpeed2
            | Effect::RaiseUserSpecialAttack2
            | Effect::RaiseUserSpecialDefense2
        => {
            // TODO: implement
        },
        Effect::Haze => {
            for target in targets {
                target.borrow_mut().stat_changes = [0; moves::CHANGEABLE_STATS];
            }
        },
        Effect::Bide => {
            // TODO: implement
        },
        Effect::Hit2To3TurnsThenConfuseUser => {
            // TODO: implement
        },
        Effect::SwitchOutTarget => {
            // TODO: implement
        },
        Effect::Hit2To5Times => {
            for target in targets {
                let acc = create_context(&target, rng).accuracy();
                if acc >= 1.0 || rng.gen_range(0.0, 1.0) < acc {
                    let hits = match rng.gen_range(0, 6) {
                        0 | 1 => 2,
                        2 | 3 => 3,
                        4 => 4,
                        5 => 5,
                        _ => unreachable!(),
                    };
                    for _ in 0..hits {
                        create_context(&target, rng).execute_basic_core(rng);
                    }
                }
            }
        },
        Effect::Conversion => {
            // TODO: implement
        },
        Effect::Toxic => {
            // TODO: implement
        },
        Effect::LightScreen => {
            // TODO: implement
        },
        Effect::TriAttack => {
            // TODO: implement
        },
        Effect::Rest => {
            // TODO: implement
        },
        Effect::OneHitKO => {
            // TODO: implement
        },
        Effect::RazorWind => {
            // TODO: implement
        },
        Effect::SuperFang => {
            for target in targets {
                if target.borrow().efficacy(mov.typ) > 0.0 {
                    let hp = target.borrow().perm.borrow().hp;
                    target.borrow_mut().direct_percentage(hp, -50);
                }
            }
        },
        Effect::DragonRage => {
            for target in targets {
                if target.borrow().efficacy(mov.typ) > 0.0 {
                    target.borrow_mut().direct_damage(40);
                }
            }
        },
        Effect::SixteenthHP2To5Turns => {
            // TODO: implement
        },
        Effect::HitTwice => {
            for target in targets {
                let acc = create_context(&target, rng).accuracy();
                if acc >= 1.0 || rng.gen_range(0.0, 1.0) < acc {
                    for _ in 0..2 {
                        create_context(&target, rng).execute_basic_core(rng);
                    }
                }
            }
        },
        Effect::HalfRecoilIfMiss => {
            for target in targets {
                let ctx = create_context(&target, rng);
                let acc = ctx.accuracy();
                if acc >= 1.0 || rng.gen_range(0.0, 1.0) < acc {
                    ctx.execute_basic_core(rng);
                } else {
                    let max = ctx.calc_max_damage();
                    let dmg = ((max * rng.gen_range(85, 101)) / 100).max(1).min(max);
                    user.borrow_mut().direct_percentage(dmg, -50);
                }
            }
        },
        Effect::Mist => {
            // TODO: implement
        },
        Effect::FocusEnergy => {
            // TODO: implement
        },
        Effect::Transform => {
            // TODO: implement
        },
        Effect::Reflect => {
            // TODO: implement
        },
        Effect::SkyAttack => {
            // TODO: implement
        },
        Effect::Twineedle => {
            // TODO: implement
        },
        Effect::Substitute => {
            // TODO: implement
        },
        Effect::RechargeNextTurn => {
            // TODO: implement
        },
        Effect::Rage => {
            // TODO: implement
        },
        Effect::Mimic => {
            // TODO: implement
        },
        Effect::Metronome => {
            // TODO: implement
        },
        Effect::LeechSeed => {
            // TODO: implement
        },
        Effect::Splash => (),
        Effect::Disable => {
            // TODO: implement
        },
        Effect::UserLevelDamage => {
            for target in targets {
                if target.borrow().efficacy(mov.typ) > 0.0 {
                    let level = user.borrow().overlay.level as u16;
                    target.borrow_mut().direct_damage(level);
                }
            }
        },
        Effect::Psywave => {
            for target in targets {
                if target.borrow().efficacy(mov.typ) > 0.0 {
                    let level = user.borrow().overlay.level as u16;
                    let dmg = (level * rng.gen_range(50, 151)) / 100;
                    target.borrow_mut().direct_damage(dmg);
                }
            }
        },
        Effect::Counter => {
            // TODO: implement
        },
        Effect::Encore => {
            // TODO: implement
        },
        Effect::PainSplit => {
            for target in targets {
                let user_hp = user.borrow().perm.borrow().hp;
                let target_hp = target.borrow().perm.borrow().hp;
                let mean = (user_hp + target_hp) / 2;
                user.borrow().perm.borrow_mut().hp = mean;
                target.borrow().perm.borrow_mut().hp = mean;
            }
        },
        Effect::Snore => {
            if !user.borrow().is_asleep() {
                return false;
            }
            for target in targets {
                create_context(&target, rng).execute_basic_move(rng);
            }
        },
        Effect::Conversion2 => {
            // TODO: implement
        },
        Effect::GuaranteeNextMoveHit => {
            // TODO: implement
        },
        Effect::Sketch => {
            // TODO: implement
        },
        Effect::SleepTalk => {
            // TODO: implement
        },
        Effect::DestinyBond => {
            // TODO: implement
        },
        Effect::MoreDamageWhenLessUserHP => {
            for target in targets {
                let user_hp = user.borrow().perm.borrow().hp as f64;
                let max_hp = user.borrow().overlay.stat(vdex::Stat::HP) as f64;
                let r = user_hp / max_hp;
                let mut ctx = create_context(&target, rng);
                ctx.power = if r < 0.0417 {
                    200
                } else if r < 0.1042 {
                    150
                } else if r < 0.2083 {
                    100
                } else if r < 0.3542 {
                    80
                } else if r < 0.6875 {
                    40
                } else {
                    20
                };
                ctx.execute_basic_move(rng);
            }
        },
        Effect::Spite => {
            // TODO: implement
        },
        Effect::FalseSwipe => {
            // TODO: implement
        },
        Effect::CurePartyStatus => {
            // TODO: implement
        },
        Effect::TripleKick => {
            // TODO: implement
        },
        _ => panic!("TODO: Not implemented yet!"),
    }
    return true;
}
