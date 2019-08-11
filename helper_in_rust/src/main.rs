use std::io;
enum Choises{
	ChangeSentiment,
	IdentifyVagueKeys,
	SearchItemInBase,
	AddItemToProject
}

fn get_i32_input(name_of_var: String) -> i32{
	loop{
		let mut input_storage = String::new();
		println!("Введите {}",name_of_var);
		io::stdin().read_line(&mut input_storage).expect("Ошибка при введении");
		let i32_project_id:i32 = match input_storage.trim().parse(){
			Ok(num)	=> num,
			Err(_)	=> {
				println!("Ошибка при введение {}. Введите {} еще раз.",name_of_var,name_of_var);
				continue
			}
		};
		return i32_project_id;
	}
}

impl Choises{
	fn start(&self) {
		match &self{
			Choises::ChangeSentiment =>{
				let i32_project_id :i32 = get_i32_input(String::from("project_id"));
				let i32_user_id :i32 = get_i32_input(String::from("user_id"));
				
			},
			Choises::IdentifyVagueKeys => {
				println!("asdf");
			},
			Choises::SearchItemInBase => {
				println!("asdf");
			},
			Choises::AddItemToProject => {
				println!("asdf");
			},
		}
	}
}

fn main() {
	loop{
		let mut user_choice = String::new();
		io::stdin().read_line(&mut user_choice).expect("It must be string");
		match user_choice.trim() {
			"ch_s" => {
				Choises::ChangeSentiment.start();
				break;
			},
			"ident_und_s"	=> {
				Choises::IdentifyVagueKeys.start();
				break;
			},
			"item_in_base" => {
				Choises::SearchItemInBase.start();
				break;
			},
			"add_to_project" => {
				Choises::AddItemToProject.start();
				break;
			},
			_=>{
				println!("Нет совпадений");
				continue;
			}
		}
	}
}
