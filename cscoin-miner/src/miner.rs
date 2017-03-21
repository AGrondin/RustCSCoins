extern crate mersenne_twister;
extern crate crypto;
extern crate rand;
use mersenne_twister::MersenneTwister;
use rand::{Rng, SeedableRng};
use crypto::digest::Digest;
use crypto::sha2::Sha256;


struct miner{
    rng: MersenneTwister,
    lastSeed: u64,
    hasher: Sha256
}

impl miner{

    fn new() -> miner {
        miner{
            rng: SeedableRng.from_seed(0),
            lastSeed: 0,
            hasher: Sha256::new()
        }
    }

    fn getMersOutput<'a>(&'a mut self, seed: u64, numInts: u32) -> Vec<u64> {
        self.rng.reseed(seed);
        let mut output=Vec::with_capacity(numInts);

        for x in 0..numInts {
            output.push(rng.next_u64());
        }

        return output;
    }


    fn sortedListsChal<'a>(&'a mut self, &mut numbers: Vec<u64>, order: F) -> String where F: FnMut(&T, &T) -> Ordering
    {
        //numbers.sort_by(order);

        let results= numbers.iter().map(|x| x.ToString()).collect().join();

        return results;

    }

    fn findNonce(&mut self, prefix:&[u8], target:&[u8]) -> [u8]{

        //Start loop

        //Reseed rng

        //Generate 8 byte Nonce using rng

        //Concatenate prefix and Nonce using push_into mut str (maybe as [u8], see what's faster)

        //Pass mutated string as input for hasher.input_str

        //Check if prefix of hash and target are equal. End loop if true

        //Return thus obtained nonce
        return [0,8]

    }

}


