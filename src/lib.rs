mod caches;

use caches::movesets::cache_moveset;
use caches::pokemon::species_ref;
use vdex::{Ability, Nature, Stat};
use vdex::items;
use vdex::moves;
use vdex::pokemon;

pub struct TeamMember {
    pub pokemon: &'static pokemon::Pokemon,
    pub gender: pokemon::Gender,
    pub ability: Ability,
    pub nature: Nature,
    pub held: Option<&'static items::Item>,
    pub friendship: u8,
    pub evs: pokemon::BaseStats,
    pub ivs: pokemon::BaseStats,
    pub moves: [Option<&'static moves::Move>; 4],
    pub pp_ups: [u8; 4],
    pub level: u8,
}

impl TeamMember {
    pub fn verify(&self, skip_moves: bool) -> bool {
        let pokemon_id = self.pokemon.id;
        let genderless = self.gender == pokemon::Gender::Genderless;
        let species_genderless = species_ref(pokemon_id).gender_rate < 0;
        let gender = genderless == species_genderless;
        let abilities = self.pokemon.abilities;
        let ability = abilities.first() == self.ability
            || abilities.second().map_or(false, |a| a == self.ability);
        let held = self.held.map_or(true,
            |item| item.flags.contains(items::Flags::HOLDABLE));
        let evs = self.evs.0.iter().fold(0, |acc, ev| acc + (*ev as u16)) <= 510;
        let ivs = !self.ivs.0.iter().any(|iv| *iv > 31);
        let has_move = self.moves.iter().any(|mov| mov.is_some());
        let moves = has_move && (skip_moves || self.moves.iter().all(
            |opt| opt.map_or(true, |mov| cache_moveset(pokemon_id,
                |set| set.contains(&mov.id)))));
        let pp_ups = self.pp_ups.iter().all(|ups| *ups <= 3);
        let level = self.level > 0 && self.level <= 100;
        gender && ability && held && evs && ivs && moves && pp_ups && level
    }

    pub fn stat(&self, stat: Stat) -> u16 {
        let base = self.pokemon.stats[stat] as u16;
        let iv = self.ivs[stat] as u16;
        let ev = self.evs[stat] as u16;
        let level = self.level as u16;
        let core = (((2 * base) + iv + (ev / 4)) * level) / 100;
        if stat == Stat::HP {
            core + level + 10
        } else if self.nature.increased().map_or(false, |s| s == stat) {
            ((core + 5) * 11) / 10
        } else if self.nature.decreased().map_or(false, |s| s == stat) {
            ((core + 5) * 9) / 10
        } else {
            core + 5
        }
    }
}
