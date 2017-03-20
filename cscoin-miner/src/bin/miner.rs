extern crate mersenne_twister;
extern crate crypto;
extern crate rand;
extern crate byteorder;
use mersenne_twister::MersenneTwister;
use rand::{Rng, SeedableRng};
use std::mem;
use std::u32;
use std::cmp::Ordering;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use byteorder::{ByteOrder};


struct miner{
    rng: MersenneTwister,
    lastSeed: u64,
    hasher: Sha256
}

struct challenge{
    name: String,
    numInts: Option<u32>
}

fn reverse(a:&u64,b:&u64) -> Ordering
{
    return a.cmp(b).reverse();
}

fn in_order(a:&u64, b:&u64) -> Ordering
{
    return a.cmp(b);
}

impl miner{

    fn new() -> miner {
        miner{
            rng: SeedableRng::from_seed(0),
            lastSeed: 0,
            hasher: Sha256::new()
        }
    }

    fn getMersOutput<'a>(&'a mut self, seed: u64, numInts: usize) -> Vec<u64> {
        self.rng.reseed(seed);
        let mut output=Vec::with_capacity(numInts);

        for x in 0..numInts {
            output.push(self.rng.next_u64());
        }

        return output;
    }


    fn sortedListsChal<'a>(&'a mut self, numbers: Vec<u64>, order: FnMut(&u64,&u64)->Ordering) -> String
    {
        numbers.sort_by(order);

        let results= numbers.iter().map(|x| x.to_string()).collect().join();

        return results;

    }



    fn findNonce(&mut self, lastSolution:String, target:&[u8], chal:challenge) -> [u8;8]
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

        let mut seed = ByteOrder::read_u64(&new_seed[0..8]);

        let challenge_ord = match chal{
            challenge::name: String::from("reverse-order"), numInts:Some(_)} => reverse,
            _ => in_order
        };

        let mut numbers = match chal{
            challenge{name:_, numInts:Some(x)} => self.getMersOutput(seed, x),
            _ => vec![0;5]
        };

        let mut concat_strings = self.sortedListsChal(numbers, challenge_ord);

        let mut hash_res:[u8;32]=[0;32];

        self.hasher.reset();

        self.hasher.input_str(&concat_strings);

        self.hasher.result(&mut hash_res);

        let mut ret_nonce:[u8;8]=[0;8];

        if &hash_res[0..2]==target{
            ByteOrder::write_u64(&mut ret_nonce, nonce);
            return ret_nonce;
        };

        //Pass mutated string as input for hasher.input_str

        //Check if prefix of hash and target are equal. End loop if true

        //Return thus obtained nonce
        return [0;8]

    }


}
