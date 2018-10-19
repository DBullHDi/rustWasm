import("./pkg/stringPack")
    .then(wasmModule => {
        wasmModule.run();
    });