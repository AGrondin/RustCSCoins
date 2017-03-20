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


pub struct miner{
    rng: MersenneTwister,
    lastSeed: u64,
    hasher: Sha256
}

pub struct challenge{
    name: String,
    numInts: Option<u32>
}

impl miner{

    pub fn new() -> miner {
        miner{
            rng: SeedableRng::from_seed(0),
            lastSeed: 0,
            hasher: Sha256::new()
        }
    }

    pub fn getMersOutput<'a>(&'a mut self, seed: u64, numInts: usize) -> Vec<u64> {
        self.rng.reseed(seed);
        let mut output=Vec::with_capacity(numInts);

        for x in 0..numInts {
            output.push(self.rng.next_u64());
        }

        return output;
    }

    pub fn sortList<'a>(&'a mut self, mut numbers: Vec<u64>)->String{
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



    pub fn findNonce(&mut self, lastSolution:String, target:&[u8], chal:challenge) -> [u8;8]
    {

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

        let mut concat_string:String =match chal.name.as_ref(){
            "reverse-order" => {
                match chal.numInts{
                    Some(x)=>{
                        let mut numbers = self.getMersOutput(seed, x as usize);
                        self.reverseSortList(numbers)
                    }
                    None =>"".to_string()
                }

            },
            "sorted-list" => {
                match chal.numInts{
                    Some(x)=>{
                        let mut numbers = self.getMersOutput(seed, x as usize);
                        self.sortList(numbers)
                    }
                    None =>"".to_string()
                }
            }
            _ => "".to_string()

        };

        let mut hash_res:[u8;32]=[0;32];

        self.hasher.reset();

        self.hasher.input_str(&concat_string);

        self.hasher.result(&mut hash_res);

        let mut ret_nonce:[u8;8]=[0;8];

        if &hash_res[0..2]==target{
            //ByteOrder::write_u64(&mut ret_nonce, nonce);
            return ret_nonce;
        };

        //Pass mutated string as input for hasher.input_str

        //Check if prefix of hash and target are equal. End loop if true

        //Return thus obtained nonce
        return [0;8]

    }

}
