use ta::indicators::RelativeStrengthIndex;
use ta::{Next, Period};


// #[test]
pub fn ta_rsi() {


    let vals = vec!(
        20., 25., 29., 32., 31., 29., 26., 32., 36., 42., 51., 60.,
        50., 55., 49., 42., 41., 39., 36., 32., 36., 42., 51., 60.,
    );

    // let firstVal = &vals[0];


    let period = 5;
    let mut rsi_fn = RelativeStrengthIndex::new(period).unwrap();
    for v in 0..vals.len() {
        let val = vals[v];
        let next = rsi_fn.next(val);
        println!("rsi1({period}): {val} = {next}");
    }


    let periods: Vec<usize> = vec!(5); //2, 3, 5, 8, 13, 21, 34, 55);
    let mut rsi_fns: Vec<RelativeStrengthIndex> = vec!();
    for period in periods {
        rsi_fns.push(RelativeStrengthIndex::new(period).unwrap());
    }

    // let mut emas: Vec<EmaMut> = lookbacks.iter()
    //     .map(|&i| {
    //         EmaMut::new(i as u8,&vals[0]).unwrap()
    //     })
    //     .collect();

    for v in 0..vals.len() {
        for mut rsi_fn in rsi_fns.iter_mut() {
            let val = vals[v];
            let period = rsi_fn.period();
            let next = rsi_fn.next(val).clone();
            println!("rsi2({period}): {val} = {:?}", next);
        }
    }
}