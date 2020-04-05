extern crate regex;
extern crate rand;
use std::env;
use std::fs::File; // Permet d'écire/lire dans un fichier.
use std::io::prelude::*;
use rand::Rng;
use regex::Regex;
use std::io;


struct GameUser{ // Structure du joueur.
	lives : i32, 
	guessedLetter : String,

}

enum userInput{ // Enumération.
	lettreTrouve,
	lettreDejaTrouve,
	lettreFausse,
}


fn main() {
	let mut user: GameUser = GameUser{
		lives : 10,
		guessedLetter : String::new(), // C'est là ou l'on mettra les lettres trouvées par l'utilisateur.
	};


	let mut the_world = s_world();
	let mut secret_line_masked = display_masked(&the_world, &user.guessedLetter);

    println!("Devine le mot :  + {} + {}", the_world, secret_line_masked);

    println!("[Hangman Game] \n\nRecherche d'un mot à trouver... ");
    println!("A vous de jouer, tenter de trouver le mot suivant : \n");
    println!("{}\n", secret_line_masked);

    loop{
    	println!("Il vous reste {} vie(s)", user.lives);
    	println!("Donne moi une lettre : ");
    	let user_guess = read_char_guess();

    	if check_guess_user(user_guess){ // On veut savoir si l'utilisateur à donner une bonne lettre.
    		let user_guess_validate = user_guess.unwrap(); // On unwrap.
    		if !user_guess_validate.is_lowercase(){ // On vérifie si il est bien en lowercase.
    			println!("Il faut que les lettres soit en minuscule.");
    			continue
    		}
    		match validate_guess_user(&the_world, user_guess_validate, &user){ // On valide ou on ne valide pas.
    			userInput::lettreTrouve => {
    				println!("Bravo tu as trouvé une lettre.");
    				user.guessedLetter.push(user_guess_validate);
    				let mut test = user_guess_validate.to_string();
    				secret_line_masked = display_masked(&the_world,&user.guessedLetter);
    				println!("{}", secret_line_masked);

    				if !secret_line_masked.contains('_'){
    					println!("Tu as gagné.");
    				}

    			}

    			userInput::lettreFausse =>{
    				println!("Tu n'as pas trouvé la bonne lettre, retente encore une fois !");
    				user.lives = user.lives -1; // Faux donc -1 vie.
    				break;

    			}

    			userInput::lettreDejaTrouve =>{
   					println!("Tu as déjà trouvé cette lettre !");
    			}

    		} 


    		println!("{}", user_guess_validate);

    	}
    }



}

fn s_world() -> String { // Sélection du mot.

	let mut fileworld = File::open("word.txt")
		.expect("Fichier non trouvé");
	let mut contents = String::new();
	fileworld.read_to_string(&mut contents) // On met les mots dans une string.
		.expect("Le fichier n'a pu être lu.");

	let world_check : Vec<&str> = contents.trim().split(",").collect(); // Ici on une variable de type VEC &STR (Slice qui peut être redimensionné) 
// Trim = On enlève les espaces blancs, split on split, collect qui lui récupère tout ce qui est itérable.
	let random_world = rand::thread_rng().gen_range(0, 7); // On crée notre index.
	
	world_check[random_world].to_string() // &STR to String.
}

fn read_char_guess() -> Option<char>{
	let mut user_guess_world = String::new();
	io::stdin().read_line(&mut user_guess_world)
		.expect("Impossible de lire la lettre.");
	user_guess_world.trim().chars().nth(0) // Sans espace blanc, en char et première itération.
}


fn check_guess_user(user_guess_world: Option <char>) -> bool{
	match user_guess_world{
		Some(user_guess_world) =>{
			if !user_guess_world.is_alphabetic(){false} // Si c'est différent d'une lettre FALSE / Else -> TRUE.
			else {true}
		}

		None => {return false;}
	}
}


fn validate_guess_user(realworld: &String, letter_guess_user: char, user: &GameUser) -> userInput { // On fait match cette fonction.

	if user.guessedLetter.contains(letter_guess_user){ // Déjà
		return userInput::lettreDejaTrouve; // On retourne la lettre s'il elle y ai déjà.
	}

	if !realworld.contains(letter_guess_user){ // Faux
		return userInput::lettreFausse;
	}
	
	userInput::lettreTrouve // Toutes les possibilités ont été testées.
}


fn display_masked(theWord : &String, userGuess: &String) -> String // Ici on donne les lettres deviné par l'utilisateur + le mot à trouvé.
{
	let mut result : String = String::new();

	for (u,c) in theWord.chars().enumerate() // Ici u itère (1,2,3...) c itère sur le mot a trouvé.
	{
		println!("{}", c);
		result.push(if c == ' '{c} // On continue.
			else if userGuess.contains(c) {c}
			else {'_'}); // Si il n'y a pas la lettre dans userGuess alors met un _ 
		result.push(' ');
	}
	result
}
