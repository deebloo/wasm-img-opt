import init, { ImageOptimizer } from "./pkg/image_opt";

const image = document.getElementById("image") as HTMLImageElement;
const file = document.getElementById("file-upload") as HTMLInputElement;
const blur = document.getElementById("blur") as HTMLButtonElement;
const grayscale = document.getElementById("grayscale") as HTMLButtonElement;
const invert = document.getElementById("invert") as HTMLButtonElement;
const thumbnail = document.getElementById("thumbnail") as HTMLButtonElement;

export async function main() {
  await init();

  const optimizer = ImageOptimizer.new();

  let source: Uint8Array = new Uint8Array();

  file.onchange = async () => {
    const files = file.files || [];

    source = new Uint8Array(await files[0].arrayBuffer());

    const blob = new Blob([source]);
    const url = URL.createObjectURL(blob);

    image.src = url;
  };

  blur.onclick = () => {
    const blob = new Blob([optimizer.blur(source, 5)]);
    const url = URL.createObjectURL(blob);

    image.src = url;
  };

  grayscale.onclick = () => {
    const blob = new Blob([optimizer.grayscale(source)]);
    const url = URL.createObjectURL(blob);

    image.src = url;
  };

  invert.onclick = () => {
    const blob = new Blob([optimizer.invert(source)]);
    const url = URL.createObjectURL(blob);

    image.src = url;
  };

  thumbnail.onclick = () => {
    const blob = new Blob([optimizer.thumbnail(source, 200, 200)]);
    const url = URL.createObjectURL(blob);

    image.src = url;
  };
}
