# Investment Portfolio Balancer

View transactions needed to rebalance an investment portfolio.

## Examples

70/30 portfolio currently containing assets worth $5,000 each.

    $ invb -a 70 30 -c 5000 5000
    ---------------------------------------
    | # | Original |    Final |    Change |
    ---------------------------------------
    | 1 | 5,000.00 | 7,000.00 |  2,000.00 |
    | 2 | 5,000.00 | 3,000.00 | -2,000.00 |
    ---------------------------------------
<br />

70/30 portfolio currently containing assets worth $5,000 each. Add $1,234.56 to the portfolio.

    $ invb -a 70 30 -c 5000 5000 -d 1234.56
    ---------------------------------------
    | # | Original |    Final |    Change |
    ---------------------------------------
    | 1 | 5,000.00 | 7,864.19 |  2,864.19 |
    | 2 | 5,000.00 | 3,370.37 | -1,629.63 |
    ---------------------------------------
<br />

70/30 portfolio currently containing assets worth $5,000 each. Allocation arguments stored as environment variable `INVB_ALLOC='70 30'`.

    $ invb -c 5000 5000
    ---------------------------------------
    | # | Original |    Final |    Change |
    ---------------------------------------
    | 1 | 5,000.00 | 7,000.00 |  2,000.00 |
    | 2 | 5,000.00 | 3,000.00 | -2,000.00 |
    ---------------------------------------
<br />