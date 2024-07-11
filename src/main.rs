use slint::format;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = Calc::new()?;
    let ui_handle = ui.as_weak();
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

    ui.run()
}
