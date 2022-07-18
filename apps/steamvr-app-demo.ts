const applicationChannel = new BroadcastChannel("steamvr.applicationName");

applicationChannel.addEventListener("message", (e) => console.log(e.data));
