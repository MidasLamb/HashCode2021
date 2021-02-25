use std::ops::Deref;
use std::sync::Arc;
use std::{cell::RefCell, collections::HashMap};

#[derive(Debug, PartialEq, Eq)]
pub struct Problem {
    pub amount_of_seconds: u32,
    pub bonus_points: u32,

    pub cars: Vec<Arc<RefCell<Car>>>,
    pub streets: Vec<Arc<RefCell<Street>>>,
    pub intersections: Vec<Intersection>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Car {
    pub id: u32,
    pub path_to_take: Vec<Arc<RefCell<Street>>>,
    pub destination_reached_at: Option<u32>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Street {
    pub name: String,
    /// Vec of cars with the amount of time they still have to be on the road to reach the end.
    pub cars: Vec<(Arc<RefCell<Car>>, u32)>,
    pub time_to_travel: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Intersection {
    pub id: u32,
    pub incoming_streets: Vec<Arc<RefCell<Street>>>,
    pub outgoing_streets: Vec<Arc<RefCell<Street>>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Solution {
    pub traffic_lights: HashMap<u32, Vec<(String, u32)>>,
}

pub fn solve(mut problem: Problem) -> Solution {
    let mut traffic_lights = HashMap::<u32, Vec<(String, u32)>>::new();

    // Single intersection
    problem
        .intersections
        .iter_mut()
        .filter(|intersection| intersection.incoming_streets.len() == 1)
        .for_each(|i| {
            traffic_lights.insert(
                i.id,
                vec![(i.incoming_streets[0].borrow_mut().name.clone(), 1 as u32)],
            );
        });

    // Remove single intersections
    problem.intersections = problem
        .intersections
        .into_iter()
        .filter(|intersection| intersection.incoming_streets.len() > 1)
        .collect::<Vec<_>>();

    let mut count_map = HashMap::<String, u64>::new();
    problem.cars.iter().for_each(|c| {
        c.borrow_mut().path_to_take.iter().for_each(|s| {
            match count_map.get_mut(&s.borrow().name.clone()) {
                Some(v) => *v += 1,
                None => {
                    count_map.insert(s.borrow().name.clone(), 1);
                }
            };
        })
    });

    problem.intersections.iter_mut().for_each(|mut i| {
        i.incoming_streets = i
            .incoming_streets
            .iter()
            .filter(|s| count_map.get(&s.borrow_mut().name.clone()).is_some())
            .map(|x| x.clone())
            .collect();
    });

    if false {
        problem.intersections.iter_mut().for_each(|i| {
            traffic_lights.insert(
                i.id,
                i.incoming_streets
                    .iter()
                    .map(|is| (is.borrow_mut().name.clone(), 1 as u32))
                    .collect::<Vec<_>>(),
            );
        });
    }

    if true {}

    Solution { traffic_lights }
}

pub fn simulate(mut problem: Problem, mut solution: Solution) {
    for time_step in 0..problem.amount_of_seconds {
        simulate_step(&mut problem, &mut solution, time_step);
    }
}

pub fn simulate_step(problem: &mut Problem, solution: &mut Solution, time: u32) {
    for intersection in problem.intersections.iter_mut() {
        let car_passed_intersection = false;
        for street in intersection.incoming_streets.iter() {
            let can_cross = car_passed_intersection
                && is_light_green(&solution, &intersection, &street.borrow_mut(), &time);
            for (car, time_before_end) in street.borrow_mut().cars.iter_mut() {
                if *time_before_end == 0 {
                    let next_street_index = car
                        .borrow_mut()
                        .path_to_take
                        .iter()
                        .position(|r| r == street)
                        .unwrap()
                        + 1;
                    if next_street_index < car.borrow_mut().path_to_take.len() {
                        if can_cross {
                            let next_street = &car.borrow_mut().path_to_take[next_street_index];
                            let c = street.borrow_mut().cars.remove(
                                street
                                    .borrow_mut()
                                    .cars
                                    .iter()
                                    .position(|c| c.0 == *car)
                                    .unwrap(),
                            );
                            next_street
                                .borrow_mut()
                                .cars
                                .push((c.0, next_street.borrow().time_to_travel));
                            car.borrow_mut().destination_reached_at = Some(time);
                        }
                    } else {
                        street.borrow_mut().cars.remove(
                            street
                                .borrow_mut()
                                .cars
                                .iter()
                                .position(|c| c.0 == *car)
                                .unwrap(),
                        );
                        car.borrow_mut().destination_reached_at = Some(time);
                    }
                } else {
                    *time_before_end -= 1;
                }
            }
        }
    }
    todo!()
}

pub fn is_light_green(
    solution: &Solution,
    intersection: &Intersection,
    street: &Street,
    time: &u32,
) -> bool {
    let street_times = &solution.traffic_lights[&intersection.id];
    let time = 0;
    let mut street_index = 0;
    let mut time_to_go = street_times[street_index].1;
    for i in 0..time {
        if time_to_go == 0 {
            street_index = (street_index + 1) % street_times.len();
            time_to_go = street_times[street_index].1;
        }
        time_to_go -= 1
    }
    return street_times[street_index].0 == street.name;
}

pub fn calculate_score(problem: Problem) -> u32 {
    let mut score: u32 = 0;
    for car in problem.cars {
        if let Some(time_of_arrival) = car.borrow().destination_reached_at {
            score += problem.bonus_points + (problem.amount_of_seconds - time_of_arrival)
        }
    }
    score
}

#[cfg(test)]
mod test {

    use super::*;
    type Result = std::result::Result<(), Box<dyn std::error::Error>>;
    #[test]
    pub fn example_is_correct() -> Result {
        let mut c1 = Arc::new(RefCell::new(Car {
            id: 0,
            path_to_take: vec![],
            destination_reached_at: None,
        }));
        let mut c2 = Arc::new(RefCell::new(Car {
            id: 1,
            path_to_take: vec![],
            destination_reached_at: None,
        }));

        let street1 = Arc::new(RefCell::new(Street {
            name: "rue-de-londres".to_owned(),
            cars: vec![(c1.clone(), 0)],
            time_to_travel: 1,
        }));

        let street2 = Arc::new(RefCell::new(Street {
            name: "rue-d-amsterdam".to_owned(),
            cars: vec![],
            time_to_travel: 1,
        }));

        let street3 = Arc::new(RefCell::new(Street {
            name: "rue-d-athenes".to_owned(),
            cars: vec![(c2.clone(), 0)],
            time_to_travel: 1,
        }));

        let street4 = Arc::new(RefCell::new(Street {
            name: "rue-de-rome".to_owned(),
            cars: vec![],
            time_to_travel: 2,
        }));

        let street5 = Arc::new(RefCell::new(Street {
            name: "rue-de-moscou".to_owned(),
            cars: vec![],
            time_to_travel: 3,
        }));

        c1.get_mut().path_to_take = vec![
            street1.clone(),
            street2.clone(),
            street5.clone(),
            street4.clone(),
        ];
        c2.get_mut().path_to_take = vec![street3.clone(), street5.clone(), street1.clone()];

        let intersection1 = Intersection {
            id: 0,
            incoming_streets: vec![street1.clone()],
            outgoing_streets: vec![street2.clone()],
        };
        let intersection2 = Intersection {
            id: 1,
            incoming_streets: vec![street2.clone(), street3.clone()],
            outgoing_streets: vec![street5.clone()],
        };
        let intersection3 = Intersection {
            id: 2,
            incoming_streets: vec![street5.clone()],
            outgoing_streets: vec![street1.clone(), street4.clone()],
        };
        let intersection4 = Intersection {
            id: 3,
            incoming_streets: vec![],
            outgoing_streets: vec![street3.clone()],
        };

        let cars = vec![c1, c2];
        let streets = vec![street1, street2, street3, street4, street5];
        let intersections = vec![intersection1, intersection2, intersection3, intersection4];

        let problem = Problem {
            amount_of_seconds: 6,
            bonus_points: 1000,
            cars: cars,
            streets: streets,
            intersections: intersections,
        };

        let s = solve(problem);
        dbg!(s);
        todo!();

        Ok(())
    }

    #[test]
    pub fn example_works() -> Result {
        Ok(())
    }
}
