use crate::caches::pokemon::pokemon_ref;
use crate::exec::moves::execute_move;
use crate::formats::SingleBattle;
use crate::hooks;
use crate::team;
use std::rc::Rc;
use vdex::Ability;
use vdex::moves;
use vdex::Nature;
use vdex::pokemon;

#[test]
fn test_move() {
    let dex = vdex::pokedex();
    let clefairy = team::TeamMember {
        pokemon: pokemon_ref(pokemon::PokemonId(34)),
        gender: pokemon::Gender::Female,
        ability: Ability::MagicGuard,
        nature: Nature::Lonely,
        held: None,
        friendship: 255,
        evs: Default::default(),
        ivs: Default::default(),
        moves: [Some(&dex.moves[moves::MoveId(0)]), None, None, None],
        pp_ups: Default::default(),
        level: 5,
    };
    assert!(clefairy.verify(false));
    let team = vec![Rc::new(clefairy)];
    let battle = SingleBattle::new(&team, &team);
    battle.hooks.critical_cancels.battle.borrow_mut().insert(
        hooks::HookKey::new_engine(0, 0, 0), true);
    let user = &battle.battler1.current;
    let target = &battle.battler2.current;
    let start_hp = target.borrow().perm.borrow().hp;
    eprintln!("Target Start HP: {}", start_hp);
    let slot = 0;
    let mov = user.borrow().overlay.moves[slot as usize].unwrap();
    execute_move(user, slot, mov, |tgts| battle.resolve_targets(tgts),
        &mut rand::thread_rng());
    let final_hp = target.borrow().perm.borrow().hp;
    eprintln!("Target HP: {}", final_hp);
    assert!(final_hp >= 15 && final_hp <= 17);
}
