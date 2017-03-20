extern crate rand;
extern crate mersenne_twister;
use rand::{Rng, SeedableRng};
use mersenne_twister::MersenneTwister;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::usize;


#[derive(Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct grid{
    size:usize,
    num_blockers:u64,
    grid_space:Vec<bool>,
    start_pt:usize,
    end_pt:usize
}

impl grid{

    fn new(sz:usize, nb:u64)->grid{
        grid{
            size:sz,
            num_blockers:nb,
            grid_space:vec![true;sz*sz],
            start_pt:0,
            end_pt:0
        }

    }

    fn neighbours(&self, pt:usize)->[Option<usize>;4]{
        let neighbours:[Option<usize>;4]=[self.maybe_neighbour(pt+self.size),self.maybe_neighbour(pt-self.size),self.maybe_neighbour(pt+1),self.maybe_neighbour(pt-1)];

        return neighbours;
    }

    fn maybe_neighbour(&self, pt:usize)->Option<usize>{
        if pt<self.grid_space.len() && self.grid_space[pt]{
            return Some(pt);
        }else{
            return None;
        }
    }

    fn at_mod(&self, x:usize, y:usize)->usize{
        return (x%self.size)*self.size+y%self.size;
    }

    fn at(&self, x:usize, y:usize)->Option<usize>{
        if x<self.size && y<self.size{
            return Some(x*self.size+y);
        }else{
            return None;
        }
    }

    fn place_walls(&mut self){
        for i in 0..(self.size-1){
            self.grid_space[i]=false;
            self.grid_space[(self.size-1)*self.size+i]=false;
            self.grid_space[self.size*i+self.size-1]=false;
            self.grid_space[self.size*i]=false;
        }
    }

    fn set_startend(&mut self){

    }

    //Place location as occupied, returns true if not already occupied
    fn place_loc(&mut self, x:usize, y:usize)->bool{
        if self.grid_space[self.at_mod(x,y)]{
            self.grid_space[self.at_mod(x,y)]=false;
            return true;
        }else{
            return false;
        }
    }

    fn populate(&mut self, random:&mut MersenneTwister){

        self.place_walls();

        let mut x = random.next_u64() as usize;
        let mut y = random.next_u64() as usize;

        //Check if not wall
        while (x%self.size)%(self.size-1)!=0 && (y%self.size)%(self.size-1)!=0{
            x=random.next_u64() as usize;
            y=random.next_u64() as usize;
        }

        self.start_pt=self.at_mod(x,y);
        self.grid_space[self.start_pt]=false;

        let mut found_end=false;
        while !found_end{
            x=random.next_u64() as usize;
            y=random.next_u64() as usize;
            found_end=self.place_loc(x,y);
        }

        self.end_pt=self.at_mod(x,y);

        for i in 0..self.num_blockers{
            self.place_loc(random.next_u64() as usize,random.next_u64() as usize);
        }
    }

}

fn Dijsktra(_grid:grid)->Option<(HashMap<usize,Option<usize>>,HashMap<usize,usize>)>{
    let mut frontier=BinaryHeap::new();

    let mut came_from=HashMap::new();
    let mut cost_so_far=HashMap::new();

    frontier.push(State{cost:0, position:_grid.start_pt});
    came_from.insert(_grid.start_pt, None);
    cost_so_far.insert(_grid.start_pt, 0);

    while let Some(State { cost, position }) = frontier.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == _grid.end_pt {return None;} //TODO return reconstructed path}

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for point in &_grid.neighbours(position) {
            match point{
                &Some(pt)=>{
                    let new_cost = cost_so_far[&position] + 1;

                    if !cost_so_far.contains_key(&pt) || cost_so_far[&pt]>new_cost{
                        cost_so_far.insert(pt, new_cost);
                    }

                    let next_state=State{cost: new_cost, position: pt};

                    frontier.push(next_state);

                    came_from.insert(pt, Some(position));

                }
                &None=>{}
            }

        }
    }

    // Goal not reachable
    None


}
