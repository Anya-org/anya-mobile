import { BlockchainClient } from '../core/ports';
import {
  Address,
  Asset,
  TransactionID,
  Amount,
  Balance,
  Transaction,
  FeeEstimates,
} from '../core/domain';

export class MockBlockchainClient implements BlockchainClient {
  async getBalance(address: Address, asset: Asset): Promise<Balance> {
    // Return a mock balance for testing purposes.
    return {
      asset,
      amount: { asset, value: '1.23' },
    };
  }

  async getTransaction(transactionID: TransactionID): Promise<Transaction> {
    // Return a mock transaction for testing purposes.
    const mockAsset: Asset = { symbol: 'BTC', name: 'Bitcoin', decimals: 8 };
    return {
      id: transactionID,
      from: 'mock-from-address',
      to: 'mock-to-address',
      asset: mockAsset,
      amount: { asset: mockAsset, value: '0.1' },
      fee: { asset: mockAsset, value: '0.0001' },
      timestamp: Date.now(),
    };
  }

  async broadcastTransaction(
    signedTransaction: Transaction
  ): Promise<TransactionID> {
    // Return a mock transaction ID for testing purposes.
    return 'mock-transaction-id';
  }

  async getFeeEstimates(asset: Asset): Promise<FeeEstimates> {
    // Return mock fee estimates for testing purposes.
    return {
      slow: { asset, value: '0.00001' },
      medium: { asset, value: '0.00002' },
      fast: { asset, value: '0.00003' },
    };
  }
}
