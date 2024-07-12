
slint::include_modules!();

fn is_first_number(string: &String) -> bool{
    if *string == "0"{
       return true;
    }
    false
}
fn number_typed(typed_string: &String, input: &String) -> String{
    let mut formatted_string = String::new();
    match typed_string.as_str() {
        "0" |"1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
            if is_first_number(&input){
                return typed_string.clone();
            }else{
                formatted_string = format!("{}{}",input, typed_string);
                return formatted_string;
            }
        }
        "." | "+" | "/" | "-" | "x" =>{
            formatted_string = format!("{}{}",input, typed_string);
            return formatted_string;
        }
        "delete" =>{
            formatted_string = input.clone();
            formatted_string.pop();
            if formatted_string == ""{
                formatted_string = "0".to_string();
                return formatted_string;
            }
            formatted_string
        }
         _ => input.clone() 
}

}

fn main() -> Result<(), slint::PlatformError> {
    
    let ui = Calc::new()?;
    let ui_handle = ui.as_weak();

   // fn de resultado 
    ui.on_result_clicked(move |string| {
        let ui = ui_handle.unwrap();
        // transformando a string em int
        let _num: f64 = match string.trim().parse() {
            Ok(parsed_num) => { 
                let result = format!("{}", parsed_num);
                ui.set_result(result.into());
                parsed_num 
            },
            Err(_) => {
                let erro = format!("A variavel \"{}\" é desconhecida.", string.trim());
                ui.set_result(erro.into());
                return;
            }
        };
    });
    
    let ui_handle = ui.as_weak();
    // fn dos botoes
    ui.global::<Button_comp_click>().on_button_pressed(move | typed | {
        let ui = ui_handle.unwrap();
        let input = number_typed(&typed.to_string(), &ui.get_inputo().to_string());
         ui.set_inputo(input.into());
    });

    let ui_handle = ui.as_weak();

    // Parte do teclado
    ui.on_word_typed(move |typed | {
        let ui = ui_handle.unwrap();
        let input = number_typed(&typed.to_string(), &ui.get_inputo().to_string());
        ui.set_inputo(input.into());
    });

    ui.run()
}
