pub fn p1(input: &str) -> String {
    todo!()
}

pub fn p2(input: &str) -> String {
    todo!()
}

pub fn parse(input: &str) -> () {
    let index_of_checksum = input.find('[').unwrap();
    let (code, checksum) = input.split_at(index_of_checksum);
    let checksum = checksum.chars().skip(1).take_while(|&it| it != ']').collect();

}