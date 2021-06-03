/// <reference lib="webworker" />

import { WorkerRequest, WorkerResponse } from "worker-swarm";

import init, { ImageOptimizer } from "./pkg/image_opt";
import { Action, Response } from "./actions";

main();

async function main() {
  await init();

  const optimizer = ImageOptimizer.new();

  self.addEventListener("message", onMessage);

  function onMessage(e: MessageEvent<WorkerRequest<Action>>) {
    switch (e.data.type) {
      case "BLUR": {
        const message: WorkerResponse<Response> = {
          jobId: e.data.jobId,
          payload: optimizer.blur(e.data.payload, 4) as Uint8Array,
        };

        self.postMessage(message);
      }

      case "GRAYSCALE": {
        const message: WorkerResponse<Response> = {
          jobId: e.data.jobId,
          payload: optimizer.grayscale(e.data.payload) as Uint8Array,
        };

        self.postMessage(message);
      }

      case "INVERT": {
        const message: WorkerResponse<Response> = {
          jobId: e.data.jobId,
          payload: optimizer.invert(e.data.payload) as Uint8Array,
        };

        self.postMessage(message);
      }

      case "THUMBNAIL": {
        const message: WorkerResponse<Response> = {
          jobId: e.data.jobId,
          payload: optimizer.thumbnail(e.data.payload, 200, 200) as Uint8Array,
        };

        self.postMessage(message);
      }
    }
  }
}
