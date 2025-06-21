# 计算 CLI 运行时间

## Linux time

只作为参考

https://www.runoob.com/linux/linux-comm-time.html

```shell
time [options] COMMAND [arguments]


参数：

-o 或 --output=FILE：设定结果输出档。这个选项会将 time 的输出写入 所指定的档案中。如果档案已经存在，系统将覆写其内容。
-a 或 --append：配合 -o 使用，会将结果写到档案的末端，而不会覆盖掉原来的内容。
-f FORMAT 或 --format=FORMAT：以 FORMAT 字串设定显示方式。当这个选项没有被设定的时候，会用系统预设的格式。不过你可以用环境变数 time 来设定这个格式，如此一来就不必每次登入系统都要设定一次。
```

## Build

```shell
cargo build --release

cp ./target/release/demo.exe d:/tools/demo.exe
cp ./target/release/time.exe d:/tools/time.exe

time -h

time demo 1 2 3 4 5 6

time -- cargo run --bin demo
```
