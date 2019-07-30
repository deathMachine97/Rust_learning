use std::io;
fn main() {
	loop{
		println!("Guess the number");
		let mut guess = String::new();
		io::stdin().read_line(&mut guess)
			.expect("This must be str");
		let i32_guess:i32 = match guess.trim().parse::<i32>(){
			Ok(num)=> num,
			Err(_)=>{
				println!("Ошибка при приоброзовании текста в i32. Попробуйте еще раз");
				continue;
			}
		};
		println!("The nubmer is {}",i32_guess);
	}
}
