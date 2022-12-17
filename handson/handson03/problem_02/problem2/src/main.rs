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

    fn patriotic_selections(n: usize, colors: &[char]) -> usize {
        let mut selections = 0;

        //find first green house, last red house, first white house, last white house and first x house and last x house
        let mut first_red = 0;
        let mut last_green = 0;
        let mut first_white = 0;
        let mut last_white = 0;
        for i in 0..n {
            if colors[i] == 'R' {
                first_red = i;
                break;
            }
            if colors[i] == 'X' {
                first_red = i;
                break;
            }
        }
        for i in (0..n).rev() {
            if colors[i] == 'G' {
                last_green = i;
                break;
            }
            if colors[i] == 'X' {
                last_green = i;
                break;
            }
        }
        for i in first_red..n {
            if colors[i] == 'W' {
                first_white = i;
                break;
            }
        }
        for i in (0..last_green).rev() {
            if colors[i] == 'W' {
                last_white = i;
                break;
            }
        }
        println!("first red: {}", first_red);
        println!("last green: {}", last_green);
        println!("first white: {}", first_white);
        println!("last white: {}", last_white);
        println!("------------------");
        let mut xbetween = 0;
        //count white houses and X houses between first red and last green
        println!("look for white houses");
        let mut xbetweenw = 0;
        let mut selectionsw = 0;
        for i in first_red..last_green {
            println!("W i: {}", i);
            if colors[i] == 'W' {
                selectionsw += 1;
            }
            if colors[i] == 'X' {
                xbetweenw += 1;
            }
            println!("count W xbetween: {}", xbetween);
            println!("Found W: {}", selectionsw);
        }
        if selectionsw > 0 {
            xbetween += xbetweenw;
        } else if xbetweenw == 1 {
            selections += 1;
        }
        selections += selectionsw;

        //count green houses after first white
        println!("look for green houses");
        let mut xbetweeng = 0;
        let mut selectionsg = 0;
        for i in first_white..n {
            println!("G i: {}", i);
            if colors[i] == 'G' {
                selectionsg += 1;
            }
            if colors[i] == 'X' {
                xbetweeng += 1;
            }
            println!("count G xbetween: {}", xbetweeng);
            println!("Found G: {}", selectionsg);
        }

        if selectionsg > 0 {
            xbetween += xbetweeng;
        } else if xbetweeng == 1 {
            selections += 1;
        }
        selections += selectionsg;
        println!("Found after G: {}", selections);

        //count red houses before last white
        println!("look for red houses");
        let mut xbetweenr = 0;
        let mut selectionsr = 0;
        for i in 0..last_white {
            println!("R i: {}", i);
            if colors[i] == 'R' {
                selectionsr += 1;
            }
            if colors[i] == 'X' {
                xbetweenr += 1;
            }
            println!("count R xbetween: {}", xbetweenr);
            println!("Found R: {}", selectionsr);
        }
        if selectionsr > 0 {
            xbetween += xbetweenr;
        } else if xbetweenr == 1 {
            selections += 1;
        }
        selections += selectionsr;

        println!("Found after r: {}", selections);
        if xbetween > 0 {
            selections += 3_usize.pow(xbetween);
        }
        println!("------------------");
        selections
    }
}
