# Logic
Megaswap accepts (as liquidity) LP tokens of the overlapping token pairs (for example, AB and BC).

If Megaswap already exists it uses the same formulas as token swap, the process of swapping from A to C is:

1. A -> LP_AB (using AB token swap)
2. LP_AB -> LP_BC (using megaswap itself, but with the same calculators as the regular token swap, using LP_AB and LP_BC pool sizes as the basis for the price)
3. LP_BC -> C (using BC token swap)

The only major difference is in calculating initial ratio of LP tokens in the newly created pool. It is possible to allow any ratio on the initial pool creation and let the market decide the actual exchange rate, but it is preferred to set it to the proper ratio from the start.

So the pool initialization should accept LP_AB EXACT amount and LP_BC MAX amount. Then the pool will use the following formulas to calculate exact LP_BC amount:

`LP_AB_B = LP_AB * LP_AB_B_TOTAL / LP_AB_TOTAL`

Where:
 - LP_AB_B is amount of token B the same in value as LP_AB amount in AB swap liquidity tokens
 - LP_AB is exact amount of AB liquidity tokens user wants to put in the new pool
 - LP_AB_B_TOTAL is total amount of B tokens in AB swap liquidity  pool
- LP_AB_TOTAL is total minted amount of AB swap liquidity tokens

`LP_BC = LP_AB_B * LP_BC_TOTAL / LP_BC_B_TOTAL`

Where:
 - LP_BC is exact amount of BC swap LP tokens required to create megaswap in correct LP token ratio
 - LP_AB_B is amount of token B calculated in the previous step
 - LP_BC_TOTAL is total minted amount of BC swap liquidity tokens
 - LP_BC_B_TOTAL is total amount of token B in BC swap liquidity pool

This math is of course valid for constant product curve only.

# Implementation
We think the best approach will be to build an interface contract which will not contain any token swap logic and call existing token swap program for all the swapping. To start with it can contain 2 instructions:

- InitPool, where user will send LP_AB and LP_BC tokens from the example above and the contract will use its logic to calculate exact amounts for the initial ratio and then call regular token swap init
- Swap, where user will send token A and C and the program will do the sequence of instructions to the underlying token swaps to execute the complex swap

# Other curves
The math above is valid for the constant product curve only. We need to decide if we can use this initial math for the other curves. After all it is simply for initial ratio calculation and uses calculation “in the moment” without any slippage, so it does not matter which curve we use (and constant product is the simplest one).

