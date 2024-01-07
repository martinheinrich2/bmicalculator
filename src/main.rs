slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_calculate_bmi(move |weight, height| {
        let ui = ui_handle.unwrap();

        let is_num = weight.trim().replace(',', ".").parse::<f64>().is_ok();
        let is_num2 = height.trim().replace(',', ".").parse::<f64>().is_ok();
        if is_num == true && is_num2 == true {
            let weight: f64 = weight.trim().replace(',', ".").parse().unwrap();
            let height: f64 = height.trim().replace(',', ".").parse().unwrap();
            let bmi: f64 = weight / (height * height);
            let result: String = format!("Your BMI is: {:.2}", bmi);
            ui.set_results(result.into())
        } else {
            let result: String = format! {"Please enter a number!"};
            ui.set_results(result.into())
        }
    });

    ui.run()
}
