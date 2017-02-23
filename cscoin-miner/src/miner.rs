extern crate mersenne_twister;
extern crate rand;
extern crate sha2;
use mersenne_twister::MersenneTwister;
use sha2::{Sha256, Digest};
use rand::{Rng, SeedableRng};


struct miner{
    rng: MersenneTwister;
    lastSeed: u64;
}

impl miner{

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


}


