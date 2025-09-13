use etimer::etimer_highres_getnow;

use std::time::Duration;
use std::thread;
use super::*;

// cspell: words etimer, highres, getnow, usecs, msecs

#[test]
fn test_highres_getnow()
{
    let start = etimer_highres_getnow();
    let five_seconds = time::Duration.new(5,0);
    thread::sleep( five_seconds );
    let later = etimer_highres_getnow();
    
    let diff = later - start;
    let etime = etimer_highres_to_usecs( diff );
    // Sometimes: "rounding errors occur" 
    // So we give a margin of +/- 100uSECS */
    assert!( etime >= 4900 );
    assert!( etime <= 5100 );

}