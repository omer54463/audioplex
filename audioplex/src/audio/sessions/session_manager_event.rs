#[derive(Debug)]
pub(crate) enum SessionManagerEvent {
    Add { process_id: usize },
}
