extern crate accountant;
extern crate csv;
extern crate rustc_serialize;

fn main() {
    let income = 85_000;
    println!("Taxable income: US${}", income);

    let profile = accountant::TaxProfile {
        filing_status: "single_filers",
        state: "CA",
        taxable_income: income,
    };

    let federal_rate = accountant::federal_income_tax::rate(&profile);
    match federal_rate {
        Some(rate) => println!("Federal income tax: {}%, US${}",
                               rate * 100.0,
                               rate * income as f32),
        None => (),
    };

    let state_rate = accountant::state_income_tax::rate(&profile);
    match state_rate {
        Some(rate) => println!("State income tax: {}%, US${}",
                               rate * 100.0,
                               rate * income as f32),
        None => (),
    };
}
