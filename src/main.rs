use std::collections::HashMap;

fn main() {
    #[derive(Debug)]
    struct Individual {
        is_fertile: bool,
        is_dead: bool,
    }
    
    let population: Vec<Individual> = vec![
        Individual { is_fertile: true, is_dead: false },
        Individual { is_fertile: true, is_dead: false },
        Individual { is_fertile: false, is_dead: false },
        ];
        
        let mut population_groups: HashMap<&str, Vec<&Individual>> = HashMap::new();

        population_groups.insert("fertile", Vec::new());
        population_groups.insert("infertile", Vec::new());
        population_groups.insert("dead", Vec::new());

        for individual in &population {
            if individual.is_dead {
                population_groups.get_mut("dead").unwrap().push(individual);
            } else if individual.is_fertile {
                population_groups.get_mut("fertile").unwrap().push(individual);
            } else if !individual.is_fertile {
                population_groups.get_mut("infertile").unwrap().push(individual);
            }
        }
        let alive_population = population_groups["fertile"].len() + population_groups["infertile"].len();
        println!("humans alive {}", alive_population);
        println!("infertile {:?}", population_groups.get("infertile").unwrap().len());
        println!("fertile {:?}", population_groups.get("fertile").unwrap().len());


        // let infertile_population: Vec<&Individual> = population
        // .iter()
        // .filter(|individual| !individual.is_fertile && !individual.is_dead)
        // .collect();
    
    println!("{} humans alive", population.len());
}
 