import { WalletService } from '../core/ports';
import { Wallet, WalletStatus } from '../core/domain';
import * as bip39 from 'bip39';
import { hdkey } from '@ethereumjs/wallet';

export class WalletServiceImpl implements WalletService {
  private wallet: Wallet | null = null;
  private status: WalletStatus = WalletStatus.Locked;

  async createWallet(passphrase: string): Promise<Wallet> {
    const mnemonic = bip39.generateMnemonic();
    const seed = await bip39.mnemonicToSeed(mnemonic);
    const hdWallet = hdkey.EthereumHDKey.fromMasterSeed(seed);
    const wallet: Wallet = {
      id: 'wallet-1',
      encryptedSeed: hdWallet.privateExtendedKey(),
      accounts: [],
    };
    this.wallet = wallet;
    this.status = WalletStatus.Unlocked;
    return wallet;
  }

  async loadWallet(passphrase: string): Promise<Wallet> {
    // This is a mock implementation. In a real wallet, you would load the
    // encrypted seed from persistence and decrypt it.
    if (!this.wallet) {
      throw new Error('Wallet not found.');
    }
    this.status = WalletStatus.Unlocked;
    return this.wallet;
  }

  async lockWallet(): Promise<void> {
    this.status = WalletStatus.Locked;
  }

  async unlockWallet(passphrase: string): Promise<void> {
    // This is a mock implementation. In a real wallet, you would decrypt the
    // seed with the passphrase.
    if (!this.wallet) {
      throw new Error('Wallet not found.');
    }
    this.status = WalletStatus.Unlocked;
  }

  async getWalletStatus(): Promise<WalletStatus> {
    return this.status;
  }
}
