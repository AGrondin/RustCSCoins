
use std::ops::Deref;
use std::sync::Arc;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

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
    threads:         Vec<JoinHandle<()>>,
    //threads
    //mainworker
    workers:         Vec<Worker>,                   //Workers[0] is reserved for main thread
    txs:             Vec<Sender<ThreadAssignment>>, //Used to send Assignments to the threads, they have the receiver
    main_rx:         Receiver<String>,              //Used to receive the solution (nonce) from any thread that has one
    main_tx:         Arc<Sender<String>>            //The sender that will be cloned and given to the threads

}

impl ThreadManager {

    pub fn new(num_threads: u64) -> ThreadManager {
        assert!(num_threads >= 1, "Must have at least one worker thread!");

        let mut workers: Vec<Worker>                   = Vec::new();
        let mut txs:     Vec<Sender<ThreadAssignment>> = Vec::new();

        let (main_tx, main_rx) = channel();



        ThreadManager {
            num_threads:     num_threads,
            threads:         Vec::new(),
            workers:         workers,
            txs:             txs,
            main_rx:         main_rx,
            main_tx:         Arc::new(main_tx)
        }
    }

    pub fn setup(&mut self, first_challenge: ThreadAssignment) {


        //let (main_tx, main_rx) = channel();
        let (tx, rx): (Sender<String>, Receiver<String>) = channel();
        //let rx_arc        = Arc::new(rx);
        //NOTE: Make main tx in arc mutex like in hunter-bot
        //NOTE: Use mutex stuff to send assignment to threads, some contention will happen but rust...
        let main_tx_arc   = self.main_tx.clone();
        let main_tx       = main_tx_arc.clone();
        let challenge_arc = Arc::new(first_challenge.clone());
        let challenge     = challenge_arc.clone();

        let mut local_tx = tx.clone();

        self.threads.push(thread::spawn(move || {

            let mut main_tx = local_tx;
            Worker::new(/*rx_arc,*/ main_tx, challenge).do_work(true);
        }));




        //thread 0 is main thread, we handle it separately
        /*for i in 1..self.num_threads {
            let (tx, rx) = channel();
            self.txs.push(tx);
            self.threads.push(thread::spawn(move || {
                Worker::new(100, rx, self.main_tx.clone(), first_challenge.clone()).do_work(true);
            }));
        }*/


        /*//Create threads dont boot just yet
       for i in 0..num_threads {
           let (tx, rx) = channel();
           txs.push(tx);
           workers.push(Worker::new(work_chunk_size, rx, main_tx.clone()));
       }*/

    }

    pub fn set_new_assignment(assignment: ThreadAssignment) {

    }

    //Returns a solution if the threads found one
    //None otherwise
    //fn get_solution() -> Option<> {
    //
    //}

}
