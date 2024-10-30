pub fn map<F, N, M>(input: Vec<N>, mut function: F) -> Vec<M>
where F: FnMut(N) -> M, {
    let mut result = Vec::new();
    for i in input {
        result.push(function(i));
    }
    result
}
