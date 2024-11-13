#[allow(dead_code)]
#[derive(Clone, Debug)]
enum PokemonType {
    Water,
    Fire,
    Grass,
}

#[derive(Clone, Debug)]
enum PokemonGender {
    Male,
    Female,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Category {
    Lizard,
    Seed,
    TinyTurtle,
}

trait PokemonMovements {
    fn fight(&self) {}
    fn heal(&self) {}
    fn run(&self) {}
}
#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Pokemon<'a> {
    name: String,
    element: PokemonType,
    category: Category,
    height: f32,
    weight: f32,
    gender: PokemonGender,
    abilities: Vec<&'a str>,
}

impl<'a> Pokemon<'a> {
    fn new(
        name: String,
        element: PokemonType,
        category: Category,
        height: f32,
        weight: f32,
        gender: PokemonGender,
        abilities: Vec<&'a str>,
    ) -> Self {
        Self {
            name,
            element,
            category,
            height,
            weight,
            gender,
            abilities,
        }
    }
}

impl<'a> PokemonMovements for Pokemon<'a> {
    fn fight(&self) {
        let ability = self.abilities.clone();
        println!("Pokemon attack with {}!", ability[0])
    }

    fn heal(&self) {
        println!("Pokemon, heal!")
    }

    fn run(&self) {
        println!("Pokemon, run!")
    }
}

struct PokemonFactory;

impl<'a> PokemonFactory {
    fn new_pokemon(pokemon_type: PokemonType) -> Pokemon<'a> {
        match pokemon_type {
            PokemonType::Fire => Pokemon::new(
                String::from("Charmander"),
                PokemonType::Fire,
                Category::Lizard,
                2.00,
                18.7,
                PokemonGender::Male,
                vec!["Blaze"],
            ),
            PokemonType::Water => Pokemon::new(
                String::from("Squirtle"),
                PokemonType::Water,
                Category::TinyTurtle,
                1.80,
                19.8,
                PokemonGender::Male,
                vec!["Torrent"],
            ),

            PokemonType::Grass => Pokemon::new(
                String::from("Bulbasaur"),
                PokemonType::Grass,
                Category::Seed,
                2.04,
                15.2,
                PokemonGender::Female,
                vec!["Overgrow"],
            ),
        }
    }
}

fn main() {
    let charmander = PokemonFactory::new_pokemon(PokemonType::Fire);
    let squirtle = PokemonFactory::new_pokemon(PokemonType::Water);
    let bulbasaur = PokemonFactory::new_pokemon(PokemonType::Grass);

    charmander.fight();
    squirtle.fight();
    bulbasaur.fight();

    charmander.heal();
    bulbasaur.run();
}
