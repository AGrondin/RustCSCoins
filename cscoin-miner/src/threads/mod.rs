
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

pub mod worker;
use self::worker::Worker;

//-----------------------------------------------------------------------------
// structs and enums
//-----------------------------------------------------------------------------

//The assignment given to the worker threads
#[derive(Clone)]
pub enum ThreadAssignment {
    Stop,
    //Last solution hash, hash prefix, nb_elements
    SortedList(String, String, u64),
    ReverseSortedList(String, String, u64),
    //Last solution hash, hash prefix, grid_size, nb_blockers
    ShortestPath(String, String, u64, u64)
}


//Worker manager

pub struct ThreadManager {
    num_threads:     u64,
    work_chunk_size: u64,
    //Workers[0] is reserved for main thread
    workers:         Vec<Worker>,
    //Used to send Assignments to the threads
    txs:             Vec<Sender<ThreadAssignment>>,
    //Used to receive the solution (nonce) from any thread that has one
    rx:              Receiver<String>

}

impl ThreadManager {

    pub fn new(num_threads: u64, work_chunk_size: u64) -> ThreadManager {
        assert!(num_threads >= 1, "Must have at least one worker thread!");

        let mut workers: Vec<Worker>                   = Vec::new();
        let mut txs:     Vec<Sender<ThreadAssignment>> = Vec::new();

        let (main_tx, main_rx) = channel();

        //Create threads dont boot just yet
        for i in 0..num_threads {
            let (tx, rx) = channel();
            txs.push(tx);
            workers.push(Worker::new(work_chunk_size, rx, main_tx.clone()));
        }

        ThreadManager {
            num_threads:     num_threads,
            work_chunk_size: work_chunk_size,
            workers:         workers,
            txs:             txs,
            rx:              main_rx
        }
    }

    pub fn setup() {



    }

    pub fn set_new_assignment() {

    }

    //Returns a solution if the threads found one
    //None otherwise
    //fn get_solution() -> Option<> {
    //
    //}

}
