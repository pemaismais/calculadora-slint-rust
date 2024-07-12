slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    
    let ui = Calc::new()?;
    let ui_handle = ui.as_weak();

   // fn de resultado 
    ui.on_result_clicked(move |string| {
        let ui = ui_handle.unwrap();
        // transformando a string em int
        let _num: i64 = match string.trim().parse() {
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
    ui.global::<Button_comp_click>().on_button_pressed(move | string | {
        let ui = ui_handle.unwrap();
        let input: String = format!("{}{}",ui.get_inputo(),string);
         ui.set_inputo(input.into());
    });

    let ui_handle = ui.as_weak();
    // Parte do teclado
    ui.on_word_typed(move |typed | {
        let ui = ui_handle.unwrap();
        match typed.as_str() {
            "0" |"1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" |  "." | "+" | "/" | "-" => {
                let input: String = format!("{}{}",ui.get_inputo(),typed);
                ui.set_inputo(input.into());
            }
            "delete" =>{
                let mut input: String = ui.get_inputo().to_string();
                input.pop();
                ui.set_inputo(input.into());
            }
             _ => println!("{typed}")
        }
    });

    ui.run()
}
