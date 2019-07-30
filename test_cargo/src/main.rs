use std::io;
fn main() {
	println!("Для выхода введите 'exit'");
	println!("Введите число");
	loop{
		let mut guess = String::new();
		io::stdin().read_line(&mut guess)
			.expect("This must be str");
		if guess.trim() == "exit".to_string(){
			break;
		}
		let i32_guess:i32 = match guess.trim().parse::<i32>(){
			Ok(num)=> num,
			Err(_)=>{
				println!("Ошибка при приоброзовании текста в i32. Попробуйте еще раз");
				continue;
			}
		};
		println!("Ваше число - {}. Введите еще одно число",i32_guess);
	}
}
