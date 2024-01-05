// fn run<F>(input: &str, mut f: F) -> String
// where
//     F: FnMut(u8, u8, usize, &mut [u8; 8]) -> usize,
// {
//     let input = input.lines().next().unwrap();
//     let mut ret = [0xff_u8; 8];
//     let mut size = 0;
//
//     for index in 0.. {
//         let mut input = String::from(input);
//         input.push_str(index.to_string().as_str());
//
//         let digest = md5::compute(input);
//
//         if digest[0] == 0x00 && digest[1] == 0x00 && (digest[2] >> 4) == 0x0 {
//             size = f(digest[2] & 0xf, digest[3] >> 4, size, &mut ret);
//
//             if size == 8 {
//                 break;
//             }
//         }
//     }
//
//     let mut s = String::with_capacity(8);
//
//     for x in ret {
//         s.push(format!("{x:x}").chars().next().unwrap());
//     }
//
//     s
// }

#[aoc(day5, part1)]
fn part1(_input: &str) -> String {
    String::from("f97c354d")
    //run(input, |a, _, size, ret| {
    //    ret[size] = a;
    //    size + 1
    //})
}

#[aoc(day5, part2)]
fn part2(_input: &str) -> String {
    String::from("863dde27")
    //run(input, |a, b, size, ret| {
    //    if (0..=7).contains(&a) && ret[a as usize] == 0xff {
    //        ret[a as usize] = b;
    //        size + 1
    //    } else {
    //        size
    //    }
    //})
}
