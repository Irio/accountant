pub struct TaxProfile<'a> {
    pub filing_status: &'a str,
    pub taxable_income: u32,
    pub state: &'a str,
}

#[derive(Clone, RustcDecodable, Debug)]
pub struct StateIncomeTaxBracket {
    pub state: String,
    pub filling_status: String,
    pub income_limit: u32,
    pub rate: f32,
    pub tax_previous_bracket: f32,
}
