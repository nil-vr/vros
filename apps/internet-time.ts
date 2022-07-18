// This code publishes the current Swatch Internet Time to a VRChat avatar parameter called time_beats.
// http://www.swatchclock.com/

const oscOut = new BroadcastChannel("vrchat.osc.out");
const GRAY = [0, 1, 3, 2];

function go() {
    // Calculate the current beat.
    const now = new Date();
    const exact = (((((now.getUTCHours() + 1) % 24) * 60 + now.getUTCMinutes()) * 60 + now.getUTCSeconds()) * 1000 + now.getUTCMilliseconds()) / 86400;
    const beat = Math.floor(exact);

    // Figure out the time until the next beat.
    const delay = (1.0 - (exact - beat)) * 86400;
    setTimeout(go, delay);

    // VRChat avatar parameter integers are 8 bits. We need 10.
    oscOut.postMessage({ address: "/avatar/parameters/time_beats_low", type: "Int", value: beat & 0xff });
    const gray = GRAY[beat >> 8];
    oscOut.postMessage({ address: "/avatar/parameters/time_beats_g0", type: "Bool", value: !!(gray & 1) });
    oscOut.postMessage({ address: "/avatar/parameters/time_beats_g1", type: "Bool", value: !!(gray >> 1) });
}

globalThis.addEventListener("load", go);
