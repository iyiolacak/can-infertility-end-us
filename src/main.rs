
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
        
        let infertile_population: Vec<&Individual> = population
        .iter()
        .filter(|individual| !individual.is_fertile && !individual.is_dead)
        .collect();
    
    println!("{} humans alive", population.len());
    println!("Infertile population is {:?}", infertile_population);
}
 