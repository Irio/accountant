mod structs;
pub use structs::*;

pub mod federal_income_tax {
    const TAX_BRACKETS: [f32; 7] = [0.1, 0.15, 0.25, 0.28, 0.33, 0.35, 0.396];

    pub fn rate(profile: ::TaxProfile) -> Option<f32> {
        let income_ceilings = match profile.filing_status {
            "single_filers" =>
                [9_075, 36_900, 89_350, 186_350, 405_100, 406_750],
            "married_filing_jointly" | "qualified_widow" =>
                [18_150, 73_800, 148_850, 226_850, 405_100, 457_600],
            "head_of_household" =>
                [12_950, 49_400, 127_550, 206_600, 405_100, 432_200],
            "married_filing_separately" =>
                [9_075, 36_900, 74_425, 113_425, 202_550, 228_800],
            _ => return None,
        };

        let ceiling_for_income =
            income_ceilings.iter().find(|&ceiling| profile.taxable_income <= *ceiling);
        let index = match ceiling_for_income {
            Some(value) => income_ceilings.iter().position(|x| x == value),
            None => Some(TAX_BRACKETS.len() - 1),
        };

        return index.and_then(|value| Some(TAX_BRACKETS[value]));
    }
}
