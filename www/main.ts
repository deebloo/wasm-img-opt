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

    image.src = URL.createObjectURL(new Blob([source]));
  };

  blur.onclick = () => {
    const blob = new Blob([optimizer.blur(source, 5)]);

    image.src = URL.createObjectURL(blob);
  };

  grayscale.onclick = () => {
    const blob = new Blob([optimizer.grayscale(source)]);

    image.src = URL.createObjectURL(blob);
  };

  invert.onclick = () => {
    const blob = new Blob([optimizer.invert(source)]);

    image.src = URL.createObjectURL(blob);
  };

  thumbnail.onclick = () => {
    const blob = new Blob([optimizer.thumbnail(source, 200, 200)]);

    image.src = URL.createObjectURL(blob);
  };
}
