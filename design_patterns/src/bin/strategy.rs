use design_patterns_bench::pokemon::{Pokemon, PokemonType};

enum DamageEnum {
    VeryEffective,
    NotEffective,
    Neutral,
}

impl DamageEnum {
    fn description(&self) -> &str {
        match self {
            DamageEnum::VeryEffective => "Very effective this Pokemon type.",
            DamageEnum::NotEffective => "Not effective against this Pokemon type.",
            DamageEnum::Neutral => "Normal effectiveness against this Pokemon type.",
        }
    }
}

trait DamageEvaluator {
    fn calculate_dmg_against(&self, _pokemon: Pokemon);
}

struct FireDamage;

impl DamageEvaluator for FireDamage {
    fn calculate_dmg_against(&self, pokemon: Pokemon) {
        let element = pokemon.element;

        match element {
            PokemonType::Fire => println!("{}", DamageEnum::Neutral.description()),
            PokemonType::Water => println!("{}", DamageEnum::NotEffective.description()),
            PokemonType::Grass => println!("{}", DamageEnum::VeryEffective.description()),
        }
    }
}

struct WaterDamage;

impl DamageEvaluator for WaterDamage {
    fn calculate_dmg_against(&self, pokemon: Pokemon) {
        let element = pokemon.element;

        match element {
            PokemonType::Fire => println!("{}", DamageEnum::VeryEffective.description()),
            PokemonType::Water => println!("{}", DamageEnum::Neutral.description()),
            PokemonType::Grass => println!("{}", DamageEnum::NotEffective.description()),
        }
    }
}

struct GrassDamage;

impl DamageEvaluator for GrassDamage {
    fn calculate_dmg_against(&self, pokemon: Pokemon) {
        let element = pokemon.element;

        match element {
            PokemonType::Fire => println!("{}", DamageEnum::NotEffective.description()),
            PokemonType::Water => println!("{}", DamageEnum::VeryEffective.description()),
            PokemonType::Grass => println!("{}", DamageEnum::Neutral.description()),
        }
    }
}

struct PokemonStrategyContext {
    evaluation_strategy: Box<dyn DamageEvaluator>,
}

impl PokemonStrategyContext {
    fn new(evaluation_strategy: Box<dyn DamageEvaluator>) -> Self {
        PokemonStrategyContext {
            evaluation_strategy,
        }
    }

    fn process_strategy(&self, pokemon: Pokemon) {
        self.evaluation_strategy.calculate_dmg_against(pokemon)
    }
}

#[allow(dead_code)]
fn main() {
    #[path = "factory.rs"]
    mod factory;
    use factory::PokemonFactory;

    let charmander = PokemonFactory::new_pokemon(PokemonType::Fire);
    let squirtle = PokemonFactory::new_pokemon(PokemonType::Water);
    let bulbasaur = PokemonFactory::new_pokemon(PokemonType::Grass);

    let fire_strategy = Box::new(FireDamage);
    let water_strategy = Box::new(WaterDamage);
    let grass_strategy = Box::new(GrassDamage);

    {
        let strategy_context = PokemonStrategyContext::new(fire_strategy);
        strategy_context.process_strategy(charmander.clone());
        strategy_context.process_strategy(squirtle.clone());
        strategy_context.process_strategy(bulbasaur.clone());
    }

    {
        let strategy_context = PokemonStrategyContext::new(water_strategy);
        strategy_context.process_strategy(charmander.clone());
        strategy_context.process_strategy(squirtle.clone());
        strategy_context.process_strategy(bulbasaur.clone());
    }

    {
        let strategy_context = PokemonStrategyContext::new(grass_strategy);
        strategy_context.process_strategy(charmander.clone());
        strategy_context.process_strategy(squirtle.clone());
        strategy_context.process_strategy(bulbasaur.clone());
    }
}
