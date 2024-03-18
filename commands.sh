# Step 0: Show running in wasmcloud
export WASH_ISSUER_KEY=SAAG6UMZDGRBNKMADYAZXZJVW3FDXSPJ2RWV6X3UITBGYPU7QTTHOTTNCQ
export WASH_SUBJECT_KEY=SMAEZNJZA3R3GEE2ZF6PXTTUQ6XWCEAXAMXOAS5PKHAHLH4GLISWZCRHT4
cd http-hello
wash build

# See wit world
wasm-tools component wit build/http_hello_world_s.wasm
wash start component file:///Users/taylor/Documents/code/wasmdayeu2024-demo/http-hello/build/http_hello_world_s.wasm http-hello

wash start provider file:///Users/taylor/Documents/code/wasmCloud/crates/providers/http-server/build/httpserver.par.gz http-server
wash config put hello-http ADDRESS=0.0.0.0:8081

wash link put --interface incoming-handler --source-config hello-http http-server http-hello wasi http

# Step 1: running in wasmtime

wasmtime serve -S common=y build/http_hello_world_s.wasm

# Step 2: Key value
cd http-hello2
wash build

# See wit world
wash update component http-hello file:///Users/taylor/Documents/code/wasmdayeu2024-demo/http-hello2/build/http_hello_world_s.wasm

wash start provider file:///Users/taylor/Documents/code/wasmCloud/crates/providers/kv-redis/build/kvredis.par.gz redis
wash config put hello-kv URL=redis://127.0.0.1:6379
wash link put --interface atomic --target-config hello-kv http-hello redis wasi keyvalue

# Step 3: Keyvalue wrapped
cd mock-kv

# See wit world
wasm-tools component wit ../http-hello2/build/http_hello_world_s.wasm
wac encode --dep mock:kv=./build/mock_kv_s.wasm --dep hello:kv=../http-hello2/build/http_hello_world_s.wasm --dep mock:transitive=../transitive/build/transitive_s.wasm -o output.wasm compose.wac
wasm-tools component wit output.wasm
wasmtime serve -S common=y output.wasm

# Step 4: Add a custom pong interface
cd http-hello3

wash update component http-hello file:///Users/taylor/Documents/code/wasmdayeu2024-demo/http-hello3/build/http_hello_world_s.wasm

# Step 5: Creating and running a ponger with virtualization

cd pong

wasm-tools component wit ./build/pong_s.wasm

wasi-virt build/pong_s.wasm --allow-random -e PONG=wasmday -o virt.wasm
wasm-tools component wit virt.wasm 

wash start component file:///Users/taylor/Documents/code/wasmdayeu2024-demo/pong/virt.wasm ponger
wash link put --interface pingpong http-hello ponger example pong

# Step 6: Composing the component for wasmtime

wac encode --dep ping:pong=./pong/virt.wasm --dep hello:there=./http-hello3/build/http_hello_world_s.wasm --dep mock:kv=./mock-kv/build/mock_kv_s.wasm --dep mock:transitive=./transitive/build/transitive_s.wasm -o output.wasm alltehthingz.wac
