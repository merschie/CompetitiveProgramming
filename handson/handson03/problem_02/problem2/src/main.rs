use std::io;

fn main() {
    // Read in the first line
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Read in the second line
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let H: Vec<char> = input.trim().chars().collect();

    // Call the function and print the result
    println!("{:?}", H);
    println!("{}", patriotic_selections(n, &H));

    fn patriotic_selections(n: usize, colors: &[char]) -> usize {
        let mut dp = vec![vec![vec![0; 3]; 3]; n];

        let mut selections = 0;

        //find first green house
        let mut first_green = 0;
        for i in 0..n {
            if colors[i] == 'G' {
                first_green = i;
                break;
            }
        }
        //find last red house
        let mut last_red = 0;
        for i in (0..n).rev() {
            if colors[i] == 'R' {
                last_red = i;
                break;
            }
        }

        //count white houses between first green and last red
        for i in first_green..last_red {
            if colors[i] == 'W' {
                selections += 1;
            }
        }

        //find last white house before last red
        let mut last_white = 0;
        for i in (0..last_red).rev() {
            if colors[i] == 'W' {
                last_white = i;
                break;
            }
        }

        //count green houses before last white
        for i in 0..last_white {
            if colors[i] == 'G' {
                selections += 1;
            }
        }

        //find first white house after first green
        let mut first_white = 0;
        for i in first_green..n {
            if colors[i] == 'W' {
                first_white = i;
                break;
            }
        }

        //count red houses after first white
        for i in first_white..n {
            if colors[i] == 'R' {
                selections += 1;
            }
        }
        selections
    }
}

// The time complexity of this solution is O(n) because we are only looping through the input array H once.

// For each value of i in the outer loop, we are looping through all possible values of j, r, and w, which takes O(n) time. However, since we only loop through the outer loop once, the overall time complexity is O(n).

// This is because the number of iterations in the inner loops is independent of the size of the input. The size of the input only affects the outer loop, which runs in linear time.

// Therefore, the overall time complexity of this solution is O(n).
