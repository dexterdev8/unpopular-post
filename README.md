# unpopular-post

### Facing issues:

- While deploying the Smart Contract with terrain:

  ` Error: Request failed with status code 400 Response: failed to execute message; message index: 0: Error calling the VM: Error executing Wasm: Wasmer runtime error: RuntimeError: unreachable: instantiate wasm contract failed: invalid request`

So I decided to use the terra-sdk.py in order to double check the error and see if I can reproduce it also in there.

Result:

`terra_sdk.exceptions.LCDResponseError: Status 400 - failed to execute message; message index: 0: Error calling the VM: Error executing Wasm: Wasmer runtime error: RuntimeError: unreachable: instantiate wasm contract failed: invalid request`

Seems like it can be reproduce.
