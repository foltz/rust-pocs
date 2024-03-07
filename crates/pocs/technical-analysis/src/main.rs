// use yata::core::{Method};

use crate::tests::ta_ema::ta_ema;
use crate::tests::ta_rsi::ta_rsi;
use crate::tests::yata_ema::yata_ema;

mod tests;

fn main() {

    // test_ema();
    ta_ema();
    // ta_rsi();
}
//     let vals = vec!(
//         20., 25., 29., 32., 31., 29., 26., 32., 36., 42., 51., 60.,
//         50., 55., 49., 42., 41., 39., 36., 32., 36., 42., 51., 60.,
//     );
//
//     // let len = 5u8;
//     // let mut ema = EMA::new(len, &vals[0]).unwrap();
//     // for v in 1..vals.len() {
//     //     let val = vals[v];
//     //     let next = ema.next(&val);
//     //     println!("ema1({len}): {val} = {next}");
//     // }
//
//     let firstVal = &vals[0];
//     let lookbacks: Vec<u8> = vec!(2,3,5,8,13,21,34,55);
//     let mut emas: Vec<EmaFn> = vec!();
//     for i in lookbacks {
//         emas.push(EmaFn::new(i, firstVal));
//     }
//
//     // let mut emas: Vec<EmaMut> = lookbacks.iter()
//     //     .map(|&i| {
//     //         EmaMut::new(i as u8,&vals[0]).unwrap()
//     //     })
//     //     .collect();
//
//     for v in 1..vals.len() {
//         for mut ema in emas.iter_mut() {
//             let val = vals[v];
//             let len = ema.length;
//             let next = ema.next(&val).clone();
//             println!("ema2({len}): {val} = {:?}", next);
//         }
//     }
// }
//
//
// struct EmaFn {
//     pub length: u8,
//     _ema: EMA,
// }
// impl EmaFn {
//     pub fn new(length: u8, value: &ValueType) -> Self {
//         Self {
//             length,
//             _ema: EMA::new(length, value).unwrap(),
//         }
//     }
//     pub fn next(&mut self, value: &ValueType) -> ValueType {
//         self._ema.next(value)
//     }
// }
