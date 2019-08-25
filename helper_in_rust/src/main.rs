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
		let i32_return:i32 = match input_storage.trim().parse(){
			Ok(num)	=> num,
			Err(_)	=> {
				println!("Ошибка при введение {}. Введите {} еще раз.",name_of_var,name_of_var);
				continue
			}
		};
		return i32_return;
	}
}

fn get_string_input(name_of_var: String) -> String{
	loop {
		let mut input_storage = String::new();
		println!("Введите {}",name_of_var);
		io::stdin().read_line(&mut input_storage).expect("Ошибка при введении");
		let string_return: String = match input_storage.trim().parse() {
			Ok(num) => num,
			Err(_) =>{
				println!("Ошибка при введение {}. Введите {} еще раз.",name_of_var,name_of_var);
				continue
			}
		};
		return string_return;
	}
}

impl Choises{
	fn start(&self) {
		match &self{
			Choises::ChangeSentiment =>{
				let i32_project_id :i32 = get_i32_input(String::from("project_id"));
				let i32_user_id :i32 = get_i32_input(String::from("user_id"));
				let i32_old_sentiment :i32 = get_i32_input(String::from("old sentiment"));
				let i32_new_sentiment :i32 = get_i32_input(String::from("new sentiment"));
				println!("SELECT P.type as smi_social, P.item_id as news_id, {} as sentiment, 'manual' as sentiment_type, {} as user_id	FROM project_items.project_items_{} P, imasv2.sentiment S WHERE P.project_id = {} AND S.sentiment = '{}' AND P.item_id = S.news_id",i32_new_sentiment,i32_user_id,i32_user_id,i32_project_id,i32_old_sentiment);
			},
			Choises::IdentifyVagueKeys => {
				let i32_new_sentiment = get_i32_input(String::from("новое значение для тональности"));
				let i32_user_id = get_i32_input(String::from("user_id"));
				let i32_project_id = get_i32_input(String::from("p_id"));
				println!("SELECT P.type as smi_social, P.item_id as news_id, {} as sentiment, 'manual' as sentiment_type, {} as user_id FROM project_items.project_items_{} P WHERE P.project_id={} AND P.item_id NOT IN (SELECT S.news_id FROM imasv2.sentiment S WHERE ((S.sentiment_type='manual'	AND S.smi_social=P.type AND S.user_id={}) OR (S.sentiment_type='auto' AND P.type=1 AND S.smi_social=1 AND S.news_id NOT IN(SELECT news_id FROM imasv2.sentiment WHERE sentiment_type='manual' AND user_id={} AND smi_social=1)) OR (S.sentiment_type='auto' AND P.type=2 AND S.smi_social=2 AND S.news_id NOT IN(SELECT news_id FROM imasv2.sentiment WHERE sentiment_type='manual' AND user_id={} AND smi_social=2))))", i32_new_sentiment,i32_user_id,i32_user_id,i32_project_id,i32_user_id,i32_user_id,i32_user_id);
			},
			Choises::SearchItemInBase => {
				let s_urls = get_string_input(String::from("url"));
				let mut _a_splited_urls = s_urls.split_whitespace();
				for _url in _a_splited_urls{
				}
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
			"upd_und_s"	=> {
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
