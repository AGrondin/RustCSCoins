
use std::sync::Arc;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use threads::ThreadAssignment;


//Number of hashes to make per processing chunk
static WORK_CHUNK_SIZE: u64 = 100;



pub struct Worker {
    current_assignment:  Arc<ThreadAssignment>,
    nonce_sender:        Sender<String>,
    /*assignment_receiver: Arc<Receiver<ThreadAssignment>>*/
}

impl Worker {

    pub fn new(/*assignment_receiver: Arc<Receiver<ThreadAssignment>>,*/
               nonce_sender:        Sender<String>,
               assignment:          Arc<ThreadAssignment>) -> Worker {
        Worker{
            current_assignment:  assignment.clone(),
            nonce_sender:        nonce_sender,
            /*assignment_receiver: assignment_receiver*/
        }
    }

    //True for workers, false for main (main does some extra stuff)
    pub fn do_work(&mut self, do_loop: bool) -> (){

        while do_loop {

        }

        ()
    }

}

