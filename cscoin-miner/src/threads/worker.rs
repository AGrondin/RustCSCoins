
use threads::ThreadAssignment;

pub struct Worker {
    work_chunk_size:    u64,
    current_assignment: ThreadAssignment
}

impl Worker {

    pub fn new(work_chunk_size: u64) -> Worker {
        Worker{
            work_chunk_size: work_chunk_size,
            current_assignment: ThreadAssignment::Stop
        }
    }

    //do_work

}

