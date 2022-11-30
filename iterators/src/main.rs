fn main() {
    let vector = vec![1, 2, 3];
    let iter = take(vector);
    println!("{:?}", iter);
}

fn take<IntoIt, It, Data>(into_iter: IntoIt) -> It
where
    IntoIt: IntoIterator<Item = Data, IntoIter = It>,
    It: Iterator<Item = Data>,
{
    into_iter.into_iter()
}
