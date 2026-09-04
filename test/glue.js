const importObject = {
    env: {
        print_str: function(offset, length) {
            const bytes = new Uint8Array(memory.buffer, offset, length);
            const string = new TextDecoder('utf8').decode(bytes);
            console.log(string);
        },
        
        print_num: function(number) {
            console.log("Risultato del calcolo:", number.toString());
        }
    }
};

WebAssembly.instantiateStreaming(fetch('main.wasm'), importObject)
    .then(obj => {
        memory = obj.instance.exports.memory;
        
        obj.instance.exports.main();
    });
