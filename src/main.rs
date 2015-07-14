extern crate accountant;

fn main() {
    let income = 85_000;
    println!("Taxable income: US${}", income);
    let federal_rate = accountant::federal_income_tax::rate("single_filers", income);
    match federal_rate {
        Some(rate) => println!("Federal income tax: {}%, US${}",
                               rate * 100 as f32,
                               rate * income as f32),
        None => (),
    };
}
