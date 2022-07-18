// This code just writes video URLs to the console.
const videoChannel = new BroadcastChannel("vrchat.videoResolve");

videoChannel.addEventListener("message", (e) => console.log(e.data));
