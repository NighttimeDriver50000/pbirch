use super::pokemon::pokemon_ref;
use std::collections::{HashMap, HashSet};
use std::sync::{Once, RwLock};
use vdex::moves::MoveId;
use vdex::pokemon::PokemonId;

type Cache = HashMap<PokemonId, HashSet<MoveId>>;

static mut CACHE: Option<RwLock<Cache>> = None;
static CACHE_ONCE: Once = Once::new();

fn lock_ref() -> &'static mut RwLock<Cache> {
    unsafe {
        CACHE_ONCE.call_once(|| {
            CACHE = Some(RwLock::new(HashMap::new()));
        });
        CACHE.as_mut().unwrap()
    }
}

pub fn cache_moveset<F, R>(pokemon_id: PokemonId, callback: F) -> R
    where F: FnOnce(&HashSet<MoveId>) -> R
{
    let lock = lock_ref();
    {
        let cache = lock.read().unwrap();
        let entry = (*cache).get(&pokemon_id);
        if let Some(moveset) = entry {
            return callback(moveset);
        }
    }
    {
        let mut moveset = HashSet::new();
        for gen_moveset in pokemon_ref(pokemon_id).moves.values() {
            for mov in gen_moveset {
                moveset.insert(mov.move_id);
            }
        }
        let mut cache = lock.write().unwrap();
        (*cache).insert(pokemon_id, moveset);
        return callback((*cache).get(&pokemon_id).unwrap());
    }
}
