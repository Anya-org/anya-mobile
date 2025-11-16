import { WalletServiceImpl } from './wallet-service';
import { WalletStatus } from '../core/domain';

describe('WalletServiceImpl', () => {
  let walletService: WalletServiceImpl;

  beforeEach(() => {
    walletService = new WalletServiceImpl();
  });

  it('should create a new wallet', async () => {
    const wallet = await walletService.createWallet('password');
    expect(wallet).toBeDefined();
    expect(wallet.id).toBe('wallet-1');
    expect(wallet.encryptedSeed).toBeDefined();
    expect(wallet.accounts).toEqual([]);
    const status = await walletService.getWalletStatus();
    expect(status).toBe(WalletStatus.Unlocked);
  });

  it('should lock and unlock the wallet', async () => {
    await walletService.createWallet('password');
    await walletService.lockWallet();
    let status = await walletService.getWalletStatus();
    expect(status).toBe(WalletStatus.Locked);
    await walletService.unlockWallet('password');
    status = await walletService.getWalletStatus();
    expect(status).toBe(WalletStatus.Unlocked);
  });

  it('should load an existing wallet', async () => {
    const createdWallet = await walletService.createWallet('password');
    const loadedWallet = await walletService.loadWallet('password');
    expect(loadedWallet).toEqual(createdWallet);
  });
});
