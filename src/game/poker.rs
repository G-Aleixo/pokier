use std::collections::HashMap;

use crate::game::{Rank, Suit, Card};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandStrength {
    HighCard(Vec<Rank>),
    OnePair(Rank, Vec<Rank>),
    TwoPair(Rank, Rank, Rank),
    ThreeOfAKind(Rank, Vec<Rank>),
    Straight(Rank),
    Flush(Vec<Rank>),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank, Rank),
    StraightFlush(Rank),
}


pub fn evaluate_hand(cards: &[Card]) -> HandStrength {
    use HandStrength::*;

    // count ranks
    let mut counts: HashMap<Rank, usize> = HashMap::new();
    for c in cards {
        *counts.entry(c.rank).or_insert(0) += 1;
    }

    let mut rank_groups: Vec<(Rank, usize)> = counts.into_iter().collect();
    rank_groups.sort_by(|a, b| b.1.cmp(&a.1).then(b.0.cmp(&a.0)));

    // flush
    let mut suit_counts: HashMap<Suit, Vec<Rank>> = HashMap::new();
    for c in cards {
        suit_counts.entry(c.suit).or_default().push(c.rank);
    }
    let flush = suit_counts.iter()
        .find(|(_, ranks)| ranks.len() >= 5)
        .map(|(_, ranks)| {
            let mut r = ranks.clone();
            r.sort();
            r.reverse();
            r
        });

    // straight
    let mut ranks: Vec<_> = cards.iter().map(|c| c.rank).collect();
    ranks.sort();
    ranks.dedup();
    let straight = find_straight(&ranks);

    // straight flush
    let straight_flush = flush.as_ref().and_then(|franks| {
        find_straight(franks)
    });

    if let Some(high) = straight_flush {
        return StraightFlush(high);
    }

    match rank_groups.as_slice() {
        [(r,4),(k,_),..] => FourOfAKind(*r,*k),
        [(r3,3),(r2,2),..] => FullHouse(*r3,*r2),
        [(r3,3),..] => {
            let kickers: Vec<_> = rank_groups.iter().filter(|(_,c)|*c==1).map(|(r,_)|*r).collect();
            ThreeOfAKind(*r3,kickers)
        }
        [(r2a,2),(r2b,2),(k,_),..] => TwoPair(*r2a,*r2b,*k),
        [(r2,2),..] => {
            let kickers: Vec<_> = rank_groups.iter().filter(|(_,c)|*c==1).map(|(r,_)|*r).collect();
            OnePair(*r2,kickers)
        }
        _ => {
            if let Some(ranks) = flush {
                Flush(ranks)
            } else if let Some(high) = straight {
                Straight(high)
            } else {
                let mut all: Vec<_> = cards.iter().map(|c| c.rank).collect();
                all.sort();
                all.reverse();
                HighCard(all)
            }
        }
    }
}

// highest straight
fn find_straight(ranks: &[Rank]) -> Option<Rank> {
    use Rank::*;
    let values: Vec<u8> = ranks.iter().map(|r| match r {
        Two=>2,Three=>3,Four=>4,Five=>5,Six=>6,Seven=>7,Eight=>8,Nine=>9,Ten=>10,
        Jack=>11,Queen=>12,King=>13,Ace=>14
    }).collect();

    for w in values.windows(5) {
        if w[4]-w[0]==4 && w.iter().zip(w[0]..=w[4]).all(|(a,b)|*a==b) {
            return Some(num_to_rank(w[4]));
        }
    }
    // A 2 3 4 5
    if values.contains(&14) && values.contains(&2) && values.contains(&3) && values.contains(&4) && values.contains(&5) {
        return Some(Five);
    }
    None
}

fn num_to_rank(n:u8) -> Rank {
    use Rank::*;
    match n {
        2=>Two,3=>Three,4=>Four,5=>Five,6=>Six,7=>Seven,8=>Eight,9=>Nine,10=>Ten,
        11=>Jack,12=>Queen,13=>King,14=>Ace,_=>Two
    }
}