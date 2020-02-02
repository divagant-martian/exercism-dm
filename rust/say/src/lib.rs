pub fn encode(n: u64) -> String {
    if n == 0 {
        return String::from("zero");
    }
    encode_helper(n)
}

fn encode_helper(n: u64) -> String {
    match n {
        // when 0 is a chunk, it produces no aditional words.
        0 => String::new(),
        1..=99 => encode_until_99(n),
        100 => String::from("one hundred"),
        101..=999 => {
            let r = n.rem_euclid(100);
            encode_until_99((n - r) / 100) + " hundred " + &encode_until_99(r)
        }
        _ => {
            let mut m = n;

            // split the number in chunks of 3 digits from right to left
            let mut chunks = Vec::<u64>::new();
            while m > 0 {
                let chunk = m.rem_euclid(1000);
                chunks.push(chunk);
                m = (m - chunk) / 1000;
            }

            chunks
                .iter()
                .enumerate()
                .rfold(String::new(), |acc, (i, &chunk)| {
                    if chunk == 0 {
                        return acc;
                    }
                    if chunks.len() == i + 1 {
                        return encode_helper(chunk) + " " + say_10th_power(i);
                    }
                    if i == 0 {
                        return acc + " " + &encode_helper(chunk);
                    }
                    acc + " " + &encode_helper(chunk) + " " + say_10th_power(i)
                })
        }
    }
}

fn say_10th_power(n: usize) -> &'static str {
    match n {
        1 => "thousand",
        2 => "million",
        3 => "billion",
        4 => "trillion",
        5 => "quadrillion",
        6 => "quintillion",
        _ => unimplemented!(),
    }
}
fn encode_until_99(n: u64) -> String {
    match n {
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 => String::from("fourteen"),
        15 => String::from("fifteen"),
        16 => String::from("sixteen"),
        17 => String::from("seventeen"),
        18 => String::from("eighteen"),
        19 => String::from("nineteen"),
        20 => String::from("twenty"),
        30 => String::from("thirty"),
        40 => String::from("forty"),
        50 => String::from("fifty"),
        60 => String::from("sixty"),
        70 => String::from("seventy"),
        80 => String::from("eighty"),
        90 => String::from("ninety"),
        _ => {
            let r = n.rem_euclid(10);
            encode_until_99(n - r) + "-" + &encode_until_99(r)
        }
    }
}
