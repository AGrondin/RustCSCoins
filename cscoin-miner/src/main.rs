extern crate mersenne_twister;
extern crate crypto;
extern crate rand;
extern crate byteorder;
extern crate itertools;
extern crate openssl;
extern crate rustc_serialize;
extern crate serde;
extern crate fnv;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate websocket;

//Everything to do with communicating with the server.
mod server_comms;
mod client_miner;
mod threads;


//-----------------------------------------------------------------------------
// Consts
//-----------------------------------------------------------------------------

//Number of threads to use
static NUM_THREADS:     u64 = 8;
//Number of hashes to make per processing chunk
static WORK_CHUNK_SIZE: u64 = 100;


fn main() {

    //TODO: Ctrl+C handling

    //Init comms
    let mut client         = server_comms::CSCoinClient::connect(server_comms::DEFAULT_URI).unwrap();
    let mut worker_manager = threads::ThreadManager::new(NUM_THREADS);

    //get first challenge and assign to workers
    let first_challenge  = client.get_current_challenge().unwrap();
    let first_assignment = get_assignment(first_challenge);
    worker_manager.setup(first_assignment.clone());

    loop {

        //check if connection dropped

        //do some work in main thread
        worker_manager.do_main_work();

        //check if a worker found a solution
        match worker_manager.get_solution() {
            Some(nonce) => {

                //Submit
                client.submission(nonce).unwrap(); //TODO: ERROR CHECKING

                //get new challenge
                let new_challenge = client.get_current_challenge().unwrap(); //TODO: ERROR CHECK
                let new_assignment = get_assignment(new_challenge);

                //Dispatch new assignment
                worker_manager.set_new_assignment(new_assignment);
            },
            None => {}
        }

        //Check if were out of time and need a new challenge

        //if solution not found continue working

    }

    println!("Stopping...");
    worker_manager.stop();

}


pub fn get_assignment(current_challenge: server_comms::cmd_response::CurrentChallenge) -> threads::ThreadAssignment {

    match &current_challenge.challenge_name[..] {
        "sorted_list" => {
            threads::ThreadAssignment::SortedList(
                current_challenge.last_solution_hash,
                current_challenge.hash_prefix,
                current_challenge.parameters.nb_elements.unwrap()
            )
        },
        "reverse_sorted_list" => {
            threads::ThreadAssignment::ReverseSortedList(
                current_challenge.last_solution_hash,
                current_challenge.hash_prefix,
                current_challenge.parameters.nb_elements.unwrap()
            )
        },
        "shortest_path " => {
            threads::ThreadAssignment::ShortestPath(
                current_challenge.last_solution_hash,
                current_challenge.hash_prefix,
                current_challenge.parameters.grid_size.unwrap(),
                current_challenge.parameters.nb_blockers.unwrap(),
            )
        }
        _ => {
            panic!("Got an invalid challenge?????");
        }
    }

}
