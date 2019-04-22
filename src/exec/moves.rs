use crate::battle::{Current, DamageContext};
use crate::formats::{AbsoluteTarget, RelativeTarget};
use vdex::moves::{self, Move, Effect};

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
    let gen_critical = |rng: &mut R| {
        let r = rng.gen_range(0, 48);
        match (user.borrow().critical_rate + mov.meta.critical_rate).max(0) {
            0 => r < 3,
            1 => r < 6,
            2 => r < 12,
            3 => r < 16,
            _ => r < 24,
        }
    };
    match mov.effect {
        Effect::RegularDamage => {
            for target in targets {
                let context = DamageContext {
                    user: user.clone(),
                    target: target.clone(),
                    slot,
                    mov,
                    typ: mov.typ,
                    power: mov.power,
                    target_count,
                    class: mov.damage_class,
                    critical: gen_critical(rng),
                };
                if rng.gen_range(0.0, 1.0) < context.accuracy() {
                    let dmg = context.damage(rng);
                    if dmg > 0 {
                    }
                }
            }
        },
        _ => panic!("TODO: Not implemented yet!"),
    }
    return true;
}
