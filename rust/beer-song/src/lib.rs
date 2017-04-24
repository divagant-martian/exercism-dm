pub fn sing(mut b:i32, e:i32) -> String {
    let mut ret = "".to_string();
    while b > e  {
        ret += verse(b).as_str();
        ret += "\n";
        b -= 1;
    }
    ret += verse(e).as_str();
    ret
}

pub fn verse(i:i32) -> String {

    let n_m =
        if i == 0 {"No more".to_string()} else { i.to_string() };
    let num =                   // el numero que hay, o ninguna si no quedan
        if i == 0 {"no more".to_string()} else { i.to_string() };
    let plu =                   // si es plural
        if i != 1 {"s".to_string()} else {"".to_string()};
    let tdo =                   // instruccion: tomar una cerveza o comprar mas
        if i == 0
	    {"Go to the store and buy some more".to_string()}
	else if i == 1
    	    {"Take it down and pass it around".to_string()}
        else
	    {"Take one down and pass it around".to_string()};
    let oth =
        if i == 0 {99.to_string()}
	else if i == 1 {"no more".to_string()}
    	else {(i-1).to_string()};
    let p_u =
        if i == 2 {"".to_string()} else {"s".to_string()};

    format!("{0} bottle{1} of beer on the wall, \
             {4} bottle{1} of beer.\n\
	     {2}, {3} bottle{5} of beer on the wall.\n",
            n_m, plu, tdo, oth, num, p_u)
}
