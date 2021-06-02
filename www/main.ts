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
    const data = optimizer.blur(source, 5);

    if (data) {
      const blob = new Blob([data]);

      image.src = URL.createObjectURL(blob);
    }
  };

  grayscale.onclick = () => {
    const data = optimizer.grayscale(source);

    if (data) {
      const blob = new Blob([data]);

      image.src = URL.createObjectURL(blob);
    }
  };

  invert.onclick = () => {
    const data = optimizer.invert(source);

    if (data) {
      const blob = new Blob([data]);

      image.src = URL.createObjectURL(blob);
    }
  };

  thumbnail.onclick = () => {
    const data = optimizer.thumbnail(source, 200, 200);

    if (data) {
      const blob = new Blob([data]);

      image.src = URL.createObjectURL(blob);
    }
  };
}
