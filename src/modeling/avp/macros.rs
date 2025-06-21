#[macro_export]
macro_rules! impl_avp_data_encode_to_numbers {
    ($ty:ty, $out:ty) => {
        fn encode_to<W: std::io::Write>(&mut self, writer: &mut W) -> DiameterResult<()> {
            writer.write_all(&self.0.to_be_bytes())?;
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! impl_avp_data_encode_to_address {
    ($ty:ty, $out:ty) => {
        fn encode_to<W: std::io::Write>(&mut self, writer: &mut W) -> DiameterResult<()> {
            writer.write_all(&self.0.octets())?;
            Ok(())
        }
    };
}
