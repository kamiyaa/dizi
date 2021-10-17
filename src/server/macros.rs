macro_rules! simple_server_request {
    ($function_name: ident, $enum_name: expr) => {
        pub fn $function_name(client_request_tx: &ClientRequestSender) -> DiziResult<()> {
            client_request_tx.send($enum_name)?;
            Ok(())
        }
    };
}

pub(crate) use simple_server_request;
