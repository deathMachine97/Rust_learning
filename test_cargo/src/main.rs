use std::io;
use rand::Rng;
use typename::TypeName;
use std::cmp::Ordering;

#[allow(dead_code)]
fn guessing_game(){
	let i32_secret_number = rand::thread_rng().gen_range(1,101);
	println!("Секртеное число - {} type is {}", i32_secret_number,i32_secret_number.type_name_of());
	println!("Для выхода введите 'exit'");
	println!("Введите число");
	loop{
		let mut guess = String::new();
		io::stdin().read_line(&mut guess)
			.expect("This must be str");
		if guess.trim() == "exit".to_string(){
			break;
		}
		let i32_guess:i32 = match guess.trim().parse(){
			Ok(num)=> num,
			Err(_)=>{
				println!("Ошибка при приоброзовании текста в i32. Попробуйте еще раз");
				continue;
			}
		};
		println!("Ваше число - {}. Введите еще одно число",i32_guess);
		match i32_guess.cmp(&i32_secret_number){
			Ordering::Less => println!("Слишком мало!"),
			Ordering::Greater => println!("Слишком много!"),
			Ordering::Equal => {
				println!("Все верно!!!");
				break
			}
		}
	}
}

fn main() {
	let mut _x = String::from("hello");
}
