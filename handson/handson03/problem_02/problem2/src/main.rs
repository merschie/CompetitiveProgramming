use std::io;

fn main() {
    // Read in the first line
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Read in the second line
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let h: Vec<char> = input.trim().chars().collect();

    // Call the function and print the result
    println!("{:?}", h);
    println!("{}", patriotic_selections(n, &h));
}

fn patriotic_selections(n: usize, colors: &[char]) -> usize {
    //create vector of all possible combinations
    let mut whites = Vec::new();


    let mut selections = 0;
    let mut redfound = false;
    let mut red = 0;
    let mut red_multiplier = 0;
    let mut white = 0;
    let mut white_multiplier = 0;
    let mut green = 0;
    let mut x_multiplier = 0;
    let mut x_replace_white = 0;
    let mut finished = false;

    for i in 0..n {
        if colors[i] == 'R' {
            println!("Found red");
            red = i;
            redfound = true;
            red_multiplier += 1;
        }
        if colors[i] == 'W' && redfound {
            println!("Found white");
            white = i;
            white_multiplier += 1;
            x_multiplier += x_replace_white;
        }
        if colors[i] == 'G' && white > 0 {
            green = i;
            println!("Found green");
            selections += red_multiplier * white_multiplier;
            if x_multiplier > 0 {
                selections += 3_usize.pow(x_multiplier as u32);
            }
            finished = true;
        }
        if colors[i] == 'X' {
            println!("Found X");
            if white > 0 {
                selections += red_multiplier * white_multiplier;
                println!(
                    "red_multiplier: {}, white_multiplier: {}",
                    red_multiplier, white_multiplier
                );

                if finished {
                    x_multiplier += 1;
                    selections += 3_usize.pow(x_multiplier as u32);
                    println!("mulllllti");
                } else {
                    white_multiplier += 1;
                    red_multiplier += 1;
                    finished = true;
                }
            } else if redfound && white_multiplier == 0 {
                white = i;
                white_multiplier += 1;
                x_replace_white += 1;
                println!("x_replace_white: {}", x_replace_white);
            } else if !redfound {
                red = i;
                redfound = true;
                red_multiplier += 1;
            }

            //x_multiplier += 1;
        }
    }
    println!("red: {}, white: {}, green: {}", red, white, green);

    selections
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_0() {
        let h = vec!['R', 'W', 'G', 'X', 'X'];
        assert_eq!(super::patriotic_selections(h.len(), &h), 16);
    }
    #[test]
    fn test_1() {
        let h = vec!['X', 'X', 'X'];
        assert_eq!(super::patriotic_selections(h.len(), &h), 1);
    }
    #[test]
    fn test_2() {
        let h = vec!['R', 'W', 'X'];
        assert_eq!(super::patriotic_selections(h.len(), &h), 1);
    }
    #[test]
    fn test_3() {
        let h = vec!['R', 'G', 'X', 'W', 'X', 'G'];
        assert_eq!(super::patriotic_selections(h.len(), &h), 24);
    }
    #[test]
    fn test_4() {
        let h = vec!['X', 'X', 'X', 'X', 'X', 'X', 'X'];
        assert_eq!(super::patriotic_selections(h.len(), &h), 2835);
    }
    #[test]
    fn test_5() {
        let h = vec!['G', 'G', 'G', 'R', 'R', 'R', 'W', 'W', 'W'];
        assert_eq!(super::patriotic_selections(h.len(), &h), 0);
    }
    #[test]
    fn test_6() {
        let h = vec!['G', 'G', 'W', 'X', 'X', 'R'];
        assert_eq!(super::patriotic_selections(h.len(), &h), 0);
    }
}
