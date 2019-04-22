#[macro_use]
extern crate bitflags;
extern crate enum_repr;

pub mod ailments;
pub mod battle;
pub mod formats;
pub mod hooks;
pub mod caches;
pub mod team;

#[cfg(test)]
mod tests;
