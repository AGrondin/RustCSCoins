

pub mod client_miner;
pub mod shortest_path;
mod server_comms;

//To be replaced by thread dispatcher

fn solve_challenge(current_challenge: CurrentChallenge)->Option<ChallengeSolution>{

    let miner = miner::new();

    let id = current_challenge.challenge_id;

    match current_challenge.parameters{
        CurrentChallengeParams { grid_size: sz ref @ Some(_), nb_blockers: num ref @ Some(_), .. }=>{
            return Some(miner.solve_shortest_path(current_challenge.last_solution_hash, sz, num));
        },
        CurrentChallengeParams { grid_size: None, nb_blockers: None, nb_elements: num_int ref @ some(_)}=>{
            let reverse = current_challenge.challenge_name=="reverse_sorted_list".toString();
            if reverse {return Some(miner.reverse_challenge(current_challenge.last_solution_hash, num_int));}
            else {
                return Some(miner.sorted_list_challenge(current_challenge.last_solution_hash, num_int));
            }
        },
        _=>None
    }
}
