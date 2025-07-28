import * as config from '/config.js';

/// 64K shared memory
const memory = new WebAssembly.Memory({ initial: 1 });

const importObject = {
    console: {
        log: (addr) => {
            const mem = new Uint8Array(memory.buffer);
        },
    },
    js: { mem: memory },
};

WebAssembly.instantiateStreaming(fetch('/hello.wasm'), importObject)
    .then((obj) => {
        console.log(obj);
    })
    .catch(console.error);

//     obj.instance.exports.hello();
// })

$(() => {
    console.log(memory);
    let screen_ = $('#screen_');
    let input = $('#input');
    screen_.css({
        width: config.width,
        height: config.height,
        right: config.icon_size * 1.1,
    });
    input.css({
        right: 0,
        top: screen_.height + config.icon_size * 1.1,
    });
});
