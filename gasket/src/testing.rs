use crate::messaging::tokio::InputPort;

pub async fn compare_inbound_sequence<M, I>(input: &mut InputPort<M>, expected: I)
where
    M: Send + Sync + PartialEq + std::fmt::Debug + Clone,
    I: IntoIterator<Item = M>,
{
    for right in expected.into_iter() {
        let msg = input.recv().await.unwrap();
        assert_eq!(msg.payload, right);
    }
}

#[macro_export]
macro_rules! quick_output_test {
    ($stage:ident.$port:ident, $expected:expr) => {{
        let mut input = $crate::InputPort::default();

        $crate::messaging::tokio::connect_ports(&mut $stage.$port, &mut input, 0);

        let tether = $crate::spawn_stage($stage);

        $crate::testing::compare_inbound_sequence(&mut input, $expected);

        tether
    }};
}
