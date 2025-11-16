import { Persistence } from '../core/ports';
import { Wallet } from '../core/domain';
import { promises as fs } from 'fs';
import * as path from 'path';

const WALLET_FILE = 'wallet.json';

export class FilePersistence implements Persistence {
  private walletFilePath: string;

  constructor(dataDir: string) {
    this.walletFilePath = path.join(dataDir, WALLET_FILE);
  }

  async saveWallet(wallet: Wallet): Promise<void> {
    // In a real implementation, the wallet would be encrypted before saving.
    const walletJson = JSON.stringify(wallet, null, 2);
    await fs.writeFile(this.walletFilePath, walletJson, 'utf-8');
  }

  async loadWallet(): Promise<Wallet> {
    try {
      const walletJson = await fs.readFile(this.walletFilePath, 'utf-8');
      // In a real implementation, the wallet would be decrypted after loading.
      return JSON.parse(walletJson) as Wallet;
    } catch (error: any) {
      if (error.code === 'ENOENT') {
        // Handle the case where the wallet file doesn't exist.
        // This could mean it's a new user, or the file is in a different location.
        throw new Error('Wallet file not found.');
      }
      throw error;
    }
  }
}
