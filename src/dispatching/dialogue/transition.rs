use crate::{
    dispatching::{dialogue::DialogueStage, UpdateWithCx},
    types::Message,
};
use futures::future::BoxFuture;

/// Represents a transition function of a dialogue FSM.
pub trait Transition: Sized {
    type Aux;
    type Error;

    /// Turns itself into another state, depending on the input message.
    ///
    /// `aux` will be passed to each subtransition function.
    fn react(
        self,
        cx: TransitionIn,
        aux: Self::Aux,
    ) -> BoxFuture<'static, TransitionOut<Self, Self::Error>>;
}

/// Like [`Transition`], but from `StateN` -> `Dialogue`.
///
/// [`Transition`]: crate::dispatching::dialogue::Transition
pub trait Subtransition
where
    Self::Dialogue: Transition<Aux = Self::Aux>,
{
    type Aux;
    type Dialogue;
    type Error;

    /// Turns itself into another state, depending on the input message.
    ///
    /// `aux` is something that is provided by the call side, for example,
    /// message's text.
    fn react(
        self,
        cx: TransitionIn,
        aux: Self::Aux,
    ) -> BoxFuture<'static, TransitionOut<Self::Dialogue, Self::Error>>;
}

/// A type returned from a FSM subtransition function.
///
/// Now it is used only inside `#[teloxide(subtransition)]` for type inference.
pub trait SubtransitionOutputType {
    type Output;
    type Error;
}

impl<D, E> SubtransitionOutputType for TransitionOut<D, E> {
    type Output = D;
    type Error = E;
}

/// An input passed into a FSM (sub)transition function.
pub type TransitionIn = UpdateWithCx<Message>;

/// A type returned from a FSM (sub)transition function.
pub type TransitionOut<D, E = crate::RequestError> = Result<DialogueStage<D>, E>;
