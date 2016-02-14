var ffi = require('ffi');

var lib = ffi.Library('target/release/libembed.so', {
  'process': ['void', []]
});

lib.process();

console.log('done!');
