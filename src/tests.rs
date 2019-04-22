use crate::battle::DamageContext;
use crate::caches::pokemon::pokemon_ref;
use crate::formats::SingleBattle;
use crate::team;
use std::rc::Rc;
use vdex::Ability;
use vdex::moves;
use vdex::Nature;
use vdex::pokemon;

#[test]
fn test_damage() {
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
    let user = battle.battler1.current;
    let target = battle.battler2.current;
    let slot = 0;
    let mov = user.borrow().overlay.moves[slot as usize].unwrap();
    let context = DamageContext {
        user: user.clone(),
        target: target.clone(),
        slot,
        mov,
        typ: mov.typ,
        power: mov.power,
        target_count: 1,
        class: mov.damage_class,
        critical: false,
    };
    let dmg = context.damage(&mut rand::thread_rng());
    let final_hp = target.borrow().perm.borrow().hp;
    eprintln!("Damage: {}; Target HP: {}", dmg, final_hp);
    assert!(final_hp >= 15 && final_hp <= 17);
}
