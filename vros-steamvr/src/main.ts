import {
  readableStreamFromReader,
  toTransformStream,
  writableStreamFromWriter,
} from "https://deno.land/std@0.148.0/streams/conversion.ts";
import {
  TextLineStream,
} from "https://deno.land/std@0.148.0/streams/delimiter.ts";

const agent = Deno.run({
  cmd: ["./target/debug/vros-steamvr-agent.exe"],
  stdin: "piped",
  stdout: "piped",
});

const applicationChannel = new BroadcastChannel("steamvr.applicationName");

const readable = readableStreamFromReader(agent.stdout)
  .pipeThrough(new TextDecoderStream())
  .pipeThrough(new TextLineStream())
  .pipeThrough(toTransformStream(async function* (src) {
    for await (const line of src) {
      yield JSON.parse(line);
    }
  }));

globalThis.addEventListener("load", async () => {
  for await (const data of readable) {
    if (Object.hasOwn(data, "ApplicationName")) {
      applicationChannel.postMessage(data.ApplicationName);
    }
  }
});
