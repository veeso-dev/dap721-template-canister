export const idlFactory = ({ IDL }) => {
  const GenericValue = IDL.Rec();
  const SupportedInterface = IDL.Variant({
    'Burn' : IDL.Null,
    'Mint' : IDL.Null,
    'Approval' : IDL.Null,
    'TransactionHistory' : IDL.Null,
  });
  const CanisterInitData = IDL.Record({
    'logo' : IDL.Opt(IDL.Text),
    'name' : IDL.Text,
    'custodians' : IDL.Vec(IDL.Principal),
    'symbol' : IDL.Text,
    'supported_interfaces' : IDL.Vec(SupportedInterface),
  });
  const NftError = IDL.Variant({
    'UnauthorizedOperator' : IDL.Null,
    'SelfTransfer' : IDL.Null,
    'TokenNotFound' : IDL.Null,
    'UnauthorizedOwner' : IDL.Null,
    'TxNotFound' : IDL.Null,
    'SelfApprove' : IDL.Null,
    'OperatorNotFound' : IDL.Null,
    'ExistedNFT' : IDL.Null,
    'OwnerNotFound' : IDL.Null,
    'Other' : IDL.Text,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : NftError });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : NftError });
  const Metadata = IDL.Record({
    'logo' : IDL.Opt(IDL.Text),
    'name' : IDL.Opt(IDL.Text),
    'created_at' : IDL.Nat64,
    'upgraded_at' : IDL.Nat64,
    'custodians' : IDL.Vec(IDL.Principal),
    'symbol' : IDL.Opt(IDL.Text),
  });
  GenericValue.fill(
    IDL.Variant({
      'Nat64Content' : IDL.Nat64,
      'Nat32Content' : IDL.Nat32,
      'BoolContent' : IDL.Bool,
      'Nat8Content' : IDL.Nat8,
      'Int64Content' : IDL.Int64,
      'IntContent' : IDL.Int,
      'NatContent' : IDL.Nat,
      'Nat16Content' : IDL.Nat16,
      'Int32Content' : IDL.Int32,
      'Int8Content' : IDL.Int8,
      'FloatContent' : IDL.Float64,
      'Int16Content' : IDL.Int16,
      'BlobContent' : IDL.Vec(IDL.Nat8),
      'NestedContent' : IDL.Vec(IDL.Tuple(IDL.Text, GenericValue)),
      'Principal' : IDL.Principal,
      'TextContent' : IDL.Text,
    })
  );
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Opt(IDL.Principal),
    'Err' : NftError,
  });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Vec(IDL.Nat), 'Err' : NftError });
  const TokenMetadata = IDL.Record({
    'transferred_at' : IDL.Opt(IDL.Nat64),
    'transferred_by' : IDL.Opt(IDL.Principal),
    'owner' : IDL.Opt(IDL.Principal),
    'operator' : IDL.Opt(IDL.Principal),
    'approved_at' : IDL.Opt(IDL.Nat64),
    'approved_by' : IDL.Opt(IDL.Principal),
    'properties' : IDL.Vec(IDL.Tuple(IDL.Text, GenericValue)),
    'is_burned' : IDL.Bool,
    'token_identifier' : IDL.Nat,
    'burned_at' : IDL.Opt(IDL.Nat64),
    'burned_by' : IDL.Opt(IDL.Principal),
    'minted_at' : IDL.Nat64,
    'minted_by' : IDL.Principal,
  });
  const Result_4 = IDL.Variant({
    'Ok' : IDL.Vec(TokenMetadata),
    'Err' : NftError,
  });
  const Stats = IDL.Record({
    'cycles' : IDL.Nat,
    'total_transactions' : IDL.Nat,
    'total_unique_holders' : IDL.Nat,
    'total_supply' : IDL.Nat,
  });
  const Result_5 = IDL.Variant({ 'Ok' : TokenMetadata, 'Err' : NftError });
  const TxEvent = IDL.Record({
    'time' : IDL.Nat64,
    'operation' : IDL.Text,
    'details' : IDL.Vec(IDL.Tuple(IDL.Text, GenericValue)),
    'caller' : IDL.Principal,
  });
  const Result_6 = IDL.Variant({ 'Ok' : TxEvent, 'Err' : NftError });
  const Result_7 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : NftError });
  return IDL.Service({
    'dip721_approve' : IDL.Func([IDL.Principal, IDL.Nat], [Result], []),
    'dip721_balance_of' : IDL.Func([IDL.Principal], [Result], ['query']),
    'dip721_burn' : IDL.Func([IDL.Nat], [Result], []),
    'dip721_custodians' : IDL.Func([], [IDL.Vec(IDL.Principal)], ['query']),
    'dip721_cycles' : IDL.Func([], [IDL.Nat], ['query']),
    'dip721_is_approved_for_all' : IDL.Func(
        [IDL.Principal, IDL.Principal],
        [Result_1],
        [],
      ),
    'dip721_logo' : IDL.Func([], [IDL.Opt(IDL.Text)], ['query']),
    'dip721_metadata' : IDL.Func([], [Metadata], ['query']),
    'dip721_mint' : IDL.Func(
        [IDL.Principal, IDL.Nat, IDL.Vec(IDL.Tuple(IDL.Text, GenericValue))],
        [Result],
        [],
      ),
    'dip721_name' : IDL.Func([], [IDL.Opt(IDL.Text)], ['query']),
    'dip721_operator_of' : IDL.Func([IDL.Nat], [Result_2], ['query']),
    'dip721_operator_token_identifiers' : IDL.Func(
        [IDL.Principal],
        [Result_3],
        ['query'],
      ),
    'dip721_operator_token_metadata' : IDL.Func(
        [IDL.Principal],
        [Result_4],
        ['query'],
      ),
    'dip721_owner_of' : IDL.Func([IDL.Nat], [Result_2], ['query']),
    'dip721_owner_token_identifiers' : IDL.Func(
        [IDL.Principal],
        [Result_3],
        ['query'],
      ),
    'dip721_owner_token_metadata' : IDL.Func(
        [IDL.Principal],
        [Result_4],
        ['query'],
      ),
    'dip721_set_approval_for_all' : IDL.Func(
        [IDL.Principal, IDL.Bool],
        [Result],
        [],
      ),
    'dip721_set_custodians' : IDL.Func([IDL.Vec(IDL.Principal)], [], []),
    'dip721_set_logo' : IDL.Func([IDL.Text], [], []),
    'dip721_set_name' : IDL.Func([IDL.Text], [], []),
    'dip721_set_symbol' : IDL.Func([IDL.Text], [], []),
    'dip721_stats' : IDL.Func([], [Stats], ['query']),
    'dip721_supported_interfaces' : IDL.Func(
        [],
        [IDL.Vec(SupportedInterface)],
        ['query'],
      ),
    'dip721_symbol' : IDL.Func([], [IDL.Opt(IDL.Text)], ['query']),
    'dip721_token_metadata' : IDL.Func([IDL.Nat], [Result_5], ['query']),
    'dip721_total_supply' : IDL.Func([], [IDL.Nat], ['query']),
    'dip721_total_transactions' : IDL.Func([], [IDL.Nat], ['query']),
    'dip721_total_unique_holders' : IDL.Func([], [IDL.Nat], ['query']),
    'dip721_transaction' : IDL.Func([IDL.Nat], [Result_6], ['query']),
    'dip721_transfer' : IDL.Func([IDL.Principal, IDL.Nat], [Result], []),
    'dip721_transfer_from' : IDL.Func(
        [IDL.Principal, IDL.Principal, IDL.Nat],
        [Result],
        [],
      ),
    'set_token_property' : IDL.Func(
        [IDL.Nat, IDL.Text, GenericValue],
        [Result_7],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  const SupportedInterface = IDL.Variant({
    'Burn' : IDL.Null,
    'Mint' : IDL.Null,
    'Approval' : IDL.Null,
    'TransactionHistory' : IDL.Null,
  });
  const CanisterInitData = IDL.Record({
    'logo' : IDL.Opt(IDL.Text),
    'name' : IDL.Text,
    'custodians' : IDL.Vec(IDL.Principal),
    'symbol' : IDL.Text,
    'supported_interfaces' : IDL.Vec(SupportedInterface),
  });
  return [CanisterInitData];
};
