const socket = Deno.listenDatagram({ transport: "udp", port: 0 });
const channel = new BroadcastChannel("vrchat.osc.out");

const TYPE_FLOAT = new TextEncoder().encode(",f\0\0");
const TYPE_INT = new TextEncoder().encode(",i\0\0");

const DESTINATION: Deno.Addr = { hostname: "127.0.0.1", port: 9000, transport: "udp" };

let chain: Promise<number> | null = null;

channel.addEventListener("message", (e) => {
    const { address, type, value } = e.data;
    if (type !== "Int" && type !== "Bool" && type !== "Float") {
        console.error(`Ignoring OSC packet with unsupported type "${type}"`);
        return;
    }
    const addressBytes = new TextEncoder().encode(address);
    const paddedAddressLength = (addressBytes.length + 4) & ~3;
    const buffer = new Uint8Array(paddedAddressLength + 8);

    buffer.set(addressBytes, 0);

    buffer.fill(0, address.length, paddedAddressLength);
    const view = new DataView(buffer.buffer, paddedAddressLength + 4, 4);
    if (type === "Float") {
        buffer.set(TYPE_FLOAT, paddedAddressLength);
        view.setFloat32(0, value, false);
    } else {
        buffer.set(TYPE_INT, paddedAddressLength);
        view.setInt32(0, value, false);
    }

    let myPromise: Promise<number> | null = null;
    if (chain === null) {
        myPromise = socket.send(buffer, DESTINATION).finally(() => { if (chain === myPromise) { chain = null } });
    } else {
        myPromise = chain.finally(() => socket.send(buffer, DESTINATION)).finally(() => { if (chain === myPromise) { chain = null } });
    }
});
