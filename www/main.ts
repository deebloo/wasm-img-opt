import { WorkerSwarm } from "worker-swarm";
import { Action, Response } from "./actions";

const image = document.getElementById("image") as HTMLImageElement;
const file = document.getElementById("file-upload") as HTMLInputElement;
const blur = document.getElementById("blur") as HTMLButtonElement;
const grayscale = document.getElementById("grayscale") as HTMLButtonElement;
const invert = document.getElementById("invert") as HTMLButtonElement;
const thumbnail = document.getElementById("thumbnail") as HTMLButtonElement;

export async function main() {
  const swarm = new WorkerSwarm<Action, Response>(
    () => new Worker("./image.worker.js", { type: "module" }),
    4
  );

  let source: Uint8Array = new Uint8Array();

  file.onchange = async () => {
    const files = file.files || [];

    source = new Uint8Array(await files[0].arrayBuffer());

    image.src = URL.createObjectURL(new Blob([source]));
  };

  blur.onclick = async () => {
    const res = await swarm.post({ type: "BLUR", payload: source });

    if (res.data.payload) {
      image.src = URL.createObjectURL(new Blob([res.data.payload]));
    }
  };

  grayscale.onclick = async () => {
    const res = await swarm.post({ type: "GRAYSCALE", payload: source });

    if (res.data.payload) {
      image.src = URL.createObjectURL(new Blob([res.data.payload]));
    }
  };

  invert.onclick = async () => {
    const res = await swarm.post({ type: "INVERT", payload: source });

    if (res.data.payload) {
      image.src = URL.createObjectURL(new Blob([res.data.payload]));
    }
  };

  thumbnail.onclick = async () => {
    const res = await swarm.post({ type: "THUMBNAIL", payload: source });

    if (res.data.payload) {
      image.src = URL.createObjectURL(new Blob([res.data.payload]));
    }
  };
}
