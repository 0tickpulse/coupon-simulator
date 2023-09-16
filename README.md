# Coupon Simulator

This is a simulation of the famous [coupon collector's problem](https://en.wikipedia.org/wiki/Coupon_collector%27s_problem). The problem is as follows:

> Given a set of $n$ coupons, each with a unique number, how many coupons do you need to draw with replacement until you have drawn each coupon at least once?

The answer is $nH_n$, where $H_n$ is the $n$ th [harmonic number](https://en.wikipedia.org/wiki/Harmonic_number). This simulation will run the experiment for a given number of coupons and write the average number of draws to the standard output.

## Usage

```bash
cargo run
```

## Explanation

The mathematical simulation lies in the trial function. The function creates an array called `collected`, which has `N` elements, each being `false`. Each element in this array represents whether that element has been collected. While any of the elements are `false`, it will keep generating a random number between 0 and `N` and setting the corresponding element in `collected` to `true`. Once all elements are `true`, it will return the number of draws it took to collect all coupons, completing one trial of the simulation.

The rest of the program is just a loop that creates a few threads and runs the trial function in each thread. It then takes the average of the results and prints it to the standard output.
