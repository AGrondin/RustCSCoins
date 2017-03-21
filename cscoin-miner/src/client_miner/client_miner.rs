use mersenne_twister::MersenneTwister;
use rand::{Rng, SeedableRng};
use std::mem;
use std::marker::Copy;
use std::u32;
use std::cmp::Ordering;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use byteorder::{ByteOrder};
use itertools::Itertools;

mod shortest_path;

pub struct miner{
    rng: MersenneTwister,
    lastSeed: u64,
    hasher: Sha256
}

impl miner{

    pub fn new() -> miner
    {
        miner{
            rng: SeedableRng::from_seed(0),
            lastSeed: 0,
            hasher: Sha256::new()
        }
    }

    pub fn getNumberList<'a>(&'a mut self, numInts: usize) -> Vec<u64>
    {
        let mut output=Vec::with_capacity(numInts);

        for x in 0..numInts {
            output.push(self.rng.next_u64());
        }

        return output;
    }

    pub fn sortList<'a>(&'a mut self, mut numbers: Vec<u64>)->String
    {
        numbers.sort_by(|a,b| a.cmp(b));

        let results: String= numbers.clone().iter().join("");

        return results;
    }


    pub fn reverseSortList<'a>(&'a mut self, mut numbers: Vec<u64>) -> String
    {
        numbers.sort_by(|a,b| b.cmp(a));

        let results: String= numbers.clone().iter().join("");

        return results;

    }



    pub fn solve_order(&mut self, lastSolution:String, target:&[u8], numInts:u64, reverse:bool) -> (String,u64)
    {

        loop{

            //TODO: Place the following into reseed with nonce function
            let mut new_seed:[u8;32]=[0;32];

            //Start loop

            //Reseed rng (use thread_rng?)

            //Generate 8 byte Nonce using rng

            let nonce = self.rng.next_u64();

            self.hasher.reset();

            self.hasher.input_str(&(format!("{}{}", lastSolution, nonce.to_string())));

            self.hasher.result(&mut new_seed);

            //Concatenate prefix and Nonce using push_into mut str (maybe as [u8], see what's faster)

            let mut seed: u64 = 16;

            //prefix.clone_from_slice(&new_seed[0..8]);

            //let mut seed: u64 = ByteOrder::read_u64(prefix);
            //

            self.rng.reseed(seed);

            let mut numbers = self.getNumberList(x as usize);

            let mut concat_string:String = if reverse {self.reverseSortList(numbers)} else {self.sortList(numbers)};

            self.hasher.reset();

            self.hasher.input_str(&concat_string);

            let hash_res=self.hasher.result_str();

            if &hash_res[0..2]==target{
                return (hash_res, nonce);
            };


        }

    }

    pub fn solve_shortest_path(&mut self, lastSolution:String, target:&[u8], size:usize, num_blockers: usize) -> (String,u64)
    {

        loop{
            //TODO: Place the following into reseed with nonce function
            let mut new_seed:[u8;32]=[0;32];

            //Start loop

            //Reseed rng (use thread_rng?)
            //Generate 8 byte Nonce using rng

            let nonce = self.rng.next_u64();


            self.hasher.reset();

            self.hasher.input_str(&(format!("{}{}", lastSolution, nonce.to_string())));

            self.hasher.result(&mut new_seed);

            //Concatenate prefix and Nonce using push_into mut str (maybe as [u8], see what's faster)

            let mut seed: u64 = 16;

            //prefix.clone_from_slice(&new_seed[0..8]);

            //let mut seed: u64 = ByteOrder::read_u64(prefix);


            self.rng.reseed(seed);

            let new_grid = grid::new(size, num_blockers);

            new_grid.populate(&mut self.rng);

            if let Some(solution) = dijsktra(new_grid){
                let (came_from, cost) = solution;
                let solution_string = reconstructPath(new_grid, came_from, cost);
            } else {
                continue;
            }

            self.hasher.reset();

            self.hasher.input_str(&solution_string);

            let hash_res=self.hasher.result_str();

            if &hash_res[0..2]==target{
                return (hash_res,nonce);
            };

        }

    }
}
