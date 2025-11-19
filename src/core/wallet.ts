import * as bip39 from 'bip39';
import { Wallet } from '@ethereumjs/wallet';

export interface AnyaWallet {
  mnemonic: string;
  privateKey: string;
  address: string;
}

export function createWallet(): AnyaWallet {
  const mnemonic = bip39.generateMnemonic();
  const seed = bip39.mnemonicToSeedSync(mnemonic);
  const wallet = Wallet.fromPrivateKey(seed.slice(0, 32));

  return {
    mnemonic,
    privateKey: wallet.getPrivateKeyString(),
    address: wallet.getAddressString(),
  };
}
