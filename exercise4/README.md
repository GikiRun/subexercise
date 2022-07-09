# OneBlock+ Substrate 入⻔课第9期第四次作业
## 作业题目及要求
> 第4题 为枚举交通信号灯实现一个 trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同
> 第5题 实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None
> 第6题 实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束

### 编译
> cargo build --release
### 测试
``` rust
cargo test
```
### 运行
``` rust
cargo run --release --bin trafficlight
cargo run --release --bin trafficlight green
cargo run --release --bin trafficlight red
cargo run --release --bin trafficlight yellow
cargo run --release --bin trafficlight xxxxx

cargo run --release --bin sum
cargo run --release --bin sum 2 5 8
cargo run --release --bin sum 123 xxx

cargo run --release --bin area square 10
cargo run --release --bin area circle 3
cargo run --release --bin area triangle 6 7 5
```