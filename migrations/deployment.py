import base64
from terra_sdk.client.lcd.api.tx import CreateTxOptions
from terra_sdk.client.localterra import LocalTerra
from terra_sdk.core.wasm import MsgStoreCode, MsgInstantiateContract, MsgExecuteContract
from terra_sdk.core.fee import Fee

terra = LocalTerra()
wallet = terra.wallets["test1"]
contract_file = open("../contracts/unpopular_post/artifacts/unpopular_post.wasm", "rb")

file_bytes = base64.b64encode(contract_file.read()).decode()
store_code = MsgStoreCode(wallet.key.acc_address, file_bytes)

store_code_tx = wallet.create_and_sign_tx(CreateTxOptions(msgs=[store_code], fee=Fee(2100000, "60000uluna")))
store_code_tx_result = terra.tx.broadcast(store_code_tx)
print(store_code_tx_result)

code_id = store_code_tx_result.logs[0].events_by_type["store_code"]["code_id"][0]
instantiate = MsgInstantiateContract(
    wallet.key.acc_address,
    wallet.key.acc_address,
    code_id,
    {},
    {"uluna": 10000000, "ukrw": 1000000},
)

instantiate_tx = wallet.create_and_sign_tx(CreateTxOptions(msgs=[instantiate]))
instantiate_tx_result = terra.tx.broadcast(instantiate_tx)
print(instantiate_tx_result)