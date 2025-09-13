use core;
// cspell: words libc etimer, highres, 
// cspell: words getnow, timeval, gettimeofday
use libc::timeval;
use libc::gettimeofday;

static mut TIME_ZERO : u64 = 0;

/// time_zero is when the application started
/// This returns the number of microseconds since start
pub fn etimer_highres_getnow() -> u64 {
    let mut tv : libc::timeval;
    unsafe {
        libc::gettimeofday( &tv, core::ptr::null_mut() );
    }
    let mut answer : u64 = tv.tv_sec.wrapping_mul( 1000000 );
    answer = answer.wrapping_add( tv.tv_usec );
    unsafe {
        if TIME_ZERO == 0 {
            TIME_ZERO = answer
        }
        answer = answer.wrapping_sub( TIME_ZERO );
    }
    return answer
}

pub fn etimer_highres_to_usecs( time_stamp : u64 ) -> u64 {
    // Other platforms may have different highres resolution.
    // This platform the high res timer is microseconds.
    // Thus we can just return the timevalue and be done with it.
    return time_stamp;
}
