use std::collections::HashMap;

pub fn nucleotide_counts(s:&str) -> Result<HashMap<char,usize>, &'static str> {
    let mut r:HashMap<char,usize> = HashMap::new();
    for c in vec!['A','C','G','T']{
        let x = count(c,s);
        if x.is_ok(){
            r.insert(c,x.unwrap());
        }
        else{
            return Err("bad dna");
        }
    }
    Ok(r)
}

pub fn count(c:char,s:&str) -> Result<usize, &'static str> {
    if !is_good(s) || (c!='A' &&  c!='C' && c!='G' && c!='T') {
        return Err("bad dna");
    }
    Ok(s.chars().filter(|&x| x==c).count() )
}

pub fn is_good(s:&str) -> bool {
    s.chars().all(|x| x=='A' || x=='C'|| x=='G' || x=='T')
}
