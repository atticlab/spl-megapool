

use spl_math::precise_number::PreciseNumber;


fn amount_of_b_for_ab<N:Into<u128>>(liquidity_pool_ab_b_total:N, provided_liquidity_pool_ab:N, liquidity_pool_ab_total:N) -> Option<PreciseNumber> {
    let liquidity_pool_ab_b_total = PreciseNumber::new(liquidity_pool_ab_b_total.into())?;
    let provided_liquidity_pool_ab = PreciseNumber::new(liquidity_pool_ab_b_total.into())?;
    let liquidity_pool_ab_total = PreciseNumber::new(liquidity_pool_ab_b_total.into())?;
    liquidity_pool_ab_b_total.checked_mul(&provided_liquidity_pool_ab)?.checked_div(liquidity_pool_ab_total)
}


#[cfg(test)]
pub mod test {
    use super::*;


    #[test]
    pub fn simple_amount_of_b_for_ab() {
        assert!(amount_of_b_for_ab(10, 5, 20).unwrap());
    }
}

// Pool initialization accepts `LP_AB_EXACT` amount and `LP_BC_MAX` amount. Then the pool will use the following formulas to calculate exact LP_BC amount:

// `LP_AB_B = LP_AB_B_TOTAL * (LP_AB_EXACT / LP_AB_TOTAL)`

// Where:
//  - LP_AB_B is amount of token B the same in value as LP_AB amount in AB swap liquidity tokens
//  - LP_AB is exact amount of AB liquidity tokens user wants to put in the new pool
//  - LP_AB_B_TOTAL is total amount of B tokens in AB swap liquidity  pool
//  - LP_AB_TOTAL is total minted amount of AB swap liquidity tokens

// `LP_BC = LP_BC_TOTAL * (LP_AB_B / LP_BC_B_TOTAL)`

// Where:
//  - LP_BC is exact amount of BC swap LP tokens required to create megaswap in correct LP token ratio
//  - LP_AB_B is amount of token B calculated in the previous step
//  - LP_BC_TOTAL is total minted amount of BC swap liquidity tokens
//  - LP_BC_B_TOTAL is total amount of token B in BC swap liquidity pool