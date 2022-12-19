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
    //create vector for elements type usize
    let mut xcounter = 0;
    let mut whites = Vec::new();
    let mut selections = 0;
    for i in 0..n {
        println!("\x1b[93mstep {}\x1b[0m", i);
        if colors[i] == 'R' {
            println!("Found red");
            whites.push((i, 0, 0, xcounter));
        }
        if colors[i] == 'W' {
            println!("Found white");
            for x in 0..whites.len() {
                whites[x].1 += 1;
            }
        }
        if colors[i] == 'G' {
            println!("Found green");
            for x in 0..whites.len() {
                println!(
                    "added {}",
                    3_usize.pow(whites[x].2 as u32 + whites[x].3 as u32) * whites[x].1
                );
                //println!("exponent: {} for x: {}", whites[x].2 + whites[x].3, x);
                selections += 3_usize.pow(whites[x].2 as u32 + whites[x].3 as u32) * whites[x].1;
                //whites[x].3 += 1;
            }
        }
        if colors[i] == 'X' {
            println!("Found X");
            //multiply all selections till now by 3
            println!("added {}", selections * 2);
            selections = selections * 3;
            for x in 0..whites.len() {
                //added for X=G
                selections += 3_usize.pow(whites[x].2 as u32) * whites[x].1;
                //use previous x as w
                whites[x].2 += 1;
                if whites[x].1 > 0 && whites[x].2 == 0 {
                    //use x as G and there is a w
                    println!("use x as G and there is a w");
                    selections +=
                        3_usize.pow(whites[x].2 as u32 - 1 + whites[x].3 as u32) * whites[x].1;
                } else if whites[x].2 >= 2 {
                    //use x as G and as w
                    println!("use x as G and as w");
                    selections += 3_usize.pow(whites[x].2 as u32 - 2 + whites[x].3 as u32)
                        * (whites[x].2 - 1);
                }
                println!("whites[2]: {}", whites[x].2);
                //add x to data
            }
            whites.push((i, 0, 0, xcounter));
            xcounter += 1;
        }
        println!("{:?}", whites);
    }

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
