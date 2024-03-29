use crate::utils;
use database::MergedFamily;
use teloxide::utils::markdown;

fn block_str(first: i32, last: i32) -> String {
    if first != last {
        return format!("{first}-{last}, ");
    }

    format!("{last}, ")
}

fn pokemon_vec_to_string(vector: &mut Vec<&i32>) -> String {
    vector.sort();

    let mut output = String::new();
    let mut first = vector[0];
    let mut last = vector[0];

    for dex in vector {
        // Non-contiguous number, add previous block to string
        if *dex > &(last + 1) {
            output.push_str(&block_str(*first, *last));
            first = dex;
        }

        last = dex;
    }

    // Add last block if not there yet
    let last_block = &block_str(*first, *last);
    if !output.ends_with(last_block) {
        output.push_str(last_block);
    }

    // Remove last comma and space
    output.trim_end_matches(", ").to_string()
}

pub async fn generate_search_string() -> String {
    let families = match database::get_merged().await {
        Err(e) => return utils::format_error("There was an error reading database`", &e),
        Ok(families) => families,
    };

    let mut filtered: Vec<&i32> = families
        .iter()
        .filter(|f| f.pokemons.iter().all(|p| p.level40 == 0))
        .flat_map(|f| f.pokemons.iter().map(|p| &p.dex))
        .collect(); // Convert Iterator into Vec

    let string = pokemon_vec_to_string(&mut filtered);
    format!("`{}`", markdown::escape(&string))
}

pub async fn stats() -> String {
    let families = match database::get_merged().await {
        Err(e) => return utils::format_error("There was an error reading database`", &e),
        Ok(families) => families,
    };

    let n_families = families.len();
    let maxed_families = families
        .iter()
        .filter(|f| f.pokemons.iter().any(|p| p.level40 > 0))
        .count();
    let maxed_pokes = families.iter().fold(0, |acc, x| {
        acc + x.pokemons.iter().fold(0, |acc, x| acc + x.level40)
    });

    format!(
        "Level40: {maxed_pokes}\nFamilies: {maxed_families}/{n_families}"
    )
}

pub fn version() -> String {
    format!(
        "🌐: _{}_\n⏰:` {}`",
        option_env!("GIT_HASH").unwrap_or("NA"),
        option_env!("DATE").unwrap_or("NA")
    )
}
