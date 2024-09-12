mod executor;
mod sender;

pub use executor::{new_executor_manager, WATERBOT_ID};

#[test]
fn test_executor() {
    let executor = new_executor_manager("");
    for _ in 0..8 {
        executor.execute(WATERBOT_ID).unwrap();        
    }

}