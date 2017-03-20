
extern crate crypto;
extern crate openssl;
extern crate rustc_serialize;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate websocket;

//Everything to do with communicating with the server.
mod server_comms;
mod threads;


//-----------------------------------------------------------------------------
// Consts
//-----------------------------------------------------------------------------

//Number of threads to use
static NUM_THREADS:     u64 = 8;
//Number of hashes to make per processing chunk
static WORK_CHUNK_SIZE: u64 = 100;


fn main() {

    //Init comms
    let mut client         = server_comms::CSCoinClient::connect(server_comms::DEFAULT_URI).unwrap();
    let mut worker_manager = threads::ThreadManager::new(NUM_THREADS, WORK_CHUNK_SIZE);

    //get first challenge and assign to workers
    let first_challenge = client.get_current_challenge().unwrap();


    //worker_manager.setup();

    loop {

        //check if connection dropped

        //do some work in main thread
        //worker_manager.main_work()

        //check if a worker found a solution
        //worker_manager.get_solution()
        //if solution found {
        //    //send solution
        //    //get new challenge
        //    //send assignment to threads
        //}

        //Check if were out of time and need a new challenge

        //if solution not found continue working

    }

}
