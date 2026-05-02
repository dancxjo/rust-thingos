use core::ops::ControlFlow;

use crate::layout::UiTree;
use crate::node::NodeId;
use crate::style::State;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PetalsEvent {
    SetState { node: NodeId, state: State, enabled: bool },
    Restyle,
    Layout,
    Shutdown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServiceAction {
    Continue,
    Shutdown,
}

impl From<ServiceAction> for ControlFlow<()> {
    fn from(value: ServiceAction) -> Self {
        match value {
            ServiceAction::Continue => ControlFlow::Continue(()),
            ServiceAction::Shutdown => ControlFlow::Break(()),
        }
    }
}

pub trait PetalsService {
    fn dispatch(&mut self, tree: &mut UiTree, event: PetalsEvent) -> ServiceAction;
}

#[cfg(feature = "stem-service-loop")]
pub mod stem_runner {
    use core::ops::ControlFlow;

    use stem::service_loop::{ServiceEvent, ServiceLoop};
    use stem::time::Duration;

    use super::{PetalsEvent, PetalsService, ServiceAction};
    use crate::layout::UiTree;

    pub type DecodePetalsEvent = fn(stem::KindId, &[u8]) -> Option<PetalsEvent>;

    pub fn run_with_stem_service_loop<S>(
        tree: &mut UiTree,
        service: &mut S,
        decode: DecodePetalsEvent,
        timeout: Option<Duration>,
    ) -> Result<(), stem::Errno>
    where
        S: PetalsService,
    {
        let mut svc = ServiceLoop::new(256)?;
        svc.set_name("petals");
        svc.run(
            |event| match event {
                ServiceEvent::Message { kind, payload, .. } => {
                    if let Some(decoded) = decode(kind, payload) {
                        ControlFlow::from(service.dispatch(tree, decoded))
                    } else {
                        ControlFlow::Continue(())
                    }
                }
                ServiceEvent::InboxClosed => {
                    ControlFlow::from(service.dispatch(tree, PetalsEvent::Shutdown))
                }
                ServiceEvent::Timeout => {
                    ControlFlow::from(service.dispatch(tree, PetalsEvent::Layout))
                }
                ServiceEvent::Ready { .. } => ControlFlow::Continue(()),
            },
            timeout,
        )
    }

    pub struct BasicPetalsService;

    impl PetalsService for BasicPetalsService {
        fn dispatch(&mut self, tree: &mut UiTree, event: PetalsEvent) -> ServiceAction {
            match event {
                PetalsEvent::SetState { node, state, enabled } => {
                    tree.set_state(node, state, enabled);
                    ServiceAction::Continue
                }
                PetalsEvent::Shutdown => ServiceAction::Shutdown,
                PetalsEvent::Restyle | PetalsEvent::Layout => ServiceAction::Continue,
            }
        }
    }
}
