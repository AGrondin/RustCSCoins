use mersenne_twister::MersenneTwister;
use rand::{Rng, SeedableRng};
use std::mem;
use std::marker::Copy;
use std::u32;
use std::cmp::Ordering;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use byteorder::{ByteOrder, LittleEndian};
use itertools::Itertools;

mod shortest_path;

##[derive(Debug)]
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


    //Returns a number of u64, for the sorted list challenges
    pub fn get_number_list<'a>(&'a mut self, numInts: usize) -> Vec<u64>
    {
        let mut output=Vec::with_capacity(numInts);

        for x in 0..numInts {
            output.push(self.rng.next_u64());
        }

        return output;
    }

    pub fn sort_list<'a>(&'a mut self, mut numbers: Vec<u64>)->String
    {
        numbers.sort_by(|a,b| a.cmp(b));

        let results: String= numbers.clone().iter().join("");

        return results;
    }


    pub fn reverse_sort_list<'a>(&'a mut self, mut numbers: Vec<u64>) -> String
    {
        numbers.sort_by(|a,b| b.cmp(a));

        let results: String= numbers.clone().iter().join("");

        return results;

    }

    //Passes the concatenation of the last_solution hash and the nonce, and plugs the result into a u64
    pub fn get_seed(&mut self, last_solution:String, nonce:u64)->u64
    {
        let mut new_seed:[u8;32]=[0;32];

        self.hasher.reset();

        self.hasher.input_str(&(format!("{}{}", last_solution, nonce.to_string())));

        self.hasher.result(&mut new_seed);

        let seed: u64 = LittleEndian::read_u64(&new_seed[0..8]);

        return seed;

    }

    //Solves the sorted list challenge, and returns a tuple (hash, nonce)
    pub fn sorted_list_challenge(&mut self, last_solution:String, numInts:u64) -> (String,u64)
    {

        let nonce = self.rng.next_u64();

        let seed = self.get_seed(last_solution, nonce);

        self.rng.reseed(seed);

        let mut numbers = self.get_number_list(x as usize);

        let mut concat_string:String = self.sort_list(numbers);

        self.hasher.reset();

        self.hasher.input_str(&concat_string);

        let hash_res=self.hasher.result_str();

        return (hash_res, nonce);
        //  if &(res_digest.as_bytes())[0..4] == prefix.as_bytes();{
        //     return Some((hash_res,nonce));
        // };
        //
        // None
    }

    pub fn reverse_challenge(&mut self, last_solution:String, numInts:u64) -> (String,u64)
    {

        let nonce = self.rng.next_u64();

        let seed = self.get_seed(last_solution, nonce);

        self.rng.reseed(seed);

        let mut numbers = self.get_number_list(x as usize);

        let mut concat_string:String = self.reverse_sort_list(numbers);

        self.hasher.reset();

        self.hasher.input_str(&concat_string);

        let hash_res=self.hasher.result_str();

        return (hash_res, nonce);
        //  if &(res_digest.as_bytes())[0..4] == prefix.as_bytes();{
        //     return Some((hash_res,nonce));
        // };
        //
        // None
    }

    pub fn solve_shortest_path(&mut self, last_solution:String, size:usize, num_blockers: usize) -> (String,u64)
    {
        //TODO: Place the following into reseed with nonce function

        let nonce = self.rng.next_u64();

        let seed=self.get_seed(last_solution, nonce);

        //prefix.clone_from_slice(&new_seed[0..8]);

        //let mut seed: u64 = ByteOrder::read_u64(prefix);
        //

        self.rng.reseed(seed);

        let new_grid = grid::new(size, num_blockers);

        new_grid.populate(&mut self.rng);

        if let Some(solution) = a_star(new_grid){
            let (came_from, cost) = solution;
            let solution_string = reconstruct_path(new_grid, came_from, cost);
        } else {
            continue;
        }

        self.hasher.reset();

        self.hasher.input_str(&solution_string);

        let hash_res=self.hasher.result_str();

        return (hash_res,nonce);

        //
        //  if &(res_digest.as_bytes())[0..4] == prefix.as_bytes();{
        // };
    }
}
