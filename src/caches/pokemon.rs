use std::collections::HashMap;
use std::sync::Once;
use vdex::pokemon;

pub struct PokemonRefs {
    pub pokemon: &'static pokemon::Pokemon,
    pub species: &'static pokemon::Species,
}

pub type PokemonRefMap = HashMap<pokemon::PokemonId, PokemonRefs>;

static mut CACHE: Option<PokemonRefMap> = None;
static CACHE_ONCE: Once = Once::new();

pub fn pokemon_ref_map() -> &'static PokemonRefMap {
    unsafe {
        CACHE_ONCE.call_once(|| {
            let mut map = HashMap::new();
            let dex = vdex::pokedex();
            for i in 0..pokemon::SPECIES_COUNT {
                let id = pokemon::SpeciesId(i as u16);
                let species = &dex.species[id];
                for pokemon in species.pokemon.iter() {
                    map.insert(pokemon.id, PokemonRefs { pokemon, species });
                }
            }
            CACHE = Some(map);
        });
        CACHE.as_ref().unwrap()
    }
}

pub fn pokemon_ref(pokemon_id: pokemon::PokemonId) -> &'static pokemon::Pokemon {
    pokemon_ref_map()[&pokemon_id].pokemon
}

pub fn species_ref(pokemon_id: pokemon::PokemonId) -> &'static pokemon::Species {
    pokemon_ref_map()[&pokemon_id].species
}
