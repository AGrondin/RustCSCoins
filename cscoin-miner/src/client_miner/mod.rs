

pub mod client_miner;
pub mod shortest_path;
mod server_comms;

//To be replaced by thread dispatcher



fn solve_challenge(current_challenge: CurrentChallenge)->Option<(String, u64)>{

    let mut miner = miner::new();

    let id = current_challenge.challenge_id;

    match current_challenge.parameters{
        CurrentChallengeParams {grid_size: sz @ Some(_), nb_blockers: num @ Some(_), .. }=>{
            return miner.solve_shortest_path(current_challenge.last_solution_hash, sz.unwrap() as usize, num.unwrap());
        },
        CurrentChallengeParams {grid_size: None, nb_blockers: None, nb_elements: num_int  @ Some(_)}=>{
            let reverse = current_challenge.challenge_name=="reverse_sorted_list".to_string();
            if reverse {return miner.reverse_challenge(current_challenge.last_solution_hash, num_int.unwrap());}
            else {
                return miner.sorted_list_challenge(current_challenge.last_solution_hash, num_int.unwrap());
            }
        },
        _=>None
    }
}
