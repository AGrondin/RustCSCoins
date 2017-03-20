
pub mod worker;

//-----------------------------------------------------------------------------
// structs and enums
//-----------------------------------------------------------------------------

//The assignment given to the worker threads
#[derive(Clone)]
enum ThreadAssignment {
    Stop,
    //Last solution hash, has prefix, nb_elements
    SortedList(String, String, u64),
    ReverseSortedList(String, String, u64),
    //Last solution hash, has prefix, grid_size, nb_blockers
    ShortestPath(String, String, u64, u64)
}


//Worker manager

pub struct ThreadManager {
    num_threads:     u64,
    work_chunk_size: u64,
    //Workers
    //Workers[0] is reserved for main thread
}

impl ThreadManager {

    pub fn new(num_threads: u64, work_chunk_size: u64) -> ThreadManager {
        assert!(num_threads >= 1, "Must have at least one worker thread!");

        //Create threads dont boot just yet

        ThreadManager {
            num_threads:     num_threads,
            work_chunk_size: work_chunk_size,
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
