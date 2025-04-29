use std::{
    collections::{hash_map::Values, HashMap},
    hash::Hash,
    ops::{Deref, DerefMut},
};

use strum::IntoDiscriminant;
use triple_buffer::{triple_buffer, Input, InputPublishGuard, Output};

#[derive(Clone, Default, Debug)]
pub struct EnumSet<E>(HashMap<E::Discriminant, E>)
where
    E: IntoDiscriminant<Discriminant: Eq + Hash>;

impl<E> EnumSet<E>
where
    E: IntoDiscriminant<Discriminant: Eq + Hash>,
{
    #[must_use]
    pub fn new() -> Self {
        Self(HashMap::<E::Discriminant, E>::new())
    }

    pub fn insert(&mut self, v: E) -> bool {
        self.0.insert(v.discriminant(), v).is_some()
    }

    #[allow(clippy::iter_without_into_iter)]
    #[must_use]
    pub fn iter(&self) -> Values<'_, <E as IntoDiscriminant>::Discriminant, E> {
        self.0.values()
    }
}

pub struct SparseUpdateOutput<E>(Output<(bool, EnumSet<E>)>)
where
    <E as IntoDiscriminant>::Discriminant: Send,
    E: IntoDiscriminant<Discriminant: Eq + Hash> + Send;

impl<E> SparseUpdateOutput<E>
where
    E: IntoDiscriminant<Discriminant: Eq + Hash + Send> + Send,
{
    pub fn try_read(&mut self) -> Option<Values<'_, <E as IntoDiscriminant>::Discriminant, E>> {
        {
            let this = &mut self.0;
            // Fetch updates from the producer
            this.update();

            // Give access to the output buffer
            let b = this.output_buffer_mut();
            if b.0 {
                b.0 = false;
                Some(b.1.iter())
            } else {
                None
            }
        }
    }
}

pub struct SparseUpdateInput<E>(Input<(bool, EnumSet<E>)>)
where
    <E as IntoDiscriminant>::Discriminant: Send,
    E: IntoDiscriminant<Discriminant: Eq + Hash> + Send;

impl<E> SparseUpdateInput<E>
where
    E: IntoDiscriminant<Discriminant: Eq + Hash + Send> + Send,
{
    pub fn input_buffer_publisher(&mut self) -> SparseUpdateInputPublishGuard<E> {
        let mut guard = self.0.input_buffer_publisher();
        guard.1 .0.clear();
        guard.0 = true;
        SparseUpdateInputPublishGuard { reference: guard }
    }
}

pub struct SparseUpdateInputPublishGuard<'a, E>
where
    E: IntoDiscriminant<Discriminant: Eq + Hash + Send> + Send,
{
    reference: InputPublishGuard<'a, (bool, EnumSet<E>)>,
}

impl<E> Deref for SparseUpdateInputPublishGuard<'_, E>
where
    E: IntoDiscriminant<Discriminant: Eq + Hash + Send> + Send,
{
    type Target = EnumSet<E>;

    fn deref(&self) -> &EnumSet<E> {
        &self.reference.1
    }
}

impl<E> DerefMut for SparseUpdateInputPublishGuard<'_, E>
where
    E: IntoDiscriminant<Discriminant: Eq + Hash + Send> + Send,
{
    fn deref_mut(&mut self) -> &mut EnumSet<E> {
        &mut self.reference.1
    }
}

#[must_use]
pub fn ui_active_sparse_update_tripple_buffer<E>() -> (SparseUpdateInput<E>, SparseUpdateOutput<E>)
where
    E: IntoDiscriminant<Discriminant: Eq + Hash + Clone + Send> + Clone + Send,
{
    let set = EnumSet::<E>::new();
    let (send, recv) = triple_buffer(&(false, set));
    let send = SparseUpdateInput(send);
    let recv = SparseUpdateOutput(recv);
    (send, recv)
}
