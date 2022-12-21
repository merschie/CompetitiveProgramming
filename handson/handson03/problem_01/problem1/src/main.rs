//import the io library
use std::io;

fn main() {
    // Read the input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let d: usize = parts.next().unwrap().parse().unwrap();

    // Read the itineraries
    let mut itineraries = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut attractions = Vec::new();
        let mut parts = input.trim().split_whitespace();
        for _ in 0..d {
            let attraction: usize = parts.next().unwrap().parse().unwrap();
            attractions.push(attraction);
        }
        itineraries.push(attractions);
    }

    fn maximum_attractions(n: usize, d: usize, attractions: Vec<Vec<usize>>) -> usize {
        // Initialize dp array with zeros
        let mut dp = vec![vec![0; d]; n];
        let mut olddp = vec![vec![0; d]; n];

        // Initialize first column of dp with values from input array
        dp[0][0] = attractions[0][0];
        for i in 0..n {
            dp[i][0] = attractions[i][0];
            olddp[i][0] = attractions[i][0];
        }
        println!("{:?}", dp);
        // Sum all rows
        for i in 0..n {
            for j in 1..d {
                dp[i][j] = attractions[i][j] + dp[i][j - 1];
                olddp[i][j] = attractions[i][j] + dp[i][j - 1];
            }
        }

        //Knapsack step modified
        for j in 0..d {
            for i in 1..n {
                if j == 0 {
                    dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j]);
                } else {
                    let mut max = 0;
                    for x in 0..j {
                        max = std::cmp::max(dp[i - 1][x] + olddp[i][j - x - 1], max);
                    }
                    max = std::cmp::max(dp[i - 1][j], max);
                    dp[i][j] = std::cmp::max(max, dp[i][j]);
                }
            }
        }
        // Return maximum value in last column of dp
        dp.iter().map(|row| row[d - 1]).max().unwrap()
    }

    // Print the result
    println!("{}", maximum_attractions(n, d, itineraries));
}
