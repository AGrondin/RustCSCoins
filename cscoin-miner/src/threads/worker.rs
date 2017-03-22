
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use threads::ThreadAssignment;
use client_miner::Miner;

//Number of hashes to make per processing chunk
static WORK_CHUNK_SIZE: u64 = 100;



pub struct Worker {
    current_assignment:  Arc<Mutex<ThreadAssignment>>,
    nonce_sender:        Arc<Mutex<Sender<String>>>,
    work_miner:          Miner
}

impl Worker {

    pub fn new(nonce_sender: Arc<Mutex<Sender<String>>>, assignment:   Arc<Mutex<ThreadAssignment>>) -> Worker {
        Worker{
            current_assignment:  assignment,
            nonce_sender:        nonce_sender,
            work_miner:          Miner::new()
        }
    }

    //True for workers, false for main (main does some extra stuff)
    pub fn do_work(&mut self, do_loop: bool) -> (){

        while do_loop {

            let assignment_arc = self.current_assignment.clone();
            let assignment;

            { //Lock is scope based, lock for as little time as possible
                assignment = (*assignment_arc.lock().unwrap()).clone();
            }

            match assignment {
                ThreadAssignment::Stop => {break;},
                ThreadAssignment::SortedList(last_hash, prefix, num_int)=>{self.work_miner.sorted_list_challenge(last_hash, prefix, num_int);},
                ThreadAssignment::ReverseSortedList(last_hash, prefix, num_int)=>{self.work_miner.reverse_challenge(last_hash, prefix, num_int);},
                ThreadAssignment::ShortestPath(last_hash, prefix, size, num_blockers)=>{self.work_miner.shortest_path_challenge(last_hash, prefix, size, num_blockers, 100);},
                _ => {}
            }


        }

        ()
    }

}
