pub trait Group {
    type Elem: GroupElement<G = Self>;

    /// retorna o elemento identidade do grupo
    fn identity(&self) -> Self::Elem;

    /// retorna um elemento aleatorio pertencente ao grupo
    fn random_element(&self) -> Self::Elem;
}

pub trait GroupElement: Clone + PartialEq {
    type G: Group<Elem = Self>;

    /// operacao generica do grupo
    fn combine(&self, other: &Self) -> Self;

    /// aplicacao repetida de `combine`
    fn repeat(&self, exp: u64) -> Self;

    /// inverso do grupo
    fn inverse(&self) -> Self;
}