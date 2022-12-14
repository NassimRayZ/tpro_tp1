use serde::{Deserialize, Serialize};

use super::array::Array;
use super::objet::Objet;

use std::mem;
#[derive(Default, Clone, Serialize, Deserialize)]
pub(super) struct Knapsack {
    objets: Vec<Objet>,
    poids_maximal: u64,
    poids_current: u64,
    gain_total: u64,
}

impl Knapsack {
    pub(super) fn new(poids_maximal: u64) -> Self {
        Knapsack {
            objets: Vec::new(),
            poids_maximal,
            poids_current: 0,
            gain_total: 0,
        }
    }
    fn left_space(&self) -> u64 {
        self.poids_maximal - self.poids_current
    }
    fn max(left_knapsack: Knapsack, right_knapsack: Knapsack) -> Knapsack {
        if left_knapsack.gain_total > right_knapsack.gain_total {
            return left_knapsack;
        }
        right_knapsack
    }
    fn insert(&mut self, objet: Objet) {
        self.poids_current += objet.poids();
        self.gain_total += objet.gain();
        self.objets.push(objet);
    }
    pub(super) fn recursive_fill(&mut self, objets: &[Objet]) -> Knapsack {
        if (self.left_space() == 0) || (objets.is_empty()) {
            return mem::take(self);
        }
        if objets[0].poids() > self.left_space() {
            return self.recursive_fill(&objets[1..]);
        }

        let mut left_knapsack = self.clone();
        left_knapsack.insert(objets[0].clone());
        left_knapsack = left_knapsack.recursive_fill(&objets[1..]);
        Knapsack::max(left_knapsack, self.recursive_fill(&objets[1..]))
    }
    //
    pub(super) fn dynamic_fill(&mut self, objets: &[Objet]) -> Knapsack {
        let width = (self.poids_maximal + 1) as usize;
        let height = objets.len() + 1;
        let mut grid = Array::new(width, height);
        for num_objet in 1..height {
            let objet = &objets[num_objet - 1];
            for poids in 0..width {
                let do_not_take = grid.get(num_objet - 1, poids).clone();

                let do_take = if objet.poids() as usize <= poids {
                    let mut knapsack_temp = grid
                        .get(num_objet - 1, poids - objet.poids() as usize)
                        .clone();
                    knapsack_temp.insert(objet.clone());
                    knapsack_temp
                } else {
                    Knapsack::new(self.poids_maximal)
                };

                let gain_optimal = Knapsack::max(do_not_take, do_take);
                grid.set(num_objet, poids, gain_optimal)
            }
        }
        grid.last()
    }
}
#[test]
fn knapsack_recursive_test() {
    let mut knapsack = Knapsack::new(100);
    let objets = vec![
        Objet::new("coffee", 30, 120),
        Objet::new("cof", 10, 60),
        Objet::new("cof", 20, 100),
        Objet::new("cof", 15, 80),
        Objet::new("cof", 10, 90),
    ];
    let knap = knapsack.recursive_fill(&objets);

    assert_eq!(knap.left_space(), 450);
}
