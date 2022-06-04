use robot_simulator::{Direction, Robot};

fn main() {
    let robot = Robot::new(0, 0, Direction::North);
    println!("NEW {:#?}", robot);
    println!("BEFORE {:#?}", robot.instructions("LAAARALA"));
}
