extern crate accountant;
mod structs;

fn main() {
    let income = 85_000;
    println!("Taxable income: US${}", income);

    let profile = structs::TaxProfile {
        filing_status: "single_filers",
        taxable_income: 85_000,
        state: "",
    };
    let federal_rate = accountant::federal_income_tax::rate(profile);
    match federal_rate {
        Some(rate) => println!("Federal income tax: {}%, US${}",
                               rate * 100 as f32,
                               rate * income as f32),
        None => (),
    };
}
