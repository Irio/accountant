pub struct TaxProfile<'a> {
    pub filing_status: &'a str,
    pub taxable_income: u32,
    pub state: &'a str,
}
