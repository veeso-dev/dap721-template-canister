import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface CanisterInitData {
  'logo' : string,
  'name' : string,
  'custodians' : Array<Principal>,
  'symbol' : string,
  'supported_interfaces' : Array<SupportedInterface>,
}
export type GenericValue = { 'Nat64Content' : bigint } |
  { 'Nat32Content' : number } |
  { 'BoolContent' : boolean } |
  { 'Nat8Content' : number } |
  { 'Int64Content' : bigint } |
  { 'IntContent' : bigint } |
  { 'NatContent' : bigint } |
  { 'Nat16Content' : number } |
  { 'Int32Content' : number } |
  { 'Int8Content' : number } |
  { 'FloatContent' : number } |
  { 'Int16Content' : number } |
  { 'BlobContent' : Uint8Array | number[] } |
  { 'NestedContent' : Array<[string, GenericValue]> } |
  { 'Principal' : Principal } |
  { 'TextContent' : string };
export interface Metadata {
  'logo' : [] | [string],
  'name' : [] | [string],
  'created_at' : bigint,
  'upgraded_at' : bigint,
  'custodians' : Array<Principal>,
  'symbol' : [] | [string],
}
export type NftError = { 'UnauthorizedOperator' : null } |
  { 'SelfTransfer' : null } |
  { 'TokenNotFound' : null } |
  { 'UnauthorizedOwner' : null } |
  { 'TxNotFound' : null } |
  { 'SelfApprove' : null } |
  { 'OperatorNotFound' : null } |
  { 'ExistedNFT' : null } |
  { 'OwnerNotFound' : null } |
  { 'Other' : string };
export type Result = { 'Ok' : bigint } |
  { 'Err' : NftError };
export type Result_1 = { 'Ok' : boolean } |
  { 'Err' : NftError };
export type Result_2 = { 'Ok' : [] | [Principal] } |
  { 'Err' : NftError };
export type Result_3 = { 'Ok' : Array<bigint> } |
  { 'Err' : NftError };
export type Result_4 = { 'Ok' : Array<TokenMetadata> } |
  { 'Err' : NftError };
export type Result_5 = { 'Ok' : null } |
  { 'Err' : NftError };
export type Result_6 = { 'Ok' : TokenMetadata } |
  { 'Err' : NftError };
export type Result_7 = { 'Ok' : TxEvent } |
  { 'Err' : NftError };
export interface Stats {
  'cycles' : bigint,
  'total_transactions' : bigint,
  'total_unique_holders' : bigint,
  'total_supply' : bigint,
}
export type SupportedInterface = { 'Burn' : null } |
  { 'Mint' : null } |
  { 'Approval' : null } |
  { 'TransactionHistory' : null };
export interface TokenMetadata {
  'transferred_at' : [] | [bigint],
  'transferred_by' : [] | [Principal],
  'owner' : [] | [Principal],
  'operator' : [] | [Principal],
  'approved_at' : [] | [bigint],
  'approved_by' : [] | [Principal],
  'properties' : Array<[string, GenericValue]>,
  'is_burned' : boolean,
  'token_identifier' : bigint,
  'burned_at' : [] | [bigint],
  'burned_by' : [] | [Principal],
  'minted_at' : bigint,
  'minted_by' : Principal,
}
export interface TxEvent {
  'time' : bigint,
  'operation' : string,
  'details' : Array<[string, GenericValue]>,
  'caller' : Principal,
}
export interface _SERVICE {
  'approve' : ActorMethod<[Principal, bigint], Result>,
  'balance_of' : ActorMethod<[Principal], Result>,
  'burn' : ActorMethod<[bigint], Result>,
  'custodians' : ActorMethod<[], Array<Principal>>,
  'cycles' : ActorMethod<[], bigint>,
  'is_approved_for_all' : ActorMethod<[Principal, Principal], Result_1>,
  'logo' : ActorMethod<[], [] | [string]>,
  'metadata' : ActorMethod<[], Metadata>,
  'mint' : ActorMethod<
    [Principal, bigint, Array<[string, GenericValue]>],
    Result
  >,
  'name' : ActorMethod<[], [] | [string]>,
  'operator_of' : ActorMethod<[bigint], Result_2>,
  'operator_token_identifiers' : ActorMethod<[Principal], Result_3>,
  'operator_token_metadata' : ActorMethod<[Principal], Result_4>,
  'owner_of' : ActorMethod<[bigint], Result_2>,
  'owner_token_identifiers' : ActorMethod<[Principal], Result_3>,
  'owner_token_metadata' : ActorMethod<[Principal], Result_4>,
  'set_approval_for_all' : ActorMethod<[Principal, boolean], Result>,
  'set_custodians' : ActorMethod<[Array<Principal>], undefined>,
  'set_logo' : ActorMethod<[string], undefined>,
  'set_name' : ActorMethod<[string], undefined>,
  'set_symbol' : ActorMethod<[string], undefined>,
  'set_token_property' : ActorMethod<[bigint, string, GenericValue], Result_5>,
  'stats' : ActorMethod<[], Stats>,
  'supported_interfaces' : ActorMethod<[], Array<SupportedInterface>>,
  'symbol' : ActorMethod<[], [] | [string]>,
  'token_metadata' : ActorMethod<[bigint], Result_6>,
  'total_supply' : ActorMethod<[], bigint>,
  'total_transactions' : ActorMethod<[], bigint>,
  'total_unique_holders' : ActorMethod<[], bigint>,
  'transaction' : ActorMethod<[bigint], Result_7>,
  'transfer' : ActorMethod<[Principal, bigint], Result>,
  'transfer_from' : ActorMethod<[Principal, Principal, bigint], Result>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: ({ IDL }: { IDL: IDL }) => IDL.Type[];
