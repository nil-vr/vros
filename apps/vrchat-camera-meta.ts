// The `vros-vrchat-logs` module publishes to these channels.
const worldChannel = new BroadcastChannel("vrchat.world");
const playersChannel = new BroadcastChannel("vrchat.players");
const cameraChannel = new BroadcastChannel("vrchat.camera");

// deno-lint-ignore no-explicit-any
let world: any | null = null;
// deno-lint-ignore no-explicit-any
let players: Array<any> = [];

// Remember the latest world and player information.
worldChannel.addEventListener("message", (e) => world = e.data);
playersChannel.addEventListener("message", (e) => players = e.data.players);

// When a picture is taken, create a .meta file containing the world and players.
cameraChannel.addEventListener("message", async (e) => {
  const metaFile = await Deno.open(`${e.data.path}.meta`, {
    createNew: true,
    write: true,
  });
  try {
    await metaFile.write(new TextEncoder().encode(JSON.stringify({
      world,
      players,
    })));
  } finally {
    metaFile.close();
  }
});
