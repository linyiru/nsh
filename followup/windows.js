ObjC.import('CoreGraphics');
var wins=ObjC.deepUnwrap(ObjC.castRefToObject($.CGWindowListCopyWindowInfo(1,0)));
var own=wins.filter(w=>w.kCGWindowName==='NSH Canvas Review');
if(own.length!==1) throw new Error('Expected one review window, found '+own.length);
console.log(own[0].kCGWindowNumber);
JSON.stringify({window_id:own[0].kCGWindowNumber,bounds:own[0].kCGWindowBounds});
