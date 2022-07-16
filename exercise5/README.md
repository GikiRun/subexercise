# OneBlock+ Substrate 入⻔课第9期 第5次作业
## 作业题目及要求
> 第5题 必填实现存证模块的功能，包括：创建存证；撤销存证。
> 第6题 为存证模块添加新的功能，转移存证，接收两个参数，一个是内容的哈希值，另一个是存证的接收账户地址。

## 编译
``` rust
cargo build --release
```
## 运行
``` rust
./target/release/node-template purge-chain --dev
./target/release/node-template --dev
```
1. 浏览器打开 polkadot.js.org/apps
2. Switch Local Node
3. Developer -> Extrinsics
4. using Alice account
5. submit the extrinsic : poe -> createClaim / revokeClaim / transferClaim (to Bob)
