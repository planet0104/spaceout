function loadResources(srcMap, cb, listener){
    var resMap = new Map();
    function check(listener){
        if(listener)
            listener(resMap.size, srcMap.size);
        if(srcMap.size == resMap.size){
            console.log('Resources Load Complete.');
            cb(resMap);
        }
    }
    for(var [key, url] of srcMap.entries()){
        if(url.endsWith('.mid')){
            function f(){
                var thatKey = key;
                fetch(url).then(response => response.arrayBuffer()).then(function(bytes){
                    resMap.set(thatKey, bytes);
                    check(listener);
                });
            }(f());
        }else if(url.endsWith('.ogg') || url.endsWith('.mp3')){
            function f(){
                var thatKey = key;
                fetch(url).then(response => response.arrayBuffer()).then(function(bytes){
                    audioCtx.decodeAudioData(bytes, function(buffer){
                        resMap.set(thatKey, buffer);
                        check(listener);
                    }, function(err){
                        console.log('err', err);
                        check(listener);
                    });
                });
            }(f());
        }else{
            var image = new Image();
            image.key = key;
            image.src = url;
            image.onload = function(){
                resMap.set(this.key, this);
                check(listener);
            };
        }
    }
}

function copyCStr(module, ptr) {
    let orig_ptr = ptr;
    const collectCString = function* () {
        let memory = new Uint8Array(module.memory.buffer);
        while (memory[ptr] !== 0) {
            if (memory[ptr] === undefined) { throw new Error("Tried to read undef mem") }
            yield memory[ptr]
            ptr += 1
        }
    }

    const buffer_as_u8 = new Uint8Array(collectCString())
    const utf8Decoder = new TextDecoder("UTF-8");
    const buffer_as_utf8 = utf8Decoder.decode(buffer_as_u8);
    module.dealloc_str(orig_ptr);
    return buffer_as_utf8
}