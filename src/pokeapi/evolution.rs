use serde::{Deserialize, Serialize};

use super::items::*;
use super::locations::*;
use super::moves::*;
use super::pokemon::*;
use super::resource_lists::*;
use super::utility::*;

use crate::{impl_id, impl_id_and_named, set_endpoint};

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EvolutionChain {
    pub id: i64,
    pub baby_trigger_item: Option<NamedAPIResource<Item>>,
    pub chain: ChainLink,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChainLink {
    pub is_baby: bool,
    pub species: NamedAPIResource<PokemonSpecies>,
    pub evolution_details: Vec<EvolutionDetail>,
    pub evolves_to: Vec<ChainLink>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EvolutionDetail {
    pub item: Option<NamedAPIResource<Item>>,
    pub trigger: NamedAPIResource<EvolutionTrigger>,
    pub gender: Option<u64>,
    pub held_item: Option<NamedAPIResource<Item>>,
    pub known_move: Option<NamedAPIResource<Move>>,
    pub known_move_type: Option<NamedAPIResource<Type>>,
    pub location: Option<NamedAPIResource<Location>>,
    pub min_level: Option<u64>,
    pub min_happiness: Option<u64>,
    pub min_beauty: Option<u64>,
    pub min_affection: Option<u64>,
    pub needs_overworld_rain: bool,
    pub party_species: Option<NamedAPIResource<PokemonSpecies>>,
    pub party_type: Option<NamedAPIResource<Type>>,
    pub relative_physical_stats: Option<u64>,
    pub time_of_day: String,
    pub trade_species: Option<NamedAPIResource<PokemonSpecies>>,
    pub turn_upside_down: bool,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct EvolutionTrigger {
    pub id: i64,
    pub name: String,
    pub names: Vec<Name>,
    pub pokemon_species: Vec<NamedAPIResource<PokemonSpecies>>,
}

set_endpoint!(EvolutionChain, APIResourceList, "evolution-chain");
set_endpoint!(EvolutionTrigger, NamedAPIResourceList, "evolution-trigger");

impl_id!(EvolutionChain);
impl_id_and_named!(EvolutionTrigger);
