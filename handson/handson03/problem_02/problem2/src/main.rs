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
    println!("{}", patriotic_selections(n, &h));
}

fn patriotic_selections(n: usize, colors: &[char]) -> usize {
    //create vector for elements type usize
    let mut whites = Vec::new();
    //number of patriotic selections
    let mut selections = 0;
    //number of Xs so far
    let mut xcounter = 0;
    //iterate through the colors
    for i in 0..n {
        if colors[i] == 'R' {
            //create new object in vector
            whites.push((0, 0, xcounter));
        }
        if colors[i] == 'W' {
            for x in 0..whites.len() {
                whites[x].0 += 1;
            }
        }
        if colors[i] == 'G' {
            for x in 0..whites.len() {
                //g as last x as start and w in middle
                selections += 3_usize.pow(whites[x].1 as u32 + whites[x].2 as u32) * whites[x].0;
                //use x as w
                if whites[x].1 >= 1 {
                    selections +=
                        3_usize.pow(whites[x].1 as u32 - 1 + whites[x].2 as u32) * whites[x].1;
                }
            }
        }
        if colors[i] == 'X' {
            //multiply all selections till now by 3
            selections = selections * 3;
            for x in 0..whites.len() {
                //added for X=G
                selections += 3_usize.pow(whites[x].1 as u32) * whites[x].0;
                //use previous x as w
                whites[x].1 += 1;
                if whites[x].1 >= 2 {
                    //use x as G and as w
                    selections += 3_usize.pow(whites[x].1 as u32 - 2 + whites[x].2 as u32)
                        * (whites[x].1 - 1);
                }
            }
            whites.push((0, 0, xcounter));
            xcounter += 1;
        }
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
