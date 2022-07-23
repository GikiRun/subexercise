# OneBlock+ Substrate 入⻔课第9期 第6次作业 附加题
## 作业题目及要求
> 第13题 使用polkadot js api来写一个订阅Event的程序

## 新建项目
``` 
yarn init
yarn add @polkadot/api
touch main.ts
```
## 运行
``` 
./target/release/node-template purge-chain --dev
./target/release/node-template --dev
yarn ts-node main.ts
```
1. 浏览器打开 polkadot.js.org/apps
2. Switch Local Node
3. Developer -> Extrinsics
4. using Alice account
5. submit the extrinsic : poe -> createClaim / revokeClaim 
6. 观察命令行输出系统事件
