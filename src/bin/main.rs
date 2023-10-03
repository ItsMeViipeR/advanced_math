use advanced_math::derived::derived;

fn main() {
    // Exemple : Calcul de la dérivée de f(x) = x^2 à x = 2 avec h = 0.0001
    let resultat = derived(|x| x.powf(5.0), 2.0, 0.0001);
    println!("Dérivée : {}", resultat);
}