use std::sync::OnceLock;

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Element {
    Water,
    Fire,
    Wind,
    Earth,
}

#[derive(Clone, Debug)]
enum PokeGender {
    Male,
    Female,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum Category {
    Lizard,
    Bird,
    Fish,
    Seed,
    Butterfly,
    Worm,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Pokemon<'a> {
    name: String,
    element: Element,
    category: Category,
    height: f32,
    weight: f32,
    gender: PokeGender,
    abilities: Vec<&'a str>,
}

impl<'a> Pokemon<'a> {
    fn new(
        name: String,
        element: Element,
        category: Category,
        height: f32,
        weight: f32,
        gender: PokeGender,
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

#[derive(Clone, Debug)]
struct PokeDex<'a> {
    data: OnceLock<Pokemon<'a>>,
    next: OnceLock<Box<PokeDex<'a>>>,
}

impl<'a> PokeDex<'a> {
    /// Never instantiate a PokeDex with PokeDex::new(), use PokeDex::instance() instead.
    unsafe fn new() -> Self {
        Self {
            data: OnceLock::new(),
            next: OnceLock::new(),
        }
    }

    fn instance() -> &'static PokeDex<'a> {
        static INSTANCE: OnceLock<PokeDex> = OnceLock::new();
        unsafe { INSTANCE.get_or_init(|| PokeDex::new()) }
    }

    fn push(&self, pokemon: Pokemon<'a>) {
        if let Err(pokemon) = self.data.set(pokemon) {
            unsafe {
                let next = self.next.get_or_init(|| Box::new(PokeDex::new()));
                next.push(pokemon);
            }
        }
    }

    fn get_pokemons(&self) -> Vec<Pokemon<'a>> {
        let mut pokemons = Vec::new();
        let mut current = Some(self);

        while let Some(pokedex) = current {
            if let Some(pokemon) = pokedex.data.get() {
                pokemons.push(pokemon.clone());
            }
            current = pokedex.next.get().map(|next| next.as_ref());
        }
        pokemons
    }
}

fn main() {
    let fire_attacks = vec!["Amber", "Flamethrower"];
    let water_attacks = vec!["Watergun", "Aqua Tail"];

    let charmander = Pokemon::new(
        String::from("Charmander"),
        Element::Fire,
        Category::Lizard,
        0.72,
        2.5,
        PokeGender::Male,
        fire_attacks,
    );

    let squirtle = Pokemon::new(
        String::from("Squirtle"),
        Element::Water,
        Category::Lizard,
        0.61,
        1.5,
        PokeGender::Female,
        water_attacks,
    );

    let pokedex = PokeDex::instance();
    pokedex.push(charmander.clone());

    let pokemons_in_pokedex = pokedex.get_pokemons();
    println!("{pokemons_in_pokedex:?}");

    let pokedex_2 = PokeDex::instance();
    pokedex_2.push(squirtle.clone());
    let pokemons_in_pokedex_2 = pokedex_2.get_pokemons();
    println!("{pokemons_in_pokedex_2:?}");

    // Example error of instatiation without unsafe
    //let pokedex_3 = PokeDex::new();
    //let pokemons_in_pokedex_3 = pokedex_3.get_pokemons();
    //println!("{pokemons_in_pokedex_3:?}");
}
