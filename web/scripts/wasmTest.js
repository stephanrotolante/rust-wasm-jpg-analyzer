// (async function (d) {




//     const instance = new WebAssembly.Instance(module);

//     instance.exports.hello();
// })(document)


// import("/rust_wasm_jpg_analyzer.js").then(async (m) => {
//     console.log("here", m)

//     const response = await fetch('http://localhost:4005/rust_wasm_jpg_analyzer_bg.wasm');
//     const buffer = await response.arrayBuffer();
//     const wasmModule = new WebAssembly.Module(buffer);
//     m.initSync(wasmModule)
//     m.hello()
// })

(function () {
    let num = 0
    num += 1
})()