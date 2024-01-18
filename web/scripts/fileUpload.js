(function (d) {
    setTimeout(() => {
        function handleFileSelect(evt) {
            let file = evt.target.files[0];

            let reader = new FileReader();

            reader.onload = function (e) {
                const data = new Int8Array(e.target.result)
                import("/rust_wasm_jpg_analyzer.js").then(async (importedModule) => {
                    const response = await fetch('http://localhost:4005/rust_wasm_jpg_analyzer_bg.wasm');
                    const buffer = await response.arrayBuffer();
                    const wasmModule = new WebAssembly.Module(buffer);
                    importedModule.initSync(wasmModule)
                    importedModule.decode(data)
                })
            };

            reader.readAsArrayBuffer(file);
        }
        d.getElementById("jpg_file_uploader").addEventListener(
            "change",
            handleFileSelect
        );
    }, 10);
})(document);
