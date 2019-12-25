use crate::aoc::utils;

pub fn day_twenty_two(input: String) -> () {
    let decksize = 10007;
    let mut deck: Vec<usize> = vec![0; decksize];
    let mut i = 0;
    while i < decksize {
        deck[i] = i;
        i += 1;
    }
    // println!("{:?}", deck);
    for line in input.split("\n") {
        if line.chars().nth(0).unwrap() == 'c' {
            let mut n = utils::parse_int(&line[4..]);
            if n < 0 {
                n = decksize as i32 + n;
            }
            // println!("cut {}", n);
            cut(&mut deck, n as usize);
        } else if line.chars().nth(5).unwrap() == 'w' {
            let n = utils::parse_int(&line[20..]);
            deck = deal(&mut deck, n as usize, decksize);
            // println!("deal with {}", n);
        } else {
            restack(&mut deck);
            // println!("deal into");
        }
    }
    let index = deck.iter().position(|&c| c == 2019).unwrap();
    println!("2019: {}", index);

    // work backwards, unwinding the position
    let pos: i64 = 2020;
    let bigdeck: i64 = 119315717514047;
    let shuffles: i64 = 101741582076661;
    let mut a: i64 = 0;
    let mut b: i64 = 0;
    let moves: Vec<&str> = input.split("\n").collect();
    for m in moves {
        if m.chars().nth(0).unwrap() == 'c' {
            let mut n = utils::parse_int_64(&m[4..]);
            if n < 0 {
                n = bigdeck + n;
            }
            // cut
            b = (b - n) % bigdeck;
        } else if m.chars().nth(5).unwrap() == 'w' {
            // deal with
            let n = utils::parse_int_64(&m[20..]);
            a = (a * n) % bigdeck;
            b = (b * n) % bigdeck;
        } else {
            // reverse
            a = (bigdeck - a) % bigdeck;
            b = (bigdeck - 1 - b) % bigdeck;
        }
    }
    let r = (b * ((1-a).pow((bigdeck - 2) as u32) % bigdeck)) % bigdeck;
    let idx = (((pos - r) * (a.pow((shuffles * (bigdeck - 2)) as u32) % bigdeck)) + r) % bigdeck;
    println!("2020: {}", idx);
}

fn restack(deck: &mut Vec<usize>) -> () {
    deck.reverse();
}

fn cut(deck: &mut Vec<usize>, n: usize) -> () {
    let mut i = 0;
    while i < n {
        let el = deck.remove(0);
        deck.push(el);
        i += 1;
    }
}

fn deal(deck: &mut Vec<usize>, n: usize, len: usize) -> Vec<usize> {
    let mut table: Vec<usize> = vec![0; len];
    let mut i = 0;
    while i < len {
        let pos = (i * n) % len;
        let x = deck.remove(0);;
        table[pos] = x;
        i += 1;
    }
    return table;
}
