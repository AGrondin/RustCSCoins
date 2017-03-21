

pub mod client_miner;
pub mod shortest_path;
mod server_comms;

//To be replaced by thread dispatcher

fn solve_challenge(current_challenge: CurrentChallenge)->Option<ChallengeSolution>{

    let miner = miner::new();

    let id = current_challenge.challenge_id;


    let solution = match current_challenge.parameters{
        CurrentChallengeParams { grid_size: sz ref @ Some(_), nb_blockers: num ref @ Some(_), .. }=>{
            Some(miner.solve_shortest_path(current_challenge.last_solution_hash, current_challenge.hash_prefix, sz, num))
        },
        CurrentChallengeParams { grid_size: None, nb_blockers: None, nb_elements: num_int ref @ some(_)}=>{
            let reverse = current_challenge.challenge_name=="reverse_sorted_list".toString();
            Some(miner.solve_order(current_challenge.last_solution_hash, current_challenge.hash_prefix, num_int, reverse))
        },
        _=>None
    }

    if let Some((new_hash, new_nonce)) = solution{
        return Some(ChallengeSolution{challenge_id : current_challenge.challenge_id, challenge_name : current_challenge.challenge_name, nonce: new_nonce, hash: new_hash});
    }

    None

}
