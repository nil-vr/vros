import "../../vros/preamble.ts";

import { expandGlob } from "https://deno.land/std@0.148.0/fs/expand_glob.ts";
import { readStringDelim } from "https://deno.land/std@0.148.0/io/buffer.ts";
export { readStringDelim } from "https://deno.land/std@0.148.0/io/mod.ts";

const vrChatDir = window.vros.expandPath("${LocalAppDataLow}\\VRChat\\VRChat");

const worldChannel = new BroadcastChannel("vrchat.world");
const playersChannel = new BroadcastChannel("vrchat.players");
const videoChannel = new BroadcastChannel("vrchat.videoResolve");
const cameraChannel = new BroadcastChannel("vrchat.camera");

async function getLatestLog(): Promise<string | null> {
  let latest = null;
  let latestTime = null;
  const logs = expandGlob(
    `${vrChatDir}/output_log_[[:digit:]][[:digit:]]-[[:digit:]][[:digit:]]-[[:digit:]][[:digit:]].txt`,
  );
  for await (const log of logs) {
    if (!log.isFile) {
      continue;
    }
    const stat = await Deno.stat(log.path);
    if (
      latestTime === null ||
      (stat.birthtime !== null && latestTime < stat.birthtime)
    ) {
      latest = log.path;
      latestTime = stat.birthtime;
    }
  }

  return latest;
}

class Session {
  file: Deno.FsFile;
  players = new Set();
  world: string | null = null;
  isNew = true;
  timeout: number | null = null;

  constructor(file: Deno.FsFile) {
    this.file = file;
  }

  async catchUp(suppressEvents: boolean) {
    let playersChanged = false;
    let worldChanged = false;

    if (this.timeout !== null) {
      globalThis.clearTimeout(this.timeout);
    }
    this.timeout = globalThis.setTimeout(() => this.catchUp(false), 100);

    const flush = () => {
      if (playersChanged || this.isNew) {
        playersChannel.postMessage({
          players: Array.from(this.players).map((p) => ({ name: p })),
        });
        playersChanged = false;
      }
      if (worldChanged || this.isNew) {
        if (this.world === null) {
          worldChannel.postMessage(null);
        } else {
          worldChannel.postMessage({ name: this.world });
        }
        worldChanged = false;
      }
      this.isNew = false;
    };

    for await (const line of readStringDelim(this.file, "\r\n")) {
      // deno-lint-ignore no-regex-spaces
      const parts = line.match(
        /^\d{4}\.\d{2}\.\d{2} \d{2}:\d{2}:\d{2} .*? -  (.*)/,
      );
      if (parts === null) {
        continue;
      }
      const body = parts[1];

      const playerJoin = body.match(/^\[Behaviour\] OnPlayerJoined (.*)/);
      if (playerJoin !== null) {
        this.players.add(playerJoin[1]);
        playersChanged = true;
        continue;
      }

      const playerLeave = body.match(/^\[Behaviour\] OnPlayerLeft (.*)/);
      if (playerLeave !== null) {
        this.players.delete(playerLeave[1]);
        playersChanged = true;
        continue;
      }

      const worldEnter = body.match(/^\[Behaviour\] Entering Room: (.*)/);
      if (worldEnter !== null) {
        this.world = worldEnter[1];
        worldChanged = true;
        continue;
      }

      if (body == "[Behaviour] OnLeftRoom") {
        this.world = null;
        worldChanged = true;
        continue;
      }

      if (!suppressEvents) {
        const screenshot = body.match(
          /^\[VRC Camera\] Took screenshot to: (.*)/,
        );
        if (screenshot !== null) {
          // Make sure location information updates before taking a screenshot.
          flush();
          cameraChannel.postMessage({ path: screenshot[1] });
          continue;
        }

        const videoResolve = body.match(
          /^\[Video Playback\] Attempting to resolve URL '(.*)'$/,
        );
        if (videoResolve !== null) {
          flush();
          videoChannel.postMessage({ url: videoResolve[1] });
          continue;
        }
      }
    }

    flush();
  }

  close() {
    if (this.timeout !== null) {
      globalThis.clearTimeout(this.timeout);
    }
    this.file.close();
  }
}

globalThis.addEventListener("load", async () => {
  const watcher = Deno.watchFs(vrChatDir, { recursive: false });
  let latestLog = await getLatestLog();
  let logFile = null;
  if (latestLog !== null) {
    logFile = new Session(await Deno.open(latestLog, { read: true }));
    await logFile.catchUp(true);
  }
  for await (const change of watcher) {
    if (
      change.kind === "create" || change.kind === "remove" ||
      change.kind === "other"
    ) {
      const newLatest = await getLatestLog();
      if (newLatest !== latestLog) {
        latestLog = newLatest;
        if (logFile !== null) {
          logFile.close();
          logFile = null;
        }
        if (latestLog !== null) {
          logFile = new Session(await Deno.open(latestLog, { read: true }));
        }
      }
    }
    if (logFile !== null) {
      await logFile.catchUp(false);
    }
  }
});
