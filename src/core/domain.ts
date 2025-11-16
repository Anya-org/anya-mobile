export interface Wallet {
  id: string;
  encryptedSeed: string;
  accounts: Account[];
}

export interface Account {
  id: string;
  name: string;
  address: Address;
  privateKey: PrivateKey;
}

export interface Transaction {
  id: TransactionID;
  from: Address;
  to: Address;
  asset: Asset;
  amount: Amount;
  fee: Amount;
  timestamp: number;
}

export interface Asset {
  symbol: string;
  name: string;
  decimals: number;
}

export interface Amount {
  value: string;
  asset: Asset;
}

export type Address = string;
export type PrivateKey = string;
export type TransactionID = string;

export interface Balance {
  asset: Asset;
  amount: Amount;
}

export interface StakingOption {
  id: string;
  name: string;
  asset: Asset;
  apy: number;
}

export interface StakingPosition {
  id: string;
  option: StakingOption;
  amount: Amount;
  rewards: Amount;
}

export interface FeeEstimates {
  slow: Amount;
  medium: Amount;
  fast: Amount;
}

export interface Price {
  asset: Asset;
  currency: string;
  value: string;
}

export enum WalletStatus {
  Locked,
  Unlocked,
}
