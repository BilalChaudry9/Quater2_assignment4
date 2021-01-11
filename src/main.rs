use std::thread;
use std::time::Duration;
#[allow(unused_mut)]
#[allow(unused_variables)]
#[allow(unused_must_use)]

fn main()
{
    // Question 1
    let mut score = 2;
    
    let mut add_2 = || { 
        score = score + 2
    };
    
    for x in 0..10
    {
        add_2();
    }
    println!("score: {}", score);
    
    // Question 2
    let mut goal = 2;
    
    let handle = thread::spawn(move||{
        for i in 0..14
        {
            goal *= 2;
            thread::sleep(Duration::from_millis(20));
        }
        println!("goal: {}", goal);
    });
    
    handle.join().unwrap();   //To Run Thread Completely
}