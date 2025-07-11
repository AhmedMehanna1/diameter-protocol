use diameter::errors::DiameterResult;
use diameter::modeling::avp::avp::Avp;
use diameter::modeling::avp::avp::AvpFlags::M;
use diameter::modeling::avp::enumerated::Enumerated;
use diameter::modeling::avp::unsigned32::Unsigned32;
use diameter::modeling::avp::utf8_string::{Identity, UTF8String};
use diameter::modeling::diameter::DiameterMessage;
use diameter::modeling::message::application_id::ApplicationId;
use diameter::modeling::message::command_code::CommandCode;
use diameter::modeling::message::command_flags::CommandFlag;
use diameter::modeling::message::dictionary;
use diameter::modeling::message::dictionary::Dictionary;
use diameter::transport::client::DiameterClient;
use std::sync::Arc;

fn main() -> DiameterResult<()> {
    let dict = Arc::new(Dictionary::new(&[&dictionary::DEFAULT_DICT_XML]));

    let mut ccr: DiameterMessage = DiameterMessage::new(
        CommandFlag::Request,
        CommandCode::CreditControl,
        ApplicationId::Gx,
        1123158611,
        3102381851,
    );

    ccr.add_avp(Avp::new(263, M, None, UTF8String::from_str("a")));
    ccr.add_avp(Avp::new(
        264,
        M,
        None,
        Identity::from_str("host.example.com"),
    ));
    ccr.add_avp(Avp::new(
        296,
        M,
        None,
        Identity::from_str("realm.example.com"),
    ));
    ccr.add_avp(Avp::new(263, M, None, UTF8String::from_str("ses;12345888")));
    ccr.add_avp(Avp::new(416, M, None, Enumerated::new(1)));
    ccr.add_avp(Avp::new(415, M, None, Unsigned32::new(1000)));
    ccr.add_avp(Avp::new(
        264,
        M,
        None,
        Identity::from_str("host.example.com"),
    ));
    ccr.add_avp(Avp::new(
        296,
        M,
        None,
        Identity::from_str("realm.example.com"),
    ));
    ccr.add_avp(Avp::new(416, M, None, Enumerated::new(1)));
    ccr.add_avp(Avp::new(415, M, None, Unsigned32::new(1000)));

    let mut client = DiameterClient::new("127.0.0.1:3868");
    client.connect()?;
    client.send_message(&mut ccr, dict)?;
    client.close()?;
    Ok(())
}
