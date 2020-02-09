use serde::{Deserialize, Serialize};

use super::encounters::*;
use super::games::*;
use super::pokemon::*;
use super::resource_lists::*;
use super::utility::*;

use crate::{impl_id_and_named, set_endpoint};

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Location {
    pub id: i16,
    pub name: String,
    pub region: Option<NamedAPIResource<Region>>,
    pub names: Vec<Name>,
    pub game_indices: Vec<GenerationGameIndex>,
    pub areas: Vec<NamedAPIResource<LocationArea>>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct LocationArea {
    pub id: i16,
    pub name: String,
    pub game_index: i16,
    pub encounter_method_rates: Vec<EncouterMethodRate>,
    pub location: NamedAPIResource<Location>,
    pub names: Vec<Name>,
    pub pokemon_encounters: Vec<PokemonEncouter>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct EncouterMethodRate {
    pub encounter_method: NamedAPIResource<EncounterMethod>,
    pub version_details: Vec<EncounterVersionDetails>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct EncounterVersionDetails {
    pub rate: u8,
    pub version: NamedAPIResource<Version>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PokemonEncouter {
    pub pokemon: NamedAPIResource<Pokemon>,
    pub version_details: Vec<VersionEncounterDetail>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PalParkArea {
    pub id: i16,
    pub name: String,
    pub names: Vec<Name>,
    pub pokemon_encounters: Vec<PalParkEncounterSpecies>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PalParkEncounterSpecies {
    pub base_score: u8,
    pub rate: u8,
    pub pokemon_species: NamedAPIResource<PokemonSpecies>,
}

#[cfg_attr(debug_assertions, serde(deny_unknown_fields))]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct Region {
    pub id: i16,
    pub locations: Vec<NamedAPIResource<Location>>,
    pub name: String,
    pub names: Vec<Name>,
    pub main_generation: NamedAPIResource<Generation>,
    pub pokedexes: Vec<NamedAPIResource<Pokedex>>,
    pub version_groups: Vec<NamedAPIResource<VersionGroup>>,
}

set_endpoint!(Location, NamedAPIResourceList, "location");
set_endpoint!(LocationArea, NamedAPIResourceList, "location-area");
set_endpoint!(PalParkArea, NamedAPIResourceList, "pal-park-area");
set_endpoint!(Region, NamedAPIResourceList, "region");

impl_id_and_named!(Location);
impl_id_and_named!(LocationArea);
impl_id_and_named!(PalParkArea);
impl_id_and_named!(Region);
