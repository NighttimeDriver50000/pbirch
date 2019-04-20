bitflags! {
    pub struct BenchAilmentFlags: u8 {
        const PARALYZED = 0x01;
        const ASLEEP = 0x02;
        const FROZEN = 0x04;
        const BURNED = 0x08;
        const POISONED = 0x10;
        const BADLY_POISONED = 0x30;
    }
}

pub struct BenchAilments {
    pub flags: BenchAilmentFlags,
    pub remaining_sleep_turns: u8,
}

bitflags! {
    pub struct BattlerAilmentFlags: u32 {
        // Does not include effects that never apply beyond the next turn.
        // Aligned to avoid offset typos.
        const TELEKINESIS   = 0x00000001;
        const SMACKED_DOWN  = 0x00000002;
        const CONFUSED      = 0x00000004;
        const INFATUATED    = 0x00000008;
        const TRAPPED       = 0x00000010;
        const NIGHTMARE     = 0x00000020;
        // Research questions:
        // Can a Tormented Pokémon use the same move via Mirror Move?
        // If a Pokemon cannot execute the selected move (e.g. paralyzed),
        // is that move allowed next turn? Is the previous move?
        // How does Torment interact with Truant?
        // Current ruling:
        // When a move *completes* (e.g. on the last turn of Rollout, even if it
        // missed, but not if stopped by (say) paralysis and the current turn
        // was the first Rollout turn), its *slot* (e.g. the slot for Mirror
        // Move or Metronome after the chained move completes) is disabled,
        // unless the move in that slot changed this turn (e.g. on successful
        // Mimic or Transform), in which case the "fifth slot" gets disabled.
        const TORMENTED     = 0x00000040;
        const MOVE_DISABLED = 0x00000080;
        const DROWSY        = 0x00000100;
        const HEAL_BLOCKED  = 0x00000200;
        const NO_GHOST_IMMUNITY = 0x00000400;
        const NO_DARK_IMMUNITY = 0x00000800;
        const SEEDED        = 0x00001000;
        const EMBARGOED     = 0x00002000;
        const PERISHING     = 0x00004000;
        const ROOTED        = 0x00008000;
        const CURSED        = 0x00010000;
        const VEILED        = 0x00020000;
        const CURLED        = 0x00040000;
        const LEVITATING    = 0x00080000;
        const MINIMIZED     = 0x00100000;
        const SUBSTITUTED   = 0x00200000;
    }
}

pub struct BattlerAilments {
    pub flags: BattlerAilmentFlags,
    pub turns_badly_poisoned: u8,
    pub remaining_confused_attacks: u8,
    pub tormented_move_slot: u8,
    pub disabled_move_slot: u8,
    pub remaining_heal_block_turns: u8,
    pub remaining_embargo_turns: u8,
    pub perish_count: u8,
    pub substitute_hp: u16,
}
