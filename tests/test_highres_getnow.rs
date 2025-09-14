use etimer::etimer_highres_getnow;
use etimer::etimer_highres_to_usecs;

use std::time;
use std::thread;

// cspell: words etimer, highres, getnow, usecs, msecs

#[test]
fn test_highres_getnow()
{
    let start = etimer_highres_getnow();
    println!("start2: {}", start);
    let start = etimer_highres_getnow();
    println!("start1: {}", start);
    let five_seconds = time::Duration::new(5,0);
    thread::sleep( five_seconds );
    let later = etimer_highres_getnow();
    println!("later {}", later);
    
    let diff = later - start;
    let etime = etimer_highres_to_usecs( diff );
    // Sometimes: "rounding errors occur" 
    // So we give a margin of +/- 500umsec
    // Note: the time unit here is micro seconds.
    assert!( etime >= 4995000 );
    assert!( etime <= 5005000 );
}