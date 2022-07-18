// This code just writes the currently active application to the console.
const applicationChannel = new BroadcastChannel("steamvr.applicationName");

applicationChannel.addEventListener("message", (e) => console.log(e.data));
