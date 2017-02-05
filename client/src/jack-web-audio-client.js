var JackWebAudio = (function () {

    var ws = null;

    var wsOpen = function() {
	console.log("open");
    }

    var wsClose = function() {
	console.log("close");
    }

    var wsMessage = function(msg) {
	console.log("message");
    }
    
    var close = function() {
	if (ws != null) {
	    ws.close();
	    ws = null;
	}
    };
    
    var open = function(url) {
	close();
	ws = new WebSocket(url);
	ws.onopen = wsOpen;
	ws.onclose = wsClose;
	ws.onmessage = wsMessage;
    }

    return {
	open: open,
	close: close,
    }

})();
