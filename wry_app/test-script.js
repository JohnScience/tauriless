const req = new XMLHttpRequest();
req.open("POST", "http://async.localhost", true);
req.responseType = "arraybuffer";
req.onload = function() {
    console.log(req.response);
}
const uint8 = new Uint8Array(2);
req.send(uint8);