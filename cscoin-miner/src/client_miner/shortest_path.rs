use rand::{Rng, SeedableRng};
use mersenne_twister::MersenneTwister;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use itertools::Itertools;
use std::usize;



#[derive(Eq, PartialEq, Copy, Clone)]
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

pub struct Grid{
    pub size:usize,
    pub num_blockers:u64,
    pub grid_space:Vec<bool>,
    pub start_pt:usize,
    pub end_pt:usize
}

impl fmt::Display for Grid {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut grid_str=String::new();

        // let mut start_neighbours:Vec<usize>=Vec::with_capacity(4);
        //
        // for i in self.neighbours(self.start_pt).iter(){
        //     match i {
        //         &Some(x)=>{start_neighbours.push(x)},
        //         _=>{}
        //     }
        // }

        grid_str.push_str(&self.start_pt.to_string());
        grid_str.push_str("\n ");

        for i in 0..self.size{
            grid_str.push_str(&i.to_string());
        }
        for i in 0..self.grid_space.len(){
            if i%self.size==0 {
                grid_str.push_str("\n");
                grid_str.push_str(&(i/self.size).to_string());
            }

            if self.grid_space[i]{
                // if start_neighbours.contains(&i){
                //     grid_str.push_str("n");
                // } else{
                    grid_str.push_str(" ");
                }
            } else if i==self.start_pt{
                grid_str.push_str("s");
            } else if i==self.end_pt{
                grid_str.push_str("e");
            } else{
                grid_str.push_str("x");
            }

        }

        write!(f, "{}", grid_str)
    }
}

impl Grid{

    pub fn new(sz:usize, nb:u64)->Grid
    {
        Grid{
            size:sz,
            num_blockers:nb,
            grid_space:vec![true;sz*sz],
            start_pt:0,
            end_pt:0
        }

    }

    pub fn neighbours(&self, pt:usize)->[Option<usize>;4]
    {
        let neighbours:[Option<usize>;4]=[self.maybe_neighbour(pt.checked_add(self.size).unwrap_or(self.grid_space.len())),
                                          self.maybe_neighbour(pt.checked_sub(self.size).unwrap_or(0)),
                                          self.maybe_neighbour(pt.checked_add(1).unwrap_or(self.grid_space.len())),
                                          self.maybe_neighbour(pt.checked_sub(1).unwrap_or(0))];

        return neighbours;
    }

    pub fn maybe_neighbour(&self, pt:usize)->Option<usize>
    {
        if pt<self.grid_space.len() && self.grid_space[pt]{
            return Some(pt);
        }else{
            return None;
        }
    }

    pub fn at_mod(&self, x:usize, y:usize)->usize
    {
        return (x%self.size)*self.size+y%self.size;
    }

    pub fn at(&self, x:usize, y:usize)->Option<usize>
    {
        if x<self.size && y<self.size{
            return Some(x*self.size+y);
        }else{
            return None;
        }
    }

    fn place_walls(&mut self)
    {
        for i in 0..(self.size){
            self.grid_space[i]=false;
            self.grid_space[(self.size-1)*self.size+i]=false;
            self.grid_space[self.size*i+self.size-1]=false;
            self.grid_space[self.size*i]=false;
        }
    }

    pub fn distance_h(&self, start:usize, goal:usize)->usize
    {
         (cmp::max(start%self.size, goal%self.size)-cmp::min(start%self.size, goal%self.size))
         +(cmp::max(start/self.size, goal/self.size)-cmp::min(start/self.size, goal/self.size))
    }


    //Place location as occupied, returns true if not already occupied
    fn place_loc(&mut self, x:usize, y:usize)->bool
    {
        let loc=self.at_mod(x,y);
        if self.grid_space[loc]{
            self.grid_space[loc]=false;
            return true;
        }else{
            return false;
        }
    }

    pub fn populate(&mut self, random:&mut MersenneTwister)
    {

        self.place_walls();

        let mut x = random.next_u64() as usize;
        let mut y = random.next_u64() as usize;

        //Check if not wall
        while (x%self.size)%(self.size-1)==0 || (y%self.size)%(self.size-1)==0{
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

        self.grid_space[self.end_pt]=true;
    }

}


pub fn a_star(_grid:&Grid)->Option<(HashMap<usize,Option<usize>>,usize)>
{
    let mut frontier=BinaryHeap::new();

    let mut came_from=HashMap::new();
    let mut cost_so_far=HashMap::new();

    frontier.push(State{cost:0, position:_grid.start_pt});
    came_from.insert(_grid.start_pt, None);
    cost_so_far.insert(_grid.start_pt, 0);

    println!("{}:{}", (_grid.end_pt%_grid.size).to_string(),(_grid.end_pt/_grid.size).to_string());


    while let Some(State { cost, position }) = frontier.pop() {
        // Alternatively we could have continued to find all shortest paths

        //println!("{}:{}", (position%_grid.size).to_string(),(position/_grid.size).to_string());
        if position == _grid.end_pt {
            //println!("Found it!");
            return Some((came_from,cost));
        }
        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for point in &_grid.neighbours(position) {
            match point{
                &Some(pt)=>{
                    let new_cost = cost_so_far[&position] + 1;

                    if !cost_so_far.contains_key(&pt) || cost_so_far[&pt]>new_cost{
                        //println!("{}:{}->{}:{}", (pt%_grid.size).to_string(),(pt/_grid.size).to_string(),new_cost.to_string(),_grid.distance_h(pt,_grid.end_pt));
                        cost_so_far.insert(pt, new_cost);
                        //println!("{}",new_cost+_grid.distance_h(pt,_grid.end_pt));
                        let next_state=State{cost: new_cost+_grid.distance_h(pt,_grid.end_pt), position: pt};

                        frontier.push(next_state);
                        came_from.insert(pt, Some(position));
                    }
                },
                &None=>{}
            }
        }
    }

    // Goal not reachable
    println!("Not reachable");
    None
}

pub fn reconstruct_path(_grid:&Grid, came_from: HashMap<usize,Option<usize>>, cost: usize)->String
{
    let mut current:Option<usize>=Some(_grid.end_pt);

    let mut path : Vec<usize>=Vec::with_capacity((cost+1)*2);

    while current!=None {
        match current{
            Some(pt)=>{
                path.push(pt%_grid.size);
                path.push(pt/_grid.size);
                current=came_from[&pt];
            }
            None=>{break;}
        }
    }

    path.reverse();

    return path.clone().iter().join("");
}
