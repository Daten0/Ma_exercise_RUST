fn main() {

    let mut tuples = (1, 5, 49.90);

    println!("Initial total value: ${:?}", invent_track(&tuples));

    tuples.1 -= 3;

    println!("After selling 3 units, the new total value is: ${:?}", invent_track(&tuples));
}

fn invent_track(_tup : &(u8, i32, f32)) -> f32 {

    let (_id, stock, prices) = _tup;

    (*stock as f32) * prices
}