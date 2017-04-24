#[derive(PartialEq)]
#[derive(Debug)]
pub struct RibonucleicAcid {string: String}
impl RibonucleicAcid {
    pub fn new(s:&str) -> RibonucleicAcid {
        RibonucleicAcid{string:s.to_string()}
    }
}

#[derive(Debug)]
pub struct DeoxyribonucleicAcid {string: String}
impl DeoxyribonucleicAcid {
    pub fn new(s:&str) -> DeoxyribonucleicAcid {
        DeoxyribonucleicAcid{string:s.to_string()}
    }
    pub fn to_rna(&self) -> RibonucleicAcid{
        RibonucleicAcid::new(
            self.string.chars().fold(String::new(),|mut acc, x|{
                acc.push(match x {
                    'G' => 'C',
                    'C' => 'G',
                    'T' => 'A',
                    _  => 'U',
                });
                acc
            }).as_str())
    }
}
