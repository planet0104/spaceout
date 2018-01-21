# spaceout a rust wasm game
Rust+WebAssembly开发的一个小游戏<br/>
游戏源码参考《游戏编程入门》：<br />
<img src="https://img3.doubanio.com/lpic/s26278533.jpg" /><br /><br />
Rust WebAssembly 入门参考这里：https://www.hellorust.com/setup/wasm-target/<br /><br />
代码结构:<br /><br />
<b>html部分:</b><br />
<i>html</i> 游戏资源文件和HTML文件<br /><br />
<b>游戏引擎部分:</b><br />
<i>src/engine.rs</i> 游戏引擎<br />
<i>src/sprite.rs</i> 精灵<br />
<i>src/timer.rs</i> 计时器<br /><br />
<b>SpaceOut游戏</b><br />
<i>src/game.rs</i> 游戏主代码<br />
<i>src/alien_sprite.rs</i> 外星人<br />
<i>src/backgkround.rs</i> 星星闪烁的背景<br /><br />
编译:<br />
cargo  +nightly build --target wasm32-unknown-unknown --release<br />
copy target\wasm32-unknown-unknown\release\spaceout.wasm html\spaceout.wasm<br />
然后浏览器打开html/index.html就可以运行了。<br />
