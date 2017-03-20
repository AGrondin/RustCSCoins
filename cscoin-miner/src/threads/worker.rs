
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

use threads::ThreadAssignment;

pub struct Worker {
    work_chunk_size:     u64,
    current_assignment:  ThreadAssignment,
    nonce_sender:        Sender<String>,
    assignment_receiver: Receiver<ThreadAssignment>
}

impl Worker {

    pub fn new(work_chunk_size: u64, assignment_receiver: Receiver<ThreadAssignment>, nonce_sender: Sender<String>) -> Worker {
        Worker{
            work_chunk_size:     work_chunk_size,
            current_assignment:  ThreadAssignment::Stop,
            nonce_sender:        nonce_sender,
            assignment_receiver: assignment_receiver
        }
    }

    //do_work

}

