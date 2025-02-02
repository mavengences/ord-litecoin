use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Restore {
  #[arg(help = "Restore wallet from <MNEMONIC>")]
  mnemonic: Mnemonic,
  #[arg(
    long,
    default_value = "",
    help = "Use <PASSPHRASE> when deriving wallet"
  )]
  pub(crate) passphrase: String,
}

impl Restore {
  pub(crate) fn run(self, _wallet_name: String, _options: Options) -> SubcommandResult {
    bail!(
      "Descriptor wallets are not supported in Litecoincore 21.2.1, copy your wallet.dat into \
      your Litecoincore data directory."
    );

    // initialize_wallet(&options, self.mnemonic.to_seed(self.passphrase))?;
    // Ok(Box::new(Empty {}))
  }
}
