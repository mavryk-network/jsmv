module Jsmv_bridge = struct

  type storage = 
    { rollup : address option
    ; ctez_contract : address
    }

  type deposit = 
    { tz4_address : bytes
    ; amount : nat
    }

  type rollup_type = (bytes * unit ticket)

  type return = operation list * storage

  type fa12_transfer = [@layout comb]
    { [@annot from] from_ : address
    ; [@annot to] to_ : address
    ; value : nat
    }

  [@entry] let deposit (deposit : deposit) (s : storage) : return =
      let from_ = Mavryk.get_sender () in
      let self = Mavryk.get_self_address () in
      let ctez_contract : fa12_transfer contract =
        Mavryk.get_entrypoint_opt "%transfer" s.ctez_contract
        |> Option.unopt 
      in
      let jsmv_rollup : rollup_type contract =
        match s.rollup with
        | None -> failwith "jsmv rollup address was not set"
        | Some rollup -> Mavryk.get_contract_opt rollup |> Option.unopt 
      in
      let ticket =
        match Mavryk.create_ticket () deposit.amount with
        | Some ticket -> ticket
        | None -> failwith "Amount must be > 0" 
      in
      let jsmv_deposit =
        Mavryk.transaction (deposit.tz4_address, ticket) 0mumav jsmv_rollup 
      in
      let ctez_transfer = 
        Mavryk.transaction 
          { from_; to_ = self; value = deposit.amount }
          0mumav 
          ctez_contract
      in
      [ctez_transfer; jsmv_deposit], s

  [@entry] let set_rollup (addr : address) (s : storage) : return =
      [], {s with rollup = Some addr}
      
end
